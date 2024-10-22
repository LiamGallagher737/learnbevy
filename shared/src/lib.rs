use derive_more::Display;
use serde::{Deserialize, Serialize};

/// The version of Bevy for a request.
///
/// When updating this for new Bevy versions, the number value
/// should also be updated so saved caches are invalidated.
#[derive(Serialize, Deserialize, Display)]
pub enum BevyVersion {
    #[serde(rename = "main")]
    #[display("main")]
    Main = 0,
    #[serde(rename = "0.14")]
    #[display("0.14")]
    V0_14 = 14,
}

/// The channel of Rust for a request.
#[derive(Serialize, Deserialize, Display)]
pub enum RustChannel {
    #[serde(rename = "stable")]
    #[display("stable")]
    Stable,
    #[serde(rename = "nightly")]
    #[display("nightly")]
    Nightly,
}

/// The error type for all handlers.
#[derive(Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Error {
    Internal,
    BadCode { stderr: String },
}

#[cfg(feature = "server")]
mod server_error_impls {
    use crate::Error;
    use axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
        Json,
    };
    use tracing::error;

    impl Error {
        #[must_use]
        pub fn internal<E: std::fmt::Display>(error: E) -> Self {
            error!("Failed to handle request: {error}");
            Self::Internal
        }
    }

    impl IntoResponse for Error {
        fn into_response(self) -> Response {
            let status = match self {
                Error::Internal => StatusCode::INTERNAL_SERVER_ERROR,
                Error::BadCode { stderr: _ } => StatusCode::BAD_REQUEST,
            };
            let mut response = Json(self).into_response();
            *response.status_mut() = status;
            response
        }
    }

    impl From<std::io::Error> for Error {
        fn from(error: std::io::Error) -> Self {
            Self::internal(error)
        }
    }
}

/// Types specific to compile requests.
pub mod compile {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct CompileRequest {
        pub code: String,
    }
}

/// Types specific to clippy requests.
pub mod clippy {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct ClippyRequest {
        pub code: String,
        pub fix: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ClippyResponse {
        pub fixed_code: Option<String>,
        pub stderr: String,
    }
}

/// Types specific to format requests.
pub mod format {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct FormatRequest {
        pub code: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct FormatResponse {
        pub formatted_code: String,
    }
}

/// Types specific to lint requests.
pub mod lint {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct LintRequest {
        pub code: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct LintResponse {
        pub stderr: String,
    }
}
