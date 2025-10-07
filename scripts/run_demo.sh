#!/usr/bin/env bash
set -euo pipefail

INPUT="${1:-rickroll.mp4}"
FPS="${2:-30}"
COLS="${3:-160}"
ASPECT="${4:-2.0}"
AUDIO="${5:-false}"

rm -rf frames out_txt
mkdir -p frames out_txt

echo "[1/4] ffmpeg extract frames…"
ffmpeg -hide_banner -loglevel error -i "$INPUT" \
  -vf "fps=${FPS},scale=${COLS}:-1:flags=area" frames/%06d.png

if [ "$AUDIO" = "true" ] || [ "$AUDIO" = "audio" ]; then
    echo "[2/4] ffmpeg extract & bitcrush audio…"
    ffmpeg -hide_banner -loglevel error -i "$INPUT" -vn -ac 1 -ar 16000 \
      -af "lowpass=f=6000,acrusher=bits=8:mode=lin:mix=1" \
      -c:a pcm_u8 audio_8bit.wav
    echo "🎵 8-bit audio saved to: audio_8bit.wav"
    echo "   Play it with: afplay audio_8bit.wav (macOS) or aplay audio_8bit.wav (Linux)"
fi

echo "[3/4] rust ascii batch…"
cargo run --release --bin ascii-batch -- frames out_txt "$COLS" "$ASPECT" triangle basic

echo "[4/4] play ascii animation…"
cargo run --release --bin ascii-play -- out_txt "$FPS"
