use bevy_app::AppExit;
use bevy_ecs::event::EventWriter;
use std::sync::atomic::{AtomicBool, Ordering};
use wasm_bindgen::prelude::wasm_bindgen;

static EXIT_FLAG: AtomicBool = AtomicBool::new(false);

#[wasm_bindgen]
pub fn exit() {
    bevy_log::info!("exitttt");
    EXIT_FLAG.store(true, Ordering::Relaxed);
}

#[wasm_bindgen]
pub fn bye() {
    bevy_log::info!("byeeeee");
}


pub fn check_exit_flag(mut exit: EventWriter<AppExit>) {
    if EXIT_FLAG.load(Ordering::Relaxed) {
        exit.send(AppExit::Success);
    }
}
