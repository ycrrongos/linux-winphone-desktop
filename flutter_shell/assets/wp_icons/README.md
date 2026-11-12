# MarkUI / 马克笔UI icons

Original source: the user's flat WP desktop worktree at
`/data/OpenMIUI/马克笔UI/包含扁平图标-WP桌面终极版  工作目录资源包`.
`tools/import-markui-icons` copies selected PNGs verbatim; `sources.json`
records individual original paths and SHA256s and the source APK SHA256.
The `*8` application variants are recognizable flat glyphs. Several unnumbered
base variants used previously appeared as solid white tile faces when isolated.

- `apps/`: selected application/category artwork from LauncherWP8/icon.
- `ui/`: original launcher/quick-action/status assets from the supplied navbar
  launcher APK, plus the original flat folder silhouette. Includes 18 battery
  levels and six signal levels.
- `nav/`: previously vendored Back/Start/Search/Task View glyphs from the same
  pack. Original navbar button PNGs include transparent padding; compensate in
  ShellGlyph painting rather than altering the bitmap.

Look up package assets through `package: win10x_shell`. Application aliases
normalize Denial's desktop/local ID prefixes; an application not in the map
retains its own artwork. Command shapes missing from the original pack use the
explicit WP line painter in markui_icons.dart. Those adaptations are not
presented as original MarkUI assets. Native third-party application toolbars and
upstream standalone Settings have separate icon/rendering implementations.

Both bundle verifiers compare every packaged PNG with source. Asset contents
also participate in the shell's source hash, so PNG-only edits invalidate stale
builds. Do not copy historical Microsoft research screenshots into this folder.
