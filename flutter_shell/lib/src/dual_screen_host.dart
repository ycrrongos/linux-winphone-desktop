// SPDX-License-Identifier: GPL-3.0-only
// Bind Denial displayLayoutProvider → DualScreenController (DU1–DU5).
import 'dart:async';

import 'package:denial_flutter_sdk/models.dart'
    show DenialWindow, DisplayLayout;
import 'package:denial_flutter_sdk/services.dart';
import 'package:denial_flutter_sdk/state.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'dual_screen.dart';

/// Watches host outputs and updates dual-screen geometry / Wonder Bar.
/// Extended Canvas spans the focused app via [DenialBridge.configureWindow].
class DualScreenHost extends ConsumerStatefulWidget {
  const DualScreenHost({super.key, required this.child, this.services});
  final Widget child;
  final ShellServices? services;

  @override
  ConsumerState<DualScreenHost> createState() => _DualScreenHostState();
}

class _DualScreenHostState extends ConsumerState<DualScreenHost> {
  DualScreenLayoutMode? _lastMode;

  void _sync(DisplayLayout? layout) {
    if (layout != null && layout.outputs.length >= 2) {
      final rects = [
        for (final output in layout.outputs)
          ShellRect.fromLTWH(
            output.logicalRect.left,
            output.logicalRect.top,
            output.logicalRect.width,
            output.logicalRect.height,
          ),
      ];
      DualScreenController.instance.updateOutputs(
        rects,
        monitorIds: [for (final o in layout.outputs) o.monitorId],
      );
    } else {
      DualScreenController.instance.updateOutputs(const []);
    }
  }

  Future<void> _applyMode(DualScreenLayoutMode mode) async {
    if (mode == DualScreenLayoutMode.extendedCanvas) {
      await DualScreenActions.spanFocusedWindow(ref, widget.services);
    }
  }

  @override
  Widget build(BuildContext context) {
    ref.listen(displayLayoutProvider, (_, next) => _sync(next));
    _sync(ref.watch(displayLayoutProvider));
    final mode = DualScreenController.instance.mode;
    if (_lastMode != mode) {
      _lastMode = mode;
      unawaited(_applyMode(mode));
    }
    return widget.child;
  }
}

/// Denial-bridge helpers for dual-output window placement (DU2 / DU5).
abstract final class DualScreenActions {
  /// Span the focused app across the dual-screen bounding box (Extended Canvas).
  static Future<bool> spanFocusedWindow(
    WidgetRef ref,
    ShellServices? services,
  ) async {
    final geometry = DualScreenController.instance.geometry;
    if (geometry == null) return false;
    final bridge = ref.read(denialBridgeProvider);
    final snapshot = await bridge.listWindows(const []);
    final target = _resolveFocused(ref, services, snapshot.windows);
    if (target == null) return false;
    final box = geometry.boundingBox;
    bridge.configureWindow(
      target,
      Rect.fromLTWH(box.left, box.top, box.width, box.height),
      exact: true,
    );
    return true;
  }

  /// Move a window onto the dual pane under [x],[y] (shell cross-app handoff).
  static Future<bool> moveWindowToPoint(
    WidgetRef ref, {
    required int windowId,
    required double x,
    required double y,
  }) async {
    CrossAppDragController.instance.updateTargetAt(x, y);
    final targetPane = CrossAppDragController.instance.targetPane;
    final monitor = targetPane == null
        ? null
        : DualScreenController.instance.monitorForPane(targetPane);
    if (monitor == null) return false;
    final bridge = ref.read(denialBridgeProvider);
    final snapshot = await bridge.listWindows(const []);
    DenialWindow? window;
    for (final candidate in snapshot.windows) {
      if (candidate.windowId == windowId) {
        window = candidate;
        break;
      }
    }
    if (window == null) return false;
    bridge.moveWindowToOutput(window, monitorId: monitor);
    CrossAppDragController.instance.clear();
    return true;
  }

  static DenialWindow? _resolveFocused(
    WidgetRef ref,
    ShellServices? services,
    List<DenialWindow> windows,
  ) {
    if (services != null) {
      for (final monitorId in DualScreenController.instance.monitorIds) {
        final list = ref.read(services.windows(monitorId));
        for (final app in list) {
          if (!app.active || app.minimized) continue;
          for (final window in windows) {
            if (window.windowId == app.id) return window;
          }
        }
      }
    }
    for (final window in windows) {
      if (!window.minimized && !window.fullscreen) return window;
    }
    return null;
  }
}

/// Shell-level cross-pane drag payload (DU5). Uses Denial
/// [moveWindowToOutput] when a window id is supplied; compositor wl_data DnD
/// between arbitrary clients still requires host surface support.
class CrossAppDragController {
  static final instance = CrossAppDragController();
  String? payload;
  int? windowId;
  ShellRect? sourcePane;
  ShellRect? targetPane;

  void begin(String data, ShellRect source, {int? windowId}) {
    payload = data;
    this.windowId = windowId;
    sourcePane = source;
    targetPane = null;
  }

  void updateTarget(ShellRect? pane) => targetPane = pane;

  /// Maps a logical point onto the non-source dual pane (list–detail / two-page).
  void updateTargetAt(double x, double y) {
    final geometry = DualScreenController.instance.geometry;
    if (geometry == null) {
      targetPane = null;
      return;
    }
    final panes = geometry.panesFor(DualScreenController.instance.mode);
    ShellRect? hit;
    for (final pane in [panes.$1, panes.$2]) {
      if (pane.isEmpty) continue;
      if (x >= pane.left &&
          x < pane.right &&
          y >= pane.top &&
          y < pane.bottom) {
        hit = pane;
        break;
      }
    }
    if (hit != null &&
        sourcePane != null &&
        hit.left == sourcePane!.left &&
        hit.top == sourcePane!.top &&
        hit.width == sourcePane!.width &&
        hit.height == sourcePane!.height) {
      targetPane = null;
      return;
    }
    targetPane = hit;
  }

  bool get active => payload != null || windowId != null;

  /// Completes a shell-level handoff when both panes are known.
  bool complete() {
    final ok =
        (payload != null || windowId != null) &&
        sourcePane != null &&
        targetPane != null;
    clear();
    return ok;
  }

  void clear() {
    payload = null;
    windowId = null;
    sourcePane = null;
    targetPane = null;
  }
}
