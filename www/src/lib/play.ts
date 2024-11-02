import type { Version } from "$lib/versions";
import type { Channel } from "$lib/channels";
import { env } from "$env/dynamic/public";

type CompileArgs = {
    code: string;
    version: Version;
    channel: Channel;
    parentId: string;
};

export async function play(args: CompileArgs): Promise<PlayResponse> {
    // Use the provided host if given
    const host = env.PUBLIC_COMPILE_HOST ?? "https://slc.compute.learnbevy.com";
    const url = `${host}/compile/${args.version}/${args.channel}`;
    // Make the request
    const res = await fetch(url, {
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

    // Handle any errors
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
            case "BadCode":
                msg = "The code failed to build";
                break;
            case "Internal":
                msg = "An internal server error occurred";
                break;
        }
        return {
            kind: "Failed",
            message: msg,
            stderr: error.kind === "BadCode" ? error.stderr : null,
        };
    }

    const wasmSize = parseInt(res.headers.get("wasm-content-length")!);
    const jsSize = parseInt(res.headers.get("js-content-length")!);

    const body = await res.blob();

    // Split the response in to its parts
    const wasm = body.slice(0, wasmSize, "application/wasm");
    const js = body.slice(wasmSize, wasmSize + jsSize, "application/javascript");
    const stderr = body.slice(wasmSize + jsSize, -1, "text/plain");

    // Convert js and stderr from bytes to strings
    const jsText = await js.text();
    const stderrText = await stderr.text();

    // For some reason the js will never return so I have to use this object to get the nessessery values out :(
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

    // Get the spawned canvas element if it exists
    const gameCanvas: HTMLCanvasElement | null =
        document.querySelector('canvas[alt="App"]') ??
        document.querySelector('canvas[alt="Bevy App"]');

    // Return if no canvas was spawned
    if (!gameCanvas) {
        return { kind: "ConsoleOnly", wasm: refObj.wasm, stderr: stderrText };
    }
    // Set the canvas's parent to the element with the given parentId
    const parent = document.getElementById(args.parentId)!;
    parent.appendChild(gameCanvas);
    // Add a new listener that resizes the canvas when the windows changes size
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
    kind: "BadCode";
    stderr: string;
};
type InternalError = {
    kind: "Internal";
};
