#!/usr/bin/env bash
# Build release binaries for host or aarch64.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

TARGET_KIND="${1:-host}"
OUT="$ROOT/packaging/out"
mkdir -p "$OUT"

need_cmd() {
  command -v "$1" >/dev/null 2>&1
}

case "$TARGET_KIND" in
  host)
    cargo build --release -p win10x-comp -p win10x-shell
    cp -f target/release/win10x-comp "$OUT/"
    cp -f target/release/win10x-shell "$OUT/"
    echo "Built host binaries in $OUT"
    ;;
  aarch64|arm64)
    TRIPLE="aarch64-unknown-linux-gnu"
    if ! rustc --print target-list | grep -qx "$TRIPLE"; then
      echo "error: rustc does not list $TRIPLE" >&2
      exit 1
    fi
    if [[ ! -d "$(rustc --print sysroot)/lib/rustlib/$TRIPLE" ]]; then
      echo "error: std for $TRIPLE is not installed." >&2
      echo "  With rustup:  rustup target add $TRIPLE" >&2
      echo "  On Arch-like: install a distro rust aarch64 target package if available," >&2
      echo "                or install rustup and add the target." >&2
      exit 2
    fi
    cargo build --release -p win10x-comp -p win10x-shell --target "$TRIPLE"
    cp -f "target/$TRIPLE/release/win10x-comp" "$OUT/win10x-comp-aarch64"
    cp -f "target/$TRIPLE/release/win10x-shell" "$OUT/win10x-shell-aarch64"
    echo "Built aarch64 binaries in $OUT"
    echo "Note: linking may still need aarch64-linux-gnu-gcc and target Wayland/EGL libs."
    ;;
  *)
    echo "Usage: $0 [host|aarch64]" >&2
    exit 1
    ;;
esac

install -Dm644 "$ROOT/packaging/win10x.desktop" "$OUT/win10x.desktop"
install -Dm755 "$ROOT/packaging/install-session.sh" "$OUT/install-session.sh"
