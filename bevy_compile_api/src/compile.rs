use std::env;

use serde::Deserialize;
use tokio::{fs, process::Command};
use tracing::{error, info};

use crate::{
    config::{self, Channel, Version},
    Error,
};

#[derive(Deserialize)]
pub struct Input {
    pub code: String,
    pub channel: Channel,
    pub version: Version,
}

pub async fn compile(id: u64, input: Input) -> Result<impl warp::Reply, warp::Rejection> {
    let id_str = id.to_string();
    let modified_code = config::edit_code_for_version(&input.code, input.version);

    let dir = env::temp_dir().join("bevy_compile_api").join(&id_str);
    fs::create_dir_all(&dir).await.unwrap();
    fs::write(dir.join("main.rs"), modified_code).await.unwrap();

    let output = Command::new("docker")
        .args([
            "run",
            "--name",
            &id_str,
            "-v",
            &format!("{}:/playground/src/", dir.display()),
            &config::image_for_config(input.version, input.channel),
        ])
        .output()
        .await
        .unwrap();

    // 101 is the rust compilers status code for failed to build (user error)
    if output.status.code() == Some(101) {
        info!("{id}: Build failed (user error)");
        let stderr = String::from_utf8(output.stderr).unwrap_or("Invalid utf8".to_string());
        return Err(Error::BuildFailed { stderr }.into());
    }

    if !output.status.success() {
        let stderr =
            String::from_utf8(output.stderr).unwrap_or("Invalid utf8".to_string());
        error!(
            "{id}: Build failed with {}. Stdeer: {stderr}",
            output.status
        );
        return Err(Error::Internal.into());
    }

    let wasm = fs::read(dir.join("game_bg.wasm")).await.unwrap();
    let js = fs::read(dir.join("game.js")).await.unwrap();
    let mut modified_js = modify_js(js);

    let mut stderr = output.stderr;

    let wasm_length = wasm.len();
    let js_length = modified_js.len();

    let mut body = wasm;
    body.append(&mut modified_js);
    body.append(&mut stderr);

    let response = warp::http::Response::builder()
        .status(200)
        .header("wasm-content-length", wasm_length)
        .header("js-content-length", js_length)
        .header("content-type", "application/wasm")
        .body(body);
    Ok(response)
}

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
