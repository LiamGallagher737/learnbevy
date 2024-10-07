use serde::Deserialize;

/// Returns the image for the given a [Version] and [Channel].
pub fn image_for_config(version: Version, channel: Channel) -> String {
    let with_version = match version {
        Version::Main => "learnbevy-main",
        Version::V0_14 => "learnbevy-0.14",
    }
    .to_string();
    let image = match channel {
        Channel::Stable => with_version + "-stable",
        Channel::Nightly => with_version + "-nightly",
    };
    format!("ghcr.io/liamgallagher737/{image}:main")
}

/// Modifies the code for the given Bevy version. This includes adding the required systems for
/// exiting the app
pub fn edit_code_for_version(code: &str, version: Version) -> String {
    match version {
        Version::Main => edit_code_v14(code),
        Version::V0_14 => edit_code_v14(code),
    }
}

/// This extra Rust code is added to every request.
/// It currently is just an [AtomicBool](std::sync::atomic::AtomicBool) that defaults to false and
/// a system which sends the Bevy exit event when the bool is set to true from the
/// JavaScript.config
const EXTRA_RUST: &str = r#"
#[allow(unused_imports)]
use playground_lib::exports::*;
#[allow(unused_imports)]
use playground_lib::dbg;
"#;

/// Monifies the code in Bevy 0.14's style. Used by [edit_code_for_version].
fn edit_code_v14(code: &str) -> String {
    let mut modified_code = code.replace(
        "App::new()",
        "App::new().add_plugins(playground_lib::Plugin)",
    );
    modified_code.push_str(EXTRA_RUST);
    modified_code
}

#[derive(Clone, Copy, Default, Deserialize, Debug)]
pub enum Channel {
    #[serde(rename = "stable")]
    Stable,
    #[default]
    #[serde(rename = "nightly")]
    Nightly,
}

impl std::fmt::Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Newest versions go last so cache keys stay the same when adding new versions
#[derive(Clone, Copy, Default, Deserialize)]
pub enum Version {
    #[serde(rename = "main")]
    Main,
    #[default]
    #[serde(rename = "0.14")]
    V0_14,
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Version::Main => f.write_str("Main"),
            Version::V0_14 => f.write_str("0.14"),
        }
    }
}
