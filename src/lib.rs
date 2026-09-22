pub mod app;
pub mod components;
pub mod routes;

use wasm_bindgen::prelude::*;

use app::App;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
    Ok(())
}
