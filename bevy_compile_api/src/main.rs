use chrono::{DateTime, Utc};
use log::{error, info, trace};
use rate_limit::RateLimitMap;
use rouille::{Request, Response, Server};
use serde::Serialize;
use std::{
    collections::HashSet,
    fs,
    net::Ipv4Addr,
    sync::{Arc, RwLock},
    time::{Instant, SystemTime},
};

mod compile;
mod rate_limit;

const ADDRESS: &str = "0.0.0.0:443";
const IMAGE: &str = "liamg737/comp";
const AUTH_TOKEN: &str = include_str!("auth-token.txt");

const RATE_LIMIT_LENGTH_SUCCESSFUL: f32 = 5.0;
const RATE_LIMIT_LENGTH_UNSUCCESSFUL: f32 = 1.0;
const RATE_LIMIT_LENGTH_INVALID: f32 = 60.0 * 5.0;

#[cfg(target_os = "linux")]
const LOG_FOLDER_PATH: &str = "/var/log/bca";
#[cfg(not(target_os = "linux"))]
const LOG_FOLDER_PATH: &str = "logs";
const LOG_FILE_PREFIX: &str = "bca.log.";

fn main() {
    fs::create_dir_all(LOG_FOLDER_PATH).expect("Failed to create log folder");
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
        .level(log::LevelFilter::Debug)
        .chain(
            fern::DateBased::new(format!("{LOG_FOLDER_PATH}/{LOG_FILE_PREFIX}"), "%Y-%m-%d")
                .utc_time(),
        )
        .chain(std::io::stdout())
        .apply()
        .expect("Failed to setup logging");

    let rate_limits = RateLimitMap::default();
    let active_ips = Arc::new(RwLock::new(HashSet::new()));

    Server::new_ssl(
        ADDRESS,
        move |request| request_handler(request, rate_limits.clone(), active_ips.clone()),
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
    active_ips: Arc<RwLock<HashSet<Ipv4Addr>>>,
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

    let Ok(ip) = ip_str.parse::<Ipv4Addr>() else {
        return Response::empty_400();
    };

    if let Some(rate_limit) = rate_limits.get(&ip) {
        let time_left = (rate_limit.length - rate_limit.start.elapsed().as_secs_f32()).ceil();
        if time_left > 0.0 {
            trace!("Rejected request from {ip} because of a rate limit");
            return Response::json(&RateLimitError {
                kind: ErrorKind::RateLimit,
                msg: "Rate limited",
                time_left,
            })
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
        return Response::json(&BasicError {
            kind: ErrorKind::ActiveRequestExists,
            msg: "A request from your IP is already being processed",
        })
        .with_status_code(429);
    }

    if request.raw_url() != "/compile" {
        info!(
            "Invalid path \"{}\" requested from {}",
            request.raw_url(),
            request.remote_addr()
        );
        rate_limits.insert(ip, RATE_LIMIT_LENGTH_INVALID);
        return Response::empty_404();
    }

    if request.method() != "POST" {
        info!(
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

    let response = compile::compile(id, request)
        .with_additional_header("access-control-allow-origin", "*")
        .with_additional_header("access-control-expose-headers", "*");

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
struct RateLimitError {
    kind: ErrorKind,
    msg: &'static str,
    time_left: f32,
}

#[derive(Serialize)]
struct BasicError {
    kind: ErrorKind,
    msg: &'static str,
}

#[derive(Serialize)]
enum ErrorKind {
    RateLimit,
    #[allow(dead_code)]
    CFRateLimit,
    ActiveRequestExists,
    InvalidBody,
    BuildFailed,
    Internal,
}
