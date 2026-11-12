# Win10X Flutter shell

The in-progress Win10X UI port for the pinned Denial runtime. This package
contributes `ShellSurface`, `ShellLauncher` and `ShellAction`; the project's
`denial/main.dart` composes them with Denial's reference desktop.

Use `tools/win10x-denial` from the repository root for the exact patched Flutter
kit, dependency resolution, analysis, tests, release assembly and Wayland UI
preview. A stock `flutter run` or installing only this plugin does not reproduce
the project's full-screen host adaptation.

See [`docs/denial-migration.md`](../docs/denial-migration.md) for commands,
configuration migration, current acceptance and incomplete behavior.

Project-authored code: GPL-3.0-only. No Microsoft artwork is included.
