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

async function app() {
    let n = 0;
    while (true) {
        console.log(`Starting new request: ${n}`);
        const start = Date.now();
        const res = await fetch("http://45.148.60.5:8080", {
            method: "POST",
            headers: {
                "content-type": "text/rust"
            },
            body: code,
        });

        const _body = await res.blob();

        const end = Date.now();
        if (!res.ok) {
            console.log(`Request failed: ${res.status} - ${res.statusText}\n${await res.text()}`);
        } else {
            console.log("Request success");
        }
        console.log(`Execution time: ${end - start} ms`);
        console.log();

        await new Promise(r => setTimeout(r, 1000));
        n += 1;
    }
}

app();
