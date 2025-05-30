use std::time::Duration;

use axum::{
    http::{header::CONTENT_TYPE, HeaderName, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
};
use tracing::{error, info};

mod clippy;
mod compile;
mod format;
mod instances;
mod lint;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/compile/:version/:channel", post(compile::compile))
        .route("/clippy/:version/:channel", post(clippy::clippy))
        .route("/lint/:version/:channel", post(lint::lint))
        .route("/format", post(format::format))
        .layer(CompressionLayer::new())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::POST])
                .allow_headers([CONTENT_TYPE])
                .max_age(Duration::from_secs(60 * 60 * 24))
                .expose_headers([
                    HeaderName::from_static("wasm-content-length"),
                    HeaderName::from_static("js-content-length"),
                ]),
        );

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening at http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

/// The version of Bevy for a request.
#[derive(Deserialize, Display, Debug)]
enum BevyVersion {
    #[serde(rename = "main")]
    #[display("main")]
    Main,
    // When updating this for new Bevy versions, the number value
    // should also be updated so saved caches are invalidated.
    #[serde(rename = "0.16")]
    #[display("0.16")]
    V0_16 = 16,
}

/// The channel of Rust for a request.
#[derive(Deserialize, Display, Debug)]
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
    BadCode { stderr: String },
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
        let status = match self {
            Error::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            Error::BadCode { stderr: _ } => StatusCode::BAD_REQUEST,
        };
        let mut response = Json(self).into_response();
        *response.status_mut() = status;
        response
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::internal(error)
    }
}
