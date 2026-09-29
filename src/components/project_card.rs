//! One portfolio project row: index, title link, blurb, tag badges.

use yew::prelude::*;

use crate::components::TagBadge;
use crate::data::Project;

/// Props for [`ProjectCard`].
#[derive(Properties, PartialEq)]
pub struct ProjectCardProps {
    /// The project to display.
    pub project: &'static Project,
    /// 1-based position in the list — rendered as a mono index label.
    pub index: usize,
}

/// A project row: numbered, title linking to the repo, blurb, and tag badges.
#[function_component(ProjectCard)]
pub fn project_card(props: &ProjectCardProps) -> Html {
    let project = props.project;

    html! {
        <article class="scroll-reveal group border-t border-[var(--border)] py-5 first:border-t-0 sm:py-6">
            <div class="flex items-baseline gap-3">
                <span class="label shrink-0 tabular-nums">{ format!("[{:02}]", props.index) }</span>
                <h3 class="font-semibold">
                    <a
                        href={project.url}
                        target="_blank"
                        rel="me noopener"
                        class="underline-offset-4 transition-colors group-hover:text-[var(--accent-text)] hover:underline"
                    >
                        { project.title }
                    </a>
                </h3>
            </div>
            <p class="mt-2 max-w-prose pl-8 text-sm text-[var(--fg-muted)]">{ project.blurb }</p>
            <div class="mt-3 flex flex-wrap gap-1.5 pl-8">
                { project.tags.iter().map(|tag| html! { <TagBadge label={*tag} /> }).collect::<Html>() }
            </div>
        </article>
    }
}
