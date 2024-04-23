use prometheus::{opts, register_counter, register_histogram, Counter, Histogram, TextEncoder};
use std::{future::Future, pin::Pin};
use tide::{Next, Request, Response, StatusCode};

lazy_static::lazy_static! {
    static ref REQUEST_COUNTER: Counter = register_counter!(opts!(
        "compile_requests_total",
        "Number of HTTP requests made.",
    )).unwrap();

    static ref REQUEST_DURATION_HISTOGRAM: Histogram = register_histogram!(
        "compile_request_duration_seconds",
        "The compile request latencies in seconds.",
        vec![2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
    )
    .unwrap();

    pub static ref CACHE_HIT_COUNTER: Counter = register_counter!(opts!(
        "cahe_hit_total",
        "Number of HTTP requests that served by a cache.",
    )).unwrap();

    pub static ref RATE_LIMIT_COUNTER: Counter = register_counter!(opts!(
        "rate_limited_requests_total",
        "Number of HTTP requests that have been rate limited.",
    )).unwrap();

    pub static ref IP_LOCK_COUNTER: Counter = register_counter!(opts!(
        "ip_locked_requests_total",
        "Number of HTTP requests that have been ip locked.",
    )).unwrap();
}

pub fn metrics_counter_middleware<'a>(
    request: Request<()>,
    next: Next<'a, ()>,
) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        REQUEST_COUNTER.inc();
        Ok(next.run(request).await)
    })
}

pub fn metrics_duration_middleware<'a>(
    request: Request<()>,
    next: Next<'a, ()>,
) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        let timer = REQUEST_DURATION_HISTOGRAM.start_timer();
        let response = next.run(request).await;
        timer.observe_duration();
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
