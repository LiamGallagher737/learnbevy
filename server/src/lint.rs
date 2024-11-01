use crate::{image, instances::Instance, BevyVersion, Error, RustChannel};
use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tracing::{error, info, instrument};

const COMMAND: &[&str] = &["bevy_lint", "--target", "wasm32-unknown-unknown"];

#[derive(Deserialize)]
pub struct LintRequest {
    code: String,
}

#[derive(Serialize)]
pub struct LintResponse {
    stderr: String,
}

#[instrument(skip(payload))]
pub async fn handler(
    Path((version, channel)): Path<(BevyVersion, RustChannel)>,
    Json(payload): Json<LintRequest>,
) -> Result<Json<LintResponse>, Error> {
    info!("Started");
    let start = Instant::now();

    let instance = Instance::new(image(version, channel), COMMAND, &payload.code).await?;

    let output = instance.execute().await?;

    // Exit code 101 means `bevy_lint` either encountered a error/deny level lint
    // or failed to build. Currently I don't know how to tell the two outputs apart
    // and treat both as 200 OK rather than using [`Error::BadCode`] for the latter.
    if !matches!(output.status.code(), Some(0 | 101)) {
        error!("Failed to run bevy_lint: {output:?}");
        return Err(Error::Internal);
    }

    info!("Success: Completed in {:.2?}", start.elapsed());
    Ok(Json(LintResponse {
        stderr: String::from_utf8(output.stderr).map_err(Error::internal)?,
    }))
}
