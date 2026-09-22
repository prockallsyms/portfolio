//! Typed site content — the single source of truth for everything shown on the site.
//!
//! All values here are owner-verified (see PLANS.md §A.2 / §A.3). Views render from
//! this module only; no hardcoded copy lives in components (§4).

/// Site identity & contact. No real name, no email.
pub struct Person {
    pub handle: &'static str,
    pub tagline: &'static str,
    pub bio: &'static str,
    pub github: &'static str,
    pub linkedin: &'static str,
    pub avatar: &'static str,
}

/// One role in the work history (kept newest-first).
pub struct ExperienceItem {
    pub org: &'static str,
    pub role: &'static str,
    pub start: &'static str,
    /// `None` means "present".
    pub end: Option<&'static str>,
    pub notes: &'static str,
}

pub struct EducationItem {
    pub school: &'static str,
    pub degree: &'static str,
    pub years: &'static str,
}

/// One portfolio project (a GitHub repo of the owner).
#[derive(PartialEq)]
pub struct Project {
    pub title: &'static str,
    pub blurb: &'static str,
    pub tags: &'static [&'static str],
    pub url: &'static str,
}

/// A skill group for the skills section.
pub struct SkillGroup {
    pub heading: &'static str,
    pub skills: &'static [&'static str],
}

/// All site content in one place. T07 replaces the placeholders with verified data.
pub const DATA: SiteData = SiteData {
    person: Person {
        handle: "prockallsyms",
        tagline: "Penetration tester & Rust developer", // §5.3 — final wording pending
        bio: "I'm a math/cyber/development sorta person. I do things @ Elton.",
        github: "https://github.com/prockallsyms",
        linkedin: "https://www.linkedin.com/in/redacted",
        avatar: "./assets/avatar.png",
    },
    experience: &[],
    education: &[],
    projects: &[],
    skills: &[],
};

pub struct SiteData {
    pub person: Person,
    pub experience: &'static [ExperienceItem],
    pub education: &'static [EducationItem],
    pub projects: &'static [Project],
    pub skills: &'static [SkillGroup],
}
