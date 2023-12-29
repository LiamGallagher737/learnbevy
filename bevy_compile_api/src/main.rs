use chrono::{DateTime, Utc};
use log::{error, info};
use rouille::{Request, Response, Server};
use std::{
    fs,
    net::Ipv4Addr,
    time::{Instant, SystemTime},
};

mod compile;

const ADDRESS: &str = "0.0.0.0:443";
const IMAGE: &str = "liamg737/comp";
const AUTH_TOKEN: &str = include_str!("auth-token.txt");

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

    Server::new_ssl(
        ADDRESS,
        request_handler,
        include_bytes!("cert.pem").to_vec(),
        include_bytes!("cert.key").to_vec(),
    )
    .expect("Failed to start server")
    .run();

    error!("The server socket closed unexpectedly");
}

fn request_handler(request: &Request) -> Response {
    if request.header("Cool-Auth") != Some(AUTH_TOKEN) {
        info!(
            "Rejected request from {} because the auth header either did not exist or was incorrect",
            request.remote_addr()
        );
        return Response::empty_404();
    }

    let Some(ip_str) = request.header("CF-Connecting-IP") else {
        info!(
            "Rejected request from {} because it did not contain a \"CF-Connecting-IP\" header",
            request.remote_addr()
        );
        return Response::empty_404();
    };

    let Ok(ip) = ip_str.parse::<Ipv4Addr>() else {
        return Response::empty_400();
    };

    if request.raw_url() != "/compile" {
        info!(
            "Invalid path \"{}\" requested from {}",
            request.raw_url(),
            request.remote_addr()
        );
        return Response::empty_404();
    }

    if request.method() != "POST" {
        info!(
            "Invalid mathod \"{}\" requested from {}",
            request.method(),
            request.remote_addr()
        );
        return Response::text("Only the POST method is allowed")
            .with_status_code(405)
            .with_additional_header("Allow", "POST");
    }

    let id = fastrand::usize(..);
    info!("{id}: Serving new request from {ip}");
    let start = Instant::now();

    let response = compile::compile(id, request)
        .with_additional_header("access-control-allow-origin", "*")
        .with_additional_header("access-control-expose-headers", "*");

    info!("{id}: Finished in {:.2?}", start.elapsed());
    response
}
