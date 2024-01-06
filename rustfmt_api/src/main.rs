use serde::{Deserialize, Serialize};
use std::{convert::Infallible, io::ErrorKind, process::Stdio};
use tokio::{io::AsyncWriteExt, process::Command};
use warp::{http::StatusCode, reply::Reply, Filter};

#[tokio::main]
async fn main() {
    let route = warp::post()
        .and(warp::path("format"))
        .and(warp::body::json())
        .and_then(handler)
        .with(warp::compression::brotli());
    warp::serve(route).run(([0; 4], 46530)).await;
}

async fn handler(request: Request) -> Result<warp::reply::Response, Infallible> {
    Ok(format(request)
        .await
        .map(|json| json.into_response())
        .map_err(|err| eprintln!("Error: {err:?}"))
        .unwrap_or({
            let mut response = warp::reply::json(&Response::ServerError).into_response();
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            response
        }))
}

async fn format(request: Request) -> Result<warp::reply::Json, std::io::Error> {
    let mut command = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()?;

    command
        .stdin
        .take()
        .ok_or(std::io::Error::new(ErrorKind::Other, "Stdin is None"))?
        .write_all(request.code.as_bytes())
        .await?;

    let output = command.wait_with_output().await?;

    if output.status.success() {
        let formatted_code = String::from_utf8(output.stdout)
            .map_err(|_| std::io::Error::new(ErrorKind::Other, "Stdout is invalid utf8"))?;
        return Ok(warp::reply::json(&Response::Success { formatted_code }));
    } else if output.status.code() == Some(1) {
        let stderr = String::from_utf8(output.stderr)
            .map_err(|_| std::io::Error::new(ErrorKind::Other, "Stderr is invalid utf8"))?;
        Ok(warp::reply::json(&Response::UserError { stderr }))
    } else {
        let code = output.status.code();
        let stderr =
            String::from_utf8(output.stderr).unwrap_or_else(|_| String::from("Invalid uft8"));
        Err(std::io::Error::new(
            ErrorKind::Other,
            format!("rustfmt failed with {code:?}: {stderr}"),
        ))
    }
}

#[derive(Deserialize)]
struct Request {
    code: String,
}

#[derive(Serialize)]
#[serde(tag = "kind")]
enum Response {
    Success { formatted_code: String },
    UserError { stderr: String },
    ServerError,
}
