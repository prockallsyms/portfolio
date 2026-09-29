//! Section wrapper: a `<section>` with a mono `//` eyebrow heading and children.

use yew::prelude::*;

/// Props for [`Section`].
#[derive(Properties, PartialEq)]
pub struct SectionProps {
    /// Section heading text.
    pub heading: &'static str,
    /// Section content.
    #[prop_or_default]
    pub children: Html,
}

/// A `<section>` with a small mono uppercase eyebrow heading, prefixed with a
/// decorative `//` (code-comment style) in the accent color.
#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    html! {
        <section class="space-y-5">
            <h2 class="label flex items-center gap-2">
                <span aria-hidden="true" class="text-[var(--accent-text)]">{"//"}</span>
                { props.heading }
            </h2>
            { props.children.clone() }
        </section>
    }
}
