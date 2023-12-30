const runBtn = document.getElementById('button-run');
const editorElement = document.getElementById('editor');
const gameElement = document.getElementById('game');

const startingCode =
    `use bevy::prelude::*;

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

let editor = null;
require.config({ paths: { 'vs': 'https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.26.1/min/vs' } });
require(["vs/editor/editor.main"], () => {
    editor = monaco.editor.create(document.getElementById('editor'), {
        value: startingCode,
        language: 'rust',
        theme: 'vs-dark',
        minimap: { enabled: false },
    });
});

window.addEventListener('resize', () => editor.layout());

async function run() {
    runBtn.disabled = true;
    gameElement.innerHTML = "";
    const code = editor.getValue();

    const res = await fetch("https://compile.learnbevy.com/", {
        method: "POST",
        body: code,
    });

    if (!res.ok) {
        const error = await res.json();

        if (error.kind === "BuildFailed") {
            const stdout = document.createElement("pre");
            stdout.innerText = error.stderr;
            gameElement.appendChild(stdout);
        }
        
        let msg = "";
        switch (error.kind) {
            case "RateLimit":
                msg = `Please wait ${error.time_left}s before submitting another request`;
                break;
            case "CFRateLimit":
                msg = "Please wait before submitting another request";
                break;
            case "ActiveRequestExists":
                msg = "A request from your IP is currently being handled, please wait until it is complete";
                break;
            case "BuildFailed":
                msg = "The code failed to build";
                break;
            case "Overloaded":
                msg = "The server failed to process your request due to being overloaded";
                break;
            case "Internal":
                msg = "An internal server error occurred";
                break;
            default:
                msg = "An error occurred: " + error.kind;
        }
        Toastify({
            text: msg,
            duration: 5000,
            close: true,
            gravity: "top",
            position: "center",
            stopOnFocus: true,
            style: {
                background: "#f87171",
                borderRadius: "4px",
            },
        }).showToast();
        runBtn.disabled = false;
        throw new Error(`Request failed with error ${error.kind}`);
    }

    const wasm_size = res.headers.get("wasm-content-length");
    const js_size = res.headers.get("js-content-length");

    const body = await res.blob();
    const wasm = body.slice(0, wasm_size, "application/wasm");
    const js = body.slice(wasm_size, wasm_size + js_size, "application/javascript");
    const js_text = await js.text();

    const AsyncFunction = async function () { }.constructor;
    const load = new AsyncFunction("wasm_blob", js_text);
    await load(wasm).catch((error) => {
        if (!error.message.startsWith("Using exceptions for control flow, don't mind me. This isn't actually an error!")) {
            throw error;
        }
    });

    const gameCanvas = document.querySelector('canvas[alt="App"]');
    gameElement.appendChild(gameCanvas);
    gameCanvas.style.width = "800px";
    gameCanvas.style.height = null;

    runBtn.disabled = false;
}
