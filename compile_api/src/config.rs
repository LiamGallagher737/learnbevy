use serde::Deserialize;

/// Returns the image for the given a [Version] and [Channel].
pub fn image_for_config(version: Version, channel: Channel) -> String {
    let with_version = match version {
        Version::Main => "learnbevy-main",
        Version::V0_14 => "learnbevy-0.14",
        Version::V0_13 => "learnbevy-0.13",
        Version::V0_12 => "learnbevy-0.12",
        Version::V0_11 => "learnbevy-0.11",
        Version::V0_10 => "learnbevy-0.10",
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
        Version::Main => edit_code_v11(code),
        Version::V0_14 => edit_code_v11(code),
        Version::V0_13 => edit_code_v11(code),
        Version::V0_12 => edit_code_v11(code),
        Version::V0_11 => edit_code_v11(code),
        Version::V0_10 => edit_code_v10(code),
    }
}

/// This extra Rust code is added to every request.
/// It currently is just an [AtomicBool](std::sync::atomic::AtomicBool) that defaults to false and
/// a system which sends the Bevy exit event when the bool is set to true from the JavaScript.
const EXTRA_RUST: &str = r#"
static __EXIT_FLAG: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn __exit() {
    __EXIT_FLAG.store(true, std::sync::atomic::Ordering::Relaxed);
}
fn __check_exit_flag(mut exit: bevy::ecs::event::EventWriter<bevy::app::AppExit>) {
    if __EXIT_FLAG.load(std::sync::atomic::Ordering::Relaxed) {
        exit.send(bevy::app::AppExit);
    }
}

use __playground_dbg::dbg;
mod __playground_dbg {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(s: &str);
    }

    macro_rules! dbg {
        () => {
            __playground_dbg::log(&format_args!("%d{}:{}:{}", file!(), line!(), column!())).to_string()
        };
        ($val:expr $(,)?) => {
            match $val {
                tmp => {
                    __playground_dbg::log(&format_args!("%d{}:{}:{} {} = {:?}",
                        file!(), line!(), column!(), stringify!($val), &tmp).to_string());
                    tmp
                }
            }
        };
        ($($val:expr),+ $(,)?) => {
            ($(dbg!($val)),+,)
        };
    }
    pub(crate) use dbg;
}
"#;

/// Monifies the code in Bevy 0.11's style. Used by [edit_code_for_version].
fn edit_code_v11(code: &str) -> String {
    let mut modified_code = code.replace(
        "App::new()",
        "App::new().add_systems(Update, __check_exit_flag)",
    );
    modified_code.push_str(EXTRA_RUST);
    modified_code
}

/// Monifies the code in Bevy 0.10's style. Used by [edit_code_for_version].
fn edit_code_v10(code: &str) -> String {
    let mut modified_code = code.replace("App::new()", "App::new().add_system(__check_exit_flag)");
    modified_code.push_str(EXTRA_RUST);
    modified_code
}

#[derive(Clone, Copy, Default, Deserialize)]
pub enum Channel {
    #[serde(rename = "stable")]
    Stable,
    #[default]
    #[serde(rename = "nightly")]
    Nightly,
}

// Newest versions go last so cache keys stay the same when adding new versions
#[derive(Clone, Copy, Default, Deserialize)]
pub enum Version {
    #[serde(rename = "main")]
    Main,
    #[serde(rename = "0.10")]
    V0_10,
    #[serde(rename = "0.11")]
    V0_11,
    #[serde(rename = "0.12")]
    V0_12,
    #[serde(rename = "0.13")]
    V0_13,
    #[default]
    #[serde(rename = "0.14")]
    V0_14,
}
