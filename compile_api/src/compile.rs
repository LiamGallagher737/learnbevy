use crate::{cache::CacheEntry, config, Error, Id, Input};
use async_std::{fs, process::Command};
use flate2::{write::GzEncoder, Compression};
use log::info;
use std::{env, io::Write as _, path::PathBuf};
use tide::{
    http::{
        headers::{CONTENT_ENCODING, CONTENT_TYPE},
        mime::WASM,
    },
    Body, Request, Response, StatusCode,
};

/// The function that handles a compile.
pub async fn compile(request: Request<()>) -> Result<Response, tide::Error> {
    let Input {
        code,
        version,
        channel,
    } = request.ext().unwrap();
    let Id(id) = request.ext().unwrap();
    let name_id = name_id(*id);

    let modified_code = config::edit_code_for_version(code, *version);

    let dir = temp_dir(&name_id);
    fs::create_dir_all(&dir).await?;
    fs::write(dir.join("main.rs"), modified_code).await?;

    let output = Command::new("docker")
        .args([
            "run",
            "--name",
            &name_id,
            "-v",
            &format!("{}:/playground/src/", dir.display()),
            &config::image_for_config(*version, *channel),
        ])
        .output()
        .await?;

    // 101 is the rust compilers status code for failed to build (user error)
    if output.status.code() == Some(101) {
        info!("{id}: Build failed (user error)");
        return Ok(Response::builder(StatusCode::BadRequest)
            .body(Body::from_json(&Error::BuildFailed {
                stderr: String::from_utf8(output.stderr)
                    .unwrap_or("Contained invalid utf8".to_owned()),
            })?)
            .build());
    }

    if !output.status.success() {
        let stderr =
            String::from_utf8(output.stderr).unwrap_or("Contained invalid utf8".to_string());
        let message = format!("Build failed with {}. Stderr: {stderr}", output.status);
        return Err(tide::Error::from_str(
            StatusCode::InternalServerError,
            message,
        ));
    }

    let wasm = fs::read(dir.join("game_bg.wasm")).await?;
    let js = fs::read(dir.join("game.js")).await?;
    let mut modified_js = modify_js(js);

    let mut stderr = output.stderr;

    let wasm_length = wasm.len();
    let js_length = modified_js.len();

    let mut body = wasm;
    body.append(&mut modified_js);
    body.append(&mut stderr);

    let mut encoder = GzEncoder::new(Vec::with_capacity(body.len() / 3), Compression::new(2));
    encoder.write_all(&body[..]).unwrap();
    let compressed_body = encoder.finish().unwrap();

    Ok({
        let mut response = Response::new(StatusCode::Ok);
        response.set_body(compressed_body.clone());
        response.insert_header("wasm-content-length", wasm_length.to_string());
        response.insert_header("js-content-length", js_length.to_string());
        response.insert_header(CONTENT_TYPE, WASM);
        response.insert_header(CONTENT_ENCODING, "gzip");
        response.insert_ext(CacheEntry {
            wasm_length,
            js_length,
            body: compressed_body,
        });
        response
    })
}

/// Deletes the temp directory and docker container once the request has completed.
pub async fn cleanup(id: usize) {
    let name_id = name_id(id);
    let _ = fs::remove_dir_all(temp_dir(&name_id)).await;
    let _ = Command::new("docker")
        .args(["container", "rm", &name_id])
        .output()
        .await;
}

fn name_id(id: usize) -> String {
    format!("bca.{id}")
}

fn temp_dir(name: &str) -> PathBuf {
    env::temp_dir().join("bca").join(name)
}

/// Any "export" tokens will cause the code to be invalid on the frontend as it's run as a function
/// and JavaScript functions cannot contain exports. This removes the export token for the __exit()
/// method
fn modify_js(mut js: Vec<u8>) -> Vec<u8> {
    // Remove "export" from "export function __exit()"
    let search_bytes = b"export function __exit()";
    let mut seen = 0;
    let mut n = 0;
    for byte in &js {
        if *byte == search_bytes[seen] {
            seen += 1;
        } else {
            seen = 0;
        }
        if seen == search_bytes.len() {
            break;
        }
        n += 1;
    }
    js.drain(n - seen + 1..n - seen + 7);
    // Remove two last lines of exports
    js.resize(js.len() - 47, 0);
    // Remove "import.meta.url" as it's not allowed outside a js module
    js.drain(js.len() - 403 - 17..js.len() - 403);
    // Add on the extra js
    js.append(&mut include_bytes!("extra.js").to_vec());
    js
}
