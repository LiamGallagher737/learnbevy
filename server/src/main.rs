use axum::{
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde::Serialize;
use tokio::net::TcpListener;
use tracing::{error, info};

mod clippy;
mod instances;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        // .route("/compile", post(compile::handler))
        .route("/clippy", post(clippy::handler))
        // .route("/lint", post(lint::handler))
     ;

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening at http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
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
