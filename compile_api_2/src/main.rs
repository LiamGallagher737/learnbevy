use rouille::{Request, Response};
use serde::Serialize;
use std::{fs, process::Command, time::Instant};

const ADDRESS: &str = "0.0.0.0:8080";
const IMAGE: &str = "liamg737/comp";

fn main() {
    println!("Listening on {ADDRESS}");

    rouille::start_server(ADDRESS, move |request| {
        if request.raw_url() != "/" {
            Response::empty_404()
        } else if request.method() != "POST" {
            Response::text("Only the POST method is allowed")
                .with_status_code(405)
                .with_additional_header("Allow", "POST")
        } else {
            let id = fastrand::usize(..);
            println!("{id}: Serving new request from {}", request.remote_addr());
            let start = Instant::now();

            let response = compile(id, request)
                .with_additional_header("access-control-allow-origin", "*")
                .with_additional_header("access-control-expose-headers", "*");

            println!("{id}: Finished in {:.2?}", start.elapsed());
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
        return Response::text("Body must be plain text").with_status_code(400);
    };

    let Ok(dir) = tempfile::tempdir() else {
        eprintln!("{id}: Failed to create tempdir");
        return e500;
    };

    if fs::write(dir.path().join("main.rs"), body).is_err() {
        eprintln!("{id}: Failed to write main.rs to tempdir");
        return e500;
    }

    let command_output = Command::new("docker")
        .args([
            "run",
            "--name",
            &docker_container_id,
            "-v",
            &format!("{}:/compile/src/", dir.path().display()),
            IMAGE,
            "sh",
            "build.sh",
        ])
        .output();

    let Ok(output) = command_output else {
        eprintln!("{id}: Failed to execute docker process");
        return e500;
    };

    // 101 is the rust compilers status code for failed to build (user error)
    if output.status.code() == Some(101) {
        println!("{id}: Build failed (user error)");
        return Response::json(&BuildError {
            msg: "Error building game",
            stdout: String::from_utf8(output.stdout).unwrap_or("Contained invalid utf8".to_owned()),
            stderr: String::from_utf8(output.stderr).unwrap_or("Contained invalid utf8".to_owned()),
        });
    }

    if !output.status.success() {
        eprintln!(
            "{id}: Build failed with code {} (server error)",
            output.status
        );
        return e500;
    }

    let Ok(wasm) = fs::read(dir.path().join("game_bg.wasm")) else {
        eprintln!("{id}: Failed to read game_bg.wasm");
        return e500;
    };

    let Ok(mut js) = fs::read(dir.path().join("game.js")) else {
        eprintln!("{id}: Failed to read game.js");
        return e500;
    };
    // Remove two last lines of exports
    js.resize(js.len() - 47, 0);
    // Remove "import.meta.url" as it's not allowed outside a js module
    js.drain(js.len() - 403 - 17..js.len() - 403);
    // Add on the extra js
    js.append(&mut include_bytes!("extra.js").to_vec());

    // Cleanup (Do NOT return 500 if there is an error)
    if let Err(err) = Command::new("docker")
        .args(["container", "rm", &docker_container_id])
        .output()
    {
        eprintln!("{id}: Failed to remove container: {err:?}");
    }
    if let Err(err) = dir.close() {
        eprintln!("{id}: Failed to remove tempdir: {err:?}");
    }

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
