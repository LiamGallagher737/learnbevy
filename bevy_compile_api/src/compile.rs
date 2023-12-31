use crate::{
    cache::{self, CacheEntry},
    Error, COMPRESSION_LEVEL, IMAGE,
};
use flate2::write::GzEncoder;
use log::{debug, error, info};
use rouille::{Request, Response};
use scopeguard::defer;
use std::{env, fs, io::Write, process::Command};

const EXTRA_RUST: &str = r#"
static __EXIT_FLAG: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn __exit() {
    __EXIT_FLAG.store(true, std::sync::atomic::Ordering::Relaxed);
}
fn __check_exit_flag(mut exit: bevy::ecs::event::EventWriter<bevy::app::AppExit>) {
    if __EXIT_FLAG.load(std::sync::atomic::Ordering::Relaxed) {
        exit.send(bevy::app::AppExit);
    }
}"#;

const DISALLOWED_WORDS: &[&str] = &[
    "include!",
    "include_str!",
    "include_bytes!",
    "embedded_asset!",
    "embedded_path",
    "load_internal_asset",
    "load_internal_binary_asset",
];

pub fn compile(id: usize, request: &Request) -> Response {
    let docker_container_id = format!("compile.{id}");
    let e500 = Response::json(&Error::Internal)
        .with_status_code(500)
        .with_additional_header("reference-number", id.to_string());

    let mut encoder = GzEncoder::new(Vec::new(), COMPRESSION_LEVEL);

    let Ok(mut code) = rouille::input::plain_text_body(request) else {
        info!("{id}: Rejected for invalid body");
        return Response::json(&Error::InvalidBody).with_status_code(400);
    };

    for word in DISALLOWED_WORDS {
        if code.contains(word) {
            info!("{id}: Rejected for containing disallowed word {word:?}");
            return Response::json(&Error::DisallowedWord { word }).with_status_code(400);
        }
    }

    // Check cache
    let minified_code = rust_minify::minify(&code).ok();
    let hash = minified_code.map(|code| fastmurmur3::hash(code.as_bytes()));
    if let Some(cache) = hash.map(|hash| cache::get(hash)).flatten() {
        info!("{id}: Hit cache");
        return Response::from_data("application/octet-stream", cache.body)
            .with_additional_header("reference-number", id.to_string())
            .with_additional_header("wasm-content-length", cache.wasm_len.to_string())
            .with_additional_header("js-content-length", cache.js_len.to_string())
            .with_additional_header("origin-cache-status", "HIT")
            .with_additional_header("content-encoding", "gzip");
    }

    code = code.replace(
        "App::new()",
        "App::new().add_systems(Update, __check_exit_flag)",
    );

    let dir = env::temp_dir()
        .join("bevy_compile_api")
        .join(id.to_string());
    if let Err(err) = fs::create_dir_all(&dir) {
        error!("{id}: Failed to create tempdir: {err:?}");
        return e500;
    }

    defer! {
        // This is cleanup so we don't return 500 on an error
        if let Err(err) = fs::remove_dir_all(&dir) {
            error!("{id}: Failed to remove tempdir: {err:?}");
        }
    }

    code.push_str(EXTRA_RUST);
    if let Err(err) = fs::write(dir.join("main.rs"), code) {
        error!("{id}: Failed to write main.rs to tempdir: {err:?}");
        return e500;
    }

    let command_output = Command::new("docker")
        .args([
            "run",
            "--name",
            &docker_container_id,
            "-v",
            &format!("{}:/compile/src/", dir.display()),
            IMAGE,
            "sh",
            "build.sh",
        ])
        .output();

    defer! {
        // This is cleanup so we don't return 500 on an error
        if let Err(err) = Command::new("docker")
            .args(["container", "rm", &docker_container_id])
            .output()
        {
            error!("{id}: Failed to remove container: {err:?}");
        }
    }

    let output = match command_output {
        Ok(output) => output,
        Err(err) => {
            error!("{id}: Failed to execute docker process: {err:?}");
            return e500;
        }
    };

    // 101 is the rust compilers status code for failed to build (user error)
    if output.status.code() == Some(101) {
        info!("{id}: Build failed (user error)");
        return Response::json(&Error::BuildFailed {
            stdout: String::from_utf8(output.stdout).unwrap_or("Contained invalid utf8".to_owned()),
            stderr: String::from_utf8(output.stderr).unwrap_or("Contained invalid utf8".to_owned()),
        })
        .with_status_code(400);
    }

    // 127 is the docker status code when the container was killed due to insufficient memory (server error)
    if output.status.code() == Some(137) {
        error!("{id}: Container was killed due to insufficient memory");
        return Response::json(&Error::Overloaded).with_status_code(500);
    }

    if !output.status.success() {
        error!(
            "{id}: Build failed with code {} (server error)",
            output.status
        );
        let stderr =
            String::from_utf8(output.stderr).unwrap_or("Contained invalid utf8".to_string());
        debug!("{id}: Stderr: {stderr}");
        return e500;
    }

    // Get WASM
    let wasm = match fs::read(dir.join("game_bg.wasm")) {
        Ok(wasm) => wasm,
        Err(err) => {
            error!("{id}: Failed to read game_bg.wasm: {err:?}");
            return e500;
        }
    };

    // Get JS
    let mut js = match fs::read(dir.join("game.js")) {
        Ok(js) => js,
        Err(err) => {
            error!("{id}: Failed to read game.js: {err:?}");
            return e500;
        }
    };

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

    // Get stderr
    let mut stderr = output.stderr;

    let wasm_len = wasm.len();
    let js_len = js.len();

    let mut body = wasm;
    body.append(&mut js);
    body.append(&mut stderr);

    encoder.write_all(&body).unwrap();
    body = encoder.finish().unwrap();

    if let Some(hash) = hash {
        cache::insert(
            hash,
            CacheEntry {
                wasm_len,
                js_len,
                body: body.clone(),
            },
        );
    }

    Response::from_data("application/octet-stream", body)
        .with_additional_header("reference-number", id.to_string())
        .with_additional_header("wasm-content-length", wasm_len.to_string())
        .with_additional_header("js-content-length", js_len.to_string())
        .with_additional_header("origin-cache-status", "MISS")
        .with_additional_header("content-encoding", "gzip")
}
