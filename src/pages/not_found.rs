//! 404 page for unknown routes.

use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;

/// NotFound: a coffee cup, a one-liner, and a link home.
#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="flex flex-col items-center gap-4 py-16 text-center">
            <img src="./assets/coffee_cup.gif" alt="" class="h-24 w-24" />
            <p class="text-lg">
                { "This page is a 404 — even the coffee is out." }
            </p>
            <Link<AppRoute> to={AppRoute::Home} classes="text-sm underline">
                { "Back to the home page" }
            </Link<AppRoute>>
        </div>
    }
}
