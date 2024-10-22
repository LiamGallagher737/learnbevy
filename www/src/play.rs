use shared::{compile::*, BevyVersion, RustChannel};
use wasm_bindgen::prelude::*;

pub const HOST: &str = "https://slc.compute.learnbevy.com";
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

    let wasm = &bytes[0..wasm_length];
    let js = String::from_utf8_lossy(&bytes[wasm_length..js_length]).to_string();
    let stderr = String::from_utf8_lossy(&bytes[js_length..]).to_string();

    let instance = run(wasm, js).await;

    let document = web_sys::window().unwrap().document().unwrap();
    let parent = document.get_element_by_id(CANVAS_PARENT_ID).unwrap();

    let Ok(Some(canvas)) = document.query_selector("canvas[alt=\"Bevy App\"]") else {
        return Ok(Ok(PlayResponse { instance, stderr }));
    };

    let _ = parent.append_child(&canvas);

    Ok(Ok(PlayResponse { instance, stderr }))
}

pub struct PlayResponse {
    pub instance: Instance,
    pub stderr: String,
}

fn parse_length_header(response: &reqwest::Response, name: &str) -> Option<usize> {
    response.headers().get(name)?.to_str().ok()?.parse().ok()
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
    async fn run(wasm: &[u8], js: String) -> Instance;
}
