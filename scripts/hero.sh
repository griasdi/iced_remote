#!/usr/bin/env bash
# Renders the README hero, assets/hero.png: examples/hero.rs driven by one
# ctl batch, its annotated screenshot next to a terminal with the batch and
# the replies, verbatim. Needs python3 with Pillow.
set -euo pipefail
cd "$(dirname "$0")/.."

cargo build --quiet --example hero --bin iced-remote
remote=target/debug/iced-remote
work=/tmp/iced-hero
sock=/tmp/hero.sock
rm -rf "$work" "$sock"
mkdir -p "$work"

target/debug/examples/hero --headless --control "$sock" --scale 2 &
cat >"$work/batch" <<'EOF'
tap #new
type Ship the release
key enter
tap Check contrast in light mode
wait-idle
screenshot --annotate /tmp/iced-hero/shot.png
EOF
"$remote" ctl "$sock" - <"$work/batch" >"$work/replies"
"$remote" ctl "$sock" quit >/dev/null
wait

python3 scripts/hero-compose.py "$sock" "$work/batch" "$work/replies" "$work/shot.png" assets/hero.png
