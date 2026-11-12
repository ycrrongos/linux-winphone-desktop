// SPDX-License-Identifier: GPL-3.0-only
import 'package:denial_desktop/denial_desktop.dart';

import 'package:flutter/widgets.dart';
import 'package:win10x_shell/denial.dart';
import 'package:win10x_shell/src/wp_toast.dart';
import 'package:win10x_shell/src/volume_hud.dart';
import 'package:win10x_shell/src/session_lock.dart';
import 'package:win10x_shell/src/user_wallpaper.dart';

/// Wires the reference host's dashboard and scene decorator to Win10X chrome.
void installWin10xShellChrome() {
  final nav = ShellNavigation.instance;
  win10xLockBuilder = (context, progress) => Win10xFluent.wrapPhone(
    Stack(
      fit: StackFit.expand,
      children: [Win10xLockScreen(services: nav.services)],
    ),
    dark: true,
    accentColor: Color(
      PreferenceController.instance.value.phoneAccentArgbValue,
    ),
  );
  win10xPhoneAudioHudBuilder = (context, revision, visible, dismissed) {
    final services = nav.services;
    return services == null
        ? const SizedBox.shrink()
        : Win10xVolumeHud(
            key: const ValueKey('win10x.native-phone-volume'),
            services: services,
            nativeRevision: revision,
            nativeVisible: visible,
            onNativeDismissed: dismissed,
          );
  };
  win10xPanelObserver = nav.observe;
  nav.dismissKeyboard = () => win10xDismissKeyboard?.call();
  nav.dashboardReveal.addListener(() {
    win10xDashboardReveal.value = nav.dashboardReveal.value;
  });
  win10xBindCommands = (home, back, tasks, close, settings, closeActive) {
    nav.nativeHome = home;
    nav.nativeBack = back;
    nav.nativeTasks = tasks;
    nav.closePanels = close;
    nav.settings = settings;
    nav.closeActive = closeActive;
  };
  Rect resolve(Rect bounds, bool launcher) {
    final prefs = PreferenceController.instance.value;
    final rect = launcher
        ? launcherGeometry(prefs, bounds.width, bounds.height)
        : actionCenterGeometry(prefs, bounds.width, bounds.height);
    return Rect.fromLTWH(
      bounds.left + rect.x,
      bounds.top + rect.y,
      rect.width,
      rect.height,
    );
  }

  win10xLauncherBounds = (bounds) => resolve(bounds, true);
  win10xDashboardBounds = (bounds) => resolve(bounds, false);
  win10xLauncherCloseDuration = () {
    final prefs = PreferenceController.instance.value;
    return StartReveal.closeDuration(prefs, rowSpan: prefs.tiles.length);
  };
  win10xDashboardBuilder = (context, onDismiss, onOpenSettings) {
    final services = Win10xShellServicesScope.maybeOf(context);
    if (services == null) {
      return const SizedBox.shrink();
    }
    final panel = Win10xActionCenter(
      services: services,
      onClose: onDismiss,
      embedded: true,
      onOpenSettings: onOpenSettings,
    );
    return PreferenceController.instance.value.isPhone
        ? Win10xFluent.wrapPhone(panel)
        : Win10xFluent.wrap(panel);
  };
  win10xDesktopDecorator = (context, services, child) {
    nav.services = services;
    nav.power = () => nav.details(services, 'power');
    return Win10xFluent.wrap(
      Win10xShellServicesScope(
        services: services,
        child: DualScreenHost(
          services: services,
          child: Win10xEdgeGestures(
            services: services,
            child: WpToastHost(
              child: Stack(
                fit: StackFit.expand,
                children: [
                  const Positioned.fill(child: UserWallpaper()),
                  child,
                  Win10xWonderBar(services: services),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  };
}
