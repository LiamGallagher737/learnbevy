use chrono::{DateTime, Utc};
use flate2::Compression;
use log::{error, info, trace};
use rate_limit::RateLimitMap;
use rouille::{Request, Response, Server};
use serde::Serialize;
use std::{
    collections::HashSet,
    fs,
    net::IpAddr,
    sync::{Arc, RwLock},
    time::{Instant, SystemTime},
};

mod cache;
mod compile;
mod rate_limit;

const ADDRESS: &str = "0.0.0.0:53740";
const IMAGE: &str = "liamg737/comp";
const AUTH_TOKEN: &str = include_str!("auth-token.txt");
const CACHE_BYPASS_TOKEN: &str = include_str!("cache-bypass-token.txt");
const COMPRESSION_LEVEL: Compression = Compression::fast();

const RATE_LIMIT_LENGTH_SUCCESSFUL: f32 = 5.0;
const RATE_LIMIT_LENGTH_UNSUCCESSFUL: f32 = 1.0;
const RATE_LIMIT_LENGTH_INVALID: f32 = 60.0 * 5.0;

#[cfg(target_os = "linux")]
const LOG_FOLDER_PATH: &str = "/var/log/bca";
#[cfg(not(target_os = "linux"))]
const LOG_FOLDER_PATH: &str = "logs";
const LOG_FILE_PREFIX: &str = "bca.log.";

#[cfg(target_os = "linux")]
const CACHE_FOLDER_PATH: &str = "/bca_cache";
#[cfg(not(target_os = "linux"))]
const CACHE_FOLDER_PATH: &str = "cache";

fn main() {
    fs::create_dir_all(LOG_FOLDER_PATH).expect("Failed to create log folder");
    fs::create_dir_all(CACHE_FOLDER_PATH).expect("Failed to create cache folder");
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                DateTime::<Utc>::from(SystemTime::now()).format("%H:%M:%S"),
                record.level(),
                record.target(),
                message
            ));
        })
        .level(log::LevelFilter::Info)
        .chain(
            fern::DateBased::new(format!("{LOG_FOLDER_PATH}/{LOG_FILE_PREFIX}"), "%Y-%m-%d")
                .utc_time(),
        )
        .chain(std::io::stdout())
        .apply()
        .expect("Failed to setup logging");

    let rate_limits = RateLimitMap::default();
    let active_ips = Arc::new(RwLock::new(HashSet::new()));

    info!("Starting server on {ADDRESS}");

    Server::new_ssl(
        ADDRESS,
        move |request| {
            request_handler(request, rate_limits.clone(), active_ips.clone())
                .with_additional_header("access-control-allow-origin", "*")
        },
        include_bytes!("cert.pem").to_vec(),
        include_bytes!("cert.key").to_vec(),
    )
    .expect("Failed to start server")
    .run();

    error!("The server socket closed unexpectedly");
}

fn request_handler(
    request: &Request,
    rate_limits: RateLimitMap,
    active_ips: Arc<RwLock<HashSet<IpAddr>>>,
) -> Response {
    if request.header("Cool-Auth") != Some(AUTH_TOKEN) {
        trace!(
            "Rejected request from {} because the auth header either did not exist or was incorrect",
            request.remote_addr()
        );
        return Response::empty_404();
    }

    let Some(ip_str) = request.header("CF-Connecting-IP") else {
        trace!(
            "Rejected request from {} because it did not contain a \"CF-Connecting-IP\" header",
            request.remote_addr()
        );
        return Response::empty_404();
    };

    let Ok(ip) = ip_str.parse::<IpAddr>() else {
        error!(
            "Request's CF-Connecting-IP header could not parse to a valid IpAddr: {ip_str}"
        );
        return Response::empty_400();
    };

    if let Some(rate_limit) = rate_limits.get(&ip) {
        let time_left = (rate_limit.length - rate_limit.start.elapsed().as_secs_f32()).ceil();
        if time_left > 0.0 {
            trace!("Rejected request from {ip} because of a rate limit");
            return Response::json(&Error::RateLimit { time_left })
                .with_status_code(429)
                .with_additional_header("Retry-After", time_left.to_string());
        } else {
            rate_limits.remove(&ip);
        }
    }

    if active_ips.read().unwrap().contains(&ip) {
        trace!(
            "Rejected request from {ip} because a request from this ip is already being processed"
        );
        return Response::json(&Error::ActiveRequestExists).with_status_code(429);
    }

    if request.raw_url() != "/compile" {
        trace!(
            "Invalid path \"{}\" requested from {}",
            request.raw_url(),
            request.remote_addr()
        );
        rate_limits.insert(ip, RATE_LIMIT_LENGTH_INVALID);
        return Response::empty_404();
    }

    if request.method() != "POST" {
        trace!(
            "Invalid mathod \"{}\" requested from {}",
            request.method(),
            request.remote_addr()
        );
        rate_limits.insert(ip, RATE_LIMIT_LENGTH_INVALID);
        return Response::text("Only the POST method is allowed")
            .with_status_code(405)
            .with_additional_header("Allow", "POST");
    }

    active_ips.write().unwrap().insert(ip);

    let id = fastrand::usize(..);
    info!("{id}: Serving new request from {ip}");
    let start = Instant::now();

    let response = compile::compile(id, request).with_additional_header(
        "access-control-expose-headers",
        "wasm-content-length, js-content-length",
    );

    rate_limits.insert(
        ip,
        match response.status_code {
            200 => RATE_LIMIT_LENGTH_SUCCESSFUL,
            _ => RATE_LIMIT_LENGTH_UNSUCCESSFUL,
        },
    );

    active_ips.write().unwrap().remove(&ip);

    info!("{id}: Finished in {:.2?}", start.elapsed());
    response
}

#[derive(Serialize)]
#[serde(tag = "kind")]
enum Error {
    RateLimit {
        time_left: f32,
    },
    #[allow(dead_code)]
    CFRateLimit,
    ActiveRequestExists,
    InvalidBody,
    DisallowedWord {
        word: &'static str,
    },
    BuildFailed {
        stdout: String,
        stderr: String,
    },
    Overloaded,
    Internal,
}
