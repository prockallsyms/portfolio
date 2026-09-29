//! Browser render tests — the real App rendered headlessly in Chrome.
//!
//! Run with `wasm-pack test --headless --chrome` (T08 DoD). Each test pushes
//! the browser to a route, renders `portfolio::App` into a fresh detached div
//! (`BrowserRouter` reads the pushed location at mount), awaits
//! `yew::scheduler::flush()` — the initial render is deferred to the
//! microtask queue — and asserts on the rendered DOM text.

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn window() -> web_sys::Window {
    match web_sys::window() {
        Some(w) => w,
        None => panic!("test needs a browser window"),
    }
}

/// Push the browser to `path` before the App mounts.
fn goto(path: &str) {
    let history = match window().history() {
        Ok(h) => h,
        Err(e) => panic!("history() failed: {e:?}"),
    };
    history
        .push_state_with_url(&JsValue::NULL, "", Some(path))
        .unwrap_or_else(|e| panic!("pushState({path}) failed: {e:?}"));
}

/// Render the real App into a fresh detached div, flush the render queue,
/// and return the div's inner HTML.
async fn rendered_app_html() -> String {
    let doc = match window().document() {
        Some(d) => d,
        None => panic!("test needs a document"),
    };
    let root = match doc.create_element("div") {
        Ok(el) => el,
        Err(e) => panic!("create_element failed: {e:?}"),
    };
    yew::Renderer::<portfolio::App>::with_root(root.clone()).render();
    yew::scheduler::flush().await;
    root.inner_html()
}

/// Home route: handle in the hero, all three nav links present.
#[wasm_bindgen_test]
async fn home_route_renders_identity_and_nav() {
    goto("/");
    let html = rendered_app_html().await;
    assert!(html.contains("prockallsyms"), "home must show the handle");
    assert!(html.contains("/about"), "nav must link to /about");
    assert!(html.contains("/projects"), "nav must link to /projects");
}

/// About route: work history and education render.
#[wasm_bindgen_test]
async fn about_route_renders_history() {
    goto("/about");
    let html = rendered_app_html().await;
    assert!(html.contains("ELTON"), "about must show the ELTON roles");
    assert!(
        html.contains("Truman State University"),
        "about must show education"
    );
}

/// Projects route: all fourteen repo cards render.
#[wasm_bindgen_test]
async fn projects_route_renders_all_repos() {
    goto("/projects");
    let html = rendered_app_html().await;
    for title in [
        "corrode",
        "analyzer",
        "TestAssist",
        "hookdb",
        "malpraxis",
        "collector-rs",
        "irda",
        "classfile-parser",
        "pharos",
        "nrf-toolkit",
        "can-toolkit",
        "sastblast",
        "sast-rules",
        "rules",
    ] {
        assert!(html.contains(title), "projects must show {title}");
    }
}

/// Unknown route: the 404 page renders (coffee cup included).
#[wasm_bindgen_test]
async fn unknown_route_renders_404() {
    goto("/definitely-not-a-route");
    let html = rendered_app_html().await;
    assert!(html.contains("404"), "unknown route must show the 404 page");
    assert!(
        html.contains("coffee_cup.gif"),
        "404 page shows the coffee gif"
    );
}
