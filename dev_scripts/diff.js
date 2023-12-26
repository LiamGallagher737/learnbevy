const measurements = 10;

const server1 = 'http://45.13.225.104:8080';
const server2 = 'http://45.148.60.5:8080';

const code = `
use bevy::prelude::*;

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
}

fn change_clear_color(input: Res<Input<KeyCode>>, mut clear_color: ResMut<ClearColor>, mut state: Local<bool>) {
    if input.just_pressed(KeyCode::Space) {
    *state = !*state;
    if *state {
        clear_color.0 = Color::PURPLE;
    } else {
        clear_color.0 = Color::RED;
    }
    }
}
`;

async function measureResponseTime(url) {
    const startTime = new Date();
    try {
        const response = await fetch(url, {
            method: 'POST',
            body: code,
        });

        const _body = await response.blob();

        const endTime = new Date();
        const responseTime = endTime - startTime;
        return responseTime;
    } catch (error) {
        console.error(`Error for ${url}: ${error.message}`);
        return Infinity;
    }
}

async function main() {
    while (true) {
        const results = await Promise.all([
            measureResponseTime(server1),
            measureResponseTime(server2),
        ]);

        console.log(`Fat: ${results[0]}ms`);
        console.log(`Small: ${results[1]}ms`);
    }
}

main();
