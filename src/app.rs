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
                class="sr-only focus:not-sr-only focus:absolute focus:top-2 focus:left-2 focus:z-50 focus:rounded focus:bg-white focus:px-3 focus:py-2 focus:text-sm focus:text-slate-900 focus:shadow-lg focus-visible:outline-2 focus-visible:outline-accent dark:focus:bg-slate-900 dark:focus:text-slate-100"
            >{"Skip to content"}</a>
            <Nav />
            <main id="main-content" class="mx-auto w-full max-w-3xl px-4 py-16 sm:px-6 sm:py-24">
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
