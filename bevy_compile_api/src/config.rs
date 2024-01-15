use serde::Deserialize;

pub fn image_for_config(version: Version, channel: Channel) -> String {
    let with_version = match version {
        Version::V0_12 => "liamg737/comp-0-12",
        Version::V0_11 => "liamg737/comp-0-11",
        Version::V0_10 => "liamg737/comp-0-10",
    }
    .to_string();
    match channel {
        Channel::Stable => with_version + "-stable",
        Channel::Nightly => with_version + "-nightly",
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
}
static __WORLD_PTR: std::sync::OnceLock::<usize> = std::sync::OnceLock::new();
#[wasm_bindgen::prelude::wasm_bindgen]
pub unsafe fn __get_entities() -> wasm_bindgen::JsValue {
    let ptr_usize = __WORLD_PTR.get().unwrap();
    let ptr = *ptr_usize  as *const bevy::ecs::world::World;
    let world = ptr.as_ref().unwrap();
    let map = js_sys::Map::new();
    for entity in world.iter_entities() {
        let id = entity.id();
        let components_info = world.inspect_entity(id);
        let len = components_info.len() as u32;
        let component_names = js_sys::Array::new_with_length(len);
        for n in 0..len {
            let js_name = wasm_bindgen::JsValue::from_str(components_info[n as usize].name());
            component_names.set(n, js_name);
        }
        map.set(&wasm_bindgen::JsValue::from_str(&format!("{id:?}")), &wasm_bindgen::JsValue::from(component_names));
    }
    wasm_bindgen::JsValue::from(map)
}
fn __set_world_ptr(world: &bevy::ecs::world::World) {
    let _ = __WORLD_PTR.set(world as *const bevy::ecs::world::World as usize);
}
#[derive(Debug)]
#[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)]
pub struct __EntityInfo {
    #[wasm_bindgen(readonly)]
    pub index: u32,
    #[wasm_bindgen(readonly)]
    pub generation: u32,
    #[wasm_bindgen(readonly)]
    pub component_names: wasm_bindgen::JsValue,
}"#;

fn edit_code_v11(code: &str) -> String {
    let mut modified_code = code.replace(
        "App::new()",
        "App::new().add_systems(Startup, __set_world_ptr).add_systems(Update, __check_exit_flag)",
    );
    modified_code.push_str(EXTRA_RUST);
    modified_code
}

fn edit_code_v10(code: &str) -> String {
    let mut modified_code = code.replace(
        "App::new()",
        "App::new().add_startup_system(__set_world_ptr).add_system(__check_exit_flag)",
    );
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
    #[serde(rename = "0.10")]
    V0_10,
    #[serde(rename = "0.11")]
    V0_11,
    #[default]
    #[serde(rename = "0.12")]
    V0_12,
}
