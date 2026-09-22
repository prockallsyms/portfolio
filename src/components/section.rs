//! Section wrapper: a `<section>` with an `<h2>` heading and children.

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

/// A `<section>` with an `<h2>` heading.
#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    html! {
        <section class="space-y-4">
            <h2 class="text-xl font-semibold">{ props.heading }</h2>
            { props.children.clone() }
        </section>
    }
}
