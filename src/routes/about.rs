use yew::prelude::*;

/// About page (placeholder until T07)
#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="app">
            <header class="app-header">
                <p>
                    { "About page — real content lands in T07." }
                </p>
            </header>
        </div>
    }
}
