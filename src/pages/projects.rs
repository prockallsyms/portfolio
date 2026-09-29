//! Projects page: a scroll-driven walk through the project list, grouped into
//! four chapters by discipline rather than shown as one flat grid. The sticky
//! rail (shared shell, see [`TwoPane`]) doubles as a chapter jump-nav.
//!
//! Motion is pure CSS (`animation-timeline: view()` on `.scroll-reveal`, see
//! `static/css/input.css`) — no scroll-position JavaScript. Chapters degrade
//! to a plain stacked list on narrow viewports, without `animation-timeline`
//! support, and under `prefers-reduced-motion`.

use yew::prelude::*;

use crate::components::{ProjectCard, TwoPane};
use crate::data::{DATA, Project};

/// One thematic grouping of projects, in display order.
struct Chapter {
    /// Anchor id — used by the rail's jump-nav.
    id: &'static str,
    /// Chapter number, shown as `[ NN ]`.
    number: &'static str,
    /// Chapter title.
    title: &'static str,
    /// One-sentence framing, grounded in the projects' own tags/blurbs —
    /// never a claim the project data doesn't already support.
    intro: &'static str,
    /// Project titles in this chapter, matched against `DATA.projects`.
    project_titles: &'static [&'static str],
}

const CHAPTERS: &[Chapter] = &[
    Chapter {
        id: "orchestration",
        number: "01",
        title: "Orchestration & analysis",
        intro: "Tools that turn scattered scanning and triage into one pipeline: plugin \
                orchestration, file-type detection, static-analysis visualization, and \
                cross-platform system collection.",
        project_titles: &["corrode", "analyzer", "TestAssist", "collector-rs"],
    },
    Chapter {
        id: "protocol-hardware",
        number: "02",
        title: "Protocol & hardware security",
        intro: "Fuzzing and protocol work across healthcare interop, a resurrected Linux \
                kernel subsystem, wireless radios, and the automotive CAN bus.",
        project_titles: &["malpraxis", "irda", "nrf-toolkit", "can-toolkit"],
    },
    Chapter {
        id: "reverse-engineering",
        number: "03",
        title: "Reverse engineering & binary analysis",
        intro: "Pulling structure back out of compiled code: binary instrumentation, JVM \
                bytecode decompilation, and C++ object recovery.",
        project_titles: &["hookdb", "classfile-parser", "pharos"],
    },
    Chapter {
        id: "static-analysis",
        number: "04",
        title: "Static analysis at scale",
        intro: "Three approaches to scanning source and binaries in bulk: a Docker-orchestrated \
                scanner farm, a 45-language Semgrep ruleset, and a binary-rule contribution to \
                VulHunt.",
        project_titles: &["sastblast", "sast-rules", "rules"],
    },
];

/// Look up a project by title, panicking (loudly, at render time) if a chapter
/// references a title that doesn't exist in `DATA.projects` — a data-entry
/// mistake, not a state a real visitor should ever be able to trigger.
fn find_project(title: &str) -> &'static Project {
    DATA.projects
        .iter()
        .find(|p| p.title == title)
        .unwrap_or_else(|| panic!("projects page chapter references unknown project {title:?}"))
}

/// One chapter: an inline heading/intro followed by its project rows.
fn render_chapter(chapter: &Chapter, start_index: usize) -> (Html, usize) {
    let projects: Vec<(&'static Project, usize)> = chapter
        .project_titles
        .iter()
        .enumerate()
        .map(|(i, title)| (find_project(title), start_index + i + 1))
        .collect();
    let next_index = start_index + chapter.project_titles.len();

    let html = html! {
        <section id={chapter.id} class="scroll-mt-24 space-y-5">
            <div class="max-w-xl space-y-2">
                <span class="label text-[var(--accent-text)]">{ format!("[ chapter {} ]", chapter.number) }</span>
                <h2 class="text-lg font-semibold tracking-tight">{ chapter.title }</h2>
                <p class="text-sm text-[var(--fg-muted)]">{ chapter.intro }</p>
            </div>
            <div>
                { for projects.into_iter().map(|(p, i)| html! { <ProjectCard project={p} index={i} /> }) }
            </div>
        </section>
    };
    (html, next_index)
}

/// Projects: a hook + four chapters (main), a chapter jump-nav (rail).
#[function_component(Projects)]
pub fn projects() -> Html {
    let empty = DATA.projects.is_empty();

    let mut index = 0usize;
    let chapters: Vec<Html> = CHAPTERS
        .iter()
        .map(|chapter| {
            let (html, next) = render_chapter(chapter, index);
            index = next;
            html
        })
        .collect();

    let rail = html! {
        <div class="space-y-3">
            <span class="label text-[var(--accent-text)]">{"[ 14 repos / 4 disciplines ]"}</span>
            <ul class="space-y-2 font-mono text-xs text-[var(--fg-muted)]">
                { CHAPTERS.iter().map(|chapter| html! {
                    <li>
                        <a
                            href={format!("#{}", chapter.id)}
                            class="underline-offset-4 hover:text-[var(--accent-text)] hover:underline"
                        >
                            { format!("{} {}", chapter.number, chapter.title) }
                        </a>
                    </li>
                }).collect::<Html>() }
            </ul>
        </div>
    };

    html! {
        <TwoPane rail={rail}>
            <div class="animate-fade-up space-y-20">
                <header class="max-w-lg space-y-4">
                    <h1 class="text-2xl font-semibold tracking-tight">{"Projects"}</h1>
                    <p class="text-[var(--fg-muted)]">
                        { "Grouped by what they actually do — orchestrate, fuzz, reverse-engineer, \
                           and scan — not by when they were pushed." }
                    </p>
                </header>
                { if empty {
                    html! {
                        <p class="max-w-prose text-[var(--fg-muted)]">
                            { "Project cards land here with the real content." }
                        </p>
                    }
                } else {
                    html! { <div class="space-y-16">{ for chapters }</div> }
                } }
                { if !empty {
                    html! {
                        <footer class="border-t border-[var(--border)] pt-10">
                            <a
                                href={DATA.person.github}
                                target="_blank"
                                rel="me noopener"
                                class="inline-flex items-center gap-2 border border-[var(--border-strong)] px-4 py-2 font-mono text-xs uppercase tracking-[0.1em] text-[var(--fg)] transition-colors hover:border-[var(--accent)] hover:text-[var(--accent-text)]"
                            >
                                { "View the full GitHub profile" }
                                <span aria-hidden="true">{"→"}</span>
                            </a>
                        </footer>
                    }
                } else { html! {} } }
            </div>
        </TwoPane>
    }
}
