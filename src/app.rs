//! App root: router, layout, and route switching.

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{Footer, Nav};
use crate::pages::{About, Home, NotFound, Projects};

/// App routes.
#[derive(Clone, Routable, PartialEq, Debug)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/projects")]
    Projects,
    #[not_found]
    #[at("/page-not-found")]
    NotFound,
}

/// The portfolio app: sticky nav, routed main content, footer.
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Nav />
            <main class="mx-auto w-full max-w-3xl px-4 py-10">
                <Switch<AppRoute> render={Callback::from(switch)} />
            </main>
            <Footer />
        </BrowserRouter>
    }
}

/// Switch app routes to pages.
pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html! { <Home /> },
        AppRoute::About => html! { <About /> },
        AppRoute::Projects => html! { <Projects /> },
        AppRoute::NotFound => html! { <NotFound /> },
    }
}
