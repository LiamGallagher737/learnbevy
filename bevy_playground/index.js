const runBtn = document.getElementById('button-run');
const shareBtn = document.getElementById('button-share');
const gameSizeInput = document.getElementById('input-game-size');
const editorElement = document.getElementById('editor');
const gameElement = document.getElementById('game');
const consoleElement = document.getElementById('console-content');

runBtn.addEventListener('click', run);
shareBtn.addEventListener('click', share);

let startingCode =
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
    info!("Here is some info");
    warn!("Here is a warning");
    error!("Here is an error");
}

fn change_clear_color(input: Res<Input<KeyCode>>, mut clear_color: ResMut<ClearColor>, mut state: Local<bool>) {
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

const urlParams = new URLSearchParams(window.location.search);
const share_id = urlParams.get('share');
if (share_id) {
    const res = await fetch(`/api/share/${share_id}`);
    if (res.ok) {
        startingCode = await res.json().then((json) => json.code);
    } else if (res.status === 404) {
        showToast(`Share "${share_id} not found"`, "#f87171");
    } else {
        showToast(`Failed to fetch share"`, "#f87171");
    }
}

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

gameSizeInput.addEventListener("input", (event) => {
    document.querySelector('main').style.gridTemplateColumns = `1fr ${event.target.value}px`;
    gameElement.style.height = `${event.target.value * 9 / 16}px`;
    const gameCanvas = document.querySelector('game-canvas');
    if (gameCanvas) gameCanvas.style.width = `${event.target.value}px`;
    editor.layout();
});

const origConsoleLog = console.log;
console.log = (...args) => {
    origConsoleLog.apply(console, args);
    if (args[0]?.startsWith("%c") && !args[0]?.includes("GPU lacks support")) {
        consoleElement.innerHTML += "\n" + args[0].replaceAll("%c", "");
    }
};

let runningWasm = null;

export async function run() {
    if (runningWasm) {
        runningWasm.__exit();
        runningWasm = null;
    }
    runBtn.disabled = true;
    gameElement.innerHTML = "";
    consoleElement.innerHTML = "";
    const code = editor.getValue();

    const res = await fetch("https://compile.learnbevy.com/", {
        method: "POST",
        body: code,
    });

    if (!res.ok) {
        const error = await res.json();

        if (error.kind === "BuildFailed") {
            consoleElement.innerHTML = error.stderr;
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
            case "DisallowedWord":
                msg = `Your code contains a disallowed word: "${error.word}"`;
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
        showToast(msg, "#f87171");
        runBtn.disabled = false;
        return;
    }

    const wasm_size = parseInt(res.headers.get("wasm-content-length"));
    const js_size = parseInt(res.headers.get("js-content-length"));

    const body = await res.blob();

    const wasm = body.slice(0, wasm_size, "application/wasm");
    const js = body.slice(wasm_size, wasm_size + js_size, "application/javascript");
    const stderr = body.slice(wasm_size + js_size, -1, "text/plain");

    const js_text = await js.text();
    const stderr_text = await stderr.text();

    let ref_obj = new Object();
    const AsyncFunction = async function () { }.constructor;
    const load = new AsyncFunction("wasm_blob", "ref_obj", js_text);
    await load(wasm, ref_obj).catch((error) => {
        if (!error.message.startsWith("Using exceptions for control flow, don't mind me. This isn't actually an error!")) {
            throw error;
        }
    });
    runningWasm = ref_obj.wasm;

    const gameCanvas = document.querySelector('canvas[alt="App"]');
    gameElement.appendChild(gameCanvas);
    gameCanvas.style.width = `${gameSizeInput.value}px`;
    gameCanvas.style.height = null;
    gameCanvas.id = "game-canvas";

    consoleElement.innerHTML = stderr_text + '\n';

    runBtn.disabled = false;
}

export async function share() {
    const code = editor.getValue();
    const res = await fetch('/api/share', {
        method: "POST",
        body: code,
    });

    if (!res.ok) {
        showToast("An error occurred when creating a share", "#f87171");
        return;
    }

    const id = await res.json().then((json) => json.id);
    const url = `https://play.learnbevy.com?share=${id}`;
    navigator.clipboard.writeText(url);
    window.history.replaceState({}, "", `/?share=${id}`);
    showToast(`Share copied to clipboard: ${url}`, "#16a34a");
}

/**
 * @param {string} text 
 * @param {string} background 
 */
function showToast(text, background) {
    Toastify({
        text,
        duration: 5000,
        close: true,
        gravity: "top",
        position: "center",
        stopOnFocus: true,
        style: {
            background,
            borderRadius: "4px",
        },
    }).showToast();
}
