export const DEFAULT_CODE = `use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, change_clear_color)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    info!("Here is some info");
    warn!("Here is a warning");
    error!("Here is an error");
}

fn change_clear_color(input: Res<ButtonInput<KeyCode>>, mut clear_color: ResMut<ClearColor>, mut state: Local<bool>) {
    if input.just_pressed(KeyCode::Space) {
        info!("Changing color");
        *state = !*state;
        if *state {
            clear_color.0 = Color::PURPLE;
        } else {
            clear_color.0 = Color::RED;
        }
    }
}
`;