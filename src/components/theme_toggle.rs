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
            class="rounded-md p-1 text-lg leading-none transition-colors hover:bg-slate-200/70 focus-visible:outline-2 focus-visible:outline-accent active:bg-slate-300/70 dark:hover:bg-slate-800/70 dark:active:bg-slate-700/70"
        >
            { if *dark { "☀" } else { "🌙" } }
        </button>
    }
}
