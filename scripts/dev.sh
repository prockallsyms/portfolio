#!/usr/bin/env bash
# Local dev loop: Tailwind watch + wasm-pack dev build + dev server on :8000.
# Re-run after Rust changes; refresh the browser after each rebuild.
set -euo pipefail
cd "$(dirname "$0")/.."
scripts/tw.sh --watch &
TW_PID=$!
trap 'kill "$TW_PID" 2>/dev/null || true' EXIT
wasm-pack build --dev --target web
exec cargo run -p dev-server
