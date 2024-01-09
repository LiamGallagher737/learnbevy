use serde::Deserialize;
use std::str::FromStr;

pub fn image_for_version(version: Version) -> &'static str {
    match version {
        Version::V0_12 => "liamg737/comp-0-12",
        Version::V0_11 => "liamg737/comp-0-11",
        Version::V0_10 => "liamg737/comp-0-10",
    }
}

pub fn edit_code_for_version(code: &str, version: Version) -> String {
    match version {
        Version::V0_12 => edit_code_v11(code),
        Version::V0_11 => edit_code_v11(code),
        Version::V0_10 => edit_code_v10(code),
    }
}

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
}"#;

fn edit_code_v11(code: &str) -> String {
    let mut modified_code = code.replace(
        "App::new()",
        "App::new().add_systems(Update, __check_exit_flag)",
    );
    modified_code.push_str(EXTRA_RUST);
    modified_code
}

fn edit_code_v10(code: &str) -> String {
    let mut modified_code = code.replace("App::new()", "App::new().add_system(__check_exit_flag)");
    modified_code.push_str(EXTRA_RUST);
    modified_code
}

// Newest versions go last so cache keys stay the same when adding new versions
#[derive(Clone, Copy, Default)]
pub enum Version {
    V0_10,
    V0_11,
    #[default]
    V0_12,
}

impl FromStr for Version {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0.12" => Ok(Self::V0_12),
            "0.11" => Ok(Self::V0_11),
            "0.10" => Ok(Self::V0_10),
            _ => Err("Invalid version"),
        }
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(serde::de::Error::custom)
    }
}
