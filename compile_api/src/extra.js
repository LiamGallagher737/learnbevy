// const imports = __wbg_get_imports();
// __wbg_init_memory(imports);

// const module = await wasm_blob.arrayBuffer();
// const instance = await WebAssembly.instantiate(module, imports);

// // return __wbg_finalize_init(instance, module);

// // wasm = instance.exports;
// // __wbg_init.__wbindgen_wasm_module = module;
// cachedFloat32Memory0 = null;
// cachedFloat64Memory0 = null;
// cachedInt32Memory0 = null;
// cachedUint32Memory0 = null;
// cachedUint8Memory0 = null;

// // wasm.__wbindgen_start();
// instance.exports.__wbindgen_start();
// // return wasm;

const imports = __wbg_get_imports();
__wbg_init_memory(imports);
const input = await wasm_blob.arrayBuffer();
const { instance, module } = await __wbg_load(input, imports);
return __wbg_finalize_init(instance, module);
