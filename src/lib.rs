//! Portfolio SPA — crate root and wasm entry point.

use wasm_bindgen::prelude::*;

mod app;
mod components;
mod routes;

pub use app::App;

/// Starts the Yew app (called from `static/main.js`).
#[wasm_bindgen]
pub fn run() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Warn));
    yew::Renderer::<App>::new().render();
}
