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
exec "$BIN" build -i static/css/input.css -o static/css/app.css "$@"
