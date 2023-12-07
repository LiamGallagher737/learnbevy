use axum::{
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};
use std::{error::Error, net::Ipv4Addr};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    process::Command,
};
use tower_http::trace::TraceLayer;
use tracing::{error, info};

const ADDRESS: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
const PORT: u16 = 8080;
const IMAGE: &str = "comp";
const ADDITIONAL_CODE: &[u8] = b"#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn bevy_playground_run_app() {
    main();
}";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new()
        .route("/", post(compile))
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind((ADDRESS, PORT))
        .await
        .unwrap();
    info!("Listening on http://{ADDRESS}:{PORT}");
    axum::serve(listener, app).await.unwrap();
}

async fn compile(headers: HeaderMap, code: String) -> Result<impl IntoResponse, impl IntoResponse> {
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(internal("Failed to get time", 0))?
        .subsec_nanos();

    info!("{id}: Started");
    let start = std::time::Instant::now();

    let mut response_headers = HeaderMap::new();
    response_headers.append(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/wasm"),
    );

    if code.is_empty() {
        info!("{id}: Denied due to empty body");
        return Err((
            StatusCode::BAD_REQUEST,
            "Request must have a body".to_string(),
        ));
    }

    let dir = std::env::temp_dir().join(".learnbevy").join(id.to_string());
    tokio::fs::create_dir_all(&dir)
        .await
        .map_err(internal("Failed to create temp dir", id))?;
    let mut file = File::create(dir.join("main.rs"))
        .await
        .map_err(internal("Failed to create code file", id))?;

    file.write_all(ADDITIONAL_CODE)
        .await
        .map_err(internal("Failed to write additional code", id))?;
    file.write_all(code.as_bytes())
        .await
        .map_err(internal("Failed to write user code", id))?;

    let build_start = std::time::Instant::now();
    let command_status = Command::new("docker")
        .args([
            "run",
            "--cpus=1",
            "--name",
            &format!("compile.{id}"),
            "-v",
            &format!("{}:/compile/src/", dir.display()),
            IMAGE,
            "sh",
            "-c",
            "sh build.sh",
        ])
        .output()
        .await
        .map_err(internal("Failed to start docker compile instance", id))?;
    info!("{id}: Built in {:.2?}", build_start.elapsed());

    match command_status.status.code() {
        Some(137) => return Err(custom_internal("Container was killed", id)),
        _ => {}
    }

    if !command_status.status.success() {
        let stdout = String::from_utf8(command_status.stdout)
            .map_err(internal("Failed to convert stdout to string", id))?;
        let stderr = String::from_utf8(command_status.stderr)
            .map_err(internal("Failed to convert stderr to string", id))?;

        if stderr.contains("docker:") {
            error!("Error with docker: {stderr}");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal Server Error"),
            ));
        }

        return Err((
            StatusCode::BAD_REQUEST,
            format!(
                "Status: {}\nStdout: {stdout}\nStderr: {stderr}",
                command_status.status
            ),
        ));
    }

    let compress = headers
        .get(header::ACCEPT_ENCODING)
        .map(|accept| accept.to_str().unwrap().to_lowercase().contains("gzip"))
        .unwrap_or(false);

    if compress {
        let compress_start = std::time::Instant::now();
        Command::new("gzip")
            .arg(dir.join("game_bg.wasm"))
            .output()
            .await
            .map_err(internal("Failed to gzip wasm", id))?;
        info!("{id}: Compressed in {:.2?}", compress_start.elapsed());
        response_headers.append(header::CONTENT_ENCODING, HeaderValue::from_static("gzip"));
    }

    let output_file = if !compress {
        dir.join("game_bg.wasm")
    } else {
        dir.join("game_bg.wasm.gz")
    };

    let mut file = File::open(output_file)
        .await
        .map_err(internal("Failed to open final wasm", id))?;
    let mut wasm = Vec::with_capacity(
        file.metadata()
            .await
            .map_err(internal("Failed to get wasm file metadata", id))?
            .len() as usize,
    );
    file.read_to_end(&mut wasm).await.unwrap();

    if let Err(e) = Command::new("docker")
        .args(["container", "rm", &format!("compile.{id}")])
        .output()
        .await
    {
        error!("{id}: Failed to remove container: {e:?}");
    }

    if let Err(e) = tokio::fs::remove_dir_all(dir).await {
        error!("{id}: Failed to remove temp dir: {e:?}");
    }

    info!("{id}: Successful in {:.2?}", start.elapsed());
    Ok((StatusCode::OK, response_headers, wasm))
}

/// Helper for maping errors to internal server errors while also logging the error
fn internal<E: Error>(msg: &str, id: u32) -> impl Fn(E) -> (StatusCode, String) + '_ {
    move |e: E| {
        error!("{id}: {msg}: {e:?}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from(format!("Internal Server Error\nReference Code: {id}")),
        )
    }
}

/// Helper for creating a 500 error
fn custom_internal(msg: &str, id: u32) -> (StatusCode, String) {
    error!("{id}: {msg}");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        String::from(format!("Internal Server Error\nReference Code: {id}")),
    )
}
