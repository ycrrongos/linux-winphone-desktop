# Separate GitHub repositories — 2026-11-13

User requested two new repositories, owner-only GitHub contribution identity,
and Codex disclosure in each repository description.

- Desktop: https://github.com/ycrrongos/win10x-denial-desktop
- Keyboard: https://github.com/ycrrongos/winphone-pinyin-keyboard

Publication uses fresh root commits by ycrrongos, with the GitHub-account-linked
noreply address. No Codex/agent coauthor trailer is added. Existing upstream
copyright/license/source notices remain; commit authorship does not assert
ownership of upstream code, dictionary data or supplied artwork.

The keyboard repository is a standalone Dart core package plus a pinned Denial
0.6.2 OSK adapter. The desktop retains verified integrated copies and records
keyboard commit/hash provenance in third_party/winphone-pinyin/UPSTREAM.json;
tools/sync-pinyin-keyboard verifies/restores that pinned integration. This keeps
normal desktop clones buildable without hidden local paths or submodules.

Snapshots are prepared in target/github-publication-20261113/; the original
worktree, old origin and existing history are preserved. The export excludes
machine-specific agent/cache/state directories, compiler outputs, SDK overrides,
phone credentials and root backup material. Binary assets retain their original
provenance. Publication scan compares exported content against known credentials
without printing secret values.

Visibility defaults to private because the optional public/private question
received no answer before publication. Both descriptions and READMEs explicitly
state that the projects were developed with Codex. Public visibility can be
selected later without changing source or authorship.
