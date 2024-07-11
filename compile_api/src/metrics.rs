use crate::config::{Channel, Version};
use prometheus::{register_counter_vec, register_histogram, CounterVec, Histogram, TextEncoder};
use std::{future::Future, pin::Pin};
use tide::{Next, Request, Response, StatusCode};

const REQUEST_LABEL_NAMES: [&str; 1] = ["status"];
const REQUEST_OPTIONS_LABEL_NAMES: [&str; 2] = ["version", "channel"];

lazy_static::lazy_static! {
    static ref REQUEST_COUNTER: CounterVec = register_counter_vec!(
        "compile_requests_total",
        "Number of HTTP requests made.",
        &REQUEST_LABEL_NAMES,
    ).unwrap();

    static ref REQUEST_DURATION_HISTOGRAM: Histogram = register_histogram!(
        "compile_request_duration_seconds",
        "The compile request latencies in seconds.",
        vec![2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
    )
    .unwrap();

    static ref REQUEST_OPTIONS_COUNTER: CounterVec = register_counter_vec!(
        "compile_requests_options_total",
        "Version of Bevy and channel of Rust for the request",
        &REQUEST_OPTIONS_LABEL_NAMES,
    ).unwrap();
}

pub fn count_request(status: &'static str) {
    REQUEST_COUNTER.with_label_values(&[status]).inc();
}

pub fn count_request_options(version: Version, channel: Channel) {
    REQUEST_OPTIONS_COUNTER
        .with_label_values(&[&version.to_string(), &channel.to_string()])
        .inc();
}

pub fn metrics_duration_middleware<'a>(
    request: Request<()>,
    next: Next<'a, ()>,
) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        let timer = REQUEST_DURATION_HISTOGRAM.start_timer();
        let response = next.run(request).await;
        if response.status() == StatusCode::Ok {
            timer.observe_duration();
        } else {
            timer.stop_and_discard();
        }
        Ok(response)
    })
}

pub async fn metrics_handler(_req: Request<()>) -> tide::Result {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = String::new();
    encoder.encode_utf8(&metric_families, &mut buffer).unwrap();

    Ok(Response::builder(StatusCode::Ok)
        .body(buffer)
        .content_type(tide::http::mime::PLAIN)
        .build())
}
