//! Sticky top navigation bar with active-route highlighting and the theme toggle.

use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;
use crate::components::ThemeToggle;

/// Nav links, in display order.
const LINKS: &[(AppRoute, &str)] = &[
    (AppRoute::Home, "home"),
    (AppRoute::About, "about"),
    (AppRoute::Projects, "projects"),
];

/// Sticky top bar: mono `root@` brand mark, nav links (active one bracketed), theme toggle.
#[function_component(Nav)]
pub fn nav() -> Html {
    // Called once, outside the loop — hooks may not run inside `for`.
    let route = use_route::<AppRoute>();

    html! {
        <nav
            aria-label="Primary"
            class="sticky top-0 z-10 border-b border-[var(--border)] bg-[var(--bg)]/85 backdrop-blur"
        >
            <div class="mx-auto flex w-full max-w-5xl items-center justify-between px-6 py-4">
                <Link<AppRoute>
                    to={AppRoute::Home}
                    classes="font-mono text-sm text-[var(--fg)] hover:text-[var(--accent-text)]"
                >
                    { "prockallsyms@localhost" }
                </Link<AppRoute>>
                <ul class="flex items-center gap-4">
                    { LINKS.iter().map(|(to, label)| {
                        let active = route.as_ref() == Some(to);
                        html! {
                            <li aria-current={if active { Some("page") } else { None }}>
                                <Link<AppRoute>
                                    to={to.clone()}
                                    classes={classes!(
                                        "font-mono",
                                        "text-xs",
                                        "tracking-[0.02em]",
                                        "transition-colors",
                                        active.then_some("text-[var(--accent-text)]"),
                                        (!active).then_some("text-[var(--fg-muted)]"),
                                        (!active).then_some("hover:text-[var(--fg)]")
                                    )}
                                >
                                    { if active {
                                        html! { <>
                                            <span aria-hidden="true">{"["}</span>
                                            { label }
                                            <span aria-hidden="true">{"]"}</span>
                                        </> }
                                    } else {
                                        html! { { label } }
                                    } }
                                </Link<AppRoute>>
                            </li>
                        }
                    }).collect::<Html>() }
                    <li><ThemeToggle /></li>
                </ul>
            </div>
        </nav>
    }
}
