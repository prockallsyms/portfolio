#!/usr/bin/env bash
# Production build: minified CSS + release wasm → site/ (the Pages artifact).
set -euo pipefail
cd "$(dirname "$0")/.."
scripts/tw.sh --minify
wasm-pack build --release --target web
rm -rf site && mkdir site
cp -R static/. site/
cp -R pkg/. site/pkg/
