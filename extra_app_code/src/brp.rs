use async_channel::Sender;
use bevy_ecs::system::Res;
use bevy_log::{debug, info};
use bevy_remote::{error_codes, BrpError, BrpMessage, BrpResult, BrpSender};
use std::sync::OnceLock;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

/// A lock containing the sender from [`bevy_remote::BrpSender`] used by the JS bindings.
static MESSAGE_SENDER: OnceLock<Sender<BrpMessage>> = OnceLock::new();

/// A system that sets up the static [`MESSAGE_SENDER`] for the Bevy Remote Protocol JS bindings.
pub fn setup(request_sender: Res<BrpSender>) {
    let _ = MESSAGE_SENDER.set(request_sender.clone());
}

/// A binding to JS that allows making BRP requests in a browser environment. If
/// the selected method does not need any params it should be left as undefined.
/// A successful request will return an array of results and if an error occurs
/// an object will be returned with an error code and a human readable message.
#[wasm_bindgen(js_name = "brpRequest")]
pub async fn brp_js_binding(method: String, params: JsValue) -> JsValue {
    debug!("Request: {method:?}\n{params:?}");
    let result = process_request(method, params).await;
    debug!("Result: {result:?}");
    match result {
        Ok(value) => match serde_wasm_bindgen::to_value(&value) {
            Ok(value) => value,
            Err(err) => serde_wasm_bindgen::to_value(&BrpError::internal(format!(
                "Failed to cast result to a JS value: {err}"
            )))
            .unwrap(),
        },
        Err(err) => serde_wasm_bindgen::to_value(&err).unwrap(),
    }
}

/// Handle a single BRP request from the JS binding
async fn process_request(method: String, params: JsValue) -> BrpResult {
    let params = if !params.is_undefined() {
        Some(
            serde_wasm_bindgen::from_value(params).map_err(|err| BrpError {
                code: error_codes::INVALID_REQUEST,
                message: format!("Invalid params: {err}"),
                data: None,
            })?,
        )
    } else {
        None
    };

    let request_sender = MESSAGE_SENDER
        .get()
        .ok_or(BrpError::internal("Failed to get message sender"))?;
    let (result_sender, result_receiver) = async_channel::bounded(1);

    let _ = request_sender
        .send(BrpMessage {
            method,
            params,
            sender: result_sender,
        })
        .await;

    result_receiver
        .recv()
        .await
        .map_err(|_| BrpError::internal("Failed to receive result"))?
}
