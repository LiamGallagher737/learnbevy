use serde::Serialize;
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use warp::{reject::Rejection, Filter};

mod compile;
mod config;

const HOST: ([u8; 4], u16) = ([0, 0, 0, 0], 53740);

#[tokio::main]
async fn main() {
    let log_file_writer =
        rolling::hourly("./logs/info", "log").with_max_level(tracing::Level::INFO);
    let important_file_writer =
        rolling::daily("./logs/important", "important").with_max_level(tracing::Level::WARN);

    let writer = log_file_writer
        .and(important_file_writer)
        .and(std::io::stdout);
    tracing_subscriber::fmt()
        .with_writer(writer)
        .with_ansi(false)
        .init();

    let route = warp::post()
        .and(warp::path("compile"))
        .and(warp::body::content_length_limit(1024 * 16)) // 16kb
        .map(|| fastrand::u64(..)) // Generate random id
        .and(input_body())
        .and_then(compile::compile)
        .recover(handle_rejection)
        .with(warp::trace::request())
        .with(warp::compression::gzip())
        .with(
            warp::cors()
                .allow_any_origin()
                .allow_method("POST")
                .allow_header("content-type")
                .expose_headers(["wasm-content-length", "js-content-length"]),
        );

    warp::serve(route)
        .tls()
        .cert_path("cert.pem")
        .key_path("cert.key")
        .run(HOST)
        .await;
}

#[derive(Serialize, Debug)]
#[serde(tag = "kind")]
enum Error {
    RateLimit { time_left: f32 },
    ActiveRequestExists,
    DisallowedWord { word: &'static str },
    BuildFailed { stderr: String },
    Internal,
}
impl warp::reject::Reject for Error {}

async fn handle_rejection(rejection: warp::Rejection) -> Result<impl warp::Reply, Rejection> {
    if let Some(error) = rejection.find::<Error>() {
        let status = match error {
            Error::RateLimit { time_left: _ } => warp::http::StatusCode::TOO_MANY_REQUESTS,
            Error::ActiveRequestExists => warp::http::StatusCode::TOO_MANY_REQUESTS,
            Error::DisallowedWord { word: _ } => warp::http::StatusCode::BAD_REQUEST,
            Error::BuildFailed { stderr: _ } => warp::http::StatusCode::BAD_REQUEST,
            Error::Internal => warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = warp::reply::json(error);
        let reply = warp::reply::with_status(body, status);
        return Ok(reply);
    }

    Err(rejection)
}

fn input_body() -> impl Filter<Extract = (compile::Input,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::body::json::<compile::Input>())
        .and_then(|input: compile::Input| async move {
            for &word in DISALLOWED_WORDS.iter() {
                if input.code.contains(word) {
                    return Err(warp::reject::custom(Error::DisallowedWord { word }));
                }
            }
            Ok(input)
        })
}

const DISALLOWED_WORDS: &[&str] = &[
    "include!",
    "include_str!",
    "include_bytes!",
    "embedded_asset!",
    "embedded_path",
    "load_internal_asset",
    "load_internal_binary_asset",
];
