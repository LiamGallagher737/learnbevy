use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[allow(unused_macros)]
macro_rules! dbg {
    () => {
        playground_lib:exports::log(&format_args!("%d{}:{}:{}", file!(), line!(), column!())).to_string()
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                playground_lib::exports::log(&format_args!("%d{}:{}:{} {} = {:?}",
                    file!(), line!(), column!(), stringify!($val), &tmp).to_string());
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(dbg!($val)),+,)
    };
}
