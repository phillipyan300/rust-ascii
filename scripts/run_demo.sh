#!/usr/bin/env bash
set -euo pipefail

INPUT="${1:-rickroll.mp4}"
FPS="${2:-30}"
COLS="${3:-160}"
ASPECT="${4:-2.0}"

rm -rf frames out_txt
mkdir -p frames out_txt

echo "[1/3] ffmpeg extract…"
ffmpeg -hide_banner -loglevel error -i "$INPUT" \
  -vf "fps=${FPS},scale=${COLS}:-1:flags=area" frames/%06d.png

echo "[2/3] rust ascii batch…"
cargo run --release --bin ascii-batch -- frames out_txt "$COLS" "$ASPECT" triangle basic

echo "[3/3] play…"
cargo run --release --bin ascii-play -- out_txt "$FPS"
