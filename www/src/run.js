function run(wasm, js) {
    // For some reason the js will never return so I have to use this object to get the nessessery values out :(
    let refObj: any = new Object();
    const AsyncFunction: any = async function () {}.constructor;
    const load = new AsyncFunction("wasm_blob", "ref_obj", js);
    await load(wasm, refObj).catch((error: { message: string }) => {
        if (
            !error.message.startsWith(
                "Using exceptions for control flow, don't mind me. This isn't actually an error!"
            )
        ) {
            throw error;
        }
    });
    return refObj.wasm;
}
