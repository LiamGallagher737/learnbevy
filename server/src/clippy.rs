use crate::{instances::Instance, Error};
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::error;

const COMMAND: &[&str] = &["cargo", "clippy", "--fix", "--allow-no-vcs"];

#[derive(Deserialize)]
pub struct ClippyRequest {
    code: String,
    fix: bool,
}

#[derive(Serialize)]
pub struct ClippyResponse {
    code: Option<String>,
    stderr: String,
}

pub async fn handler(Json(payload): Json<ClippyRequest>) -> Result<Json<ClippyResponse>, Error> {
    let commands = if payload.fix { COMMAND } else { &COMMAND[0..2] };

    let instance = Instance::new(
        "ghcr.io/liamgallagher737/learnbevy-0.14-nightly:main",
        commands,
        &payload.code,
    )
    .await?;

    let output = instance.execute().await?;

    if output.status.code() != Some(0) {
        error!("Failed to run clippy: {output:?}");
        return Err(Error::Internal);
    }

    let fixed_code = if payload.fix {
        Some(instance.read_to_string("main.rs").await?)
    } else {
        None
    };

    Ok(Json(ClippyResponse {
        code: fixed_code,
        stderr: String::from_utf8(output.stderr).map_err(Error::internal)?,
    }))
}
