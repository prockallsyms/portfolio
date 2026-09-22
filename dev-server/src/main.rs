//! Local development server for the portfolio.
//!
//! Serves `static/` (site files) and `pkg/` (wasm-pack output) on
//! `http://localhost:8000/`. There is no file watcher: after changing
//! Rust code, re-run `wasm-pack build --dev` and refresh the browser.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use tiny_http::{Header, Request, Response, Server};

const PORT: &str = "8000";

type ServerError = Box<dyn std::error::Error + Send + Sync>;

fn main() -> Result<(), ServerError> {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("dev-server lives in a workspace member directory");
    let server = Server::http(format!("127.0.0.1:{PORT}"))?;
    println!("portfolio dev server: http://localhost:{PORT}/");
    println!("  {workspace:?}/static -> site files");
    println!("  {workspace:?}/pkg    -> wasm-pack output");

    for request in server.incoming_requests() {
        handle(request, workspace)?;
    }
    Ok(())
}

fn handle(request: Request, workspace: &Path) -> Result<(), ServerError> {
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

    if let Some(rel_path) = safe_join(&base, &rel) {
        let abs = base.join(&rel_path);
        if abs.is_file() {
            let bytes = fs::read(&abs)?;
            let response = Response::from_data(bytes)
                .with_header(Header::from_bytes("Content-Type", mime_for(&abs)).unwrap());
            request.respond(response)?;
        } else {
            request.respond(Response::from_string("404 — not found").with_status_code(404))?;
        }
    } else {
        request.respond(Response::from_string("404 — not found").with_status_code(404))?;
    }
    Ok(())
}

/// Join `base/rel`, rejecting `..` traversal and symlink escapes.
/// Returns the relative path under `base` after canonicalization.
fn safe_join(base: &Path, rel: &str) -> Option<PathBuf> {
    if rel.contains("..") {
        return None;
    }
    let path = base.join(rel);
    let canonical_base = base.canonicalize().ok()?;
    let canonical_path = path.canonicalize().ok()?;
    Some(
        canonical_path
            .strip_prefix(&canonical_base)
            .ok()?
            .to_path_buf(),
    )
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
