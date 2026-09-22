use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use portfolio::App;

#[wasm_bindgen_test]
fn app_has_a_home_page() {
    yew::Renderer::<App>::new().render();

    let learn_yew = gloo_utils::document()
        .get_element_by_id("learn_yew")
        .expect("No learn yew anchor or no home page")
        .inner_html();
    assert_eq!(learn_yew, "Learn Yew");
}
