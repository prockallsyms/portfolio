//! About page: bio + experience timeline in the main column, education /
//! certifications / personal labs as "quick facts" in the sticky rail.

use yew::prelude::*;

use crate::components::{Section, TwoPane};
use crate::data::DATA;

/// About: bio + experience (main), quick facts (rail).
#[function_component(About)]
pub fn about() -> Html {
    let person = &DATA.person;
    let show_experience = !DATA.experience.is_empty();
    let show_education = !DATA.education.is_empty();
    let show_certifications = !DATA.certifications.is_empty();
    let show_labs = !DATA.personal_labs.is_empty();

    let rail = html! {
        <>
            { if show_education {
                html! {
                    <div class="space-y-3">
                        <span class="label text-[var(--accent-text)]">{"// education"}</span>
                        <ul class="space-y-2 text-sm">
                            { DATA.education.iter().map(|item| html! {
                                <li>
                                    <span class="font-medium">{ item.degree }</span>
                                    <span class="text-[var(--fg-muted)]">
                                        { ", " }{ item.school }{ " (" }{ item.years }{ ")" }
                                    </span>
                                </li>
                            }).collect::<Html>() }
                        </ul>
                    </div>
                }
            } else { html! {} } }
            { if show_certifications {
                html! {
                    <div class="space-y-3">
                        <span class="label text-[var(--accent-text)]">{"// certifications"}</span>
                        <p class="text-sm text-[var(--fg-muted)]">{ DATA.certifications }</p>
                    </div>
                }
            } else { html! {} } }
            { if show_labs {
                html! {
                    <div class="space-y-3">
                        <span class="label text-[var(--accent-text)]">{"// personal labs"}</span>
                        <p class="text-sm text-[var(--fg-muted)]">{ DATA.personal_labs }</p>
                    </div>
                }
            } else { html! {} } }
        </>
    };

    html! {
        <TwoPane rail={rail}>
            <div class="animate-fade-up space-y-14">
                <h1 class="text-2xl font-semibold tracking-tight">{"About"}</h1>
                <Section heading="Bio">
                    <p class="max-w-prose text-[var(--fg-muted)]">{ person.bio }</p>
                </Section>
                { if show_experience {
                    html! {
                        <Section heading="Experience">
                            <ol class="space-y-6 border-l border-[var(--border)] pl-5">
                                { DATA.experience.iter().map(|item| html! {
                                    <li class="relative space-y-1">
                                        <span class="absolute -left-[23px] top-1.5 h-1.5 w-1.5 rounded-full bg-[var(--accent)]"></span>
                                        <h3 class="font-semibold">
                                            { item.role }
                                            <span class="text-[var(--fg-muted)]">{ " — " }{ item.org }</span>
                                        </h3>
                                        <p class="font-mono text-xs text-[var(--fg-faint)]">
                                            { item.start }{ " – " }{ item.end.unwrap_or("present") }
                                        </p>
                                        { if item.notes.is_empty() {
                                            html! {}
                                        } else {
                                            html! { <p class="max-w-prose text-sm text-[var(--fg-muted)]">{ item.notes }</p> }
                                        } }
                                    </li>
                                }).collect::<Html>() }
                            </ol>
                        </Section>
                    }
                } else { html! {} } }
            </div>
        </TwoPane>
    }
}
