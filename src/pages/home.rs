//! Home page: hero (avatar, tagline, bio) + featured projects in the main
//! column, GitHub link + skills in the sticky rail.

use yew::prelude::*;

use crate::components::{ProjectCard, Section, TagBadge, TwoPane};
use crate::data::{DATA, Project};

/// Titles to feature on Home, in display order — explicit rather than
/// "whatever's first in `DATA.projects`" so the choice survives reordering
/// or additions to the full project list.
const FEATURED_TITLES: &[&str] = &["corrode", "collector-rs", "malpraxis"];

/// Home: hero + featured projects (main), contact + skills (rail).
#[function_component(Home)]
pub fn home() -> Html {
    let person = &DATA.person;
    let featured: Vec<&'static Project> = FEATURED_TITLES
        .iter()
        .map(|title| {
            DATA.projects
                .iter()
                .find(|p| p.title == *title)
                .unwrap_or_else(|| panic!("FEATURED_TITLES references unknown project {title:?}"))
        })
        .collect();
    let show_featured = !featured.is_empty();
    let show_skills = !DATA.skills.is_empty();

    let rail = html! {
        <>
            <div class="space-y-3">
                <span class="label text-[var(--accent-text)]">{"// contact"}</span>
                <div class="flex flex-col items-start gap-2 font-mono text-xs uppercase tracking-[0.1em]">
                    <a
                        href={person.github}
                        target="_blank"
                        rel="me noopener"
                        class="underline-offset-4 hover:text-[var(--accent-text)] hover:underline"
                    >
                        { "GitHub" }
                    </a>
                </div>
            </div>
            { if show_skills {
                html! {
                    <div class="space-y-4">
                        <span class="label text-[var(--accent-text)]">{"// skills"}</span>
                        { DATA.skills.iter().map(|group| html! {
                            <div class="space-y-2">
                                <h3 class="text-sm font-semibold">{ group.heading }</h3>
                                <div class="flex flex-wrap gap-1.5">
                                    { group.skills.iter().map(|skill| html! { <TagBadge label={*skill} /> }).collect::<Html>() }
                                </div>
                            </div>
                        }).collect::<Html>() }
                    </div>
                }
            } else { html! {} } }
        </>
    };

    html! {
        <TwoPane rail={rail}>
            <div class="space-y-16">
                <section class="animate-fade-up relative flex flex-col items-start gap-5 pb-2">
                    <div aria-hidden="true" class="grid-texture pointer-events-none absolute -inset-x-6 -top-16 -z-10 h-56"></div>
                    <div class="relative inline-block">
                        <img
                            src={person.avatar}
                            alt={format!("Avatar of {}", person.handle)}
                            class="h-16 w-16 rounded-sm border border-[var(--border-strong)] object-cover"
                        />
                        <span aria-hidden="true" class="absolute -left-1.5 -top-1.5 h-3 w-3 border-l border-t border-[var(--accent)]"></span>
                        <span aria-hidden="true" class="absolute -right-1.5 -top-1.5 h-3 w-3 border-r border-t border-[var(--accent)]"></span>
                        <span aria-hidden="true" class="absolute -bottom-1.5 -left-1.5 h-3 w-3 border-b border-l border-[var(--accent)]"></span>
                        <span aria-hidden="true" class="absolute -bottom-1.5 -right-1.5 h-3 w-3 border-r border-b border-[var(--accent)]"></span>
                    </div>
                    <div class="space-y-2">
                        <h1 class="text-2xl font-semibold tracking-tight">{ person.handle }</h1>
                        <p class="font-mono text-sm text-[var(--accent-text)]">
                            { person.tagline }
                            <span aria-hidden="true" class="cursor-blink">{"▌"}</span>
                        </p>
                    </div>
                    <p class="max-w-lg text-[var(--fg-muted)]">{ person.bio }</p>
                </section>
                { if show_featured {
                    html! {
                        <Section heading="Featured projects">
                            <div>
                                { featured.iter().enumerate().map(|(i, p)| html! {
                                    <ProjectCard project={p} index={i + 1} />
                                }).collect::<Html>() }
                            </div>
                        </Section>
                    }
                } else { html! {} } }
            </div>
        </TwoPane>
    }
}
