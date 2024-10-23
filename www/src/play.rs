use dioxus_logger::tracing::info;
use gloo_file::ObjectUrl;
use js_sys::{Array, Promise, Uint8Array};
use shared::{compile::*, BevyVersion, RustChannel};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::BlobPropertyBag;

//pub const HOST: &str = "https://slc.compute.learnbevy.com";
pub const HOST: &str = "http://localhost:3000";
//pub const CANVAS_PARENT_ID: &str = "game-card";

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

    let wasm = Uint8Array::from(&bytes[0..wasm_length]);
    let js = &bytes[wasm_length..wasm_length + js_length];
    let stderr = String::from_utf8_lossy(&bytes[wasm_length + js_length..]).to_string();

    let js_array = Array::new();
    js_array.push(&Uint8Array::from(js));
    let js_type_set = BlobPropertyBag::new();
    js_type_set.set_type("application/javascript");

    let blob = gloo_file::Blob::new_with_options(js, Some("text/javascript"));
    let module_address = ObjectUrl::from(blob).to_string();

    info!("2: {module_address}");

    let module_promise: Promise = js_sys::eval(&format!("import (\"{module_address}\")"))
        .unwrap()
        .into();
    info!("3");
    let module: InstanceModule = JsFuture::from(module_promise).await.unwrap().into();
    info!("4");

    let wasm_blob = web_sys::Blob::new_with_u8_array_sequence(&wasm.clone().into()).unwrap();
    info!("5");
    module.start(wasm_blob).await;
    info!("6");

    //let instance = run(wasm_blob, js).await;

    //let document = web_sys::window().unwrap().document().unwrap();
    //let parent = document.get_element_by_id(CANVAS_PARENT_ID).unwrap();

    //let Ok(Some(canvas)) = document.query_selector("canvas[alt=\"Bevy App\"]") else {
    //    return Ok(Ok(PlayResponse { instance, stderr }));
    //};

    //let _ = parent.append_child(&canvas);

    Ok(Ok(PlayResponse { module, stderr }))
}

pub struct PlayResponse {
    pub module: InstanceModule,
    pub stderr: String,
}

fn parse_length_header(response: &reqwest::Response, name: &str) -> Option<usize> {
    response.headers().get(name)?.to_str().ok()?.parse().ok()
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type InstanceModule;

    #[wasm_bindgen(method)]
    pub async fn start(this: &InstanceModule, wasm: web_sys::Blob);

    #[wasm_bindgen(method)]
    pub fn exit(this: &InstanceModule);
}

#[wasm_bindgen]
extern "C" {
    pub type Instance;

    // Stop this instance from running. Any method invokations after calling
    // this will do nothing.
    #[wasm_bindgen(method)]
    fn exit(this: &Instance);
}

#[wasm_bindgen(module = "/src/run.js")]
extern "C" {
    async fn run(wasm: web_sys::Blob, js: String) -> Instance;
}
