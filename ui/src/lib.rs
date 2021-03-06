mod app;
mod client;
mod config;
mod pages;
mod proto;
mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn run_app() -> Result<(), JsValue> {
    tracing_wasm::set_as_global_default();
    utils::set_panic_hook();
    yew::start_app::<app::App>();
    Ok(())
}
