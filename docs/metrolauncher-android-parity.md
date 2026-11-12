# MetroLauncherAndroid → Win10X phone parity

Upstream reference:
[lorddie/MetroLauncherAndroid](https://github.com/lorddie/MetroLauncherAndroid)

Operation logic and Windows Phone Start/All-Apps behavior for phone mode follow
this launcher unless a newer explicit user decision overrides it. Visual motion
still prefers legacy Rust / `research/windows-phone-launchers.md` when they
disagree on curves or durations.

## Feature matrix

| Feature | MetroLauncherAndroid | Win10X Flutter | Status |
|---|---|---|---|
| Resizable tiles Small/Medium/Wide/Large | yes | yes (grid sizes) | keep |
| 4-col / 6-col (more tiles) | yes | `phoneMoreTiles` 6/8 | keep; align naming to WP “Show more tiles” |
| Long-press edit + drag reorder | yes (wobble) | long-press edit + wobble + drag | done 2026-11-11 |
| Auto gap fill top-to-left | yes | freeform / pack helpers | verify |
| Swipe ← / ↑ / › / double-tap → All Apps | yes | swipe + › + double-tap | done 2026-11-11 |
| Alphabetical All Apps + side index | yes | catalog jump grid + phone side rail | done 2026-11-11 |
| Search in All Apps | yes | yes | keep |
| Pin / unpin / uninstall-style actions | yes | pin/remove + app info + uninstall MessageBox | done 2026-11-11 |
| Per-tile accent / transparency | yes | tile color + transparent | done 2026-11-11 |
| Live tile badge / text / media | yes | notification / MPRIS media + Disco flip | done 2026-11-11 |
| Start wallpaper image | yes / WP 8.1 | desktop wallpaper + theme veil | done 2026-11-11 |
| Theme light/dark + accent | WP start+theme | prefs + settings UI | done 2026-11-11 |
| WP-style menus / toasts | Android menus | phone bottom-bar menus + `WpToast` | done 2026-11-11 |

## Icon pack

Product icons for phone chrome and Metro-styled tiles come from the local
马克笔UI flat pack (user path; “最新的图表” resolved to the flat-icon worktree):

`/data/OpenMIUI/马克笔UI/包含扁平图标-WP桌面终极版  工作目录资源包/LauncherWP8/icon`

Navigation glyphs are vendored under `flutter_shell/assets/wp_icons/nav/`
(extracted from the 马克笔UI Win10M navbar APK). Do not redistribute Microsoft
marks beyond study; keep vendored assets to the needed glyphs.

## 2026-11-12 review corrections

The earlier “done”/“keep” entries describe code paths, not complete functional
acceptance. Current upstream README explicitly specifies a four-column classic
and six-column compact grid. Phone projection and resize/drop helpers now use
4/6; saved freeform tile cells remain untouched. This supersedes the earlier
6/8 note. Hybrid status/navigation and the selected animation remain intentional.

The current auto-color code reads the host accent, not icon pixels. Its setting
now says “固定时使用系统强调色”; actual icon palette extraction remains partial.
Uninstall opens a software center for an explicit user choice. It no longer
executes an unattended terminal Flatpak uninstall or opens a desktop entry as a
fallback (which could launch the application). App/package removal itself must
be checked against the target distribution and is not marked accepted.

Actual preview exercises cover WP menu lifecycle, Back/forced cancellation,
non-modal feedback and a scrollable landscape information dialog. Notifications,
media controls, package removal and all native-window claims still require real
session evidence; see the current review entry in development-log.md.

## 2026-11-12 — MarkUI rendering coverage

`markui_catalog.dart` maps exact desktop, Flatpak and Waydroid identities to the
supplied pack. Start tiles, All Apps, taskbar and phone task cards use that shared
lookup. `markui_icons.dart` replaces Material command rendering throughout our
shell, including Action Center, menus, notifications, media controls and status.
Status battery fill uses the pack's original 18-level images. Navigation uses
package-qualified assets rather than silently falling back when embedded.

The selected original bitmaps and their hashes are recorded in
`flutter_shell/assets/wp_icons/sources.json`; `tools/import-markui-icons` copies
them verbatim from the user's flat pack/APK. Missing power/pause/skip/pin/resize
command silhouettes use explicit WP line drawings; these are adaptations, not
claimed original MarkUI assets. Unmapped third-party applications retain their
own icon. The separate upstream Settings application and native clients retain
their own internal toolbars; shell icon coverage does not imply their restyle.

## 2026-11-12 — researched task cards and enhanced alphabet rail

The Microsoft Lumia manual and Windows10 Mobile hardware-button contract confirm
hold Back→tasks, card tap→switch, ×→close; the 10051 announcement documents large-
phone grid and landscape presentation. Versioned details and implementation
boundaries are in research/windows10-modes/task-switcher-and-app-index.md.

Phone cards now use app captions, actual window previews, stable IDs, saved
accent, narrow carousel / larger portrait2×2, and an explicit non-toggle Start
card. Downward dismissal follows distance as well as velocity; cancellation has
no spring overshoot. Rejected/pending closes retain their real window card.
No seven-window cap or fabricated Start screenshot is introduced.

The side rail is an explicit user enhancement beyond the manual's header jump
menu. Tap and vertical scrub share exact list padding/header/row geometry, show
the active letter, and retain lazy distant-section navigation and accessible
letter choices. Dragging uses immediate target updates rather than competing
scroll animations. Filtering rebuilds the available sections/offsets.
