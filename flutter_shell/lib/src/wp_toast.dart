// SPDX-License-Identifier: GPL-3.0-only
import 'dart:async';

import 'package:denial_flutter_sdk/input.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'fluent.dart';
import 'preferences.dart';
import 'wp_motion.dart';

/// Non-modal feedback. Only the visible strip participates in input routing.
abstract final class WpToast {
  static final _current = ValueNotifier<_ToastRequest?>(null);
  static void show(
    WidgetRef ref,
    String message, {
    Duration hold = const Duration(milliseconds: 4000),
    List<({String label, VoidCallback onTap})> actions = const [],
  }) {
    _current.value = _ToastRequest(message, hold, actions);
  }

  static void _finish(_ToastRequest request) {
    if (identical(_current.value, request)) _current.value = null;
  }
}

class _ToastRequest {
  _ToastRequest(this.message, this.hold, this.actions);
  final String message;
  final Duration hold;
  final List<({String label, VoidCallback onTap})> actions;
}

class WpToastHost extends StatelessWidget {
  const WpToastHost({super.key, required this.child});
  final Widget child;
  @override
  Widget build(BuildContext context) => ValueListenableBuilder<_ToastRequest?>(
    valueListenable: WpToast._current,
    builder: (context, request, child) => Stack(
      fit: StackFit.expand,
      children: [
        child!,
        if (request != null)
          _ToastBody(key: ObjectKey(request), request: request),
      ],
    ),
    child: child,
  );
}

class _ToastBody extends StatefulWidget {
  const _ToastBody({super.key, required this.request});
  final _ToastRequest request;
  @override
  State<_ToastBody> createState() => _ToastBodyState();
}

class _ToastBodyState extends State<_ToastBody>
    with SingleTickerProviderStateMixin {
  late final AnimationController _ctrl;
  Timer? _timer;
  bool _closing = false, _dragging = false, _cancelled = false;
  final _bodyKey = GlobalKey();
  double get _height =>
      (_bodyKey.currentContext?.findRenderObject() as RenderBox?)
          ?.size
          .height ??
      80;
  @override
  void initState() {
    super.initState();
    _ctrl = AnimationController(
      vsync: this,
      duration: WpMotion.toastEnter,
      reverseDuration: WpMotion.toastExit,
    );
    _ctrl.animateTo(1, curve: WpMotion.enter);
    _timer = Timer(widget.request.hold, _exit);
  }

  Future<void> _exit() async {
    if (_closing || !mounted) return;
    _closing = true;
    _timer?.cancel();
    try {
      await _ctrl
          .animateBack(0, duration: WpMotion.toastExit, curve: WpMotion.exit)
          .orCancel;
    } on TickerCanceled {
      return;
    }
    if (mounted) WpToast._finish(widget.request);
  }

  @override
  void dispose() {
    _timer?.cancel();
    _ctrl.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final prefs = PreferenceController.instance.value;
    final phone = prefs.isPhone;
    final dark = phone && prefs.phoneThemeDark;
    final accent = Color(prefs.phoneAccentArgbValue);
    return Align(
      alignment: phone ? Alignment.topCenter : Alignment.bottomRight,
      child: Padding(
        padding: EdgeInsets.only(
          top: phone ? prefs.topInset : 0,
          bottom: phone ? 0 : prefs.bottomInset + 12,
          left: phone ? 0 : 12,
          right: phone ? 0 : 12,
        ),
        child: AnimatedBuilder(
          animation: _ctrl,
          builder: (context, child) {
            final progress = _ctrl.value;
            return Opacity(
              opacity: progress.clamp(0.0, 1.0),
              child: FractionalTranslation(
                translation: Offset(0, (phone ? -1 : 1) * (1 - progress)),
                child: child,
              ),
            );
          },
          child: ShellInputRegion(
            debugLabel: 'Win10X toast actions',
            child: Listener(
              onPointerDown: (_) => _cancelled = false,
              onPointerCancel: (_) {
                if (!_dragging) return;
                _cancelled = true;
                _dragging = false;
                _ctrl.animateTo(1, curve: WpMotion.enter);
                _timer = Timer(widget.request.hold, () => unawaited(_exit()));
              },
              child: GestureDetector(
                behavior: HitTestBehavior.opaque,
                onTap: () => unawaited(_exit()),
                onVerticalDragStart: phone
                    ? (_) {
                        _timer?.cancel();
                        _dragging = true;
                        _closing = false;
                        _ctrl.stop();
                      }
                    : null,
                onVerticalDragUpdate: phone
                    ? (d) {
                        _ctrl.value =
                            (_ctrl.value + (d.primaryDelta ?? 0) / _height)
                                .clamp(0, 1);
                      }
                    : null,
                onVerticalDragEnd: phone
                    ? (d) {
                        if (_cancelled) return;
                        _dragging = false;
                        if (_ctrl.value < .7 ||
                            (d.primaryVelocity ?? 0) < -150) {
                          unawaited(_exit());
                        } else {
                          _ctrl.animateTo(1, curve: WpMotion.enter);
                          _timer = Timer(
                            widget.request.hold,
                            () => unawaited(_exit()),
                          );
                        }
                      }
                    : null,
                onVerticalDragCancel: phone
                    ? () {
                        if (!_dragging) return;
                        _dragging = false;
                        _ctrl.animateTo(1, curve: WpMotion.enter);
                        _timer = Timer(
                          widget.request.hold,
                          () => unawaited(_exit()),
                        );
                      }
                    : null,
                child: Material(
                  key: _bodyKey,
                  color: dark ? const Color(0xff1f1f1f) : Win10xFluent.chrome,
                  child: Container(
                    width: phone ? double.infinity : 360,
                    decoration: BoxDecoration(
                      border: Border(left: BorderSide(color: accent, width: 4)),
                    ),
                    padding: const EdgeInsets.symmetric(
                      horizontal: 14,
                      vertical: 12,
                    ),
                    child: Column(
                      mainAxisSize: MainAxisSize.min,
                      crossAxisAlignment: CrossAxisAlignment.stretch,
                      children: [
                        Text(
                          widget.request.message,
                          style: TextStyle(
                            color: dark ? Colors.white : Win10xFluent.text,
                            fontSize: phone ? 18 : 14,
                          ),
                        ),
                        if (widget.request.actions.isNotEmpty) ...[
                          const SizedBox(height: 8),
                          Wrap(
                            alignment: WrapAlignment.end,
                            children: [
                              for (final action in widget.request.actions)
                                TextButton(
                                  onPressed: () {
                                    action.onTap();
                                    unawaited(_exit());
                                  },
                                  child: Text(
                                    action.label,
                                    style: TextStyle(color: accent),
                                  ),
                                ),
                            ],
                          ),
                        ],
                      ],
                    ),
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
