//! Site footer: contact links and a one-liner.

use yew::prelude::*;

use crate::data::DATA;

/// Footer with GitHub/LinkedIn links and a short one-liner (no name, no email).
#[function_component(Footer)]
pub fn footer() -> Html {
    let person = &DATA.person;

    html! {
        <footer class="border-t border-slate-200 py-8 dark:border-slate-800">
            <div class="mx-auto flex w-full max-w-3xl flex-col items-center gap-2 px-4 text-sm">
                <div class="flex gap-4 font-medium">
                    <a href={person.github} target="_blank" rel="me noopener" class="underline">
                        { "GitHub" }
                    </a>
                    <a href={person.linkedin} target="_blank" rel="me noopener" class="underline">
                        { "LinkedIn" }
                    </a>
                </div>
                <p class="text-slate-500 dark:text-slate-400">{ person.handle }</p>
            </div>
        </footer>
    }
}
