use config::{Channel, Version};
use log::error;
use serde::{Deserialize, Serialize};
use std::{future::Future, net::IpAddr, pin::Pin, str::FromStr};
use tide::{http::headers::HeaderValue, utils::After, Body, Next, Request, Response, StatusCode};
use tide_rustls::TlsListener;

mod cache;
mod compile;
mod config;
mod ip_lock;
mod logging;
mod rate_limiting;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    cache::setup().await;
    logging::setup().await;
    let mut app = tide::new();

    app.with(
        tide::security::CorsMiddleware::new()
            .allow_methods(HeaderValue::from_str("POST").unwrap())
            .allow_headers(HeaderValue::from_str("content-type").unwrap())
            .expose_headers(
                HeaderValue::from_str("wasm-content-length, js-content-length").unwrap(),
            ),
    );
    app.with(peer_addr_middleware);
    app.with(rate_limiting::RateLimitMiddleware::new());
    app.with(ip_lock::IpLockMiddleware::new());
    app.with(id_middleware);
    app.with(logging::logging_middleware);
    app.with(input_middleware);
    app.with(disallowed_words_middleware);
    app.with(hash_middleware);
    app.with(cache::cache_middleware);
    app.with(After(|response: Response| async move {
        let Id(id) = response.ext().unwrap();
        compile::cleanup(*id).await;
        Ok(response)
    }));
    app.with(After(|mut response: Response| async {
        let Id(id) = response.ext().unwrap();
        if let Some(err) = response.error() {
            error!("{id}: Failed with error: {err:?}");
            response.set_body(Body::from_json(&Error::Internal)?);
        }
        Ok(response)
    }));
    app.with(transfer_id_middleware);

    app.at("/compile").post(compile::compile);
    app.listen(
        TlsListener::build()
            .addrs("0.0.0.0:53740")
            .cert("./cert.pem")
            .key("./cert.key"),
    )
    .await?;

    Ok(())
}

#[derive(Clone)]
struct PeerAddr(IpAddr);
fn peer_addr_middleware<'a>(
    mut request: Request<()>,
    next: Next<'a, ()>,
) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        let ip = if !cfg!(feature = "dev-mode") {
            request
                .header("CF-Connecting-IP")
                .and_then(|addr| addr.as_str().parse::<IpAddr>().ok())
                .ok_or(tide::Error::from_str(
                    StatusCode::BadRequest,
                    "Could not get peer address",
                ))?
        } else {
            "1.1.1.1".parse::<IpAddr>().unwrap()
        };
        request.set_ext(PeerAddr(ip));
        Ok(next.run(request).await)
    })
}

#[derive(Clone, Copy)]
struct Id(usize);
fn id_middleware<'a>(
    mut request: Request<()>,
    next: Next<'a, ()>,
) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        let id = fastrand::usize(..);
        request.set_ext(Id(id));
        let mut response = next.run(request).await;
        response.insert_header("refrence-number", id.to_string());
        Ok(response)
    })
}

fn transfer_id_middleware<'a>(
    request: Request<()>,
    next: Next<'a, ()>,
) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        let id = *request.ext::<Id>().unwrap();
        let mut response = next.run(request).await;
        response.insert_ext(id);
        Ok(response)
    })
}

#[derive(Deserialize)]
struct Input {
    code: String,
    #[serde(default)]
    version: Version,
    #[serde(default)]
    channel: Channel,
}
fn input_middleware<'a>(
    mut request: Request<()>,
    next: Next<'a, ()>,
) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        let input: Input = request.body_json().await?;
        request.set_ext(input);
        Ok(next.run(request).await)
    })
}

fn disallowed_words_middleware<'a>(
    request: Request<()>,
    next: Next<'a, ()>,
) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        let input = request.ext::<Input>().unwrap();
        for word in DISALLOWED_WORDS {
            if input.code.contains(word) {
                return Ok(Response::builder(StatusCode::BadRequest)
                    .body(Body::from_json(&Error::DisallowedWord { word })?)
                    .build());
            }
        }
        Ok(next.run(request).await)
    })
}

#[derive(Clone)]
struct MinifiedHash(Option<u128>);
fn hash_middleware<'a>(
    mut request: Request<()>,
    next: Next<'a, ()>,
) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        let Input {
            code,
            version,
            channel,
        } = request.ext().unwrap();
        let hash = rust_minify::minify(code).ok().map(|code| {
            let mut hash = fastmurmur3::hash(code.as_bytes());
            hash += *version as u128;
            hash = hash.rotate_left(16);
            hash += *channel as u128;
            hash
        });
        request.set_ext(MinifiedHash(hash));
        let response = next.run(request).await;
        Ok(response)
    })
}

#[derive(Serialize)]
#[serde(tag = "kind")]
enum Error {
    RateLimit {
        time_left: f32,
    },
    #[allow(dead_code)]
    CFRateLimit,
    ActiveRequestExists,
    DisallowedWord {
        word: &'static str,
    },
    BuildFailed {
        stderr: String,
    },
    Internal,
}

const DISALLOWED_WORDS: &[&str] = &[
    "include!",
    "include_str!",
    "include_bytes!",
    "embedded_asset!",
    "embedded_path",
    "load_internal_asset",
    "load_internal_binary_asset",
];
