use bevy_app::{App, Startup};

mod brp;

/// Items that should be accessable from the sandbox app.
///
/// The following is added to every app prior to compiling.
/// ```rs
/// use extra_app_code::exports::*;
/// ```
pub mod exports {}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, brp::setup);
}
