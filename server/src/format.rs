use crate::Error;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process;
use tracing::error;

#[derive(Deserialize)]
pub struct FormatRequest {
    code: String,
}

#[derive(Serialize)]
pub struct FormatResponse {
    formatted_code: String,
}

pub async fn handler(Json(payload): Json<FormatRequest>) -> Result<Json<FormatResponse>, Error> {
    // Spawn a new rustfmt child process
    let mut command = process::Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()?;

    // Write the requests code to the stdin of the rustfmt process
    command
        .stdin
        .take()
        .ok_or(Error::Internal)?
        .write_all(payload.code.as_bytes())
        .await?;

    // Wait for rustfmt to complete and collect the output
    let output = command.wait_with_output().await?;

    // Respond based on result of rustfmt
    if output.status.success() {
        Ok(Json(FormatResponse {
            formatted_code: String::from_utf8(output.stdout).map_err(Error::internal)?,
        }))
    } else if output.status.code() == Some(1) {
        Err(Error::BadCode {
            stderr: String::from_utf8(output.stderr).map_err(Error::internal)?,
        })
    } else {
        error!(
            "Failed to run rustfmt: {}",
            String::from_utf8(output.stderr).map_err(Error::internal)?
        );
        Err(Error::Internal)
    }
}
