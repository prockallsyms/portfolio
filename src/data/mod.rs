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

#[cfg(test)]
mod tests {
    use super::DATA;

    /// The DoD email pattern: `@local.tld` with a com/net/org TLD
    /// (case-insensitive). The bio's "@ Elton" must not count — no domain
    /// follows the `@`.
    fn contains_email(s: &str) -> bool {
        let lower = s.to_ascii_lowercase();
        let after = lower.find('@').map(|i| &lower[i + 1..]).unwrap_or_default();
        let local: String = after
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '.')
            .collect();
        local.ends_with(".com") || local.ends_with(".net") || local.ends_with(".org")
    }

    fn every_data_string() -> Vec<&'static str> {
        let mut out = vec![
            DATA.person.handle,
            DATA.person.tagline,
            DATA.person.bio,
            DATA.person.github,
            DATA.person.linkedin,
            DATA.person.avatar,
            DATA.certifications,
            DATA.personal_labs,
        ];
        for e in DATA.experience {
            out.extend([e.org, e.role, e.start, e.end.unwrap_or(""), e.notes]);
        }
        for e in DATA.education {
            out.extend([e.school, e.degree, e.years]);
        }
        for p in DATA.projects {
            out.push(p.title);
            out.push(p.blurb);
            out.push(p.url);
            out.extend(p.tags);
        }
        for g in DATA.skills {
            out.push(g.heading);
            out.extend(g.skills);
        }
        out
    }

    /// Identity is the handle only — no real name, no email (§1).
    #[test]
    fn identity_is_handle_only() {
        assert_eq!(DATA.person.handle, "prockallsyms");
        assert!(!DATA.person.bio.is_empty());
        assert!(
            DATA.person
                .github
                .starts_with("https://github.com/prockallsyms")
        );
        assert!(
            DATA.person
                .linkedin
                .starts_with("https://www.linkedin.com/in/")
        );
    }

    /// No email-shaped string anywhere in the content (DoD pattern).
    #[test]
    fn no_email_in_any_data_string() {
        for s in every_data_string() {
            assert!(!contains_email(s), "email-like string: {s:?}");
        }
    }

    /// All six repos present, well-formed, GitHub-owned.
    #[test]
    fn all_six_projects_present_and_well_formed() {
        let expected = [
            "corrode",
            "analyzer",
            "TestAssist",
            "hookdb",
            "malpraxis",
            "collector-rs",
        ];
        let titles: Vec<_> = DATA.projects.iter().map(|p| p.title).collect();
        assert_eq!(titles.len(), 6);
        for t in expected {
            assert!(titles.contains(&t), "missing {t}");
        }
        for p in DATA.projects {
            assert!(!p.title.is_empty() && !p.blurb.is_empty());
            assert!(p.url.starts_with("https://github.com/prockallsyms/"));
            assert!(!p.tags.is_empty());
        }
    }

    /// Experience is non-empty, current role first, every entry shaped.
    #[test]
    fn experience_order_and_shape() {
        assert!(!DATA.experience.is_empty());
        assert!(
            DATA.experience[0].end.is_none(),
            "current role must be first"
        );
        for e in DATA.experience {
            assert!(!e.org.is_empty() && !e.role.is_empty() && !e.start.is_empty());
        }
    }
}
