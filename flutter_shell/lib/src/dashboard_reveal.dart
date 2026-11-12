// SPDX-License-Identifier: GPL-3.0-only
import 'dart:async';

import 'package:denial_flutter_sdk/services.dart';
import 'package:flutter/scheduler.dart';

import 'motion_math.dart';
import 'shell_navigation.dart';

/// Finger-follow Action Center reveal shared by edge open and panel dismiss.
abstract final class DashboardReveal {
  static int _generation = 0;

  static void setProgress(double progress) {
    // A new finger contact owns progress; an older settle must stop writing.
    _generation++;
    ShellNavigation.instance.dashboardReveal.value = progress.clamp(0.0, 1.0);
  }

  static void clear() {
    _generation++;
    ShellNavigation.instance.dashboardReveal.value = null;
  }

  /// Open the host dashboard if needed, then drive interactive progress.
  static void followOpen({
    required ShellServices services,
    required double progress,
  }) {
    final p = progress.clamp(0.0, 1.0);
    if (p <= 0.001) return;
    setProgress(p);
    if (!ShellNavigation.instance.dashboardVisible) {
      services.openDashboard();
    }
  }

  static Future<void> settleOpen({
    required ShellServices services,
    required double from,
    required double velocity,
    bool cancelled = false,
  }) async {
    final commit = !cancelled && (from >= 0.32 || velocity > 700);
    if (commit) {
      if (!ShellNavigation.instance.dashboardVisible) {
        services.openDashboard();
      }
      if (!await _animate(from.clamp(0.0, 1.0), 1.0)) return;
      clear();
      return;
    }
    if (!await _animate(from.clamp(0.0, 1.0), 0.0)) return;
    if (ShellNavigation.instance.dashboardVisible) {
      services.toggleDashboard();
    }
    await _clearAfterClose();
  }

  static Future<void> settleClose({
    required ShellServices services,
    required double from,
    required double closeVelocity,
    bool cancelled = false,
  }) async {
    final dismiss = !cancelled && (from <= 0.68 || closeVelocity > 700);
    if (dismiss) {
      if (!await _animate(from.clamp(0.0, 1.0), 0.0)) return;
      if (ShellNavigation.instance.dashboardVisible) {
        services.toggleDashboard();
      }
      await _clearAfterClose();
      return;
    }
    if (!await _animate(from.clamp(0.0, 1.0), 1.0)) return;
    clear();
  }

  static Future<void> _clearAfterClose() async {
    final generation = _generation;
    // Keep progress zero while the host retains its closing surface. Clearing
    // it first would reveal the fully open panel for the host's exit animation.
    await Future<void>.delayed(const Duration(milliseconds: 350));
    if (generation == _generation) clear();
  }

  static Future<bool> _animate(double from, double to) async {
    final generation = ++_generation;
    final durationMs = (180 + (from - to).abs() * 200).round().clamp(140, 340);
    final completer = Completer<void>();
    late final Ticker ticker;
    ticker = Ticker((elapsed) {
      if (generation != _generation) {
        ticker.stop();
        if (!completer.isCompleted) completer.complete();
        return;
      }
      final t = (elapsed.inMilliseconds / durationMs).clamp(0.0, 1.0);
      final curved = to >= from ? easeDecelerate(t) : 1 - easeDecelerate(1 - t);
      ShellNavigation.instance.dashboardReveal.value =
          from + (to - from) * curved;
      if (t >= 1) {
        ticker.stop();
        if (!completer.isCompleted) completer.complete();
      }
    });
    ShellNavigation.instance.dashboardReveal.value = from;
    ticker.start();
    try {
      await completer.future.timeout(Duration(milliseconds: durationMs + 80));
    } on TimeoutException {
      // Fall through; dispose still runs.
    }
    ticker.dispose();
    return generation == _generation;
  }
}
