//! 404 page for unknown routes.

use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;

/// NotFound: a coffee cup, a one-liner, and a link home.
#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="animate-fade-up flex flex-col items-center gap-4 py-8 text-center">
            <span class="label">{"[ 404 ]"}</span>
            <img src="./assets/coffee_cup.gif" alt="" aria-hidden="true" class="h-20 w-20 rounded-sm border border-[var(--border)]" />
            <p class="max-w-sm text-[var(--fg-muted)]">
                { "This route doesn't exist — like a clean pentest report, it simply isn't here." }
            </p>
            <Link<AppRoute>
                to={AppRoute::Home}
                classes="font-mono text-xs uppercase tracking-[0.1em] text-[var(--accent-text)] underline-offset-4 hover:underline"
            >
                { "Back to the home page" }
            </Link<AppRoute>>
        </div>
    }
}
