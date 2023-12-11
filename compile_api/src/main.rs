use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    routing::post,
    Router,
};
use std::{error::Error, net::Ipv4Addr, sync::Arc, time::Instant};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    process::Command,
    sync::RwLock,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{error, info, warn};

const ADDRESS: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
const PORT: u16 = 8080;
const IMAGE: &str = "liamg737/comp";
const COOLDOWN_DURATION: u64 = 10;

#[derive(Default)]
struct AppState {
    cooldown_start: RwLock<Option<Instant>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let shared_state = Arc::new(AppState::default());

    let app = Router::new()
        .route("/", post(compile))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::new().allow_origin(HeaderValue::from_static("*")))
        .with_state(shared_state);
    let listener = tokio::net::TcpListener::bind((ADDRESS, PORT))
        .await
        .unwrap();
    info!("Listening on http://{ADDRESS}:{PORT}");
    axum::serve(listener, app).await.unwrap();
}

async fn compile(
    State(state): State<Arc<AppState>>,
    request_headers: HeaderMap,
    code: String,
) -> Result<(StatusCode, HeaderMap, Vec<u8>), (StatusCode, HeaderMap, String)> {
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(internal("Failed to get time", 0))?
        .subsec_nanos();

    let mut response_headers = HeaderMap::new();
    response_headers.append("ref-code", HeaderValue::from(id));

    {
        let cooldown_read = state.cooldown_start.read().await;
        if let Some(cooldown_start) = *cooldown_read {
            if cooldown_start.elapsed().as_secs() < COOLDOWN_DURATION {
                let time_left = COOLDOWN_DURATION - cooldown_start.elapsed().as_secs();
                warn!("{id}: Rejected due to cooldown - {time_left}s remaining");
                response_headers.append(header::RETRY_AFTER, HeaderValue::from(time_left));
                return Err((
                    StatusCode::SERVICE_UNAVAILABLE,
                    response_headers,
                    format!(
                        "The server has been placed in a cooldown state due to being overloaded, try again in {time_left} secconds",
                    ),
                ));
            }
            drop(cooldown_read);
            *state.cooldown_start.write().await = None;
        }
    }

    info!("{id}: Started");
    let start = Instant::now();

    if code.is_empty() {
        info!("{id}: Rejected due to empty body");
        return Err((
            StatusCode::BAD_REQUEST,
            response_headers,
            String::from("Request must have a body"),
        ));
    }

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

    let build_start = Instant::now();
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

    if command_status.status.code() == Some(137) {
        *state.cooldown_start.write().await = Some(Instant::now());
        warn!("Server is now in a cooldown state due to being overloaded, all requests will be rejected for the next 5 seconds");
        response_headers.append(header::RETRY_AFTER, HeaderValue::from(COOLDOWN_DURATION));
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            response_headers,
            format!(
                "The server has been placed in a cooldown state due to being overloaded, try again in {COOLDOWN_DURATION} secconds"
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

    let compress = request_headers
        .get(header::ACCEPT_ENCODING)
        .is_some_and(|accept| accept.to_str().unwrap().to_lowercase().contains("gzip"));

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

    let output_file_name = if compress {
        dir.join("game_bg.wasm.gz")
    } else {
        dir.join("game_bg.wasm")
    };

    let mut output_file = File::open(output_file_name)
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
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/wasm"),
    );

    response_headers.append(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_static("inline"),
    );

    info!("{id}: Successful in {:.2?}", start.elapsed());
    Ok((StatusCode::OK, response_headers, wasm))
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
