import type { Version } from "$lib/versions";
import type { Channel } from "$lib/channels";

type CompileArgs = {
    code: string;
    version: Version;
    channel: Channel;
    parentId: string;
}

export async function play(args: CompileArgs) {
    const res = await fetch("https://compile.learnbevy.com/compile", {
        method: "POST",
        body: JSON.stringify({
            code: args.code,
            version: args.version,
            channel: args.channel,
        }),
        headers: {
            "Content-Type": "application/json",
        }
    });

    if (!res.ok) {
        console.log("fuck");
        console.log(await res.json());
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

    // for some reason the js will never return so i have to use this object to get the nessessery values out :(
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

    const gameCanvas: HTMLCanvasElement | null = document.querySelector('canvas[alt="App"]') ?? document.querySelector('canvas[alt="Bevy App"]');
    if (!gameCanvas) {
        return { gameCanvas: null, wasm: refObj.wasm, stderr: stderrText };
    }
    const parent = document.getElementById(args.parentId)!;
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

