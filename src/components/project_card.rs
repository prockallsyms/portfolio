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
        <article class="rounded-lg border border-slate-200 p-4 dark:border-slate-800">
            <h3 class="font-semibold">
                <a
                    href={project.url}
                    target="_blank"
                    rel="me noopener"
                    class="underline hover:text-slate-700 dark:hover:text-slate-200"
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
