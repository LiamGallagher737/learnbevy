use std::ops::Deref;

use gloo_file::ObjectUrl;
use js_sys::{Array, ArrayBuffer, Promise, Uint8Array};
use shared::{compile::*, BevyVersion, RustChannel};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{BlobPropertyBag, Element};

//pub const HOST: &str = "https://slc.compute.learnbevy.com";
pub const HOST: &str = "http://localhost:3000";
pub const CANVAS_PARENT_ID: &str = "game-card";

pub async fn play(
    code: String,
    version: BevyVersion,
    channel: RustChannel,
) -> Result<Result<PlayResponse, shared::Error>, reqwest::Error> {
    let url = format!("{HOST}/compile/{version}/{channel}");
    let request = CompileRequest { code };

    let client = reqwest::Client::new();
    let response = client.post(url).json(&request).send().await?;

    if !response.status().is_success() {
        let error = response.json::<shared::Error>().await?;
        return Ok(Err(error));
    }

    let wasm_length = parse_length_header(&response, "wasm-content-length").unwrap();
    let js_length = parse_length_header(&response, "js-content-length").unwrap();
    let bytes = response.bytes().await?;

    let wasm = Uint8Array::from(&bytes[0..wasm_length]).buffer();
    let js = &bytes[wasm_length..wasm_length + js_length];
    let stderr = String::from_utf8_lossy(&bytes[wasm_length + js_length..]).to_string();

    let js_array = Array::new();
    js_array.push(&Uint8Array::from(js));
    let js_type_set = BlobPropertyBag::new();
    js_type_set.set_type("application/javascript");

    let blob = gloo_file::Blob::new_with_options(js, Some("text/javascript"));

    let module_address = ObjectUrl::from(blob);

    let module_promise: Promise = js_sys::eval(&format!("import (\"{}\")", module_address.deref()))
        .unwrap()
        .into();
    let module: InstanceModule = JsFuture::from(module_promise).await.unwrap().into();

    // The error is just for control flow should not blow up the website.
    let _ = module.start(wasm).await;

    let document = web_sys::window().unwrap().document().unwrap();
    let parent = document.get_element_by_id(CANVAS_PARENT_ID).unwrap();

    let Ok(Some(canvas)) = document.query_selector("canvas[alt=\"App\"]") else {
        return Ok(Ok(PlayResponse {
            module,
            stderr,
            canvas: None,
        }));
    };

    let _ = canvas.set_attribute("class", "rounded-lg absolute top-0 left-0");
    let _ = parent.append_child(&canvas);

    Ok(Ok(PlayResponse {
        module,
        stderr,
        canvas: Some(canvas),
    }))
}

pub struct PlayResponse {
    pub module: InstanceModule,
    pub stderr: String,
    pub canvas: Option<Element>,
}

fn parse_length_header(response: &reqwest::Response, name: &str) -> Option<usize> {
    response.headers().get(name)?.to_str().ok()?.parse().ok()
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type InstanceModule;

    #[wasm_bindgen(method, catch)]
    pub async fn start(this: &InstanceModule, wasm: ArrayBuffer) -> Result<(), JsValue>;

    #[wasm_bindgen(method)]
    pub fn exit(this: &InstanceModule);
}
