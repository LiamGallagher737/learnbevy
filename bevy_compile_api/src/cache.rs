use crate::{compile::Lengths, Id, MinifiedHash};
use async_std::{fs, stream::StreamExt};
use log::{error, info, warn};
use std::{future::Future, pin::Pin};
use tide::{
    http::{
        headers::{CONTENT_ENCODING, CONTENT_TYPE},
        mime::WASM,
    },
    Body, Next, Request, Response, Result, StatusCode,
};

#[cfg(target_os = "linux")]
const CACHE_FOLDER_PATH: &str = "/bca_cache";
#[cfg(not(target_os = "linux"))]
const CACHE_FOLDER_PATH: &str = "cache";

pub async fn setup() {
    fs::create_dir_all(CACHE_FOLDER_PATH)
        .await
        .expect("Failed to create log folder");
}

pub fn cache_middleware<'a>(
    request: Request<()>,
    next: Next<'a, ()>,
) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        let Id(id) = request.ext().unwrap();
        let MinifiedHash(Some(hash)) = request.ext().cloned().unwrap() else {
            return Ok(next.run(request).await);
        };

        let cache_bypass = request.header("cache-bypass").map(|v| v.as_str())
            == Some(include_str!("cache-bypass.token"));

        if !cache_bypass {
            if let Ok(Some(cache)) = get_cache(hash).await {
                info!("{id}: Responded with cache");
                return Ok(Response::builder(StatusCode::Ok)
                    .body(Body::from_bytes(cache.body))
                    .header("wasm-content-length", cache.wasm_length.to_string())
                    .header("js-content-length", cache.js_length.to_string())
                    .header(CONTENT_ENCODING, "gzip")
                    .header(CONTENT_TYPE, WASM)
                    .header("origin-cache-status", "HIT")
                    .build());
            }
        } else {
            warn!("{id}: Bypassed cache");
        }

        let mut response = next.run(request).await;

        // Only cache successful and compressed responses
        if response.status() != StatusCode::Ok
            && response.header(CONTENT_ENCODING).map(|v| v.as_str()) == Some("gzip")
        {
            return Ok(response);
        }

        let Some(Lengths {
            wasm_length,
            js_length,
        }) = response.ext().cloned()
        else {
            return Ok(response);
        };

        let body = response.take_body();
        let bytes = body.into_bytes().await.unwrap();
        insert_cache(
            hash,
            CacheEntry {
                wasm_length,
                js_length,
                body: bytes.clone(),
            },
        )
        .await;
        response.set_body(Body::from_bytes(bytes));
        response.insert_header(
            "origin-cache-status",
            if !cache_bypass { "MISS" } else { "BYPASS" },
        );

        Ok(response)
    })
}

pub struct CacheEntry {
    pub wasm_length: usize,
    pub js_length: usize,
    pub body: Vec<u8>,
}

async fn get_cache(hash: u128) -> Result<Option<CacheEntry>> {
    let hash_string = hash.to_string();
    let mut entries = fs::read_dir(CACHE_FOLDER_PATH).await?;
    while let Some(res) = entries.next().await {
        let entry = res?;
        if !entry.file_name().eq_ignore_ascii_case(&hash_string) {
            continue;
        }
        let mut bytes = fs::read(entry.path()).await?;
        let len = bytes.len();
        let wasm_len = usize::from_be_bytes(bytes[len - 16..len - 8].try_into().unwrap());
        let js_len = usize::from_be_bytes(bytes[len - 8..len].try_into().unwrap());
        bytes.resize(len - 16, 0);
        return Ok(Some(CacheEntry {
            wasm_length: wasm_len,
            js_length: js_len,
            body: bytes,
        }));
    }
    Ok(None)
}

async fn insert_cache(hash: u128, entry: CacheEntry) {
    let mut data = entry.body;
    data.append(&mut entry.wasm_length.to_be_bytes().to_vec());
    data.append(&mut entry.js_length.to_be_bytes().to_vec());
    if let Err(err) = fs::write(format!("{CACHE_FOLDER_PATH}/{hash}"), data).await {
        error!("Failed to write cache entry to disk: {err:?}");
    }
}
