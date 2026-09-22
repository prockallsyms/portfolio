use yew::prelude::*;
use yew_router::prelude::*;

pub mod about;
pub mod home;

use about::About;
use home::Home;

/// App routes
#[derive(Clone, Routable, PartialEq, Debug)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/page-not-found")]
    NotFound,
}

/// Switch app routes
pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html! { <Home /> },
        AppRoute::About => html! { <About /> },
        AppRoute::NotFound => html! { "Page not found" },
    }
}
