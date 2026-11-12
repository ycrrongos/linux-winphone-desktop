# Windows 10 Mobile task cards and app-list navigation

Researched 2026-11-12 for the user's task-switcher / fast alphabet-index request.
No Microsoft screenshot or manual artwork is copied into product assets.

## Evidence and versions

- [Microsoft Lumia with Windows 10 Mobile User Guide, issue1.0 (2015)](https://files.customersaas.com/files/Microsoft_Lumia_950_User_manual.pdf),
  printed p17 and p24: hold Back to open tasks; tap a card to switch; upper-right
  × closes; illustrated horizontal card browsing. Printed p24 also describes
  tapping an app-list letter then choosing a letter/character in the jump menu.
  Read text and visually inspected the complete p24 illustration.
- [Microsoft Windows10 button behavior](https://learn.microsoft.com/en-us/windows-hardware/drivers/gpiobtn/button-behavior)
  independently specifies long Back→switcher UI for Windows10 Mobile.
- [Microsoft build10051 announcement (2015-04-10)](https://blogs.windows.com/windows-insider/2015/04/10/new-windows-10-technical-preview-build-now-available-for-phones/)
  explicitly adds landscape operation and a grid layout for large phones such
  as Lumia1520. This is an Insider introduction, not proof of every late build's
  exact geometry or entry-count limits.
- [Steve Litchfield's first-hand 10586-era comparison (2015-12-08)](https://allaboutwindowsphone.com/features/item/21118_Microsoft_trimmed_the_Windows_.php)
  shows single-height and double-height carousels by screen resolution, app
  icon/name/close control above thumbnails and a Start entry. It also documents
  changing recent-app limits; that cap is not imposed on real Linux windows.
- [Lumia950 user's interaction notes (2016-09-24)](https://cloudadvisors.net/2016/09/24/windows-10-mobile-shortcuts-and-tips/)
  describe both downward swipe dismissal and the close button. This is a
  contemporary usage report, not a Microsoft timing/gesture-threshold spec.

Do not use Windows Phone7 Mango's five-card limit or Android's task gestures as
Windows10 Mobile evidence. The older Microsoft Mango navigation article only
establishes the historical alphabet-header jump design.

## Implementation decisions

Our phone mode keeps Back/Start/Search and hybrid status/navigation chrome.
Long Back opens one task layer; short Back dismisses it. Cards show real client
previews, application names/icons and ×. Horizontal pages browse, taps activate
exact native IDs; downward drag follows the finger and closes on sufficient
travel or velocity, with a non-overshooting return for cancellation. A card is
removed only after real backend removal; a still-live/rejected/confirmation-
pending close returns to view. Start is an explicit shell destination without
an invented native window or fabricated screenshot, and cannot be closed.

Remove the added Return/app-count toolbar. Use the saved accent for the task
background. Narrow/landscape phone views show a single-height carousel; larger
portrait viewports use a two-row/two-column page. The 420×720 logical viewport
threshold is our responsive adaptation, not a claimed original OS threshold.
Preserve all real Linux windows and stable order while the panel is open;
active-first entry is not a claim that the host supplies full historical MRU.

The user explicitly requests a side alphabet rail. Retain Microsoft's header→
alphabet-grid flow and add this product enhancement: tap or drag the rail,
immediate finger-follow jumps, active-letter feedback, and shared measured
section offsets. Keep short nonlinear tap jumps; do not enqueue animated scrolls
while dragging. Letter-grid navigation must retain accessible phone Back/Start/
Search. Group order is # then present A–Z; existing non-Latin # grouping remains.

## 2026-11-12 — user lock and volume screenshots, popup verification

The user supplied two study references: a WP picture lock page (time/date low
on the left, notifications below, small connectivity/battery at the top), and a
Lumia top volume strip (channel/slider/value, chevron and optional media rows).
They are not vendored as wallpaper/product art. Use the user's desktop wallpaper.
Swipe is directly coupled to the finger; only released/cancelled motion eases.
Sliding to credentials never changes native secure state or acknowledges a
successful unlock; the existing authentication service/PAM still owns that.

Microsoft's [2014 universal XAML controls article](https://blogs.windows.com/windowsdeveloper/2014/07/22/tips-and-tricks-for-using-xaml-controls-in-your-universal-windows-apps/)
distinguishes phone flyout elevator/escalator and fullscreen letter pickers.
[Microsoft's animation overview](https://learn.microsoft.com/en-us/windows/apps/develop/motion/xaml-animation)
distinguishes fade dialogs, pop-in/out context flyouts, and edge CommandBars.
Do not call every old WP8/8.1 template Windows10 Mobile pixel/timing evidence.
Original Microsoft phone8.1 template text is inspectable in the research-only
[generic.xaml mirror](https://gist.github.com/tmyt/dca3b0ccb3374c90a11a1b2e5b04ff2c):
ContentDialog has separate title/content/two-button rows, MenuFlyout uses a
bordered panel and horizontal-then-vertical reveal; no supplied phone bitmap is
copied from it. The full-screen→settled-dialog uniform zoom is an explicit user
requested presentation here; no primary evidence found proves that all W10M
system dialogs use it. Reverse it on exit, preserve correct input transforms,
and do not introduce independent axis stretch into real application surfaces.

WP Toolkit pinned4e4fd5d36c769c004f7ccc7daf78a5ffb8ef18df has original
SlideUpFadeIn350ms / SlideDownFadeOut250ms with ExponentialEase exponent6.
Our shared WP enter curve now uses the normalized exponent6 expression and
mirrored exit, rather than calling arbitrary base2/exponent10 easing original.
This is a documented source-derived adaptation, not all-build timing proof.

The Lumia950 2015 manual, printed pp26–27 (local research copy
`/tmp/win10x-research-20261111/lumia950-manual.pdf`), explicitly distinguishes
ringer0–10 from media/apps0–30. The phone media HUD therefore labels0–30 with
30 slider intervals, mapping to the actual Linux normalized output service;
Action Center's percentage control remains a percentage. Do not invent an
independent ringer mixer where the session has no such channel.

## 2026-11-13 — contextual menu sizing and dimming

User explicitly requests content-width phone context menus, transparent outside
regions for ordinary menus/volume, retaining dimming for confirmation dialogs /
QS. UWP distinguishes MenuFlyout/ContentDialog and exposes a separate
LightDismissOverlayMode; there is not one dimming policy for every Windows build.
Sources: [Microsoft MenuFlyout API](https://learn.microsoft.com/en-us/uwp/api/windows.ui.xaml.controls.menuflyout)
and [UWP flyouts](https://learn.microsoft.com/en-us/windows/uwp/design/controls-and-patterns/dialogs-and-flyouts/flyouts).
Use intrinsic longest-item width bounded to viewport, scroll tall content, retain
outside-touch dismissal and focus trapping. CommandBars remain edge surfaces.
The screen wake/sleep600ms timeline is the user's own specification, not claimed
historical Windows motion.
