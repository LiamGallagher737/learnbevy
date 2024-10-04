use bevy_app::{App, Startup};

mod brp;

/// Items that should be accessable from the sandbox app.
///
/// The following is added to every app prior to compiling.
/// ```rs
/// use extra_app_code::exports::*;
/// ```
pub mod exports {}

pub struct Plugin;
impl bevy_app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_remote::RemotePlugin::default())
            .add_systems(Startup, brp::setup);
    }
}
