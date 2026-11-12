// SPDX-License-Identifier: GPL-3.0-only
@Plugin()
library;

import 'package:denial_flutter_sdk/actions.dart';
import 'package:denial_flutter_sdk/launcher.dart';
import 'package:denial_flutter_sdk/surfaces.dart';
import 'package:denial_sdk/composition.dart';
import 'package:flutter/material.dart';

import 'src/fluent.dart';
import 'src/preferences.dart';
import 'src/session_lock.dart';
import 'src/start.dart';
import 'src/status_bar.dart';
import 'src/taskbar.dart';
import 'src/model.dart';

export 'src/action_center.dart' show Win10xActionCenter;
export 'src/fluent.dart' show Win10xFluent;
export 'src/gestures.dart' show Win10xEdgeGestures;
export 'src/model.dart' show ShellMode, launcherGeometry, actionCenterGeometry;
export 'src/shell_navigation.dart';
export 'src/preferences.dart' show PreferenceController;
export 'src/start_motion.dart' show StartReveal;
export 'src/status_bar.dart' show Win10xStatusBar;
export 'src/wonder_bar.dart' show Win10xWonderBar;
export 'src/dual_screen.dart' show DualScreenController, DualScreenLayoutMode;
export 'src/dual_screen_host.dart'
    show DualScreenHost, DualScreenActions, CrossAppDragController;

@Provides(ShellSurface)
final class Win10xTaskbarPlugin implements ShellSurface {
  const Win10xTaskbarPlugin({this.mode = ShellMode.tablet});
  final ShellMode mode;
  @override
  String get id => 'win10x_shell.taskbar';
  @override
  ShellSurfaceLayer get layer => ShellSurfaceLayer.aboveWindows;
  @override
  ShellSurfacePlacement? place(ShellSurfaceEnvironment environment) {
    if (!environment.isMainOutput) return null;
    return ShellSurfacePlacement(
      bounds: ShellSurfacePlacement.edgeBounds(
        environment.output.logicalRect,
        PanelEdge.bottom,
        mode == ShellMode.desktop ? 40 : 48,
      ),
      visible: !environment.locked && !environment.wallpaperSelectorVisible,
    );
  }

  @override
  Widget build(BuildContext context, {required ShellSurfaceContext surface}) {
    final bar = Win10xTaskbar(
      services: surface.services,
      monitorId: surface.environment.output.monitorId,
    );
    if (mode != ShellMode.phone) return Win10xFluent.wrap(bar);
    return ListenableBuilder(
      listenable: PreferenceController.instance,
      builder: (context, _) {
        final prefs = PreferenceController.instance.value;
        return Win10xFluent.wrapPhone(
          bar,
          dark: prefs.phoneThemeDark,
          accentColor: Color(prefs.phoneAccentArgbValue),
        );
      },
    );
  }
}

@Provides(ShellSurface)
final class Win10xStatusBarPlugin implements ShellSurface {
  const Win10xStatusBarPlugin({this.mode = ShellMode.tablet});
  final ShellMode mode;
  @override
  String get id => 'win10x_shell.status_bar';
  @override
  ShellSurfaceLayer get layer => ShellSurfaceLayer.aboveWindows;
  @override
  ShellSurfacePlacement? place(ShellSurfaceEnvironment environment) {
    if (!environment.isMainOutput || mode != ShellMode.phone) return null;
    return ShellSurfacePlacement(
      bounds: ShellSurfacePlacement.edgeBounds(
        environment.output.logicalRect,
        PanelEdge.top,
        Win10xStatusBar.height,
      ),
      visible: !environment.locked && !environment.wallpaperSelectorVisible,
    );
  }

  @override
  Widget build(BuildContext context, {required ShellSurfaceContext surface}) =>
      ListenableBuilder(
        listenable: PreferenceController.instance,
        builder: (context, _) {
          final prefs = PreferenceController.instance.value;
          return Win10xFluent.wrapPhone(
            Win10xStatusBar(services: surface.services),
            dark: prefs.phoneThemeDark,
            accentColor: Color(prefs.phoneAccentArgbValue),
          );
        },
      );
}

@Provides(ShellLauncher)
final class Win10xStartLauncher implements ShellLauncher {
  const Win10xStartLauncher();
  @override
  Widget build(BuildContext context, {required LauncherContext launcher}) =>
      ListenableBuilder(
        listenable: PreferenceController.instance,
        builder: (context, _) {
          final prefs = PreferenceController.instance.value;
          final start = Win10xStart(launcher: launcher);
          if (!prefs.isPhone) return Win10xFluent.wrap(start);
          return Win10xFluent.wrapPhone(
            start,
            dark: prefs.phoneThemeDark,
            accentColor: Color(prefs.phoneAccentArgbValue),
          );
        },
      );
}

@Provides(ShellAction)
final class OpenStartAction implements ShellAction {
  const OpenStartAction();
  @override
  String get id => 'win10x_shell.openStart';
  @override
  String get provider => 'Win10X';
  @override
  String label(BuildContext context) => '打开开始屏幕';
  @override
  String description(BuildContext context) => 'Win10X 全屏磁贴开始屏幕';
  @override
  void invoke(ShellActionContext context) => context.services.toggleLauncher();
}

@Provides(ShellSurface)
final class Win10xLockScreenPlugin implements ShellSurface {
  const Win10xLockScreenPlugin();
  @override
  String get id => 'win10x_shell.lock_screen';
  @override
  ShellSurfaceLayer get layer => ShellSurfaceLayer.aboveWindows;
  @override
  ShellSurfacePlacement? place(ShellSurfaceEnvironment environment) {
    if (!environment.locked) return null;
    return ShellSurfacePlacement(
      bounds: environment.output.logicalRect,
      visible: true,
    );
  }

  @override
  Widget build(BuildContext context, {required ShellSurfaceContext surface}) =>
      Win10xFluent.wrap(Win10xLockScreen(services: surface.services));
}

@Provides(ShellSurface)
final class Win10xOobePlugin implements ShellSurface {
  const Win10xOobePlugin();
  @override
  String get id => 'win10x_shell.oobe';
  @override
  ShellSurfaceLayer get layer => ShellSurfaceLayer.aboveWindows;
  @override
  ShellSurfacePlacement? place(ShellSurfaceEnvironment environment) {
    final prefs = PreferenceController.instance.value;
    if (prefs.oobeCompleted || environment.locked) return null;
    return ShellSurfacePlacement(
      bounds: environment.output.logicalRect,
      visible: true,
    );
  }

  @override
  Widget build(BuildContext context, {required ShellSurfaceContext surface}) =>
      Win10xFluent.wrap(const Win10xOobe());
}

final class Win10xWorkArea implements ShellWorkArea {
  const Win10xWorkArea(this.mode);
  final ShellMode mode;
  @override
  ShellWorkAreaReservation reserve(ShellLayoutSettings settings) =>
      ShellWorkAreaReservation(
        edge: PanelEdge.bottom,
        thickness: mode == ShellMode.desktop ? 40 : 48,
      );
}
