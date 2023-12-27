use chrono::{DateTime, Utc};
use log::{error, info};
use rouille::{Request, Response};
use scopeguard::defer;
use serde::Serialize;
use std::{
    env, fs,
    process::Command,
    time::{Instant, SystemTime},
};

const ADDRESS: &str = "0.0.0.0:8080";
const IMAGE: &str = "liamg737/comp";

#[cfg(target_os = "linux")]
const LOG_FOLDER_PATH: &str = "/var/log/bevy_compiler_api";
#[cfg(not(target_os = "linux"))]
const LOG_FOLDER_PATH: &str = "logs";
const LOG_FILE_PREFIX: &str = "bevy_compiler_api.log.";

fn main() {
    fs::create_dir_all(LOG_FOLDER_PATH).expect("Failed to create log folder");
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                DateTime::<Utc>::from(SystemTime::now()).format("%H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(
            fern::DateBased::new(format!("{LOG_FOLDER_PATH}/{LOG_FILE_PREFIX}"), "%Y-%m-%d")
                .utc_time(),
        )
        .chain(std::io::stdout())
        .apply()
        .expect("Failed to setup logging");

    rouille::start_server(ADDRESS, move |request| {
        if request.raw_url() != "/" {
            Response::empty_404()
        } else if request.method() != "POST" {
            Response::text("Only the POST method is allowed")
                .with_status_code(405)
                .with_additional_header("Allow", "POST")
        } else {
            let id = fastrand::usize(..);
            info!("{id}: Serving new request from {}", request.remote_addr());
            let start = Instant::now();

            let response = compile(id, request)
                .with_additional_header("access-control-allow-origin", "*")
                .with_additional_header("access-control-expose-headers", "*");

            info!("{id}: Finished in {:.2?}", start.elapsed());
            response
        }
    });
}

fn compile(id: usize, request: &Request) -> Response {
    let docker_container_id = format!("compile.{id}");
    let e500 = Response::text("Internal Server Error")
        .with_status_code(500)
        .with_additional_header("reference-number", id.to_string());

    let Ok(body) = rouille::input::plain_text_body(&request) else {
        info!("{id}: Rejected for invalid body");
        return Response::json(&OtherUserError {
            msg: "Body must be plain utf8 text",
        })
        .with_status_code(400);
    };

    let dir = env::temp_dir()
        .join("bevy_compile_api")
        .join(&id.to_string());
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

    if let Err(err) = fs::write(dir.join("main.rs"), body) {
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
        return Response::json(&BuildError {
            msg: "Error building game",
            stdout: String::from_utf8(output.stdout).unwrap_or("Contained invalid utf8".to_owned()),
            stderr: String::from_utf8(output.stderr).unwrap_or("Contained invalid utf8".to_owned()),
        })
        .with_status_code(400);
    }

    if !output.status.success() {
        error!(
            "{id}: Build failed with code {} (server error)",
            output.status
        );
        return e500;
    }

    let wasm = match fs::read(dir.join("game_bg.wasm")) {
        Ok(wasm) => wasm,
        Err(err) => {
            error!("{id}: Failed to read game_bg.wasm: {err:?}");
            return e500;
        }
    };

    let mut js = match fs::read(dir.join("game.js")) {
        Ok(js) => js,
        Err(err) => {
            error!("{id}: Failed to read game.js: {err:?}");
            return e500;
        }
    };
    // Remove two last lines of exports
    js.resize(js.len() - 47, 0);
    // Remove "import.meta.url" as it's not allowed outside a js module
    js.drain(js.len() - 403 - 17..js.len() - 403);
    // Add on the extra js
    js.append(&mut include_bytes!("extra.js").to_vec());

    let wasm_len = wasm.len();
    let js_len = js.len();

    let mut body = wasm;
    body.append(&mut js);

    Response::from_data("application/octet-stream", body)
        .with_additional_header("reference-number", id.to_string())
        .with_additional_header("wasm-content-length", wasm_len.to_string())
        .with_additional_header("js-content-length", js_len.to_string())
}

#[derive(Serialize)]
struct BuildError {
    msg: &'static str,
    stdout: String,
    stderr: String,
}

#[derive(Serialize)]
struct OtherUserError {
    msg: &'static str,
}
