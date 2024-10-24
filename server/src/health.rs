use crate::{image, BevyVersion, Error, RustChannel};
use axum::{extract::Path, Json};
use serde::Serialize;
use tokio::process;

#[derive(Serialize)]
pub struct HealthResponse {
    available: bool,
}

pub async fn handler(
    Path((version, channel)): Path<(BevyVersion, RustChannel)>,
) -> Result<Json<HealthResponse>, Error> {
    let image = image(version, channel);
    let result = process::Command::new("docker")
        .args(["image", "inspect", &image])
        .output()
        .await?;

    Ok(Json(HealthResponse {
        available: result.status.success(),
    }))
}
