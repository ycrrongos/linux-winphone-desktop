# Windows 10 / Windows 10 Mobile shell reference

Researched 2026-11-11, before changing the mode implementation. Current user
priority: implement Windows 10 **tablet** mode first; also provide an ordinary
Windows 10 desktop and a separate Windows 10 Mobile phone mode. These are shell
modes, not aliases for the existing selectable tile layouts.

## Baselines and evidence

- Windows 10 PC/tablet: 1903-era light shell (18362), using the tablet interaction
  introduced in 1607. This preserves the requested light Fluent 1 material and
  colored Live Tiles rather than mixing in Windows 11 or the cancelled Win10X
  launcher. Microsoft explicitly introduced light Start, taskbar and Action
  Center in build 18282: https://blogs.windows.com/windows-insider/2018/11/14/announcing-windows-10-insider-preview-build-18282/
- Tablet behavior: full-screen Start, separate pinned/all-apps views in the left
  rail, touch taskbar with Back, and full-screen applications. Right edge opens
  Action Center, left edge opens Task View. Official device experience:
  https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/continuum
  and the contemporary build announcement:
  https://blogs.windows.com/windows-insider/2016/04/22/announcing-windows-10-insider-preview-build-14328-for-pc-and-mobile/
- Microsoft Press's touchscreen chapter distinguishes the hidden app buttons,
  Back/search/task-view taskbar and two-app snap from ordinary desktop behavior:
  https://www.microsoftpressstore.com/articles/article.aspx?p=3104533&seqNum=10
- Phone: late Windows 10 Mobile (1709/15254), with 1511 Lumia manual screenshots
  corrected for the documented 1607 notification changes. BetaWiki's version
  index confirms the released families; direct article retrieval returned 403,
  so build metadata was read from the search index, not invented as a gallery:
  https://betawiki.net/wiki/Windows_10_Mobile
- Original Microsoft reviewer PDF, visually inspected page 8: top-down dark
  Action Center, quick actions **above** notifications, expandable grid, one
  icon in each app-group header. It is evidence, not an asset to redistribute:
  https://news.microsoft.com/wp-content/uploads/2016/04/Microsoft-LUMIA-550-Reviewers-Guide-FINAL.pdf
- Lumia with Windows 10 Mobile User Guide, original Microsoft-authored manual
  retrieved from a device-support mirror. Inspected navigation and view-switch
  text (printed pages 17, 23–26); illustrations remain research-only:
  https://files.customersaas.com/files/Microsoft_Lumia_950_User_manual.pdf
  Three navigation keys are Back / Start / Search; hold Back for live tasks;
  Start leaves an application running; apps are alphabetized with letter jump.
- Microsoft's confirmed reachability behavior (hold Start; tap upper blank area
  or hold Start again to return):
  https://blogs.windows.com/windows-insider/2015/09/14/announcing-windows-10-mobile-insider-preview-build-10536/
- 1607 changes: grouped icons, three notifications per application followed by
  expansion, customizable quick actions:
  https://blogs.windows.com/windows-insider/2016/04/14/announcing-windows-10-mobile-insider-preview-build-14322/

- Desktop version metadata was also cross-checked against BetaWiki's
  18362.1 page and 1903 version index (search-index content available):
  https://betawiki.net/wiki/Windows_10_build_18362.1
  https://betawiki.net/wiki/Windows_10_May_2019_Update
  Mobile 15254.1 is the released 1709 family, distinct from reconstructed
  self-host ARM64 packages:
  https://betawiki.net/wiki/Windows_10_Mobile_build_15254.1

## Intentional boundaries

The goal is close shell behavior and visual geometry, not a claim that Linux
applications implement UWP page navigation or that Microsoft cloud services,
telephony, authentication, OOBE or every system application are recreated.
Native back must use Denial's real back operation; no fake app windows or
successful radio switches are painted. Continue to use installed Linux app
identities/icons. Keep the existing no-acrylic decision: plain alpha surfaces
are an explicit material exception to historical translucency.

## Local saved references (secondary only)

UsefulSaved #335, “Metro动态磁贴Ui windows10mobile 手机”:
https://www.bilibili.com/video/BV1nGnE6zEtw — metadata only, no transcript read;
it describes a third-party desktop and is not original-Windows proof.
UsefulSaved #6000, “X86原生运行的Windows10Mobile？探索Windows Mobile的前世今生”:
https://www.bilibili.com/video/BV1F5eMeKESd — metadata only, no transcript read.
UsefulSaved #80, “WP Dark 精神续作，Windows10 风格主题 MarkUI 将迎来秋季更新”:
https://www.coolapk.com/feed/68171188 — saved body available, an independent
MIUI theme rather than the original system. No code/assets from these were used.

## Acceptance targets

- Persist and switch three modes without rewriting shared tiles or the saved
  optional Start animation/layout choices.
- Tablet first: full-screen tile page and full-screen alphabetical app view,
  left rail, left-aligned touch taskbar, dedicated Action Center entry, no phone
  status strip, correct distinct left/right edge commands.
- Desktop: bounded Start at lower left, app list beside tiles, running/pinned
  app buttons, notification/clock/tray at right, no phone strip.
- Phone: six-column tiles, no desktop header/taskbar, dark alphabetized app
  page with letter jump, persistent Back/Start/Search navigation while apps are
  open, long-press task switcher, centered status clock with connectivity left /
  battery right, and full-width top-down notification shade. Soft keyboard
  dismisses reachability and pads shell panels.
- Check layout at phone, portrait tablet, landscape tablet and desktop sizes;
  test native window/input behavior separately from Wayland UI preview.

- [Task cards and fast app indexing](task-switcher-and-app-index.md): Microsoft
  Lumia manual, long-Back entry, large-phone card grids and app-letter jump.
