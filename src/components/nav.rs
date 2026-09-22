//! Sticky top navigation bar with active-route highlighting and the theme toggle.

use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;
use crate::components::ThemeToggle;

/// Nav links, in display order.
const LINKS: &[(AppRoute, &str)] = &[
    (AppRoute::Home, "Home"),
    (AppRoute::About, "About"),
    (AppRoute::Projects, "Projects"),
];

/// Sticky top bar: nav links (active one highlighted) + theme toggle.
#[function_component(Nav)]
pub fn nav() -> Html {
    // Called once, outside the loop — hooks may not run inside `for`.
    let route = use_route::<AppRoute>();

    html! {
        <nav
            aria-label="Primary"
            class="sticky top-0 z-10 border-b border-slate-200 bg-white/80 backdrop-blur dark:border-slate-800 dark:bg-slate-950/80"
        >
            <div class="mx-auto flex w-full max-w-3xl items-center justify-between px-4 py-3">
                <ul class="flex gap-4">
                    { LINKS.iter().map(|(to, label)| {
                        let active = route.as_ref() == Some(to);
                        html! {
                            <li aria-current={if active { Some("page") } else { None }}>
                                <Link<AppRoute>
                                    to={to.clone()}
                                    classes={classes!(
                                        "py-2",
                                        "text-sm",
                                        "font-medium",
                                        "rounded",
                                        "focus-visible:outline-2",
                                        "focus-visible:outline-accent",
                                        active.then_some("text-slate-900"),
                                        active.then_some("dark:text-slate-100"),
                                        (!active).then_some("text-slate-500"),
                                        (!active).then_some("hover:text-slate-800"),
                                        (!active).then_some("dark:text-slate-400"),
                                        (!active).then_some("dark:hover:text-slate-200")
                                    )}
                                >
                                    { label }
                                </Link<AppRoute>>
                            </li>
                        }
                    }).collect::<Html>() }
                </ul>
                <ThemeToggle />
            </div>
        </nav>
    }
}
