//! One portfolio project card: title link, blurb, tag badges.

use yew::prelude::*;

use crate::components::TagBadge;
use crate::data::Project;

/// Props for [`ProjectCard`].
#[derive(Properties, PartialEq)]
pub struct ProjectCardProps {
    /// The project to display.
    pub project: &'static Project,
}

/// A project card: title linking to the repo, blurb, and tag badges.
#[function_component(ProjectCard)]
pub fn project_card(props: &ProjectCardProps) -> Html {
    let project = props.project;

    html! {
        <article class="rounded-lg border border-slate-200 p-4 transition-colors hover:border-accent/60 hover:shadow-sm dark:border-slate-800">
            <h3 class="font-semibold">
                <a
                    href={project.url}
                    target="_blank"
                    rel="me noopener"
                    class="underline-offset-4 hover:underline hover:text-slate-700 focus-visible:outline-2 focus-visible:outline-accent dark:hover:text-slate-200"
                >
                    { project.title }
                </a>
            </h3>
            <p class="mt-2 text-sm text-slate-600 dark:text-slate-300">{ project.blurb }</p>
            <div class="mt-3 flex flex-wrap gap-2">
                { project.tags.iter().map(|tag| html! { <TagBadge label={*tag} /> }).collect::<Html>() }
            </div>
        </article>
    }
}
