const imports = __wbg_get_imports();
__wbg_init_memory(imports);
const input = await wasm_blob.arrayBuffer();
const { instance, module } = await __wbg_load(input, imports);
return __wbg_finalize_init(instance, module);