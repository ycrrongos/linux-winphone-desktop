// SPDX-License-Identifier: GPL-3.0-only
import 'dart:async';

import 'package:denial_flutter_sdk/services.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';

import 'dashboard_reveal.dart';
import 'preferences.dart';
import 'model.dart';
import 'shell_navigation.dart';
import 'fluent_motion.dart';

import 'package:denial_flutter_sdk/input.dart';

/// Edge strips that open Action Center. Right/left inward matches Windows
/// tablet Action Center; top pull-down matches Windows Phone notification
/// shade habits. Native edge-swipe regions hand app-area contacts to Flutter.
class Win10xEdgeGestures extends StatefulWidget {
  const Win10xEdgeGestures({
    super.key,
    required this.services,
    required this.child,
  });
  final ShellServices services;
  final Widget child;

  @override
  State<Win10xEdgeGestures> createState() => _Win10xEdgeGesturesState();
}

class _Win10xEdgeGesturesState extends State<Win10xEdgeGestures>
    with SingleTickerProviderStateMixin {
  late final AnimationController _reach;
  bool _reachable = false;
  double _keyboard = 0;
  @override
  void initState() {
    super.initState();
    _reach =
        AnimationController(
          vsync: this,
          duration: FluentMotion.slower,
          reverseDuration: FluentMotion.slow,
        )..addListener(() {
          final usable =
              1.0 -
              (_keyboard / MediaQuery.sizeOf(context).height).clamp(0.0, 0.6);
          ShellNavigation.instance.reachOffset.value =
              FluentMotion.standard.transform(_reach.value) * .45 * usable;
        });
    ShellNavigation.instance.addListener(_reachChanged);
  }

  void _reachChanged() {
    final target = ShellNavigation.instance.oneHanded;
    if (_reachable == target) return;
    _reachable = target;
    if (!PreferenceController.instance.value.isPhone ||
        MediaQuery.disableAnimationsOf(context)) {
      _reach.value = target ? 1 : 0;
    } else if (target) {
      _reach.forward();
    } else {
      _reach.reverse();
    }
  }

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    final keyboard = MediaQuery.viewInsetsOf(context).bottom;
    if ((keyboard - _keyboard).abs() > 0.5) {
      _keyboard = keyboard;
      ShellNavigation.instance.adaptToKeyboard(keyboard);
      if (_reach.value > 0) {
        final usable =
            1.0 -
            (keyboard / MediaQuery.sizeOf(context).height).clamp(0.0, 0.6);
        ShellNavigation.instance.reachOffset.value =
            FluentMotion.standard.transform(_reach.value) * .45 * usable;
      }
    }
  }

  @override
  void dispose() {
    ShellNavigation.instance.removeListener(_reachChanged);
    _reach.dispose();
    super.dispose();
  }

  static const _edgeWidth = 18.0;
  static const _trigger = 56.0;
  static const _fling = 700.0;

  @override
  Widget build(BuildContext context) {
    final prefs = PreferenceController.instance.value;
    final top = prefs.topInset;
    final bottom = prefs.bottomInset;
    final keyboard = MediaQuery.viewInsetsOf(context).bottom;
    final size = MediaQuery.sizeOf(context);
    return Stack(
      fit: StackFit.expand,
      children: [
        widget.child,
        ListenableBuilder(
          listenable: ShellNavigation.instance.reachOffset,
          builder: (context, _) {
            if (!prefs.isPhone ||
                ShellNavigation.instance.reachOffset.value <= 0) {
              return const SizedBox.shrink();
            }
            final height = MediaQuery.sizeOf(context).height;
            final shift =
                (height - keyboard) *
                    ShellNavigation.instance.reachOffset.value -
                24;
            return Positioned(
              left: 0,
              right: 0,
              top: 24,
              height: shift.clamp(0, double.infinity),
              child: ShellInputRegion(
                debugLabel: 'Win10X reachability blank area',
                child: GestureDetector(
                  behavior: HitTestBehavior.opaque,
                  onTap: () => ShellNavigation.instance.setReachability(false),
                  child: const ColoredBox(color: Colors.black),
                ),
              ),
            );
          },
        ),
        if (!prefs.isPhone)
          Positioned(
            left: 0,
            top: top,
            bottom: bottom,
            width: _edgeWidth,
            child: _EdgeStrip(
              edge: _ShellEdge.left,
              travelExtent: size.width * 0.42,
              trigger: _trigger,
              fling: _fling,
              onOpen:
                  ShellNavigation.instance.nativeTasks ??
                  widget.services.openDashboard,
            ),
          ),
        if (!prefs.isPhone)
          Positioned(
            right: 0,
            top: top,
            bottom: bottom,
            width: _edgeWidth,
            child: _EdgeStrip(
              edge: _ShellEdge.right,
              travelExtent: mathMin(380, size.width),
              trigger: _trigger,
              fling: _fling,
              onOpen: widget.services.openDashboard,
              onCancelFollow: (p) => unawaited(
                DashboardReveal.settleOpen(
                  services: widget.services,
                  from: p,
                  velocity: 0,
                  cancelled: true,
                ),
              ),
              onFollow: (progress) => DashboardReveal.followOpen(
                services: widget.services,
                progress: progress,
              ),
              onSettle: (progress, velocity) => unawaited(
                DashboardReveal.settleOpen(
                  services: widget.services,
                  from: progress,
                  velocity: velocity,
                ),
              ),
            ),
          ),
        if (prefs.mode != ShellMode.desktop)
          Positioned(
            left: 0,
            right: 0,
            top: 0,
            height: 24,
            child: _EdgeStrip(
              edge: _ShellEdge.top,
              travelExtent: prefs.isPhone
                  ? size.height - prefs.topInset - prefs.bottomInset
                  : size.height * .65,
              trigger: prefs.isPhone
                  ? _trigger
                  : MediaQuery.sizeOf(context).height * .65,
              fling: prefs.isPhone ? _fling : 100000,
              onOpen: prefs.isPhone
                  ? widget.services.openDashboard
                  : () => ShellNavigation.instance.closeActive?.call(),
              onCancelFollow: prefs.isPhone
                  ? (p) => unawaited(
                      DashboardReveal.settleOpen(
                        services: widget.services,
                        from: p,
                        velocity: 0,
                        cancelled: true,
                      ),
                    )
                  : null,
              onFollow: prefs.isPhone
                  ? (progress) => DashboardReveal.followOpen(
                      services: widget.services,
                      progress: progress,
                    )
                  : null,
              onSettle: prefs.isPhone
                  ? (progress, velocity) => unawaited(
                      DashboardReveal.settleOpen(
                        services: widget.services,
                        from: progress,
                        velocity: velocity,
                      ),
                    )
                  : null,
            ),
          ),
      ],
    );
  }
}

double mathMin(double a, double b) => a < b ? a : b;

enum _ShellEdge { left, right, top }

class _EdgeStrip extends StatefulWidget {
  const _EdgeStrip({
    required this.edge,
    required this.travelExtent,
    required this.trigger,
    required this.fling,
    required this.onOpen,
    this.onFollow,
    this.onSettle,
    this.onCancelFollow,
  });
  final _ShellEdge edge;
  final double travelExtent;
  final double trigger;
  final double fling;
  final VoidCallback onOpen;
  final ValueChanged<double>? onFollow;
  final void Function(double progress, double velocity)? onSettle;
  final ValueChanged<double>? onCancelFollow;

  @override
  State<_EdgeStrip> createState() => _EdgeStripState();
}

class _EdgeStripState extends State<_EdgeStrip> {
  bool _cancelled = false;
  Offset _origin = Offset.zero;
  double _travel = 0;
  bool _following = false;

  double _inward(Offset delta) => switch (widget.edge) {
    _ShellEdge.left => delta.dx,
    _ShellEdge.right => -delta.dx,
    _ShellEdge.top => delta.dy,
  };

  double get _progress =>
      (_travel / widget.travelExtent.clamp(1.0, double.infinity)).clamp(
        0.0,
        1.0,
      );

  void _begin(Offset global) {
    _origin = global;
    _travel = 0;
    _following = widget.onFollow != null;
  }

  void _update(Offset global) {
    _travel = _inward(global - _origin);
    if (_following) {
      widget.onFollow!(_progress);
    }
  }

  void _end(Offset velocity) {
    if (_cancelled) return;
    final speed = _inward(velocity);
    if (_following && widget.onSettle != null) {
      widget.onSettle!(_progress, speed);
      _following = false;
      return;
    }
    if (_travel >= widget.trigger || (speed >= widget.fling && _travel >= 24)) {
      widget.onOpen();
    }
  }

  void _cancel() {
    if (_following && widget.onSettle != null) {
      widget.onCancelFollow?.call(_progress);
    }
    _following = false;
    _travel = 0;
  }

  @override
  Widget build(BuildContext context) {
    final horizontal = widget.edge != _ShellEdge.top;
    return Listener(
      onPointerDown: (_) => _cancelled = false,
      onPointerCancel: (_) {
        _cancelled = true;
        _cancel();
      },
      child: RawGestureDetector(
        behavior: HitTestBehavior.translucent,
        gestures: <Type, GestureRecognizerFactory>{
          if (horizontal)
            HorizontalDragGestureRecognizer:
                GestureRecognizerFactoryWithHandlers<
                  HorizontalDragGestureRecognizer
                >(() => HorizontalDragGestureRecognizer(debugOwner: this), (
                  recognizer,
                ) {
                  recognizer
                    ..gestureSettings = const DeviceGestureSettings(
                      touchSlop: 8,
                    )
                    ..dragStartBehavior = DragStartBehavior.down
                    ..onStart = (details) {
                      _begin(details.globalPosition);
                    }
                    ..onUpdate = (details) {
                      _update(details.globalPosition);
                    }
                    ..onEnd = (details) {
                      _end(details.velocity.pixelsPerSecond);
                    }
                    ..onCancel = _cancel;
                })
          else
            VerticalDragGestureRecognizer:
                GestureRecognizerFactoryWithHandlers<
                  VerticalDragGestureRecognizer
                >(() => VerticalDragGestureRecognizer(debugOwner: this), (
                  recognizer,
                ) {
                  recognizer
                    ..gestureSettings = const DeviceGestureSettings(
                      touchSlop: 8,
                    )
                    ..dragStartBehavior = DragStartBehavior.down
                    ..onStart = (details) {
                      _begin(details.globalPosition);
                    }
                    ..onUpdate = (details) {
                      _update(details.globalPosition);
                    }
                    ..onEnd = (details) {
                      _end(details.velocity.pixelsPerSecond);
                    }
                    ..onCancel = _cancel;
                }),
        },
      ),
    );
  }
}
