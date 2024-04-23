use async_std::fs;
use chrono::{DateTime, Utc};
use log::info;
use std::{
    future::Future,
    pin::Pin,
    time::{Instant, SystemTime},
};
use tide::{Next, Request};

use crate::{Id, PeerAddr};

/// The directory where the logs are stored on the filesystem
const LOG_FOLDER_PATH: &str = "logs";
/// The prefix for the log files
const LOG_FILE_PREFIX: &str = "bca.log.";

/// Sets up a new [fern::Dispatch] and creates the log directory defined by [LOG_FOLDER_PATH].
pub async fn setup() {
    fs::create_dir_all(LOG_FOLDER_PATH)
        .await
        .expect("Failed to create log folder");
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
}

/// Logs each request with its id and the duration of the request.
pub fn logging_middleware<'a>(
    request: Request<()>,
    next: Next<'a, ()>,
) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        let Id(id) = request.ext().cloned().unwrap();
        let PeerAddr(ip) = request.ext().cloned().unwrap();
        info!("{id}: Request received from {ip}");

        let start = Instant::now();
        let response = next.run(request).await;
        let elapsed = start.elapsed();
        info!("{id}: Completed in {elapsed:.2?}");

        Ok(response)
    })
}
