use crate::{image, instances::Instance, BevyVersion, Error, RustChannel};
use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tracing::{error, info, instrument};

const COMMAND: &[&str] = &[
    "cargo",
    "clippy",
    "--target",
    "wasm32-unknown-unknown",
    "--fix",
    "--allow-no-vcs",
];

#[derive(Deserialize)]
pub struct ClippyRequest {
    code: String,
    fix: bool,
}

#[derive(Serialize)]
pub struct ClippyResponse {
    fixed_code: Option<String>,
    stderr: String,
}

#[instrument(skip(payload))]
pub async fn clippy(
    Path((version, channel)): Path<(BevyVersion, RustChannel)>,
    Json(payload): Json<ClippyRequest>,
) -> Result<Json<ClippyResponse>, Error> {
    info!("Started");
    let start = Instant::now();

    let commands = if payload.fix { COMMAND } else { &COMMAND[0..4] };

    let instance = Instance::new(image(version, channel), commands, &payload.code).await?;

    let output = instance.execute().await?;

    // Exit code 101 means clippy executed successfully but a denied lint
    // was encountered.
    if !matches!(output.status.code(), Some(0 | 101)) {
        error!("Failed to run clippy: {output:?}");
        return Err(Error::Internal);
    }

    let fixed_code = if payload.fix {
        Some(instance.read_to_string("main.rs").await?)
    } else {
        None
    };

    info!("Success: Completed in {:.2?}", start.elapsed());
    Ok(Json(ClippyResponse {
        fixed_code,
        stderr: String::from_utf8(output.stderr).map_err(Error::internal)?,
    }))
}
