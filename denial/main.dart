// SPDX-License-Identifier: GPL-3.0-only
import 'package:denial_desktop/denial_desktop.dart';
import 'package:denial_flutter_sdk/shell.dart';
import 'package:flutter/widgets.dart';
import 'package:win10x_shell/denial.dart';

import 'dart:io';

import 'preview_capture.dart';
import 'shell_chrome.dart';

import 'package:win10x_shell/src/screen_power.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await PreferenceController.instance.load();
  void syncWindowMotion() {
    final prefs = PreferenceController.instance.value;
    final nav = ShellNavigation.instance;
    final changed = nav.mode.value != prefs.mode;
    nav.mode.value = prefs.mode;
    win10xShellMode.value = prefs.mode.index;
    if (changed) {
      nav.closePanels?.call();
      if (nav.oneHanded) nav.setReachability(false);
      nav.send(StartRequest.home);
      if (prefs.mode != ShellMode.desktop)
        WidgetsBinding.instance.addPostFrameCallback(
          (_) => nav.nativeHome?.call(),
        );
    }
    win10xWindowMotionEnabled.value =
        PreferenceController.instance.value.switchAnimation;
  }

  syncWindowMotion();
  PreferenceController.instance.addListener(syncWindowMotion);
  win10xManagedScreenPower =
      Platform.environment['DENIAL_COMPOSITOR_BINARY'] != null &&
      Platform.environment['WIN10X_PREVIEW_CAPTURE'] == null;
  installWin10xShellChrome();
  ShellNavigation.instance.reachOffset.addListener(() {
    win10xReachability.value = ShellNavigation.instance.reachOffset.value;
  });
  final shell = ValueListenableBuilder<ShellMode>(
    valueListenable: ShellNavigation.instance.mode,
    builder: (context, mode, _) => ReferenceDesktop(
      surfaces: [
        Win10xTaskbarPlugin(mode: mode),
        Win10xStatusBarPlugin(mode: mode),
      ],
      workArea: Win10xWorkArea(mode),
      launcher: Win10xStartLauncher(),
      actions: [OpenStartAction()],
      // Windows 10 maximized clients reserve the navigation/taskbar strip.
      // Fullscreen clients retain native fullscreen semantics.
    ).createShell(),
  );
  final capture = Platform.environment['WIN10X_PREVIEW_CAPTURE'];
  await runDenialShell(
    shell: capture == null
        ? ScreenPowerHost(child: shell)
        : PreviewCapture(
            path: capture,
            child: ScreenPowerHost(child: shell),
          ),
  );
}
