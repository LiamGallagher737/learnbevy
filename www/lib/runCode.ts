export async function run(code: string, parentId: string) {
    const res = await fetch("https://compile.learnbevy.com/", {
        method: "POST",
        body: code,
    });

    if (!res.ok) {
        const error: BcaError = await res.json();
        let msg = "";
        switch (error.kind) {
            case "RateLimit":
                msg = `Please wait ${error.time_left}s before submitting another request`;
                break;
            case "CFRateLimit":
                msg = "Please wait before submitting another request";
                break;
            case "ActiveRequestExists":
                msg =
                    "A request from your IP is currently being handled, please wait until it is complete";
                break;
            case "DisallowedWord":
                msg = `Your code contains a disallowed word: "${error.word}"`;
                break;
            case "BuildFailed":
                msg = "The code failed to build";
                break;
            case "Overloaded":
                msg =
                    "The server failed to process your request due to being overloaded";
                break;
            case "Internal":
                msg = "An internal server error occurred";
                break;
            default:
                msg = "An error occurred: " + error.kind;
        }
        throw new Error(msg, {
            cause: {
                stderr: error.kind === "BuildFailed" ? error.stderr : null,
            }
        });
    }

    const wasmSize = parseInt(res.headers.get("wasm-content-length")!);
    const jsSize = parseInt(res.headers.get("js-content-length")!);

    const body = await res.blob();

    const wasm = body.slice(0, wasmSize, "application/wasm");
    const js = body.slice(
        wasmSize,
        wasmSize + jsSize,
        "application/javascript"
    );
    const stderr = body.slice(wasmSize + jsSize, -1, "text/plain");

    const jsText = await js.text();
    const stderrText = await stderr.text();

    let refObj: any = new Object();
    const AsyncFunction: any = async function () { }.constructor;
    const load = new AsyncFunction("wasm_blob", "ref_obj", jsText);
    await load(wasm, refObj).catch((error: { message: string }) => {
        if (
            !error.message.startsWith(
                "Using exceptions for control flow, don't mind me. This isn't actually an error!"
            )
        ) {
            throw error;
        }
    });

    const gameCanvas: HTMLCanvasElement | null = document.querySelector('canvas[alt="App"]');
    if (!gameCanvas) {
        return { gameCanvas: null, wasm: refObj.wasm, stderr: stderrText };
    }
    const parent = document.getElementById(parentId)!;
    parent.appendChild(gameCanvas);
    window.addEventListener('resize', () => {
        gameCanvas.style.width = `${parent.clientWidth}px`;
        gameCanvas.style.height = `${parent.clientWidth * (9 / 16)}px`;
    });
    gameCanvas.style.width = `${parent.clientWidth}px`;
    gameCanvas.style.height = `${parent.clientWidth * (9 / 16)}px`;
    gameCanvas.style.borderRadius = "0.5rem";

    return { gameCanvas, wasm: refObj.wasm, stderr: stderrText };
}

type BcaError = RateLimitError | CFRateLimitError | ActiveRequestExistsError | InvalidBodyError | DisallowedWordError | BuildFailedError | OverloadedError | InternalError;

type RateLimitError = {
    kind: "RateLimit";
    time_left: number;
}

type CFRateLimitError = {
    kind: "CFRateLimit";
}

type ActiveRequestExistsError = {
    kind: "ActiveRequestExists";
}

type InvalidBodyError = {
    kind: "InvalidBody";
}

type DisallowedWordError = {
    kind: "DisallowedWord";
    word: string;
}

type BuildFailedError = {
    kind: "BuildFailed";
    stdout: string;
    stderr: string;
}

type OverloadedError = {
    kind: "Overloaded";
}

type InternalError = {
    kind: "Internal";
}
