// SPDX-License-Identifier: GPL-3.0-only
// Dual-screen / hinge / Wonder Bar geometry from
// research/windows10x/dual-screen-hardware/notes/ui-and-interaction.md
// Pure Dart so unit tests do not load Flutter foundation.
import 'dart:math' as math;

/// Device posture for dual-screen / foldable hardware (Surface Duo vocabulary).
enum DualScreenPosture { flat, book, tent, folded, keyboardCover }

/// Infer posture from output geometry when hinge/fold sensors are absent.
/// Research: stacked outputs → tent; strongly unequal areas → keyboard cover;
/// side-by-side peers → book; nearly coplanar equal → flat.
DualScreenPosture inferPosture(ShellRect primary, ShellRect secondary) {
  final horizontal =
      (primary.centerX - secondary.centerX).abs() >
      (primary.centerY - secondary.centerY).abs();
  final areaA = primary.width * primary.height;
  final areaB = secondary.width * secondary.height;
  final larger = math.max(areaA, areaB);
  final smaller = math.min(areaA, areaB);
  if (larger <= 0) return DualScreenPosture.folded;
  final ratio = smaller / larger;
  if (ratio < 0.45) return DualScreenPosture.keyboardCover;
  if (!horizontal) return DualScreenPosture.tent;
  if (ratio > 0.92) {
    final gap = horizontal
        ? (secondary.left - primary.right).abs()
        : (secondary.top - primary.bottom).abs();
    if (gap < 8) return DualScreenPosture.flat;
  }
  return DualScreenPosture.book;
}

/// Five dual-screen app layout modes from Microsoft dual-screen design notes.
enum DualScreenLayoutMode {
  extendedCanvas,
  listDetail,
  twoPage,
  companionPane,
  dualView,
}

/// Axis-aligned rectangle in logical pixels (avoids dart:ui in dart test).
class ShellRect {
  const ShellRect(this.left, this.top, this.width, this.height);
  const ShellRect.fromLTWH(this.left, this.top, this.width, this.height);
  factory ShellRect.fromCenter({
    required double centerX,
    required double centerY,
    required double width,
    required double height,
  }) => ShellRect(centerX - width / 2, centerY - height / 2, width, height);

  static const zero = ShellRect(0, 0, 0, 0);

  final double left, top, width, height;
  double get right => left + width;
  double get bottom => top + height;
  double get centerX => left + width / 2;
  double get centerY => top + height / 2;
  bool get isEmpty => width <= 0 || height <= 0;

  bool overlaps(ShellRect other) =>
      left < other.right &&
      right > other.left &&
      top < other.bottom &&
      bottom > other.top;

  ShellRect expandToInclude(ShellRect other) {
    if (isEmpty) return other;
    if (other.isEmpty) return this;
    final l = math.min(left, other.left);
    final t = math.min(top, other.top);
    final r = math.max(right, other.right);
    final b = math.max(bottom, other.bottom);
    return ShellRect(l, t, r - l, b - t);
  }
}

/// Hinge / occlusion strip between two logical outputs.
class DualScreenGeometry {
  const DualScreenGeometry({
    required this.left,
    required this.right,
    required this.hinge,
    required this.posture,
    this.wonderBar = ShellRect.zero,
  });

  final ShellRect left;
  final ShellRect right;
  final ShellRect hinge;
  final DualScreenPosture posture;
  final ShellRect wonderBar;

  ShellRect get boundingBox =>
      left.expandToInclude(right).expandToInclude(hinge);

  bool get hasWonderBar => !wonderBar.isEmpty;

  factory DualScreenGeometry.fromOutputs({
    required ShellRect primary,
    required ShellRect secondary,
    double hingeThickness = 24,
    DualScreenPosture posture = DualScreenPosture.book,
    ShellRect wonderBar = ShellRect.zero,
  }) {
    final horizontal =
        (primary.centerX - secondary.centerX).abs() >
        (primary.centerY - secondary.centerY).abs();
    late final ShellRect left;
    late final ShellRect right;
    late final ShellRect hinge;
    if (horizontal) {
      final a = primary.left <= secondary.left ? primary : secondary;
      final b = identical(a, primary) ? secondary : primary;
      left = a;
      right = b;
      final mid = (a.right + b.left) / 2;
      hinge = ShellRect.fromCenter(
        centerX: mid,
        centerY: (a.top + a.bottom) / 2,
        width: hingeThickness,
        height: math.max(a.height, b.height),
      );
    } else {
      final a = primary.top <= secondary.top ? primary : secondary;
      final b = identical(a, primary) ? secondary : primary;
      left = a;
      right = b;
      final mid = (a.bottom + b.top) / 2;
      hinge = ShellRect.fromCenter(
        centerX: (a.left + a.right) / 2,
        centerY: mid,
        width: math.max(a.width, b.width),
        height: hingeThickness,
      );
    }
    return DualScreenGeometry(
      left: left,
      right: right,
      hinge: hinge,
      posture: posture,
      wonderBar: wonderBar,
    );
  }

  static ShellRect wonderBarRegion(ShellRect covered, ShellRect keyboard) {
    if (!covered.overlaps(keyboard)) return ShellRect.zero;
    final topGap = keyboard.top - covered.top;
    if (topGap >= 48) {
      return ShellRect(
        covered.left,
        covered.top,
        covered.width,
        keyboard.top - covered.top,
      );
    }
    final bottomGap = covered.bottom - keyboard.bottom;
    if (bottomGap >= 48) {
      return ShellRect(
        covered.left,
        keyboard.bottom,
        covered.width,
        covered.bottom - keyboard.bottom,
      );
    }
    return ShellRect.zero;
  }

  (ShellRect, ShellRect) panesFor(DualScreenLayoutMode mode) {
    final safeLeft = _avoidHinge(left);
    final safeRight = _avoidHinge(right);
    switch (mode) {
      case DualScreenLayoutMode.extendedCanvas:
        return (boundingBox, ShellRect.zero);
      case DualScreenLayoutMode.listDetail:
      case DualScreenLayoutMode.companionPane:
      case DualScreenLayoutMode.dualView:
      case DualScreenLayoutMode.twoPage:
        return (safeLeft, safeRight);
    }
  }

  ShellRect _avoidHinge(ShellRect pane) {
    if (!pane.overlaps(hinge)) return pane;
    if (hinge.width <= hinge.height) {
      if (pane.centerX < hinge.centerX) {
        return ShellRect(
          pane.left,
          pane.top,
          hinge.left - pane.left,
          pane.height,
        );
      }
      return ShellRect(
        hinge.right,
        pane.top,
        pane.right - hinge.right,
        pane.height,
      );
    }
    if (pane.centerY < hinge.centerY) {
      return ShellRect(pane.left, pane.top, pane.width, hinge.top - pane.top);
    }
    return ShellRect(
      pane.left,
      hinge.bottom,
      pane.width,
      pane.bottom - hinge.bottom,
    );
  }
}

/// Runtime dual-screen controller. Populated when the host reports ≥2 outputs.
class DualScreenController {
  DualScreenGeometry? _geometry;
  DualScreenLayoutMode _mode = DualScreenLayoutMode.listDetail;
  List<int> _monitorIds = const [];
  final _listeners = <void Function()>[];

  DualScreenGeometry? get geometry => _geometry;
  DualScreenLayoutMode get mode => _mode;
  List<int> get monitorIds => List.unmodifiable(_monitorIds);
  bool get active => _geometry != null;

  void addListener(void Function() listener) => _listeners.add(listener);
  void removeListener(void Function() listener) => _listeners.remove(listener);
  void _notify() {
    for (final listener in List.of(_listeners)) {
      listener();
    }
  }

  void updateOutputs(
    List<ShellRect> outputs, {
    List<int> monitorIds = const [],
    DualScreenPosture? posture,
  }) {
    if (outputs.length < 2) {
      if (_geometry != null || _monitorIds.isNotEmpty) {
        _geometry = null;
        _monitorIds = const [];
        _notify();
      }
      return;
    }
    final resolved = posture ?? inferPosture(outputs[0], outputs[1]);
    _monitorIds = monitorIds.length >= 2
        ? List<int>.from(monitorIds.take(2))
        : const [];
    _geometry = DualScreenGeometry.fromOutputs(
      primary: outputs[0],
      secondary: outputs[1],
      posture: resolved,
    );
    _notify();
  }

  void setMode(DualScreenLayoutMode mode) {
    if (_mode == mode) return;
    _mode = mode;
    _notify();
  }

  void setWonderBar(ShellRect region) {
    final g = _geometry;
    if (g == null) return;
    _geometry = DualScreenGeometry(
      left: g.left,
      right: g.right,
      hinge: g.hinge,
      posture: DualScreenPosture.keyboardCover,
      wonderBar: region,
    );
    _notify();
  }

  /// Target monitor for a pane (0 = left/primary, 1 = right/secondary).
  int? monitorForPane(ShellRect pane) {
    final g = _geometry;
    if (g == null || _monitorIds.length < 2) return null;
    final leftDist =
        (pane.centerX - g.left.centerX).abs() +
        (pane.centerY - g.left.centerY).abs();
    final rightDist =
        (pane.centerX - g.right.centerX).abs() +
        (pane.centerY - g.right.centerY).abs();
    return leftDist <= rightDist ? _monitorIds[0] : _monitorIds[1];
  }

  static final instance = DualScreenController();
}
