// SPDX-License-Identifier: GPL-3.0-only
import 'package:denial_flutter_sdk/services.dart';
import 'package:denial_flutter_sdk/popups.dart';
import 'package:denial_flutter_sdk/input.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'fluent.dart';
import 'motion_math.dart';
import 'preferences.dart';

/// Legacy Rust app-switch handoff: taskbar zoom 200 ms or page slide 500 ms.
abstract final class AppSwitchHandoff {
  static Future<void> activate({
    required WidgetRef ref,
    required ShellServices services,
    required ApplicationWindow target,
    required Rect origin,
    ApplicationWindow? outgoing,
    required Size output,
  }) async {
    final prefs = PreferenceController.instance.value;
    final reduceMotion = WidgetsBinding
        .instance
        .platformDispatcher
        .accessibilityFeatures
        .disableAnimations;
    if (!prefs.switchAnimation || reduceMotion) {
      services.activateWindow(target.id);
      return;
    }
    // Activate immediately so cold focus matches Rust (proxy is visual only).
    services.activateWindow(target.id);
    final taskbarZoom = outgoing == null;
    final duration = Duration(milliseconds: taskbarZoom ? 200 : 500);
    final direction = (target.id >= (outgoing?.id ?? target.id)) ? 1.0 : -1.0;
    final handle = ref
        .read(shellPopupControllerProvider.notifier)
        .show(
          debugLabel: 'Win10X app switch proxy',
          keyName: 'win10x.app-switch',
          pointerPolicy: ShellPointerPolicy.none,
          keyboardPolicy: ShellKeyboardPolicy.none,
          dismissPolicy: ShellDismissPolicy.none,
          transitionDuration: Duration.zero,
          barrierColor: Colors.transparent,
          builder: (context, popup) => _AppSwitchProxy(
            services: services,
            target: target,
            outgoing: outgoing,
            origin: origin,
            output: output,
            taskbarZoom: taskbarZoom,
            direction: direction,
            duration: duration,
            onDone: popup.close,
          ),
        );
    await Future<void>.delayed(duration + const Duration(milliseconds: 32));
    handle.close();
  }
}

class _AppSwitchProxy extends StatefulWidget {
  const _AppSwitchProxy({
    required this.services,
    required this.target,
    required this.outgoing,
    required this.origin,
    required this.output,
    required this.taskbarZoom,
    required this.direction,
    required this.duration,
    required this.onDone,
  });
  final ShellServices services;
  final ApplicationWindow target;
  final ApplicationWindow? outgoing;
  final Rect origin;
  final Size output;
  final bool taskbarZoom;
  final double direction;
  final Duration duration;
  final VoidCallback onDone;

  @override
  State<_AppSwitchProxy> createState() => _AppSwitchProxyState();
}

class _AppSwitchProxyState extends State<_AppSwitchProxy>
    with SingleTickerProviderStateMixin {
  late final AnimationController _ctrl;

  @override
  void initState() {
    super.initState();
    _ctrl = AnimationController(vsync: this, duration: widget.duration)
      ..forward().whenComplete(widget.onDone);
  }

  @override
  void dispose() {
    _ctrl.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final prefs = PreferenceController.instance.value;
    final desktop = Rect.fromLTRB(
      0,
      prefs.topInset,
      widget.output.width,
      widget.output.height - prefs.bottomInset,
    );
    return ClipRect(
      clipper: _AppViewportClip(
        Rect.fromLTRB(
          0,
          prefs.topInset,
          widget.output.width,
          widget.output.height - prefs.bottomInset,
        ),
      ),
      child: IgnorePointer(
        child: AnimatedBuilder(
          animation: _ctrl,
          builder: (context, _) {
            final t = _ctrl.value;
            if (widget.taskbarZoom) {
              final progress = 1 - (1 - t) * (1 - t); // ease-out-quad
              final rect = Rect.lerp(widget.origin, desktop, progress)!;
              return Stack(
                fit: StackFit.expand,
                children: [
                  Positioned.fromRect(
                    rect: rect,
                    child: _preview(widget.target.id, widget.target.appId),
                  ),
                ],
              );
            }
            // Page slide: outgoing retreats 100%→80%; incoming slides/scales in.
            final reverseInterval = ((1 - t - 0.4) / 0.6).clamp(0.0, 1.0);
            final outScale = 0.8 + 0.2 * flutterEaseOutCubic(reverseInterval);
            final slide = flutterEaseOutCubic(
              ((t - 0.4) / 0.3).clamp(0.0, 1.0),
            );
            final grow = flutterEaseOutCubic(((t - 0.7) / 0.3).clamp(0.0, 1.0));
            final inScale = 0.5 + 0.5 * grow;
            final inCenter =
                desktop.center +
                Offset(widget.direction * desktop.width * 1.5 * (1 - slide), 0);
            return ColoredBox(
              color: Win10xFluent.metroStart,
              child: Stack(
                fit: StackFit.expand,
                children: [
                  if (widget.outgoing != null)
                    Center(
                      child: SizedBox(
                        width: desktop.width * outScale,
                        height: desktop.height * outScale,
                        child: _preview(
                          widget.outgoing!.id,
                          widget.outgoing!.appId,
                        ),
                      ),
                    ),
                  Positioned.fromRect(
                    rect: Rect.fromCenter(
                      center: inCenter,
                      width: desktop.width * inScale,
                      height: desktop.height * inScale,
                    ),
                    child: _preview(widget.target.id, widget.target.appId),
                  ),
                ],
              ),
            );
          },
        ),
      ),
    );
  }

  Widget _preview(int windowId, String appId) => ColoredBox(
    color: const Color(0xff1d1d1d),
    child: widget.services.buildWindowPreview(context, windowId),
  );
}

class _AppViewportClip extends CustomClipper<Rect> {
  const _AppViewportClip(this.viewport);
  final Rect viewport;
  @override
  Rect getClip(Size size) => viewport.intersect(Offset.zero & size);
  @override
  bool shouldReclip(_AppViewportClip oldClipper) =>
      viewport != oldClipper.viewport;
}
