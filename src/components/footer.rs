//! Site footer: contact link and a one-liner.

use yew::prelude::*;

use crate::data::DATA;

/// Footer with the GitHub link and a short one-liner (no name, no email, GitHub only).
#[function_component(Footer)]
pub fn footer() -> Html {
    let person = &DATA.person;

    html! {
        <footer class="border-t border-[var(--border)]">
            <div class="mx-auto flex w-full max-w-5xl flex-col items-center gap-3 px-6 py-10 text-sm sm:flex-row sm:justify-between">
                <p class="font-mono text-xs text-[var(--fg-faint)]">{ person.handle }</p>
                <div class="flex gap-5 font-mono text-xs uppercase tracking-[0.1em] text-[var(--fg-muted)]">
                    <a
                        href={person.github}
                        target="_blank"
                        rel="me noopener"
                        class="underline-offset-4 hover:text-[var(--fg)] hover:underline"
                    >
                        { "GitHub" }
                    </a>
                </div>
            </div>
        </footer>
    }
}
