# Flutter vs legacy Rust animation / feel gaps

Living checklist for Denial/Flutter parity with `shell/` (pure Rust).
When a row is closed, record the closing date and the verifying path.

Finger-follow Start↔apps page push (Phone horizontal / Win8 vertical) was
shipped 2026-11-11 and is **out of scope** for the remaining work below.
Those two items intentionally go **beyond** the old Rust release-then-animate
behavior, following MetroLauncherAndroid ViewPager2.

Acceptance rule (2026-11-11 user decision): every remaining row must match the
legacy Rust animation unless the Rust path itself feels stiff — then improve
both the feel and document the intentional divergence here.

## Remaining gaps

| ID | Area | Legacy Rust | Flutter today | Severity | Target |
|---|---|---|---|---|---|
| A1 | Disco Start turnstile | Four-vertex mesh true perspective | `DiscoTurnstileTile` projected quads + content rotate | closed 2026-11-11 | Visual accept |
| A2 | Factor Start entrance | Slide + scale about tile center | Present | low | Keep constants; verify timing |
| A3 | W10M launch handoff | Settled 100% geometry, row lift, 80 ms selected delay, 200 ms veil hold | Mostly present | medium | Frame-compare vs Rust / research frames |
| A4 | Win8 flying tile launch | 720 ms; scene retreat 72%/240 ms; lift→edge→expand | `Windows8FlyingTile` present | medium | Match segment curves and retreat |
| A5 | Disco launch turnstile | Source-tile true perspective exit | Disco closing frames during launch | closed 2026-11-11 | Visual accept |
| A6 | Factor clip-reveal launch | Source-rectangle clip reveal | `FactorClipReveal` overlay | closed 2026-11-11 | Visual accept |
| A7 | App-switch snapshot handoff | Taskbar zoom 200 ms; app↔app 500 ms retreat+slide; proxy above taskbar | `AppSwitchHandoff` on taskbar / Task View / phone | closed 2026-11-11 | Visual accept |
| A8 | Tile drop settle | `ease_out_back` overshoot | `easeOutBack` | closed 2026-11-11 | Keep / visual accept |
| A9 | Action Center / edge panels | Largely end-triggered | Live drag via `DashboardReveal` / `win10xDashboardReveal` | closed 2026-11-11 | Visual accept |
| A10 | Wallpaper + veil timing | Sync with last delayed tile; W10M launch keeps veil | Same policy + acrylic veil when enabled | closed 2026-11-11 (code path); visual accept | Visual acceptance |
| A11 | Phone Start background | Wallpaper + veil (not solid black) | Theme veil over desktop wallpaper | closed 2026-11-11 | Visual accept |
| A12 | Theme background light/dark | N/A in Rust shell prefs | `phone_theme_dark` | closed 2026-11-11 | Visual accept |
| A13 | Accent color | Fluent accent fixed | Selectable WP accents | closed 2026-11-11 | Visual accept |

## Intentional optimizations past Rust

| ID | Change | Why |
|---|---|---|
| O1 | Start↔apps finger-follow while dragging | Rust only settled after release; MetroLauncher ViewPager2 is the feel target |
| O2 | Edge / Action Center live drag | Same ViewPager/scroll coupling; shipped with A9 |

## Related product baselines (not animation-only)

See `AGENTS.md` (MetroLauncherAndroid operation logic) and
`docs/metrolauncher-android-parity.md` for the Windows Phone feature port list.

## 2026-11-12 audit correction

“Closed” above means an implementation exists; it does not prove all-frame or
native-session acceptance. This review corrected A1/A5's face/content projection
mismatch with a shared homography numerically checked against Rust for square,
wide and tall tiles. A7 now uses a non-interactive proxy clipped to the app
viewport, without popup focus capture/restoration or a second host fade. Native
app-to-app activation and texture refresh still need device acceptance.

A9 corrected the tablet left-edge command (Task View, not dashboard), gesture
cancellation and stale settle ownership. Uninterrupted and interrupted panel
motion require separate preview checks. General popup exit durations now use
`reverseDuration`; retained menus no longer combine two independent show/close
animations. Visual parity with original WP releases remains a separate criterion
from numeric Rust parity. WP navigation/chrome hybrid choices are retained.

## 2026-11-12 — remove the second native entrance / preserve client proportions

The user's new request explicitly removes the remaining Denial random-origin
squircle/lead-in entrance. The prepared window frame disables its tracked
initial reveal; the selected Rust-derived Start/launch handoff remains the sole
launch presentation, including cold starts. This does not change the separate
Start entrance, launch-style or application-switch preferences.

Retained window/popup transitions previously used separate X/Y scales, and
local Flutter content used BoxFit.fill even on ordinary live layout. Native
buffers could likewise be stretched while waiting for a configure commit.
Shared uniform fitting now governs retained paint/hit transforms, committed
client presentation and native input/popup coordinates. Local Flutter clients
reflow normally; only transformed presentations retain their canonical layout.
Phone/tablet first placement configures even when initial geometry is absent.
A non-responsive or minimum-size-constrained client may temporarily be centered
with unused space; it is never claimed to have reflowed just because it fits.

Native acceptance is pending reconnection and reproduction of the specific
applications. The opt-in client-proportions fixture uses the real prepared
retained/composition widgets with a labelled synthetic buffer; it is a rendering
regression check, not a substitute for native Wayland/XWayland client testing.
