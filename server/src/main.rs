use axum::{
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tracing::{error, info};
use derive_more::Display;

mod clippy;
mod instances;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        // .route("/compile", post(compile::handler))
        .route("/clippy/:version/:channel", post(clippy::handler))
        // .route("/lint", post(lint::handler))
     ;

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening at http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

/// The version of Bevy for a request.
#[derive(Deserialize, Display)]
enum BevyVersion {
    #[serde(rename = "main")]
    #[display("main")]
    Main,
    // When updating this for new Bevy versions, the number value
    // should also be updated so saved caches are invalidated.
    #[serde(rename = "0.14")]
    #[display("0.14")]
    V0_14 = 14,
}

/// The channel of Rust for a request.
#[derive(Deserialize, Display)]
enum RustChannel {
    #[serde(rename = "stable")]
    #[display("stable")]
    Stable,
    #[serde(rename = "nightly")]
    #[display("nightly")]
    Nightly,
}

fn image(version: BevyVersion, channel: RustChannel) -> String {
    format!("ghcr.io/liamgallagher737/learnbevy-{version}-{channel}:main")
}

/// The error type for all handlers.
#[derive(Serialize)]
#[serde(tag = "kind")]
enum Error {
    Internal,
}

impl Error {
    #[must_use]
    pub fn internal<E: std::fmt::Display>(error: E) -> Self {
        error!("Failed to handle request: {error}");
        Self::Internal
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::internal(error)
    }
}
