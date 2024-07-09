use config::{Channel, Version};
use log::error;
use metrics::count_request;
use serde::{Deserialize, Serialize};
use std::{future::Future, net::IpAddr, pin::Pin, str::FromStr};
use tide::{http::headers::HeaderValue, utils::After, Body, Next, Request, Response, StatusCode};

mod cache;
mod compile;
mod config;
mod ip_lock;
mod logging;
mod metrics;
mod rate_limiting;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    logging::setup().await;
    cache::setup().await;
    let mut app = tide::new();

    app.with(
        tide::security::CorsMiddleware::new()
            .allow_methods(HeaderValue::from_str("GET,POST").unwrap())
            .allow_headers(HeaderValue::from_str("content-type").unwrap())
            .expose_headers(
                // These headers need to be accessible by the frontend to decode the response
                HeaderValue::from_str("wasm-content-length, js-content-length").unwrap(),
            ),
    );

    app.at("/compile")
        .with(peer_addr_middleware)
        .with(rate_limiting::RateLimitMiddleware::new())
        .with(ip_lock::IpLockMiddleware::new())
        .with(id_middleware)
        .with(logging::logging_middleware)
        .with(input_middleware)
        .with(disallowed_words_middleware)
        .with(hash_middleware)
        .with(cache::cache_middleware)
        .with(After(|response: Response| async move {
            let Id(id) = response.ext().unwrap();
            compile::cleanup(*id).await;
            Ok(response)
        }))
        .with(After(|mut response: Response| async {
            let Id(id) = response.ext().unwrap();
            if let Some(err) = response.error() {
                error!("{id}: Failed with error: {err:?}");
                count_request("internal_server_error");
                response.set_body(Body::from_json(&Error::Internal)?);
            }
            Ok(response)
        }))
        .with(transfer_id_middleware)
        .with(metrics::metrics_duration_middleware)
        .post(compile::compile);

    app.at("/metrics").get(metrics::metrics_handler);

    app.listen("0.0.0.0:53740").await?;

    Ok(())
}

/// The extention added by [peer_addr_middleware].
#[derive(Clone)]
struct PeerAddr(IpAddr);
/// Gets the IP address of the user for use in ip locking and rate limiting.
fn peer_addr_middleware<'a>(
    mut request: Request<()>,
    next: Next<'a, ()>,
) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        let ip = request
            .peer_addr()
            .and_then(|a| a.parse::<IpAddr>().ok())
            .ok_or(tide::Error::from_str(
                StatusCode::BadRequest,
                "Could not get peer address",
            ))?;
        request.set_ext(PeerAddr(ip));
        Ok(next.run(request).await)
    })
}

/// The extention added by [id_middleware].
#[derive(Clone, Copy)]
struct Id(usize);
/// Generates a random ID for the request.
/// This ID is used in the logging and is returned to the user in the "refrence-number" header.
/// This means if a user is having issues they can report it with the ID and we can find more about
/// it in the logs.
fn id_middleware<'a>(
    mut request: Request<()>,
    next: Next<'a, ()>,
) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        let id = fastrand::usize(..);
        request.set_ext(Id(id));
        let mut response = next.run(request).await;
        response.insert_header("refrence-number", id.to_string());
        Ok(response)
    })
}

/// Adds the random ID on the request to the response.
/// Request extentions don't automaticly get added to the response, this middleware adds it.
fn transfer_id_middleware<'a>(
    request: Request<()>,
    next: Next<'a, ()>,
) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        let id = *request.ext::<Id>().unwrap();
        let mut response = next.run(request).await;
        response.insert_ext(id);
        Ok(response)
    })
}

/// The input for the request added as an extention by [input_middleware].
#[derive(Deserialize)]
struct Input {
    code: String,
    #[serde(default)]
    version: Version,
    #[serde(default)]
    channel: Channel,
}
/// Deserializes the input json in to the [Input] struct and adds it as a request extention.
fn input_middleware<'a>(
    mut request: Request<()>,
    next: Next<'a, ()>,
) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        let input: Input = request.body_json().await?;
        request.set_ext(input);
        Ok(next.run(request).await)
    })
}

/// Rejects any requests with [DISALLOWED_WORDS].
/// To avoid users including file from the container, if any macro which could possible include
/// files at build time is found, the request is rejected.
fn disallowed_words_middleware<'a>(
    request: Request<()>,
    next: Next<'a, ()>,
) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        let input = request.ext::<Input>().unwrap();
        for word in DISALLOWED_WORDS {
            if input.code.contains(word) {
                return Ok(Response::builder(StatusCode::BadRequest)
                    .body(Body::from_json(&Error::DisallowedWord { word })?)
                    .build());
            }
        }
        Ok(next.run(request).await)
    })
}

/// The extention added by [hash_middleware].
#[derive(Clone)]
struct MinifiedHash(Option<u128>);
/// Generates a hash of the code in its minified form.
/// The hash is used for caching. The code is first minified to make sure adding comments or
/// whitespace won't result in cache misses.
fn hash_middleware<'a>(
    mut request: Request<()>,
    next: Next<'a, ()>,
) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        let Input {
            code,
            version,
            channel,
        } = request.ext().unwrap();
        let hash = rust_minify::minify(code).ok().map(|code| {
            let mut hash = fastmurmur3::hash(code.as_bytes());
            hash += *version as u128;
            hash = hash.rotate_left(16);
            hash += *channel as u128;
            hash
        });
        request.set_ext(MinifiedHash(hash));
        let response = next.run(request).await;
        Ok(response)
    })
}

/// The errors.
#[derive(Serialize)]
#[serde(tag = "kind")]
enum Error {
    RateLimit {
        time_left: f32,
    },
    #[allow(dead_code)]
    CFRateLimit,
    ActiveRequestExists,
    DisallowedWord {
        word: &'static str,
    },
    BuildFailed {
        stderr: String,
    },
    Internal,
}

/// The list of disallowed words that are rejected by [disallowed_words_middleware].
const DISALLOWED_WORDS: &[&str] = &[
    "include!",
    "include_str!",
    "include_bytes!",
    "embedded_asset!",
    "embedded_path",
    "load_internal_asset",
    "load_internal_binary_asset",
];
