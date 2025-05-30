use bevy_app::{App, Update};

mod exit;
mod log;

/// Items that should be accessable from the sandbox app.
///
/// The following is added to every app prior to compiling.
/// ```rs
/// use playground_lib::exports::*;
/// ```
pub mod exports {}

pub struct Plugin;
impl bevy_app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, exit::check_exit_flag);
    }
}
