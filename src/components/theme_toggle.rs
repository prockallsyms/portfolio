//! Dark/light theme toggle, persisted to `localStorage["theme"]`.

use yew::prelude::*;

/// Apply or remove the `.dark` class on `<html>`.
fn set_dark(dark: bool) {
    // web-sys 0.3.105 has no free `document()` — go through `window()`.
    let Some(win) = web_sys::window() else {
        return;
    };
    let Some(doc) = win.document() else {
        return;
    };
    let Some(root) = doc.document_element() else {
        return;
    };
    let list = root.class_list();
    if dark {
        let _ = list.add_1("dark");
    } else {
        let _ = list.remove_1("dark");
    }
}

/// Initial dark state, mirroring the inline bootstrap in `static/index.html`:
/// an explicit `localStorage["theme"]` wins, else `prefers-color-scheme`.
fn initial_dark() -> bool {
    let Some(win) = web_sys::window() else {
        return false;
    };
    let stored = win
        .local_storage()
        .ok()
        .flatten()
        .and_then(|storage| storage.get_item("theme").ok().flatten());
    match stored.as_deref() {
        Some("dark") => true,
        Some("light") => false,
        _ => win
            .match_media("(prefers-color-scheme: dark)")
            .ok()
            .flatten()
            .map(|media| media.matches())
            .unwrap_or(false),
    }
}

/// Toggles dark/light on the page and persists the choice.
#[function_component(ThemeToggle)]
pub fn theme_toggle() -> Html {
    let dark = use_state(initial_dark);
    let on_click = {
        let dark = dark.clone();
        Callback::from(move |_| {
            let next = !*dark;
            dark.set(next);
            set_dark(next);
            if let Some(win) = web_sys::window() {
                if let Ok(Some(storage)) = win.local_storage() {
                    let _ = storage.set_item("theme", if next { "dark" } else { "light" });
                }
            }
        })
    };

    html! {
        <button
            type="button"
            onclick={on_click}
            aria-label={if *dark { "Switch to light mode" } else { "Switch to dark mode" }}
            class="rounded-sm p-1 text-[var(--fg-muted)] transition-colors hover:text-[var(--fg)]"
        >
            { if *dark {
                html! {
                    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" aria-hidden="true">
                        <circle cx="8" cy="8" r="3.5" stroke="currentColor" stroke-width="1.3"/>
                        <path stroke="currentColor" stroke-width="1.3" stroke-linecap="round" d="M8 0.75v1.7M8 13.55v1.7M15.25 8h-1.7M2.45 8H.75M13.13 2.87l-1.2 1.2M4.07 11.93l-1.2 1.2M13.13 13.13l-1.2-1.2M4.07 4.07l-1.2-1.2"/>
                    </svg>
                }
            } else {
                html! {
                    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" aria-hidden="true">
                        <path stroke="currentColor" stroke-width="1.3" stroke-linejoin="round" d="M14 9.7A6.2 6.2 0 0 1 6.3 2 6.2 6.2 0 1 0 14 9.7Z"/>
                    </svg>
                }
            } }
        </button>
    }
}
