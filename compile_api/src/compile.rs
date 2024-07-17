use crate::{cache::CacheEntry, config, metrics::count_request, Error, Id, Input};
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

    let output = Command::new("podman")
        .args([
            "run",
            "--name",
            &name_id,
            "-v",
            &format!("{}:/playground/src/:z", dir.display()),
            "--quiet",
            "--pull",
            "never",
            &config::image_for_config(*version, *channel),
        ])
        .output()
        .await?;

    // 101 is the rust compilers status code for failed to build (user error)
    if output.status.code() == Some(101) {
        info!("{id}: Build failed (user error)");
        count_request("user_error");
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
    let js = fs::read_to_string(dir.join("game.js")).await?;
    let modified_js = modify_js(js);

    let mut stderr = output.stderr;

    let wasm_length = wasm.len();
    let js_length = modified_js.len();

    let mut body = wasm;
    body.extend_from_slice(&mut modified_js.as_bytes());
    body.append(&mut stderr);

    let mut encoder = GzEncoder::new(Vec::with_capacity(body.len() / 3), Compression::new(2));
    encoder.write_all(&body[..]).unwrap();
    let compressed_body = encoder.finish().unwrap();

    count_request("successful");

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

/// Deletes the temp directory and container once the request has completed.
pub async fn cleanup(id: usize) {
    let name_id = name_id(id);
    let _ = fs::remove_dir_all(temp_dir(&name_id)).await;
    let _ = Command::new("podman")
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

fn modify_js(mut js: String) -> String {
    // The space after export is very important as some function names contain "export"
    js = js.replace("export ", "").replace("import.meta.url", "");
    // Remove the last two lines that break things
    js.truncate(js.len() - 35);
    js.push_str(include_str!("extra.js"));
    js
}
