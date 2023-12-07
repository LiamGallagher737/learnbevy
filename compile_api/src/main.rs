use axum::{
    http::{header, StatusCode},
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
        .with_max_level(tracing::Level::INFO)
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

async fn compile(code: String) -> Result<impl IntoResponse, impl IntoResponse> {
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(internal("Failed to get time"))?
        .subsec_nanos();

    info!("{id}: Started");
    let start = std::time::Instant::now();

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
        .map_err(internal("Failed to create temp dir"))?;
    let mut file = File::create(dir.join("main.rs"))
        .await
        .map_err(internal("Failed to create code file"))?;

    file.write_all(ADDITIONAL_CODE)
        .await
        .map_err(internal("Failed to write additional code"))?;
    file.write_all(code.as_bytes())
        .await
        .map_err(internal("Failed to write user code"))?;

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
        .map_err(internal("Failed to start docker compile instance"))?;

    if !command_status.status.success() {
        let stdout = String::from_utf8(command_status.stdout)
            .map_err(internal("Failed to convert stdout to string"))?;
        let stderr = String::from_utf8(command_status.stderr)
            .map_err(internal("Failed to convert stderr to string"))?;
        return Err((
            StatusCode::BAD_REQUEST,
            format!(
                "Status: {}\nStdout: {stdout}\nStderr: {stderr}",
                command_status.status
            ),
        ));
    }

    let mut file = File::open(dir.join("game_bg.wasm"))
        .await
        .map_err(internal("Failed to open final wasm"))?;
    let mut wasm = Vec::with_capacity(
        file.metadata()
            .await
            .map_err(internal("Failed to get wasm file metadata"))?
            .len() as usize,
    );
    file.read_to_end(&mut wasm).await.unwrap();

    if let Err(e) = Command::new("docker")
        .args(["container", "rm", &format!("compile.{id}")])
        .output()
        .await
    {
        error!("Failed to remove container {id}: {e:?}");
    }

    if let Err(e) = tokio::fs::remove_dir_all(dir).await {
        error!("Failed to remove temp dir {id}: {e:?}");
    }

    info!("{id}: Successful in {:.2?}", start.elapsed());
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/wasm")],
        wasm,
    ))
}

/// Helper for maping errors to internal server errors while also logging the error
fn internal<E: Error>(msg: &str) -> impl Fn(E) -> (StatusCode, String) + '_ {
    move |e: E| {
        error!("{msg}: {e:?}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Internal Server Error"),
        )
    }
}
