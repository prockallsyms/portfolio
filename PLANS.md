 # Portfolio Rebuild — Master Plan

 > **Single source of truth.** One file, self-contained tasks, status tracked in §2.
 > Audit: 2026-07-19 · Toolchain: rustc/cargo 1.97.1 (current stable) · Plan version: 2 (detailed)
 >
 > After a context compaction, **do not rely on chat history.** Read §0, the §2 index,
 > and your task's section only.

 ## §0. Resume protocol (mandatory for every execution session)

 1. Pick the first task in §2 whose status is `⬜` **and whose deps are all `✅`** (the chain is linear: T01→T02→…→T10).
 2. Load **only** the files in that task's "Context to load" list — nothing else from `src/`.
 3. Execute the numbered steps exactly. Every command shown is the exact command; every
    file-content block is the exact file content. If a step's output differs from the
    "expect" note, **stop and fix before continuing** — do not paper over it.
 4. Run every DoD check. All must pass.
 5. `git add -A && git commit -m "Tnn: <title>"`, then set status `✅` in §2 and fill the
    `Done` column with the short SHA.
 6. If stuck: add `BLOCKED: <reason>` under the task, stop, and report. Never modify
    §1 (fixed decisions) or §5 without owner input.

 ## §1. Fixed decisions (confirmed by owner — do not revisit)

 | Topic | Decision |
 |---|---|
 | Display identity | **"prockallsyms"** only. **No real name, no email, no old username anywhere in the site, README, or repo metadata.** |
 | Projects | `corrode`, `collector-rs`, `analyzer`, `TestAssist`, `hookdb`, `malpraxis` — all `github.com/prockallsyms/<repo>` (verified accessible via SSH; blurbs in §A.3) |
 | Contact links | GitHub (`github.com/prockallsyms`) + LinkedIn (`linkedin.com/in/redacted`). **No email on site.** |
 | Deploy | GitHub Pages, project site → `https://prockallsyms.github.io/portfolio/` (**confirmed**) |
 | Avatar | GitHub profile picture `https://avatars.githubusercontent.com/u/18057702?v=4` → vendored as `static/assets/avatar.png` (T07) |
 | History | `git filter-repo` rewrite before public Pages deploy (T10) — **confirmed** |
 | CSS | **Tailwind v4 standalone CLI** v4.3.3 pinned (no Node, no bundler — §A.5) |
 | Theme | System light/dark via `prefers-color-scheme` **plus** manual toggle (persisted in `localStorage["theme"]`) |
 | Rust stack | edition 2024 · yew 0.23 (feature `csr`) · yew-router 0.20 · **no yew-hooks** (hooks are in yew core) · **no reqwest** (content is static typed Rust data) · **no wee_alloc** |
 | Dev server | Rust `tiny_http` workspace member `dev-server/` (zero external tooling) |
 | Yew target | `wasm32-unknown-unknown` (the old `wasm32-unknown-linux-gnu` target no longer exists) |

 ## §1b. Baseline audit (verified facts about the current repo)

 | Area | Current | Target |
 |---|---|---|
 | Edition | 2018 | 2024 |
 | yew / yew-router / yew-hooks | 0.19.3 / 0.16.0 / 0.1.56 | 0.23.0 / 0.20.0 / dropped |
 | reqwest, wee_alloc | present | **dropped** |
 | wasm target | `wasm32-unknown-linux-gnu` (removed from rustc 1.97) | `wasm32-unknown-unknown` |
 | Build | Parcel 1.x + wasm plugin (EOL, Node 26-incompatible) | wasm-pack + ES module + Tailwind CLI |
 | Content | pure create-yew-app demo (counter, "Learn Yew", repo-fetch demo) | real portfolio |
 | Tests | 3 demo assertions, unrunnable (no wasm-pack installed yet) | rewritten + CI |
 | Repo metadata | `name = "yew-app"`, author "You <you@example.com>", repo URL = create-yew-app | fixed |
 | CI | none | GitHub Actions |
 | Git history | 3 commits, authors `[redacted] <[redacted email]>` + `[redacted]<[redacted email]>` | sanitized (T10) |

 Old code still `cargo check`s (host and wasm) — the breakage is the Parcel pipeline +
 demo content, not compilation.

 ## §2. Task index (linear chain)

 | ID | Task | Deps | Status | Done (SHA) |
 |---|---|---|---|---|
 | T01 | Toolchain: wasm-pack, retarget wasm32-unknown-unknown, kill Parcel, module loader, dev server | — | ✅ | b5a210a |
 | T02 | Cargo manifest: edition 2024, metadata, deps, profiles | T01 | ✅ | 0f394df |
 | T03 | Migrate existing app to yew 0.23 / yew-router 0.20 (mechanical, no features) | T02 | ⬜ | |
 | T04 | Skeleton: typed data layer, pages, components, routes (placeholder content) | T03 | ⬜ | |
 | T05 | Tailwind v4: pinned CLI, design tokens, theme toggle wiring | T04 | ⬜ | |
 | T06 | Visual polish, responsive pass, a11y pass | T05 | ⬜ | |
 | T07 | Real content into `src/data` (owner-verified, §A.2/§A.3) | T06 | ⬜ | |
 | T08 | Tests: native data tests + wasm render tests | T07 | ⬜ | |
 | T09 | CI + GitHub Pages deploy | T08 | ⬜ | |
 | T10 | Ship: history sanitize, README, LICENSE, perf, v1.0.0 | T09 | ⬜ | |

 ## §3. Tasks

 ### T01 — Toolchain: wasm-pack, retarget, kill Parcel, dev server

 **Goal.** A minimal build pipeline that works on this machine, with zero Node tooling.

 **Context to load:** `Cargo.toml`, `static/index.html`, `static/index.ts`, `.sassrc.js`,
 `package.json`, `.gitignore`, `src/lib.rs` (entry only — don't study it).

 **Steps.**

 1. Install `wasm-pack`:
    ```bash
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh | sh
    wasm-pack --version        # expect: "wasm-pack 0.1x.x"
    ```
    (Installs into `~/.cargo/bin`; make sure it's on PATH.)

 2. Verify the wasm target (already installed; verify only):
    ```bash
    rustup target list --installed | grep -x wasm32-unknown-unknown \
      || rustup target add wasm32-unknown-unknown
    ```

 3. Delete the Node-era files:
    ```bash
    rm -f package.json package-lock.json .sassrc.js static/index.ts
    ```

 4. Rewrite `static/index.html` with **exactly** this content:
    ```html
    <!doctype html>
    <html lang="en">
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <meta name="description" content="prockallsyms — penetration testing, security research, Rust" />
        <title>prockallsyms</title>
        <!-- GH Pages project-site subpath. yew-router 0.20's BrowserRouter reads this
             automatically — no basename prop needed (§A.4). -->
        <base href="/portfolio/" />
        <link rel="icon" type="image/svg+xml" href="./assets/favicon.svg" />
        <link rel="stylesheet" href="./css/app.css" />
        <script>
          // Theme bootstrap — runs before first paint (prevents dark-mode flash).
          // The Yew ThemeToggle (T04) reads the same key and keeps it in sync.
          (function () {
            var t = localStorage.getItem("theme");
            var dark = t === "dark" || (t !== "light" && matchMedia("(prefers-color-scheme: dark)").matches);
            if (dark) document.documentElement.classList.add("dark");
          })();
        </script>
      </head>
      <body class="bg-white text-slate-900 antialiased dark:bg-slate-950 dark:text-slate-100">
        <div id="app"></div>
        <script type="module" src="./main.js"></script>
      </body>
    </html>
    ```
    Note: the `bg-white`/`dark:bg-slate-950` classes only take effect once T05 generates
    `app.css` — an unstyled page during T01–T04 is expected.

 5. Write `static/main.js` exactly:
    ```js
    // Boots the wasm package, then starts the Yew app (see src/lib.rs).
    import init, { run } from "./pkg/portfolio.js";

    init.then(() => run());
    ```

 6. Create placeholder assets:
    ```bash
    mkdir -p static/css static/assets
    : > static/css/app.css        # real file generated in T05
    ```
    `static/assets/favicon.svg` exactly:
    ```svg
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
      <rect width="64" height="64" rx="12" fill="#0f172a"/>
      <text x="32" y="43" font-family="monospace" font-size="34" fill="#22d3ee" text-anchor="middle">p</text>
    </svg>
    ```

 7. Add the dev server as workspace member `dev-server/`.

    `dev-server/Cargo.toml` exactly:
    ```toml
    [package]
    name = "dev-server"
    version = "0.1.0"
    edition = "2024"
    publish = false

    [dependencies]
    tiny_http = "0.12"
    ```

    `dev-server/src/main.rs` exactly:
    ```rust
    //! Local development server for the portfolio.
    //!
    //! Serves `static/` (site files) and `pkg/` (wasm-pack output) on
    //! `http://localhost:8000/`. There is no file watcher: after changing
    //! Rust code, re-run `wasm-pack build --dev` and refresh the browser.

    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};

    use tiny_http::{Request, Response, Server};

    const PORT: &str = "8000";

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("dev-server lives in a workspace member directory");
        let server = Server::http(format!("127.0.0.1:{PORT}"))?;
        println!("portfolio dev server: http://localhost:{PORT}/");
        println!("  {workspace:?}/static -> site files");
        println!("  {workspace:?}/pkg    -> wasm-pack output");

        for request in server.incoming_requests() {
            handle(&request, workspace)?;
        }
        Ok(())
    }

    fn handle(request: &Request, workspace: &Path) -> Result<(), Box<dyn std::error::Error>> {
        // Strip the query string; serve `pkg/...` from wasm output, everything else from static/.
        let url = request
            .url()
            .split('?')
            .next()
            .unwrap_or("")
            .trim_start_matches('/');
        let (base, rel) = if let Some(rest) = url.strip_prefix("pkg/") {
            (workspace.join("pkg"), rest.to_string())
        } else if url.is_empty() {
            (workspace.join("static"), "index.html".to_string())
        } else {
            (workspace.join("static"), url.to_string())
        };

        match safe_join(&base, &rel) {
            Some(path) if path.is_file() => {
                let bytes = fs::read(path)?;
                Response::from_bytes(bytes, mime_for(&path)).send(request)?;
            }
            _ => Response::from_string("404 — not found")
                .with_status_code(404)
                .send(request)?,
        }
    }

    /// Join `base/rel`, rejecting `..` traversal and symlink escapes.
    fn safe_join(base: &Path, rel: &str) -> Option<PathBuf> {
        if rel.contains("..") {
            return None;
        }
        let path = base.join(rel);
        let canonical_base = base.canonicalize().ok()?;
        let canonical_path = path.canonicalize().ok()?;
        Some(canonical_path.strip_prefix(&canonical_base).ok()?.to_path_buf())
    }

    fn mime_for(path: &Path) -> &'static str {
        match path.extension().and_then(|e| e.to_str()).unwrap_or("") {
            "html" => "text/html; charset=utf-8",
            "js" => "text/javascript",
            "css" => "text/css; charset=utf-8",
            "wasm" => "application/wasm",
            "png" => "image/png",
            "svg" => "image/svg+xml",
            "gif" => "image/gif",
            "json" => "application/json",
            "txt" => "text/plain; charset=utf-8",
            _ => "application/octet-stream",
        }
    }
    ```

 8. Register the member in the root `Cargo.toml` **only** (T02 finalizes the rest of that
    file): add
    ```toml
    [workspace]
    members = ["dev-server"]
    ```

 9. Write `scripts/dev.sh` exactly and `chmod +x` it (T05 adds the CSS watcher):
    ```bash
    #!/usr/bin/env bash
    # Local dev loop: wasm-pack dev build + dev server on :8000.
    # Re-run after Rust changes; refresh the browser after each rebuild.
    set -euo pipefail
    cd "$(dirname "$0")/.."
    wasm-pack build --dev
    exec cargo run -p dev-server
    ```

 10. Fix `.gitignore`: **remove the `Cargo.lock` ignore line** (apps commit lockfiles),
     keep `/target`, `**/*.rs.bk`, `/pkg`, `/wasm-pack.log`, `.cache`, and **add**:
     ```
     # tailwind pinned binary + Pages staging dir
     /.bin
     /site
     ```
     (`node_modules` / `/dist` may stay or go — nothing Node remains.)

 **DoD.**
 - `wasm-pack build --dev` succeeds → `pkg/portfolio.js` exists.
 - `scripts/dev.sh` serves the app:
   - `curl -s http://localhost:8000/ | grep -o '<base href="/portfolio/">'` → prints the base tag
   - `curl -s http://localhost:8000/main.js | grep portfolio` → 200 with the import line
   - `curl -s http://localhost:8000/assets/favicon.svg | head -1` → `<svg`
   - `curl -s -o /dev/null -w '%{http_code}\n' http://localhost:8000/nope` → `404`
 - `rg -i "parcel|sass" --glob '!PLANS.md' .` → empty (zero Node tooling).

 **Done:** ⬜

 ---

 ### T02 — Cargo manifest: edition 2024, metadata, deps, profiles

 **Context to load:** `Cargo.toml`, `dev-server/Cargo.toml`, `.gitignore`.

 **Steps.**

 1. Replace root `Cargo.toml` with **exactly**:
    ```toml
    [package]
    name = "portfolio"
    version = "0.1.0"
    edition = "2024"
    rust-version = "1.85"
    description = "Portfolio of prockallsyms — penetration testing, security research, and Rust"
    authors = ["prockallsyms"]
    license = "MIT"
    repository = "https://github.com/prockallsyms/portfolio"

    [workspace]
    members = ["dev-server"]

    [lib]
    crate-type = ["cdylib", "rlib"]   # cdylib for wasm-pack, rlib for native tests

    [dependencies]
    # yew 0.23: default features are empty — "csr" is REQUIRED for browser rendering (§A.4).
    yew = { version = "0.23", features = ["csr"] }
    yew-router = "0.20"
    serde = { version = "1", features = ["derive"] }
    log = "0.4"
    wasm-bindgen = "0.2"
    wasm-logger = "0.2"
    # DOM access for the theme toggle (features verified against web-sys 0.3.105, §A.4).
    web-sys = { version = "0.3", features = ["Document", "Element", "DomTokenList", "Storage", "Window"] }

    [dev-dependencies]
    wasm-bindgen-test = "0.3"
    gloo-utils = "0.3"

    [profile.release]
    opt-level = 3
    lto = true
    codegen-units = 1
    panic = "abort"
    strip = "symbols"
    ```
    Notes:
    - **No reqwest, no wee_alloc, no yew-hooks.** Every dep above is justified; wasm size matters.
    - `web-sys` compiles on host too (bindings only); keeping it ungated keeps `cargo check` honest.
    - The workspace root package is the only default member for root-level `cargo` invocations,
      so `cargo check`/`cargo clippy` at root cover `portfolio` only; use `-p dev-server` explicitly.

 2. `cargo check` (host) and `cargo check --target wasm32-unknown-unknown`.
    If the old demo sources fail to compile under yew 0.23, apply **minimal mechanical
    fixes only** in this task (e.g. an entry-point rename); actual migration/content work is T03/T04.

 3. Verify dependency hygiene:
    ```bash
    cargo tree | grep -Ei "reqwest|wee_alloc|yew-hooks"   # expect: no output
    ```

 4. `cargo fmt --all` and
    ```bash
    cargo clippy -p portfolio --target wasm32-unknown-unknown --all-targets -- -D warnings
    ```

 **DoD.** Both `cargo check`s green · `cargo tree` clean · fmt + clippy green · lockfile committed.

 **Done:** ⬜

 ---

 ### T03 — Migrate existing app to yew 0.23 / yew-router 0.20 (mechanical only)

 **Goal.** Isolate API-drift fixes from new features. The demo pages stay functionally the same.

 **Context to load:** `src/lib.rs`, `src/app.rs`, `src/routes/mod.rs`, `src/routes/home.rs`,
 `src/routes/about.rs`, `src/components/nav.rs`, `tests/lib.rs` (compile-level only; tests
 stay broken until T08), **and §A.4 (verified API notes) — read it before editing.**

 **Verified API deltas (from §A.4 — do not guess, verify against these):**
 - `yew::start_app` **no longer exists** in 0.23 → entry is
   `yew::Renderer::<App>::new().render()` (feature `csr` already enabled in T02).
   `Renderer::with_root(el)` renders into a specific element (needed by T08 tests).
 - `Callback<T>` (owned IN) and `CallbackRef<T>` (`&IN`) both exist in `yew::prelude`;
   prefer `CallbackRef` for event handlers.
 - `#[component]`, `#[derive(Props)]`, `use_state`, `use_effect_with_deps` — unchanged.
 - `classes!` macro available in prelude for conditional class strings.
 - yew-router 0.20: `BrowserRouter` (**`basename` prop optional — it defaults to the page's
   `<base href>`**, which is `/portfolio/` from T01 → no prop needed);
   `Switch` takes a `render` prop `fn(AppRoute) -> Html`; unknown routes fall to the
   `#[not_found]` variant; `Link` props are `to` + `classes` (+ `query`, `state`);
   hooks: `use_route::<R>() -> Option<R>`, `use_navigator()`, `use_location()`.

 **Steps.**

 1. `src/lib.rs` →
    ```rust
    //! Portfolio SPA — crate root and wasm entry point.

    mod app;
    mod components;
    mod routes;

    pub use app::App;

    /// Starts the Yew app (called from `static/main.js`).
    pub fn run() {
        wasm_logger::init(wasm_logger::Config::new(log::Level::Warn));
        yew::Renderer::<App>::new().render();
    }
    ```
 2. `src/app.rs` — wrap the app in `<BrowserRouter>{...}</BrowserRouter>` (no `basename` prop).
 3. `src/routes/mod.rs` — rewrite `AppRoute` as
    ```rust
    #[derive(Clone, Routable, PartialEq, Debug)]
    pub enum AppRoute {
        #[at("/")]
        Home,
        #[at("/about")]
        About,
        #[not_found]
        NotFound,
    }
    pub fn switch(route: AppRoute) -> Html { /* match, keep demo pages for now */ }
    ```
 4. `src/components/nav.rs` — `Link` props are `to` + `classes`.
    **Note:** yew-router 0.20's `Link` does **not** auto-mark the active route — active-state
    styling via `use_route` is added in T04. Keep nav minimal here.
 5. `src/routes/about.rs` — the `reqwest` fetch demo is the only reqwest use site: replace the
    fetch with a static placeholder paragraph (real content lands in T07).
 6. `cargo fmt --all` + clippy (same command as T02 step 4).

 **DoD.**
 - `wasm-pack build --dev` green; served at `/` and `/about` on the new pipeline (old demo text OK).
 - Unknown path renders the `NotFound` branch.
 - `rg "reqwest|yew_hooks|yew-hooks" src/ Cargo.toml` → empty.
 - Clippy clean.

 **Done:** ⬜

 ---

 ### T04 — Skeleton: typed data layer, pages, components, routes

 **Goal.** New architecture: typed content data as the single source of truth, placeholder values.

 **Context to load:** all of `src/` (it is being rebuilt — read current files only to know what
 to delete), §A.3 (project blurb shapes), §4 (style rules).

 **Target file tree (exact):**
 ```
 src/
 ├── lib.rs              # run() — from T03
 ├── app.rs              # App: BrowserRouter → Nav, <main> (Switch), Footer
 ├── data/
 │   └── mod.rs          # SiteData + typed content (single source of truth)
 ├── pages/
 │   ├── mod.rs          # re-exports: Home, About, Projects, NotFound
 │   ├── home.rs         # hero (avatar, tagline, bio, contact links) + featured projects + skills teaser
 │   ├── about.rs        # bio, experience timeline, education, certifications
 │   ├── projects.rs     # full project grid from data
 │   └── not_found.rs    # 404: assets/coffee_cup.gif + one-liner + link home
 └── components/
     ├── mod.rs
     ├── nav.rs          # sticky top bar, active route highlight, ThemeToggle
     ├── footer.rs       # GitHub/LinkedIn links + one-line footer
     ├── theme_toggle.rs # dark/light toggle, persisted
     ├── section.rs      # <section> with h2 heading + children
     ├── project_card.rs # one project card (title link, blurb, tag badges)
     └── tag_badge.rs    # small rounded chip
 ```
 Route set: `Home /` · `About /about` · `Projects /projects` · `NotFound #[not_found]`.
 No separate contact page — contact links live in the hero and footer (owner decision:
 GitHub + LinkedIn only).

 **`src/data/mod.rs` — exact content** (placeholder strings until T07):
 ```rust
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
 ```

 **Component contracts** (props + behavior; write the `html!` bodies per §4/§6):
 - `Nav` (no props): sticky top bar, `aria-label="Primary"`. Links from
   `const LINKS: &[(AppRoute, &str)] = &[(AppRoute::Home, "Home"), (AppRoute::About, "About"), (AppRoute::Projects, "Projects")];`
   — **call `use_route::<AppRoute>()` once before the loop** (hooks may not be called inside
   `for`), highlight the active link (`aria-current="page"`), render `<ThemeToggle/>` right.
 - `ThemeToggle` (no props): initial state from `localStorage["theme"]` else `prefers-color-scheme`;
   on click, flip `.dark` on `<html>` via web-sys and persist:
   ```rust
   fn set_dark(dark: bool) {
       let doc = web_sys::document().expect("browser document");
       let root = doc.document_element().expect("html element");
       let list = root.class_list();
       if dark {
           let _ = list.add_1("dark");
       } else {
           let _ = list.remove("dark");
       }
   }
   ```
   `aria-label` describing the action; swap icon ☀/🌙 (text symbols are fine).
 - `Footer` (no props): GitHub + LinkedIn external links from `DATA.person`
   (`target="_blank" rel="me noopener"`) + a short one-liner (no name/email).
 - `Section`: props `heading: &'static str` + child content; renders `<section>` + `<h2>`.
 - `ProjectCard`: prop `project: &'static Project`; title links to `url`, blurb, `TagBadge` per tag.
 - `TagBadge`: prop `label: &'static str`; small rounded chip.
 - Pages render **exclusively from `data::DATA`** — zero copy hardcoded in components.

 **Delete** all demo logic (counter, fetch stub, "Learn Yew" anchor, yew logo).

 **DoD.**
 - All four routes render (placeholder text OK).
 - `rg -i "learn yew|counter|create-yew|jetli" src/` → empty.
 - Module tree matches the layout above; `cargo clippy -p portfolio --target wasm32-unknown-unknown --all-targets -- -D warnings` clean.

 **Done:** ⬜

 ---

 ### T05 — Tailwind v4: pinned CLI, design tokens, theme wiring

 **Context to load:** `static/index.html`, `static/css/input.css` (new), `scripts/dev.sh`,
 `scripts/build.sh` (new), `scripts/tw.sh` (new), §A.5.

 **Steps.**

 1. `scripts/tw.sh` exactly, then `chmod +x`:
    ```bash
    #!/usr/bin/env bash
    # Tailwind v4 standalone CLI — zero Node.
    # Downloads the pinned binary on first use, verifies sha256, builds app.css.
    set -euo pipefail
    cd "$(dirname "$0")/.."

    VERSION="4.3.3"
    SHA256="dc61b3ac6b8c9ca874c0cc4c57b2409791a64c5540404ca5f5367360babc313a"
    URL="https://github.com/tailwindlabs/tailwindcss/releases/download/v${VERSION}/tailwindcss-linux-x64"
    BIN=".bin/tailwindcss-${VERSION}"

    if [ ! -x "$BIN" ]; then
      mkdir -p .bin
      curl -fsSL -o "$BIN" "$URL"
      echo "${SHA256}  ${BIN}" | sha256sum -c -
      chmod +x "$BIN"
    fi

    # Args pass through: "--watch" (dev) or "--minify" (release).
    exec "$BIN" css -i static/css/input.css -o static/css/app.css "$@"
    ```

 2. `static/css/input.css` exactly:
    ```css
    @import "tailwindcss";

    /* Class-based dark mode: <ThemeToggle/> flips `.dark` on <html>;
       the inline script in index.html seeds it before first paint. */
    @custom-variant dark (&:where(.dark, .dark *));

    @theme {
      /* System font stacks only — no webfont fetches (site must work offline). */
      --font-sans: ui-sans-serif, system-ui, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
      --font-mono: ui-monospace, "Cascadia Code", "Source Code Pro", Menlo, Consolas, monospace;

      /* Brand accent (cyan). Usable as text-accent, bg-accent, border-accent, ring-accent. */
      --color-accent: #22d3ee;
    }
    ```

 3. Content scanning: v4 auto-scans project source files (including the `html!` macros in
    `.rs`). **Verify** by grepping the built CSS for a class that only appears in `.rs`
    (e.g. `max-w-3xl`). If absent, add `@source "../src";` to `input.css` and rebuild.

 4. Update `scripts/dev.sh` to exactly:
    ```bash
    #!/usr/bin/env bash
    # Local dev loop: Tailwind watch + wasm-pack dev build + dev server on :8000.
    # Re-run after Rust changes; refresh the browser after each rebuild.
    set -euo pipefail
    cd "$(dirname "$0")/.."
    scripts/tw.sh --watch &
    TW_PID=$!
    trap 'kill "$TW_PID" 2>/dev/null || true' EXIT
    wasm-pack build --dev
    exec cargo run -p dev-server
    ```

 5. `scripts/build.sh` exactly, then `chmod +x`:
    ```bash
    #!/usr/bin/env bash
    # Production build: minified CSS + release wasm → site/ (the Pages artifact).
    set -euo pipefail
    cd "$(dirname "$0")/.."
    scripts/tw.sh --minify
    wasm-pack build --release
    rm -rf site && mkdir site
    cp -R static/. site/
    cp -R pkg/. site/pkg/
    ```

 **DoD.**
 - `scripts/tw.sh` produces a non-trivial `static/css/app.css`; spot-check
   `grep -c "max-w-3xl" static/css/app.css` and `grep -c "dark\\\\:bg-slate-950" static/css/app.css` → both ≥ 1.
 - In the browser: light **and** dark styled; toggle persists across reload; no theme flash on reload.
 - `scripts/build.sh` produces `site/` containing `index.html`, `main.js`, `css/app.css` (minified), `pkg/`.
 - Still zero Node tooling; clippy clean.

 **Done:** ⬜

 ---

 ### T06 — Visual polish, responsive pass, a11y pass

 **Context to load:** `src/pages/*.rs`, `src/components/*.rs`, `static/css/input.css`,
 `static/index.html`, `assets/coffee_cup.gif`.

 **Steps (concrete checklist — all items required).**
 1. **Layout rhythm:** page shell `max-w-3xl mx-auto px-4 sm:px-6`; consistent section spacing
    `py-16 sm:py-24`; exactly one `<h1>` per page (the page title); long text blocks get
    `max-w-prose` for ~72ch line length.
 2. **Header:** `sticky top-0 z-10 border-b border-slate-200 bg-white/80 backdrop-blur
    dark:border-slate-800 dark:bg-slate-950/80`.
 3. **Responsive:** nav fits one row at 360px (compact labels); project grid
    `grid gap-4 sm:grid-cols-2`; verify **no horizontal scroll** at 360 / 768 / 1280 px.
 4. **A11y:**
    - skip-to-content link (visually hidden until focused) as first focusable element;
    - visible `:focus-visible` rings: `focus-visible:outline-2 focus-visible:outline-accent`;
    - `aria-current="page"` on the active nav link (already T04 contract);
    - avatar `alt="Avatar of prockallsyms"`; coffee gif `aria-hidden="true" alt=""` (decorative);
    - contrast ≥ AA in both themes; all motion behind `motion-safe:`.
    - `prefers-reduced-motion`: the coffee gif cannot be CSS-paused — it is small & decorative;
      acceptable, documented here.
 5. **States:** cards `transition-colors hover:border-accent/60 hover:shadow-sm`;
    links `underline-offset-4 hover:underline`; button hover/active states.
 6. **Meta:** `index.html` gets `og:title` / `og:description` + keep `description`.

 **DoD.**
 - Manual pass at 360/768/1280 px in **both** themes; a11y checklist above all green.
 - No console errors; no inline `style=` attributes in any `html!`.
 - `cargo clippy` still clean.

 **Done:** ⬜

 ---

 ### T07 — Real content (owner-verified)

 **Context to load:** `src/data/mod.rs` (the only file heavily edited), §A.2 (LinkedIn data),
 §A.3 (project blurbs), §5 (open tagline).

 **Steps.**
 1. `Person`: `handle = "prockallsyms"`; tagline from §5.3 (keep the placeholder until the
    owner picks); bio = §A.2 self-description; GitHub/LinkedIn URLs as fixed; avatar
    `./assets/avatar.png`.
 2. `experience` (newest-first), from §A.2:
    | org | role | start | end | notes |
    |---|---|---|---|---|
    | ELTON | Lead Penetration Tester | `Apr 2026` | `None` | penetration testing, client/account management |
    | ELTON | Penetration Tester | `Oct 2024` | `Apr 2026` | "" |
    | Level Nine Group | Penetration Tester / Product Vulnerability Researcher | `Jul 2023` | `Jun 2025` | security research, threat & vulnerability management |
    | City of Bridgeton | System Engineer (part-time) | `Jun 2018` | `Jul 2023` | Windows Server network for ~200 endpoints, remote VPNs, Untangle/Arista firewalls; SQL & Linux support |
 3. `education`: Truman State University — "BS, Mathematics & Computer Science" — "2019–2023".
 4. `projects` — all six repos, one-sentence site-voice blurbs from §A.3, tags, and
    `url = "https://github.com/prockallsyms/<repo>"`: `corrode`, `analyzer`, `TestAssist`,
    `hookdb`, `malpraxis`, `collector-rs`.
 5. `skills` groups (only evidenced skills):
    - "Security": Penetration Testing · Malware Analysis · Reverse Engineering ·
      Vulnerability Assessment · Exploit Development (OSED)
    - "Development": Rust · TypeScript · Docker · Web services (Actix-web) · CLI tooling
 6. Optional About extras (owner call, default: include):
    - Certifications line: OffSec Exploit Developer (OSED) 2024 ·
      Taggart Institute binary-exploitation-defenses course 2023.
    - Personal labs: home malware-analysis lab (2019–present) · home pen-testing lab (2019–present).
    - Location: omit ([redacted city/state] stays off-site by default — owner call).
 7. Vendor the avatar:
    ```bash
    curl -fsSL -o static/assets/avatar.png "https://avatars.githubusercontent.com/u/18057702?v=4"
    file static/assets/avatar.png    # expect: "PNG image data"
    ```
 8. 404 copy (`pages/not_found.rs`), e.g.: *"This route doesn't exist — like a clean pentest
    report, it simply isn't here."* + coffee gif + link home.
 9. Delete every remaining placeholder string.

 **DoD.**
 - `rg -i "placeholder|lorem|TODO|FIXME" src/` → empty.
 - Every external link verified reachable; contact = GitHub + LinkedIn only (no email anywhere:
   `rg -i "mailto|@[a-z0-9.-]+\.(com|net|org)" src/ README.md static/` → empty).
 - About page reads as a real CV; Projects page shows all six repos with working links.

 **Done:** ⬜

 ---

 ### T08 — Tests: native data tests + wasm render tests

 **Context to load:** `tests/lib.rs` (replace), `src/data/mod.rs`, `src/lib.rs`,
 `src/pages/mod.rs`, §A.4 (Renderer API).

 **Steps.**

 1. **Native data tests** (run on host, cheap in CI): add `#[cfg(test)] mod tests` inside
    `src/data/mod.rs` (or `tests/data.rs`). Assert:
    - `DATA.person.handle == "prockallsyms"`; bio non-empty; `github` parses and starts
      with `https://github.com/prockallsyms`; `linkedin` starts with
      `https://www.linkedin.com/in/`; **no email anywhere in any data string** (no `@` in
      handle/bio/tagline).
    - every `Project`: title/blurb/url non-empty; `url.starts_with("https://github.com/prockallsyms/")`;
      tags non-empty; all six expected titles present (`corrode`, `analyzer`, `TestAssist`,
      `hookdb`, `malpraxis`, `collector-rs`).
    - experience non-empty; first item's `end.is_none()` (the current role); all org names
      non-empty.
 2. **Wasm render tests** — rewrite `tests/lib.rs` from scratch:
    ```rust
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    ```
    Render pattern (verified API, §A.4): create a detached `<div>` in the document, then
    `yew::Renderer::<portfolio::App>::new().with_root(el.clone()).render();` and assert on
    the rendered DOM text. Tests:
    - home renders `h1` containing "prockallsyms";
    - nav contains links to Home, About, Projects;
    - navigating to `/about` (drive `history.pushState` + dispatch `popstate`, or assert by
      rendering `About` directly into a root div) shows "ELTON";
    - navigating to `/projects` shows "corrode";
    - unknown route renders the 404 page (coffee image present).
    Keep it to ~4–6 focused tests; one `Renderer` per test (fresh root div each).
 3. Run locally (Chrome is installed here):
    ```bash
    cargo test -p portfolio                                   # native
    wasm-pack test --headless --chrome                       # wasm, run from repo root
    ```

 **DoD.**
 - `cargo test -p portfolio` green.
 - `wasm-pack test --headless --chrome` green.
 - Test commands recorded verbatim for CI (T09) and README (T10).

 **Done:** ⬜

 ---

 ### T09 — CI + GitHub Pages deploy

 **Context to load:** `scripts/*.sh`, `static/index.html`, `static/404.html` (new),
 `.github/workflows/ci.yml` (new), §A.4, §A.5.

 **Steps.**

 1. `static/404.html` exactly (SPA fallback for deep paths — GH Pages serves this for any
    unknown path, then the SPA takes over at `/portfolio/`):
    ```html
    <!doctype html>
    <html lang="en">
      <head>
        <meta charset="utf-8" />
        <base href="/portfolio/" />
        <meta http-equiv="refresh" content="0; url=/portfolio/" />
        <script>location.replace("/portfolio/");</script>
      </head>
      <body>
        <p>Redirecting to <a href="/portfolio/">prockallsyms.github.io/portfolio/</a>…</p>
      </body>
    </html>
    ```
    Tradeoff (documented): direct deep links (e.g. `/portfolio/about` typed in the address bar)
    land on Home after the redirect — inherent to static hosting; in-app navigation is unaffected.

 2. `.github/workflows/ci.yml` exactly:
    ```yaml
    name: CI

    on:
      push:
        branches: [main]
      pull_request:

    jobs:
      test:
        runs-on: ubuntu-latest
        steps:
          - uses: actions/checkout@v4
          - uses: dtolnay/rust-toolchain@stable
            with:
              components: rustfmt, clippy
              targets: wasm32-unknown-unknown
          - uses: taiki-e/install-action@v2
            with: { tool: wasm-pack }
          - name: Format
            run: cargo fmt --all --check
          - name: Clippy (wasm)
            run: cargo clippy -p portfolio --target wasm32-unknown-unknown --all-targets -- -D warnings
          - name: Clippy (host, dev-server)
            run: cargo clippy -p dev-server --all-targets -- -D warnings
          - name: Native tests
            run: cargo test -p portfolio
          - name: Wasm tests (headless Chrome)
            run: wasm-pack test --headless --chrome
          - name: Tailwind (release)
            run: bash scripts/tw.sh --minify
          - name: Wasm (release)
            run: wasm-pack build --release
          - name: Assemble site
            run: |
              rm -rf site && mkdir site
              cp -R static/. site/
              cp -R pkg/. site/pkg/
          - name: Upload Pages artifact
            if: github.ref == 'refs/heads/main' && github.event_name == 'push'
            uses: actions/upload-pages-artifact@v3
            with:
              path: site

      deploy:
        if: github.ref == 'refs/heads/main' && github.event_name == 'push'
        needs: test
        runs-on: ubuntu-latest
        permissions:
          pages: write
          id-token: write
        environment:
          name: github-pages
          url: ${{ steps.deploy.outputs.page_url }}
        steps:
          - id: deploy
            uses: actions/deploy-pages@v4
    ```

 3. **One-time owner step** (repo Settings → Pages): set build source to **GitHub Actions**
    (not a branch). The `github-pages` environment + OIDC deploy token are created automatically
    on first run. Record in README (T10).

 4. After the first green run, verify live:
    - `https://prockallsyms.github.io/portfolio/` renders home (both themes);
    - in-app links to /about and /projects work;
    - typing `/portfolio/definitely-not-a-page` → 404.html → Home;
    - `curl -s https://prockallsyms.github.io/portfolio/main.js | grep portfolio` → import line.

 **DoD.**
 - Workflow green on `main`; Pages site live at the confirmed URL; fallback verified;
   no console errors on the live site.

 **Done:** ⬜

 ---

 ### T10 — Ship: history sanitize, README, LICENSE, perf, v1.0.0

 **Context to load:** `README.md`, `LICENSE` (new), `.gitignore`, `git log`, §A.6.

 **Steps.**

 1. `LICENSE` exactly (MIT):
    ```
    MIT License

    Copyright (c) 2026 prockallsyms

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
    ```

 2. `README.md` (exact structure; write the body):
    - `# portfolio` — one-line description matching the site voice.
    - **Stack:** Rust 2024 · Yew 0.23 · yew-router 0.20 · wasm-pack · Tailwind v4 (standalone) · GitHub Pages.
    - **Dev:** `scripts/dev.sh` → http://localhost:8000 (re-run after Rust changes).
    - **Build:** `scripts/build.sh` → `site/` (Pages artifact).
    - **Tests:** `cargo test -p portfolio` · `wasm-pack test --headless --chrome`.
    - **Deploy:** GitHub Actions (Settings → Pages → GitHub Actions).
    - No personal information beyond the "prockallsyms" handle.

 3. **History sanitize** (owner-confirmed: `git filter-repo`). Prereq: working tree clean
    (everything committed). Legacy identities to erase: `[redacted] <[redacted email]>`,
    `[redacted]<[redacted email]>` → `prockallsyms <prockallsyms@users.noreply.github.com>`.
    ```bash
    # 0) Backup the working repo
    cp -r ~/Projects/portfolio ~/portfolio-backup-$(date +%F)

    # 1) Fresh clone (filter-repo refuses to run in a repo that has a remote)
    git clone ~/Projects/portfolio /tmp/portfolio-clean
    cd /tmp/portfolio-clean
    git remote remove origin

    # 2) Mailmap — rewrite both legacy identities
    cat > .mailmap <<'EOF'
    prockallsyms <prockallsyms@users.noreply.github.com> = [redacted] <[redacted email]>
    prockallsyms <prockallsyms@users.noreply.github.com> = [redacted]<[redacted email]>
    EOF

    # 3) Rewrite refs + replace identity strings anywhere in commit messages
    git filter-repo --mailmap .mailmap \
      --replace-message '[redacted]' \
      --replace-message '[redacted email regex]' \
      --replace-message '[redacted]'

    # 4) Verify — each command must show ONLY the canonical identity / "clean"
    git log --all --format='%an <%ae>' | sort -u
    git log --all --format='%cn <%ce>' | sort -u
    git grep -I "v\.samuel88" HEAD || echo "clean: no email strings in trees"
    git grep -I "[redacted]" HEAD || echo "clean: no old handle in trees"

    # 5) Swap the clean repo back into place (working tree was clean, contents identical)
    cd ~
    mv Projects/portfolio portfolio-worktree-old
    mv /tmp/portfolio-clean Projects/portfolio
    cd Projects/portfolio
    git remote add origin git@github.com:prockallsyms/portfolio.git
    git push --force origin main

    # 6) Remove the old working tree once you've confirmed the new one is complete
    rm -rf ~/portfolio-worktree-old
    ```
    Caveat (accepted): GitHub keeps unreachable objects for a while server-side; the
    rewritten history is what clones/API/Pages see from now on.
    The force push triggers the CI/Pages workflow (T09) → site rebuilds automatically.

 4. **Perf report:**
    ```bash
    ls -lh pkg/portfolio.wasm
    gzip -c pkg/portfolio.wasm | wc -c    # goal: < ~1 MiB gzipped; record the number
    ```

 5. **Tag & push:**
    ```bash
    git tag v1.0.0
    git push origin v1.0.0
    ```

 **DoD.**
 - `git log --format='%an %ae'` (whole history) shows only `prockallsyms <prockallsyms@users.noreply.github.com>`.
 - No tree contains the old email/handle (step 4 greps).
 - README + LICENSE in place; site live at the Pages URL; `v1.0.0` tag pushed.

 **Done:** ⬜

 ## §4. Rust best-practices checklist (applies to every task)

 - Edition 2024 idioms; no legacy patterns; `#![forbid(unsafe_code)]` in `lib.rs`.
 - `cargo fmt --all` + `clippy -D warnings` green at every task boundary.
 - No `unwrap()`/`expect()` outside tests. The documented exceptions: app bootstrap
   (no DOM = broken app) and the dev server's workspace path. web-sys `Result`s in
   non-critical UI code use `.ok()` / `let _ =`.
 - `thiserror` only if an error type ever actually needs one (expected: never).
 - Content as typed data → views; **no hardcoded copy in components** (§3 T04 contract).
 - One page/component per file; `//!` module docs on every module; doc comments on public items.
 - Dependency discipline: every dep justified (wasm size!); wasm-bindgen family in lockstep;
   lockfile committed.
 - Hooks only at component top level (never in loops/conditionals).
 - Two test layers: native (data invariants) + wasm (rendered DOM).

 ## §5. Open items

 **Resolved:** collector repo = `collector-rs` ✓ · work history from LinkedIn PDF ✓ (§A.2) ·
 avatar = GitHub profile picture ✓ · history sanitize = `git filter-repo` ✓ ·
 deploy = project site `prockallsyms.github.io/portfolio/` ✓ · yew target = 0.23 ✓.

 1. **Hero tagline** — options: *"Penetration tester & Rust developer"* /
    *"I build security tools in Rust"* / *"Rust + security tooling"*. Pick one or replace.
    (T07 uses the placeholder until then; **nothing else is blocked.**)

 ## Appendix A — Verified research notes

 ### A.1 Environment (verified 2026-07-19)
 - rustc/cargo 1.97.1; `wasm32-unknown-unknown` installed; Node 26.5 present but unused.
 - `wasm-pack` and `git-filter-repo` **not** installed (install steps in T01 / T10).
 - Chrome available (for `wasm-pack test --headless --chrome`).
 - `pip` available (for `pip install git-filter-repo` in T10).



### A.2 LinkedIn data (owner-verified)

Source: owner-provided export `~/Downloads/[redacted] _ LinkedIn.pdf` (the public
LinkedIn page masks job titles — the PDF is authoritative). Extracted 2026-07-19 via
`pdftotext`; this transcription is complete — no need to re-read the PDF in T07.

- **Self-description:** "I'm a math/cyber/development sorta person. I do things @ Elton."
- **Location:** [redacted city/state] (default: omit from site — owner call).
- **Experience** (newest first):
  | org | role | period | notes |
  |---|---|---|---|
  | ELTON | Lead Penetration Tester | Apr 2026 – present | Penetration Testing, Account Management |
  | ELTON | Penetration Tester | Oct 2024 – Apr 2026 | — |
  | Level Nine Group | Penetration Tester / Product Vulnerability Researcher | Jul 2023 – Jun 2025 | Security Research, Threat & Vulnerability Management |
  | City of Bridgeton | System Engineer (part-time) | Jun 2018 – Jul 2023 | Administers a Windows Server network of 200 endpoints + 7 remote VPN networks with Untangle (Arista) firewalls; SQL, Linux |
- **Education:** Truman State University — BS, Mathematics and Computer Science — Aug 2019 – May 2023.
- **Top skills (LinkedIn):** Malware Analysis, Rust, Penetration Testing, Reverse
  Engineering, Vulnerability Assessment.
- **Licenses & certifications** (5 on profile; the two relevant ones):
  - OffSec Exploit Developer (OSED) — issued Oct 2024 (skills: Binary Exploitation, Exploit Development).
  - "Oral History of Binary Exploitation Defenses" course — The Taggart Institute — issued Jul 2023 (skills: Defense Bypass, Shellcoding).
- **Personal projects (LinkedIn profile):**
  - Home Malware Analysis Lab (Aug 2019 – present) — PowerEdge T320 hosting a vLAN with
    Windows client/server VMs, Ubuntu/Red Hat/Rocky servers, REMnux VM.
  - Home Penetration Testing Lab (Jul 2019 – present) — Metasploitable, DVWA, and
    VulnHub machines.
- **Honors:** NSA Codebreaker Challenge Solver — Jun 2023.

Privacy: this section lives in the plan file only (not shipped — the Pages artifact is
`site/` = static + pkg). The site shows **no real name, no email** — identity is
"prockallsyms" only (§1).

### A.3 Project blurbs (verified from shallow clones, 2026-07-19)

All six are `github.com/prockallsyms/<repo>` (private repos; each verified accessible via
SSH `git ls-remote`). One-sentence site-voice blurbs for T07 — polish wording, keep facts:

- **corrode** — Security scanning orchestration in Rust: coordinates 135 security tools
  through a plugin-based pipeline with automatic DAG wiring, SQLite storage, encrypted
  credentials, and cron scheduling. Tags: Rust, security, orchestration, SQLite.
- **collector-rs** — Cross-platform system information collector & security analysis
  toolkit in Rust: 25+ assessment areas (hardware, network, processes, security,
  packages), structured JSON output, fully offline single binary; includes an MCP server
  (44 tools) and an MCP client proxy. Tags: Rust, forensics, MCP, security.
- **analyzer** — File-type detection & analysis-execution microservice: DiE signature
  detection with content-based fallback, module execution (incl. Docker-in-Docker
  modules); CLI + Actix-web server. Tags: Rust, Actix, file analysis, Docker.
- **TestAssist** — Containerized web app for visualizing static-analysis output
  (SARIF, CycloneDX, SPDX, Swagger/JSON/Markdown viewers) with Docker-based analysis
  pipelines; Rust backend, TypeScript frontend, TUI client. Tags: Rust, TypeScript,
  web app, Docker.
- **hookdb** — Static binary instrumentation driven by git-managed hook DBs —
  "Semgrep for binary instrumentation" across JAR/.NET/ELF/PE/Mach-O/BEAM/DEX/Lua/
  APK/IPA. Tags: Rust, binary analysis, instrumentation.
- **malpraxis** — Rogue server + malicious client testing for healthcare protocols
  (MLLP/HL7v2, DICOM, FHIR, NCPDP, CDA, IHE XDS): 261 adversarial behaviors across
  7 attack categories. Tags: Rust, security testing, healthcare protocols.

### A.4 Verified API notes (checked against vendored crate sources, 2026-07-19)

Read before editing `src/` in T03/T04/T08. Do not guess APIs.

**yew 0.23.0**
- `yew::start_app` **does not exist**. Browser entry: `yew::Renderer::<App>::new().render()`.
- `Renderer` (feature `csr`): `.with_root(root: web_sys::Element)`, `.with_props(props)`,
  `.with_root_and_props(root, props)`, `.render() -> AppHandle<COMP>`.
  `with_root` is what the T08 render tests use.
- `yew::tests` is internal-only — **not** a public render helper.
- `Callback<T>` (owned IN) and `CallbackRef<T>` (`&IN`) both in `yew::prelude`; prefer
  `CallbackRef` for event handlers. `#[component]`, `#[derive(Props)]`, `use_state`,
  `use_effect_with_deps`, `classes!` unchanged from 0.19.
- **Default features are empty** — the manifest MUST enable `features = ["csr"]` (done in T02).

**yew-router 0.20.0**
- `BrowserRouter` prop `basename: Option<AttrValue>` is optional — **it defaults to the
  page's `<base href>`** (set to `/portfolio/` in T01). No prop needed.
- Exports: `Router`, `BrowserRouter`, `HashRouter`, `Link`, `Redirect`, `Switch`,
  `Routable`, hooks `use_route`, `use_navigator`, `use_location`.
- `Switch` takes a `render` prop `fn(AppRoute) -> Html`; unknown routes fall to the
  `#[not_found]` variant.
- `Link` props: `to`, `classes`, `query`, `state`. **No automatic active-route styling** —
  compute it with `use_route::<AppRoute>()` (T04 Nav contract).

**web-sys 0.3.105** (feature requirements listed — all enabled in T02)
- `web_sys::window() -> Option<Window>` — feature `Window`.
- `Window::local_storage(&self) -> Result<Option<Storage>, JsValue>` — `Window` + `Storage`.
- `web_sys::document() -> Document` — `Document`.
- `Document::document_element(&self) -> Option<Element>` — `Document` + `Element`.
- `Element::class_list(&self) -> DomTokenList` — `Element` + `DomTokenList`.
- `DomTokenList::add_1(&self, tokens_1: &str) -> Result<(), JsValue>`; single-token
  `remove(&self, token: &str) -> Result<(), JsValue>`.

**wasm-logger 0.2.0**
- Only entry point: `pub fn init(config: Config)`; `wasm_logger::Config::new(level: log::Level)`.
- Correct call: `wasm_logger::init(wasm_logger::Config::new(log::Level::Warn));`

**tiny_http 0.12** (dev-server, host-only)
- `Server::http("127.0.0.1:8000")`, `server.incoming_requests()`, `Request::url()`,
  `Response::from_bytes(bytes, mime)` / `Response::from_string(s)`,
  `.with_status_code(code)`, `.send(&request)`.

### A.5 Tailwind v4 (standalone CLI, zero Node)

- Pin: **v4.3.3**, linux-x64, 111,749,248 bytes.
  URL: `https://github.com/tailwindlabs/tailwindcss/releases/download/v4.3.3/tailwindcss-linux-x64`
  sha256: `dc61b3ac6b8c9ca874c0cc4c57b2409791a64c5540404ca5f5367360babc313a`
  (verified locally: download OK, sha256 matches, binary runs.)
- `scripts/tw.sh` downloads it to `.bin/` on first use (`.bin` gitignored), verifies
  sha256, then runs `css -i static/css/input.css -o static/css/app.css "$@"`
  (`--watch` in dev, `--minify` in release). No Node, no bundler, no config file.
- CSS-first configuration in `static/css/input.css`: `@import "tailwindcss";` +
  `@custom-variant dark (&:where(.dark, .dark *));` (class-based dark mode, driven by the
  inline bootstrap script in index.html + the ThemeToggle component) + `@theme` tokens
  (system font stacks, `--color-accent: #22d3ee`).
- v4 auto-scans project source files for class names (including `html!` macros in `.rs`).
  **Verify** after building: grep `app.css` for a class that only appears in `.rs`
  (e.g. `max-w-3xl`); if missing, add `@source "../src";` to `input.css` and rebuild.

### A.6 Privacy notes

- Git history (pre-T10) contains legacy identities `[redacted] <[redacted email]>`
  and `[redacted]<[redacted email]>` → sanitized via `git filter-repo` per T10
  (owner-confirmed). Caveat: GitHub retains unreachable objects server-side for a while;
  the rewritten history is what clones/API/Pages see from now on.
- The site, README, and repo metadata show **only "prockallsyms"** — no real name, no
  email, no old username (§1; enforced by the T07 DoD grep).
- `PLANS.md` itself contains owner-only data (e.g., the LinkedIn PDF file name). It is
  **not shipped** (Pages artifact is `site/` = static + pkg only) but will live in the
  public repo — decide before the T10 force-push: scrub it or accept it. Owner call.
