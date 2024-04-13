import type { Version } from "$lib/versions";
import type { Channel } from "$lib/channels";

type CompileArgs = {
    code: string;
    version: Version;
    channel: Channel;
    parentId: string;
};

export async function play(args: CompileArgs): Promise<PlayResponse> {
    const res = await fetch("https://compile.learnbevy.com/compile", {
        method: "POST",
        body: JSON.stringify({
            code: args.code,
            version: args.version,
            channel: args.channel,
        }),
        headers: {
            "Content-Type": "application/json",
        },
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
            case "Internal":
                msg = "An internal server error occurred";
                break;
        }
        return {
            kind: "Failed",
            message: msg,
            stderr: error.kind === "BuildFailed" ? error.stderr : null,
        };
    }

    const wasmSize = parseInt(res.headers.get("wasm-content-length")!);
    const jsSize = parseInt(res.headers.get("js-content-length")!);

    const body = await res.blob();

    const wasm = body.slice(0, wasmSize, "application/wasm");
    const js = body.slice(wasmSize, wasmSize + jsSize, "application/javascript");
    const stderr = body.slice(wasmSize + jsSize, -1, "text/plain");

    const jsText = await js.text();
    const stderrText = await stderr.text();

    // for some reason the js will never return so i have to use this object to get the nessessery values out :(
    let refObj: any = new Object();
    const AsyncFunction: any = async function () {}.constructor;
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

    const gameCanvas: HTMLCanvasElement | null =
        document.querySelector('canvas[alt="App"]') ??
        document.querySelector('canvas[alt="Bevy App"]');
    if (!gameCanvas) {
        return { kind: "ConsoleOnly", wasm: refObj.wasm, stderr: stderrText };
    }
    const parent = document.getElementById(args.parentId)!;
    parent.appendChild(gameCanvas);
    window.addEventListener("resize", () => {
        gameCanvas.style.width = `${parent.clientWidth}px`;
        gameCanvas.style.height = `${parent.clientWidth * (9 / 16)}px`;
    });
    gameCanvas.style.width = `${parent.clientWidth}px`;
    gameCanvas.style.height = `${parent.clientWidth * (9 / 16)}px`;
    gameCanvas.style.borderRadius = "0.5rem";

    return { kind: "Success", gameCanvas, wasm: refObj.wasm, stderr: stderrText };
}

type PlayResponse = Success | ConsoleOnly | Failed;
type Success = {
    kind: "Success";
    gameCanvas: HTMLCanvasElement;
    wasm: any;
    stderr: string;
};
type ConsoleOnly = {
    kind: "ConsoleOnly";
    wasm: any;
    stderr: string;
};
type Failed = {
    kind: "Failed";
    message: string;
    stderr: string | null;
};

type BcaError =
    | RateLimitError
    | CFRateLimitError
    | ActiveRequestExistsError
    | DisallowedWordError
    | BuildFailedError
    | InternalError;
type RateLimitError = {
    kind: "RateLimit";
    time_left: number;
};
type CFRateLimitError = {
    kind: "CFRateLimit";
};
type ActiveRequestExistsError = {
    kind: "ActiveRequestExists";
};
type DisallowedWordError = {
    kind: "DisallowedWord";
    word: string;
};
type BuildFailedError = {
    kind: "BuildFailed";
    stdout: string;
    stderr: string;
};
type InternalError = {
    kind: "Internal";
};
