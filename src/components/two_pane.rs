//! Two-pane page shell: a sticky left rail (quick facts, contact links,
//! chapter nav — whatever the page needs) beside a wide main column.
//!
//! Stacks to one column below the `md` breakpoint, main content first in
//! DOM/reading order; the rail moves to the first grid column and becomes
//! sticky only at `md:` and up, via `order` rather than reordering markup.

use yew::prelude::*;

/// Props for [`TwoPane`].
#[derive(Properties, PartialEq)]
pub struct TwoPaneProps {
    /// Sticky rail content — secondary, page-specific context.
    pub rail: Html,
    /// Primary page content.
    #[prop_or_default]
    pub children: Html,
}

/// A sticky-rail-beside-wide-column shell, shared by every page that isn't
/// the 404 (which stays a single centered block).
#[function_component(TwoPane)]
pub fn two_pane(props: &TwoPaneProps) -> Html {
    html! {
        <div class="grid gap-10 md:grid-cols-[15rem_1fr] md:items-start md:gap-16">
            <div class="min-w-0">
                { props.children.clone() }
            </div>
            <div class="space-y-10 md:sticky md:top-24 md:order-first md:self-start">
                { props.rail.clone() }
            </div>
        </div>
    }
}
