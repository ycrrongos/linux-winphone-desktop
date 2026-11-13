# Native PC-keyboard extension

Base: Denial0.6.2, revision3f08a93ec42b9851ebdd73039def1ff12a639858,
https://github.com/denialwm/denial . The upstream GPL-3.0-or-later source and
licenses are obtained by tools/win10x-denial-native; pc-keyboard.patch adds only
named evdev mappings and a focused mapping test. It does not weaken secure
session routing or use root/uinput injection. SDK press/tap/release messages
retain the compositor's existing key ownership and cancellation handling.

```
tools/win10x-denial-native build --target x64
tools/win10x-denial-native test
```

ARM64 cross-build with a matching installed Rust target and C cross compiler:

```
tools/prepare-denial-native-sysroot
tools/win10x-denial-native build --target arm64 --sysroot target/denial-native/sysroot
```

The sysroot helper downloads SHA256-pinned Debian trixie packages listed in
arm64-sysroot.lock.json into target only; no phone/host packages are installed.
It requires Python3, bsdtar and HTTPS access. Cargo dependencies are pinned by
upstream Cargo.lock. Compiler/patch/binary provenance is emitted in
`target/denial-native/{x64,arm64}.json`; packages reject missing/stale/wrong-arch
native artifacts instead of silently bundling the upstream incomplete key map.

For vendor Rust builds with incompatible official rust-std metadata, the
optional `--bootstrap-std` builds a matching standard library from locked Rust
source and prints the environment for the subsequent native build. It does not
replace system Rust. This fallback is currently pinned to Rust1.99.1; update the
lock for another compiler. Building std before third-party crates is necessary:
io-lifetimes probes target std during its build script and otherwise selects a
fallback AsFd trait incompatible with Smithay's std::os::fd::AsFd. Normal rustup
matching compiler/target installations do not need this fallback.

Packaging copies pc-keyboard.patch/native-build.json beside the installed
runtime, retains Denial's license files, and requires the matching Flutter bundle.
A successful cross-build or `--help` under QEMU is not a real-seat input test.
