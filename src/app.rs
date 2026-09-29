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
            <a
                href="#main-content"
                class="sr-only focus:not-sr-only focus:absolute focus:top-2 focus:left-2 focus:z-50 focus:rounded-sm focus:border focus:border-[var(--border-strong)] focus:bg-[var(--surface)] focus:px-3 focus:py-2 focus:font-mono focus:text-sm focus:text-[var(--fg)]"
            >{"Skip to content"}</a>
            <Nav />
            <main id="main-content" class="mx-auto w-full max-w-5xl px-6 py-16 sm:py-24">
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
