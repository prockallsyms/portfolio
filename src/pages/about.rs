//! About page: bio, experience timeline, and education.

use yew::prelude::*;

use crate::components::Section;
use crate::data::DATA;

/// About: bio, work experience (newest first), and education.
#[function_component(About)]
pub fn about() -> Html {
    let person = &DATA.person;
    let show_experience = !DATA.experience.is_empty();
    let show_education = !DATA.education.is_empty();
    let show_certifications = !DATA.certifications.is_empty();
    let show_labs = !DATA.personal_labs.is_empty();

    html! {
        <div class="space-y-12">
            <h1 class="text-3xl font-bold">{"About"}</h1>
            <Section heading="Bio">
                <p class="max-w-prose">{ person.bio }</p>
            </Section>
            { if show_experience {
                html! {
                    <Section heading="Experience">
                        <ol class="space-y-4">
                            { DATA.experience.iter().map(|item| html! {
                                <li class="space-y-1">
                                    <h3 class="font-semibold">
                                        { item.role }{ " — " }{ item.org }
                                    </h3>
                                    <p class="text-sm text-slate-500 dark:text-slate-400">
                                        { item.start }{ " – " }{ item.end.unwrap_or("present") }
                                    </p>
                                    { if item.notes.is_empty() {
                                        html! {}
                                    } else {
                                        html! { <p class="text-sm">{ item.notes }</p> }
                                    } }
                                </li>
                            }).collect::<Html>() }
                        </ol>
                    </Section>
                }
            } else { html! {} } }
            { if show_education {
                html! {
                    <Section heading="Education">
                        <ul class="space-y-2">
                            { DATA.education.iter().map(|item| html! {
                                <li>
                                    <span class="font-medium">{ item.degree }</span>
                                    { ", " }{ item.school }{ " (" }{ item.years }{ ")" }
                                </li>
                            }).collect::<Html>() }
                        </ul>
                    </Section>
                }
            } else { html! {} } }
            { if show_certifications {
                html! {
                    <Section heading="Certifications">
                        <p class="max-w-prose">{ DATA.certifications }</p>
                    </Section>
                }
            } else { html! {} } }
            { if show_labs {
                html! {
                    <Section heading="Personal labs">
                        <p class="max-w-prose">{ DATA.personal_labs }</p>
                    </Section>
                }
            } else { html! {} } }
        </div>
    }
}
