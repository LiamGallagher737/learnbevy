use axum::{
    http::{header, HeaderMap, HeaderName, HeaderValue, StatusCode},
    routing::post,
    Router,
};
use std::{error::Error, io, net::Ipv4Addr, time::Instant};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    process::Command,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{error, info};

const ADDRESS: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
const PORT: u16 = 8080;
const IMAGE: &str = "liamg737/comp";

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let app = Router::new()
        .route("/", post(compile))
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(HeaderValue::from_static("*"))
                .expose_headers(Any),
        );

    let listener = tokio::net::TcpListener::bind((ADDRESS, PORT)).await?;

    info!("Listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn compile(
    code: String,
) -> Result<(StatusCode, HeaderMap, Vec<u8>), (StatusCode, HeaderMap, String)> {
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(internal("Failed to get time", 0))?
        .subsec_nanos();

    let mut response_headers = HeaderMap::new();
    response_headers.append("reference-code", HeaderValue::from(id));

    if code.is_empty() {
        info!("{id}: Rejected due to empty body");
        return Err((
            StatusCode::BAD_REQUEST,
            response_headers,
            String::from("Request must have a body"),
        ));
    }

    info!("{id}: Started");
    let start = Instant::now();

    let dir = std::env::temp_dir().join(".learnbevy").join(id.to_string());
    tokio::fs::create_dir_all(&dir)
        .await
        .map_err(internal("Failed to create temp dir", id))?;
    let mut code_file = File::create(dir.join("main.rs"))
        .await
        .map_err(internal("Failed to create code file", id))?;

    code_file
        .write_all(code.as_bytes())
        .await
        .map_err(internal("Failed to write user code", id))?;

    let command_status = Command::new("docker")
        .args([
            "run",
            // "--cpus=1",
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

    if command_status.status.code() == Some(137) {
        error!("A docker compile instance returned a 137 status");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            response_headers,
            String::from(
                "The server was unable to process request, possibly due to being overloaded",
            ),
        ));
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
                response_headers,
                String::from("Internal Server Error"),
            ));
        }

        return Err((
            StatusCode::BAD_REQUEST,
            response_headers,
            format!(
                "Status: {}\nStdout: {stdout}\nStderr: {stderr}",
                command_status.status
            ),
        ));
    }

    let mut output_file = File::open(dir.join("game_bg.wasm"))
        .await
        .map_err(internal("Failed to open final wasm", id))?;
    let mut wasm = Vec::with_capacity(
        output_file
            .metadata()
            .await
            .map_err(internal("Failed to get wasm file metadata", id))?
            .len() as usize,
    );
    output_file
        .read_to_end(&mut wasm)
        .await
        .map_err(internal("Failed to read final wasm", id))?;

    let mut output_file = File::open(dir.join("game.js"))
        .await
        .map_err(internal("Failed to open final js", id))?;
    let mut js = Vec::with_capacity(
        output_file
            .metadata()
            .await
            .map_err(internal("Failed to get js file metadata", id))?
            .len() as usize,
    );
    output_file
        .read_to_end(&mut js)
        .await
        .map_err(internal("Failed to read final js", id))?;
    js.resize(js.len() - 47, 0);
    js.drain(js.len() - 403 - 17..js.len() - 403);
    js.append(&mut include_bytes!("extra.js").to_vec());

    if let Err(err) = Command::new("docker")
        .args(["container", "rm", &format!("compile.{id}")])
        .output()
        .await
    {
        error!("{id}: Failed to remove container: {err:?}");
    }

    if let Err(err) = tokio::fs::remove_dir_all(dir).await {
        error!("{id}: Failed to remove temp dir: {err:?}");
    }

    response_headers.append(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_static("inline"),
    );

    response_headers.append(
        HeaderName::from_static("wasm-content-length"),
        HeaderValue::from_str(&wasm.len().to_string()).unwrap(),
    );

    response_headers.append(
        HeaderName::from_static("js-content-length"),
        HeaderValue::from_str(&js.len().to_string()).unwrap(),
    );

    let mut response_body = wasm;
    response_body.append(&mut js);

    info!("{id}: Successful in {:.2?}", start.elapsed());
    Ok((StatusCode::OK, response_headers, response_body))
}

/// Helper for maping errors to internal server errors while also logging the error
fn internal<E: Error>(msg: &str, id: u32) -> impl Fn(E) -> (StatusCode, HeaderMap, String) + '_ {
    move |err: E| {
        let mut headers = HeaderMap::new();
        headers.append("ref-code", HeaderValue::from(id));
        error!("{id}: {msg}: {err:?}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            headers,
            format!("Internal Server Error\nReference Code: {id}"),
        )
    }
}
