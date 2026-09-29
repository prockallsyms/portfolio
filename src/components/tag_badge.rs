//! Small mono hairline chip for project tags.

use yew::prelude::*;

/// Props for [`TagBadge`].
#[derive(Properties, PartialEq, Clone)]
pub struct TagBadgeProps {
    /// Badge text.
    pub label: &'static str,
}

/// A small hairline-bordered mono chip.
#[function_component(TagBadge)]
pub fn tag_badge(props: &TagBadgeProps) -> Html {
    html! {
        <span class="rounded-sm border border-[var(--border)] px-1.5 py-0.5 font-mono text-[0.6875rem] text-[var(--fg-muted)]">
            { props.label }
        </span>
    }
}
