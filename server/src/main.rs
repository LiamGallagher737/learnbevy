use axum::{
    http::{header::CONTENT_TYPE, HeaderName, Method},
    routing::post,
    Router,
};
use shared::{BevyVersion, RustChannel};
use tokio::net::TcpListener;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
};
use tracing::info;

mod clippy;
mod compile;
mod format;
mod instances;
mod lint;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/compile/:version/:channel", post(compile::handler))
        .route("/clippy/:version/:channel", post(clippy::handler))
        .route("/lint/:version/:channel", post(lint::handler))
        .route("/format", post(format::handler))
        .layer(CompressionLayer::new())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::POST])
                .allow_headers([CONTENT_TYPE])
                .expose_headers([
                    HeaderName::from_static("wasm-content-length"),
                    HeaderName::from_static("js-content-length"),
                ]),
        );

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening at http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn image(version: BevyVersion, channel: RustChannel) -> String {
    format!("ghcr.io/liamgallagher737/learnbevy-{version}-{channel}:main")
}
