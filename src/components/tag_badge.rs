//! Small rounded chip for project tags.

use yew::prelude::*;

/// Props for [`TagBadge`].
#[derive(Properties, PartialEq, Clone)]
pub struct TagBadgeProps {
    /// Badge text.
    pub label: &'static str,
}

/// A small rounded chip.
#[function_component(TagBadge)]
pub fn tag_badge(props: &TagBadgeProps) -> Html {
    html! {
        <span class="rounded-full bg-slate-100 px-2 py-0.5 text-xs text-slate-700 dark:bg-slate-800 dark:text-slate-300">
            { props.label }
        </span>
    }
}
