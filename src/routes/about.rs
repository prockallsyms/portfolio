use yew::prelude::*;

/// About page
#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="app">
            <header class="app-header">
                <p>
                    { "Set up a modern yew web app by running one command." }
                </p>
                <p>
                    { "Repo fetch demo removed — real content lands in T07." }
                </p>
                <p>
                    { "Edit " } <code>{ "src/components/about.rs" }</code> { " and save to reload." }
                </p>
            </header>
        </div>
    }
}
