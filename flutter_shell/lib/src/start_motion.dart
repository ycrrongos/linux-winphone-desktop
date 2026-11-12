// SPDX-License-Identifier: GPL-3.0-only
// Pure Dart motion constants/helpers matching shell/src/start_tiles.rs.
import 'dart:math' as math;

import 'model.dart';
import 'motion_math.dart';
export 'motion_math.dart';

/// Windows 10 Mobile / Disco / Metro / Factor / flutter_metro_ui entrance math.
abstract final class StartReveal {
  static const mobileStaggerSpan = 0.16;
  static const mobileDuration = 0.24;
  static const mobileTotalDuration = mobileStaggerSpan + mobileDuration;
  static const mobileSelectedDelay = 0.08;
  static const mobileAppBackgroundHold = 0.20;
  static const mobileScaleFrom = 0.91;
  static const mobileRowTravel = 0.18;

  static const discoStaggerSpan = 0.20;
  static const discoOuterInDuration = 0.50;
  static const discoInnerInDuration = 0.35;
  static const discoOutDuration = 0.175;
  static const discoAppListDuration = 0.20;
  static const discoSelectedAppListDelay = 0.30;

  static const metroRowStagger = 0.040;
  static const metroDuration = 0.350;

  static const factorItemStagger = 0.030;
  static const factorDuration = 0.200;
  static const factorClipRevealDuration = 0.300;

  static const flutterMetroGroupDuration = 1.050;
  static const windows8RetreatDuration = 0.240;
  static const windows8LaunchDuration = 0.720;

  /// Duration of the `_launch` controller for the selected launch style.
  static Duration launchDuration(
    LaunchAnimation style, {
    int tileCount = 1,
    bool fromAppList = false,
  }) {
    final seconds = switch (style) {
      LaunchAnimation.windows8 => windows8LaunchDuration,
      LaunchAnimation.discoTurnstile =>
        fromAppList
            ? discoSelectedAppListDelay + discoAppListDuration
            : discoStaggerSpan + discoOutDuration + 0.08,
      LaunchAnimation.factorClipReveal => factorClipRevealDuration,
      LaunchAnimation.windows10Mobile =>
        mobileTotalDuration + mobileAppBackgroundHold,
      LaunchAnimation.systemDefault => 0.01,
    };
    return Duration(milliseconds: math.max(1, (seconds * 1000).round()));
  }

  /// Disco app-list exit (`disco_app_list_frame` in start_tiles.rs).
  static ({bool visible, double progress})? discoAppListFrame({
    required double elapsed,
    required int normalRank,
    required int count,
    required bool selected,
    required bool launchingApp,
  }) {
    if (!launchingApp) return null;
    final reverseRank = math.max(0, count - normalRank - 1);
    final delay = selected
        ? discoSelectedAppListDelay
        : (count <= 1 ? 0.0 : reverseRank / (count - 1) * discoStaggerSpan);
    final raw = ((elapsed - delay) / discoAppListDuration).clamp(0.0, 1.0);
    return (
      visible: raw < 1.0,
      progress: cubicBezier(raw, 0.75, 0.0, 1.0, 0.0),
    );
  }

  /// Effective Start tile animation for the current layout selection.
  static StartAnimation styleFor(ShellPreferences prefs) =>
      prefs.effectiveLayout == StartLayout.windows8
      ? StartAnimation.flutterMetroGroup
      : prefs.startAnimation;

  static double flutterMetroGroupProgress({
    required double elapsed,
    required bool closing,
    required int groupIndex,
  }) {
    final start = (groupIndex * 0.04).clamp(0.0, 0.7) * 3.0;
    final duration = 0.35 * 3.0;
    final raw = ((elapsed - start) / duration).clamp(0.0, 1.0);
    if (closing) return 1.0 - easeInExpo(raw);
    if (elapsed < start) return 0.0;
    return easeOutExpo(raw);
  }

  static double flutterMetroHeaderProgress({
    required double elapsed,
    required bool closing,
  }) {
    if (closing) {
      return 1.0 - flutterEaseOutCubic((elapsed / 0.9).clamp(0.0, 1.0));
    }
    if (elapsed < 0.6) return 0.0;
    return flutterEaseOutCubic(((elapsed - 0.6) / 0.9).clamp(0.0, 1.0));
  }

  static double flutterMetroStartDuration(int groupCount) {
    if (groupCount <= 0) return 1.5;
    final last = (((groupCount - 1) * 0.04).clamp(0.0, 0.7) + 0.35) * 3.0;
    return math.max(last, 1.5);
  }

  static double mobileContentDuration(int rowSpan) =>
      rowSpan > 0 ? mobileTotalDuration : mobileDuration;

  static Duration openDuration(ShellPreferences prefs, {int rowSpan = 1}) {
    final style = styleFor(prefs);
    final seconds = switch (style) {
      // Always reserve the full stagger window so late rows finish.
      StartAnimation.windows10Mobile => mobileTotalDuration,
      StartAnimation.discoTurnstile => discoStaggerSpan + discoOuterInDuration,
      StartAnimation.metroRise =>
        metroRowStagger * math.max(rowSpan, 4) + metroDuration,
      StartAnimation.factorSlideScale => factorItemStagger * 8 + factorDuration,
      StartAnimation.flutterMetroGroup => flutterMetroStartDuration(
        prefs.groups.length,
      ),
    };
    return Duration(milliseconds: math.max(1, (seconds * 1000).round()));
  }

  static Duration closeDuration(ShellPreferences prefs, {int rowSpan = 1}) {
    final style = styleFor(prefs);
    final seconds = switch (style) {
      StartAnimation.windows10Mobile => mobileTotalDuration,
      StartAnimation.discoTurnstile => discoStaggerSpan + discoOutDuration,
      StartAnimation.metroRise =>
        metroRowStagger * math.max(rowSpan, 4) + metroDuration,
      StartAnimation.factorSlideScale => factorItemStagger * 8 + factorDuration,
      StartAnimation.flutterMetroGroup => flutterMetroStartDuration(
        prefs.groups.length,
      ),
    };
    return Duration(milliseconds: math.max(1, (seconds * 1000).round()));
  }

  /// Row delay for W10M. Opening builds bottom→top; closing tears top→bottom.
  static double mobileRowDelay({
    required int rowIndex,
    required int rowSpan,
    required bool closing,
  }) {
    if (rowSpan <= 0) return 0;
    final order = closing ? rowIndex : (rowSpan - rowIndex);
    return order / rowSpan * mobileStaggerSpan;
  }

  static (bool visible, double progress) animationProgress({
    required double local,
    required double duration,
    required bool closing,
  }) {
    final progress = closing
        ? 1.0 - easeAccelerate((local / duration).clamp(0.0, 1.0))
        : local < 0
        ? 0.0
        : easeDecelerate((local / duration).clamp(0.0, 1.0));
    final visible = closing ? progress > 0.001 : local >= 0;
    return (visible, progress);
  }

  /// Scene/wallpaper opacity. W10M application launch keeps the veil at 1.
  static double sceneProgress({
    required StartAnimation style,
    required double elapsed,
    required bool closing,
    required bool launchingApp,
    required double contentDuration,
    required int groupCount,
  }) {
    if (style == StartAnimation.windows10Mobile && closing && launchingApp) {
      return 1.0;
    }
    if (style == StartAnimation.flutterMetroGroup) {
      return flutterMetroGroupProgress(
        elapsed: elapsed,
        closing: closing,
        groupIndex: math.max(0, groupCount - 1),
      );
    }
    final raw = (elapsed / math.max(0.001, contentDuration)).clamp(0.0, 1.0);
    return closing ? 1.0 - easeAccelerate(raw) : easeDecelerate(raw);
  }
}
