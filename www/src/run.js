export async function run(wasm, js) {
    console.log("12432")
    // For some reason the js will never return so I have to use this object to get the nessessery values out :(
    // let refObj = new Object();
    // const AsyncFunction = async function () {}.constructor;
    // const load = new AsyncFunction("wasm_blob", "ref_obj", js);
    // await load(wasm, refObj).catch((error) => {
    //     if (
    //         !error.message.startsWith(
    //             "Using exceptions for control flow, don't mind me. This isn't actually an error!"
    //         )
    //     ) {
    //         throw error;
    //     }
    // });
    // return refObj.wasm;
}
