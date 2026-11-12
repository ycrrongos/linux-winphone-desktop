#!/usr/bin/env bash
# Run explicitly after reviewing the staged package. Does not restart sessions.
set -euo pipefail
ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
STAGE="${1:-$ROOT/packaging/out/denial}"
PREFIX="${PREFIX:-/usr}"
DESTDIR="${DESTDIR:-}"

[[ -f "$STAGE/lib/win10x-denial/shell/win10x-build.json" ]] || {
    echo 'Build and stage first: tools/win10x-denial bundle && tools/win10x-denial package' >&2
    exit 1
}
install -d "$DESTDIR$PREFIX/lib/win10x-denial"
cp -a "$STAGE/lib/win10x-denial/." "$DESTDIR$PREFIX/lib/win10x-denial/"
# Temporary build generations may start at 0700. Installed public runtime
# directories must remain traversable after their owner changes to root.
chmod -R a+rX "$DESTDIR$PREFIX/lib/win10x-denial"
install -Dm755 "$STAGE/bin/win10x-denial-session" "$DESTDIR$PREFIX/bin/win10x-denial-session"
install -Dm755 "$STAGE/bin/win10x-denial-polkit-agent" "$DESTDIR$PREFIX/bin/win10x-denial-polkit-agent"
install -Dm644 "$STAGE/share/wayland-sessions/win10x-denial.desktop" \
    "$DESTDIR$PREFIX/share/wayland-sessions/win10x-denial.desktop"
echo "Installed the separate experimental Win10X (Denial / Flutter) session under $DESTDIR$PREFIX."
echo 'The legacy session and the currently running desktop were not restarted.'

# Explicit root helper for authorizing only the user's two saved screen hooks.
install -Dm755 "$STAGE/libexec/win10x-screen-hooks" "$DESTDIR/usr/local/libexec/win10x-screen-hooks"
