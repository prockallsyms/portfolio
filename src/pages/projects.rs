//! Projects page: the full project grid.

use yew::prelude::*;

use crate::components::ProjectCard;
use crate::data::DATA;

/// Projects: one card per project in `DATA.projects`.
#[function_component(Projects)]
pub fn projects() -> Html {
    let empty = DATA.projects.is_empty();

    html! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">{"Projects"}</h1>
            { if empty {
                html! {
                    <p class="max-w-prose text-slate-600 dark:text-slate-300">
                        { "Project cards land here with the real content." }
                    </p>
                }
            } else {
                html! {
                    <div class="grid gap-4 sm:grid-cols-2">
                        { DATA.projects.iter().map(|p| html! { <ProjectCard project={p} /> }).collect::<Html>() }
                    </div>
                }
            } }
        </div>
    }
}
