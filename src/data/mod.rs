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

/// One degree on the education record.
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

/// All site content in one place.
pub const DATA: SiteData = SiteData {
    person: Person {
        handle: "prockallsyms",
        tagline: "Penetration tester & Rust developer", // §5 — final wording pending owner pick
        bio: "I'm a math/cyber/development sorta person. I do things @ Elton.",
        github: "https://github.com/prockallsyms",
        linkedin: "https://www.linkedin.com/in/redacted",
        avatar: "./assets/avatar.jpg",
    },
    experience: &[
        ExperienceItem {
            org: "ELTON",
            role: "Lead Penetration Tester",
            start: "Apr 2026",
            end: None,
            notes: "Penetration testing, client/account management",
        },
        ExperienceItem {
            org: "ELTON",
            role: "Penetration Tester",
            start: "Oct 2024",
            end: Some("Apr 2026"),
            notes: "",
        },
        ExperienceItem {
            org: "Level Nine Group",
            role: "Penetration Tester / Product Vulnerability Researcher",
            start: "Jul 2023",
            end: Some("Jun 2025"),
            notes: "Security research, threat & vulnerability management",
        },
        ExperienceItem {
            org: "City of Bridgeton",
            role: "System Engineer (part-time)",
            start: "Jun 2018",
            end: Some("Jul 2023"),
            notes: "Windows Server network for ~200 endpoints, remote VPNs, Untangle/Arista \
                    firewalls; SQL & Linux support",
        },
    ],
    education: &[EducationItem {
        school: "Truman State University",
        degree: "BS, Mathematics & Computer Science",
        years: "2019–2023",
    }],
    projects: &[
        Project {
            title: "corrode",
            blurb: "Security scanning orchestration in Rust: coordinates 135 security tools \
                    through a plugin-based pipeline with automatic DAG wiring, SQLite storage, \
                    encrypted credentials, and cron scheduling.",
            tags: &["Rust", "security", "orchestration", "SQLite"],
            url: "https://github.com/prockallsyms/corrode",
        },
        Project {
            title: "analyzer",
            blurb: "File-type detection & analysis-execution microservice: DiE signature \
                    detection with content-based fallback, module execution (incl. \
                    Docker-in-Docker modules); CLI + Actix-web server.",
            tags: &["Rust", "Actix", "file analysis", "Docker"],
            url: "https://github.com/prockallsyms/analyzer",
        },
        Project {
            title: "TestAssist",
            blurb: "Containerized web app for visualizing static-analysis output (SARIF, \
                    CycloneDX, SPDX, Swagger/JSON/Markdown viewers) with Docker-based analysis \
                    pipelines; Rust backend, TypeScript frontend, TUI client.",
            tags: &["Rust", "TypeScript", "web app", "Docker"],
            url: "https://github.com/prockallsyms/TestAssist",
        },
        Project {
            title: "hookdb",
            blurb: "Static binary instrumentation driven by git-managed hook DBs — \
                    “Semgrep for binary instrumentation” across JAR/.NET/ELF/PE/Mach-O/BEAM/DEX/Lua/APK/IPA.",
            tags: &["Rust", "binary analysis", "instrumentation"],
            url: "https://github.com/prockallsyms/hookdb",
        },
        Project {
            title: "malpraxis",
            blurb: "Rogue server + malicious client testing for healthcare protocols \
                    (MLLP/HL7v2, DICOM, FHIR, NCPDP, CDA, IHE XDS): 261 adversarial behaviors \
                    across 7 attack categories.",
            tags: &["Rust", "security testing", "healthcare protocols"],
            url: "https://github.com/prockallsyms/malpraxis",
        },
        Project {
            title: "collector-rs",
            blurb: "Cross-platform system information collector & security analysis toolkit in \
                    Rust: 25+ assessment areas (hardware, network, processes, security, \
                    packages), structured JSON output, fully offline single binary; includes an \
                    MCP server (44 tools) and an MCP client proxy.",
            tags: &["Rust", "forensics", "MCP", "security"],
            url: "https://github.com/prockallsyms/collector-rs",
        },
    ],
    skills: &[
        SkillGroup {
            heading: "Security",
            skills: &[
                "Penetration Testing",
                "Malware Analysis",
                "Reverse Engineering",
                "Vulnerability Assessment",
                "Exploit Development (OSED)",
            ],
        },
        SkillGroup {
            heading: "Development",
            skills: &[
                "Rust",
                "TypeScript",
                "Docker",
                "Web services (Actix-web)",
                "CLI tooling",
            ],
        },
    ],
    certifications: "OffSec Exploit Developer (OSED) 2024 · Taggart Institute \
                    binary-exploitation-defenses course 2023",
    personal_labs: "Home malware-analysis lab (2019–present) · Home pen-testing lab \
                    (2019–present)",
};

pub struct SiteData {
    pub person: Person,
    pub experience: &'static [ExperienceItem],
    pub education: &'static [EducationItem],
    pub projects: &'static [Project],
    pub skills: &'static [SkillGroup],
    /// One-line certifications summary for the About page.
    pub certifications: &'static str,
    /// One-line personal-lab summary for the About page.
    pub personal_labs: &'static str,
}
