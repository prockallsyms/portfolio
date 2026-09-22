//! Home page: hero (avatar, tagline, bio, contact links), featured projects, skills teaser.

use yew::prelude::*;

use crate::components::{ProjectCard, Section, TagBadge};
use crate::data::{DATA, Project};

/// Home: hero, up to three featured projects, and a skills teaser.
#[function_component(Home)]
pub fn home() -> Html {
    let person = &DATA.person;
    let featured: Vec<&'static Project> = DATA.projects.iter().take(3).collect();
    let show_featured = !featured.is_empty();
    let show_skills = !DATA.skills.is_empty();

    html! {
        <div class="space-y-12">
            <section class="flex flex-col items-center gap-4 text-center">
                <img
                    src={person.avatar}
                    alt={person.handle}
                    class="h-24 w-24 rounded-full"
                />
                <h1 class="text-3xl font-bold">{ person.handle }</h1>
                <p class="text-lg text-slate-600 dark:text-slate-300">{ person.tagline }</p>
                <p class="max-w-xl">{ person.bio }</p>
                <div class="flex gap-4 text-sm font-medium">
                    <a href={person.github} target="_blank" rel="me noopener" class="underline">
                        { "GitHub" }
                    </a>
                    <a href={person.linkedin} target="_blank" rel="me noopener" class="underline">
                        { "LinkedIn" }
                    </a>
                </div>
            </section>
            { if show_featured {
                html! {
                    <Section heading="Featured projects">
                        <div class="grid gap-4 sm:grid-cols-2">
                            { featured.iter().map(|p| html! { <ProjectCard project={p} /> }).collect::<Html>() }
                        </div>
                    </Section>
                }
            } else { html! {} } }
            { if show_skills {
                html! {
                    <Section heading="Skills">
                        <div class="space-y-3">
                            { DATA.skills.iter().map(|group| html! {
                                <div class="space-y-1">
                                    <h3 class="text-sm font-semibold">{ group.heading }</h3>
                                    <div class="flex flex-wrap gap-2">
                                        { group.skills.iter().map(|skill| html! { <TagBadge label={*skill} /> }).collect::<Html>() }
                                    </div>
                                </div>
                            }).collect::<Html>() }
                        </div>
                    </Section>
                }
            } else { html! {} } }
        </div>
    }
}
