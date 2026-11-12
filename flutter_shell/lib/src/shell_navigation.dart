// SPDX-License-Identifier: GPL-3.0-only
import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:denial_flutter_sdk/services.dart';

import 'model.dart';

enum StartRequest { home, reveal, apps, search, back }

/// Presentation commands share the native host's real panel state. No duplicate
/// home/activity stack is inferred from which apps the launcher happened to run.
class ShellNavigation extends ChangeNotifier {
  static final instance = ShellNavigation();
  final mode = ValueNotifier(ShellMode.tablet);
  bool launcherVisible = false, dashboardVisible = false;
  StartRequest request = StartRequest.home;
  int revision = 0, detailRevision = 0;
  String? actionDetail;
  VoidCallback? nativeHome,
      nativeBack,
      nativeTasks,
      closePanels,
      closeActive,
      settings,
      power,
      dismissKeyboard;

  /// Host services for Start live tiles / media (set by desktop decorator).
  ShellServices? services;
  bool oneHanded = false;
  final reachOffset = ValueNotifier<double>(0);

  /// Interactive Action Center progress (0..1). Null = host controller owns it.
  final dashboardReveal = ValueNotifier<double?>(null);
  Timer? _reachTimer;

  void observe(bool launcher, bool dashboard) {
    launcherVisible = launcher;
    dashboardVisible = dashboard;
  }

  void send(StartRequest value) {
    request = value;
    revision++;
    notifyListeners();
  }

  void home(ShellServices services) {
    dismissKeyboard?.call();
    // Always publish the request so Start can toggle-close when already on
    // the tiles page (phone mode previously only reset scroll/apps).
    send(StartRequest.home);
    if (!launcherVisible) {
      if (nativeHome != null) {
        nativeHome!();
      } else {
        services.toggleLauncher();
      }
    }
  }

  /// Explicit Start-card destination; unlike pressing Start, never toggle it off.
  void revealHome(ShellServices services) {
    dismissKeyboard?.call();
    send(StartRequest.reveal);
    if (!launcherVisible) {
      if (nativeHome != null) {
        nativeHome!();
      } else {
        services.toggleLauncher();
      }
    }
  }

  void search(ShellServices services) {
    dismissKeyboard?.call();
    send(StartRequest.search);
    if (!launcherVisible) {
      if (nativeHome != null) {
        nativeHome!();
      } else {
        services.toggleLauncher();
      }
    }
  }

  void details(ShellServices services, String? value) {
    actionDetail = value;
    detailRevision++;
    notifyListeners();
    services.openDashboard();
  }

  void back(ShellServices services) {
    if (oneHanded) {
      setReachability(false);
      return;
    }
    if (dashboardVisible) {
      services.toggleDashboard();
      return;
    }
    if (launcherVisible) {
      send(StartRequest.back);
      return;
    }
    nativeBack?.call();
  }

  void setReachability(bool enabled) {
    _reachTimer?.cancel();
    oneHanded = enabled;
    if (enabled) {
      _reachTimer = Timer(
        const Duration(seconds: 5),
        () => setReachability(false),
      );
    }
    notifyListeners();
  }

  /// Soft keyboard replaces one-handed mode; keep shell chrome reachable.
  void adaptToKeyboard(double keyboardInset) {
    if (keyboardInset > 0 && oneHanded) setReachability(false);
  }
}
