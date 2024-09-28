const imports = __wbg_get_imports();
__wbg_init_memory(imports);
const input = await wasm_blob.arrayBuffer();
const { instance, module } = await __wbg_load(input, imports);
ref_obj.wasm = instance.exports;
ref_obj.exit = __exit;
ref_obj.brpRequest = brpRequest;
__wbg_finalize_init(instance, module);
