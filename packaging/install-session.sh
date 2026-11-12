#!/usr/bin/env bash
# Install session files for greetd/SDDM-style session selection (system-wide).
set -euo pipefail
PREFIX="${PREFIX:-/usr}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

install -Dm755 "${1:-$ROOT/packaging/out/win10x-comp}" "$PREFIX/bin/win10x-comp"
install -Dm755 "${2:-$ROOT/packaging/out/win10x-shell}" "$PREFIX/bin/win10x-shell" || true
install -Dm644 "$ROOT/packaging/win10x.desktop" "$PREFIX/share/wayland-sessions/win10x.desktop"
echo "Installed win10x-comp and wayland-sessions/win10x.desktop under $PREFIX"
