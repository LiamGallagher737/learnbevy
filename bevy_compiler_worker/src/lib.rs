use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use worker::*;

#[event(fetch)]
async fn main(mut req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    if req.method() != Method::Post {
        return Response::error("Method not allowed", 405);
    }

    let body = req.text().await?;
    if body.is_empty() {
        return Response::error("Body must not be empty", 400);
    }
    let minified_body = rust_minify::minify(&body);
    let hash = fastmurmur3::hash(minified_body.unwrap_or_else(|_| body.clone()).as_bytes());

    let cache_url = format!("{}/__hash{hash}__", req.url()?);
    let cache_key = Request::new_with_init(
        &cache_url,
        RequestInit::new()
            .with_method(Method::Get)
            .with_headers(req.headers().to_owned()),
    )?;

    let cache = Cache::default();
    if let Some(cached_response) = cache.get(&cache_key, false).await? {
        return Ok(cached_response);
    }

    let global = js_sys::global()
        .dyn_into::<web_sys::ServiceWorkerGlobalScope>()
        .unwrap();

    let mut opts = web_sys::RequestInit::new();
    opts.method("POST");
    opts.headers(&JsValue::from(req.headers().0.clone()));
    opts.body(Some(&JsValue::from(body)));

    let request = web_sys::Request::new_with_str_and_init(&req.url()?.to_string(), &opts)?;
    let resp_value = JsFuture::from(global.fetch_with_request(&request)).await?;
    let resp: web_sys::Response = resp_value.dyn_into().unwrap();
    let blob: web_sys::Blob = JsFuture::from(resp.blob()?).await?.dyn_into()?;

    Ok(Response::from_body(ResponseBody::Stream(blob.stream()))?
        .with_headers(Headers(resp.headers()))
        .with_status(resp.status()))
}
