# Integrated keyboard source

The offline pinyin engine, dictionary and WP keyboard adapter are maintained in
https://github.com/ycrrongos/winphone-pinyin-keyboard . UPSTREAM.json pins the
exact commit and every imported file hash. Integrated copies keep a normal
clone buildable; no submodule checkout or build-time network fetch is needed.

Run `tools/sync-pinyin-keyboard --check /path/to/winphone-pinyin-keyboard` to
verify parity, or omit `--check` to restore the pinned integrated files.
The adapter's Flutter asset package prefix is the only integration substitution.
Dictionary notices/licenses remain beside the dictionary in flutter_shell/assets/ime/.
