#!/usr/bin/env bash
# Local dev loop: wasm-pack dev build + dev server on :8000.
# Re-run after Rust changes; refresh the browser after each rebuild.
set -euo pipefail
cd "$(dirname "$0")/.."
wasm-pack build --dev
exec cargo run -p dev-server
