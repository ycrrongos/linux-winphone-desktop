// SPDX-License-Identifier: GPL-3.0-only
import 'dart:math' as math;

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import 'markui_icons.dart';
import 'wp_motion.dart';
import 'screen_motion.dart';
import 'phone_lock_model.dart';

/// WP picture page over the real credential page. During a drag the picture
/// tracks the finger; release settles nonlinearly without a spring or overshoot.
class PhoneLockPane extends StatefulWidget {
  const PhoneLockPane({
    super.key,
    required this.time,
    required this.wallpaper,
    required this.credentials,
    required this.onCredentialsChanged,
    this.credentialsVisible = false,
    this.onRevealSettled,
    this.notificationCount = 0,
    this.batteryLabel,
    this.connectivity = const SizedBox.shrink(),
  });
  final DateTime time;
  final Widget wallpaper, credentials, connectivity;
  final bool credentialsVisible;
  final int notificationCount;
  final String? batteryLabel;
  final ValueChanged<bool> onCredentialsChanged;
  final VoidCallback? onRevealSettled;
  @override
  State<PhoneLockPane> createState() => _PhoneLockPaneState();
}

class _PhoneLockPaneState extends State<PhoneLockPane>
    with SingleTickerProviderStateMixin {
  bool _cancelled = false;
  late final _slide = AnimationController(
    vsync: this,
    duration: WpMotion.dialogEnter,
    value: widget.credentialsVisible ? 1 : 0,
  );
  @override
  void didUpdateWidget(covariant PhoneLockPane old) {
    super.didUpdateWidget(old);
    if (old.credentialsVisible != widget.credentialsVisible) {
      _settle(widget.credentialsVisible);
    }
  }

  void _settle(bool show) {
    final reduce = MediaQuery.disableAnimationsOf(context);
    if (reduce) {
      _slide.value = show ? 1 : 0;
      if (show) widget.onRevealSettled?.call();
      return;
    }
    _slide
        .animateTo(
          show ? 1 : 0,
          duration: show ? WpMotion.dialogEnter : WpMotion.dialogExit,
          curve: show ? WpMotion.enter : WpMotion.exit,
        )
        .then((_) {
          if (mounted && show && _slide.value >= .99) {
            widget.onRevealSettled?.call();
          }
        });
  }

  void _open() {
    widget.onCredentialsChanged(true);
    _settle(true);
  }

  @override
  void dispose() {
    _slide.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => LayoutBuilder(
    builder: (context, c) {
      final height = c.maxHeight;
      final time = widget.time;
      final weekdays = Localizations.localeOf(context).languageCode == 'zh'
          ? ['星期一', '星期二', '星期三', '星期四', '星期五', '星期六', '星期日']
          : [
              'Monday',
              'Tuesday',
              'Wednesday',
              'Thursday',
              'Friday',
              'Saturday',
              'Sunday',
            ];
      final months = [
        'January',
        'February',
        'March',
        'April',
        'May',
        'June',
        'July',
        'August',
        'September',
        'October',
        'November',
        'December',
      ];
      final date = Localizations.localeOf(context).languageCode == 'zh'
          ? '${time.month}月${time.day}日'
          : '${months[time.month - 1]} ${time.day}';
      final scale = math.min(1.0, c.maxWidth / 390);
      return ClipRect(
        child: Focus(
          autofocus: true,
          onKeyEvent: (_, event) {
            if (event is KeyDownEvent &&
                (event.logicalKey == LogicalKeyboardKey.arrowUp ||
                    event.logicalKey == LogicalKeyboardKey.enter ||
                    event.logicalKey == LogicalKeyboardKey.space)) {
              if (_slide.value < .98) {
                _open();
                return KeyEventResult.handled;
              }
            }
            return KeyEventResult.ignored;
          },
          child: AnimatedBuilder(
            animation: _slide,
            builder: (context, _) => Stack(
              fit: StackFit.expand,
              children: [
                const ColoredBox(color: Color(0xff080808)),
                IgnorePointer(
                  ignoring: _slide.value < .99,
                  child: ExcludeFocus(
                    excluding: _slide.value < .99,
                    child: Opacity(
                      opacity: (_slide.value * _slide.value).clamp(0, 1),
                      child: Padding(
                        padding: EdgeInsets.only(
                          top: 24,
                          bottom: 48 + MediaQuery.viewInsetsOf(context).bottom,
                        ),
                        child: widget.credentials,
                      ),
                    ),
                  ),
                ),
                if (_slide.value < 1 || !widget.credentialsVisible)
                  Transform.translate(
                    offset: Offset(0, -height * _slide.value),
                    child: Semantics(
                      label: '锁屏图片；向上滑动输入密码',
                      onTap: _open,
                      button: true,
                      child: Listener(
                        onPointerDown: (_) => _cancelled = false,
                        onPointerCancel: (_) {
                          _cancelled = true;
                          _settle(widget.credentialsVisible);
                        },
                        child: GestureDetector(
                          behavior: HitTestBehavior.opaque,
                          onTap: _open,
                          onVerticalDragStart: (_) => _slide.stop(),
                          onVerticalDragUpdate: (d) => _slide.value =
                              (_slide.value -
                                      (d.primaryDelta ?? 0) /
                                          math.max(1, height))
                                  .clamp(0, 1),
                          onVerticalDragCancel: () =>
                              _settle(widget.credentialsVisible),
                          onVerticalDragEnd: (d) {
                            if (_cancelled) return;
                            final open = revealLockCredentials(
                              _slide.value,
                              d.primaryVelocity ?? 0,
                            );
                            widget.onCredentialsChanged(open);
                            _settle(open);
                          },
                          child: Stack(
                            fit: StackFit.expand,
                            children: [
                              _WakeWallpaper(child: widget.wallpaper),
                              const DecoratedBox(
                                decoration: BoxDecoration(
                                  gradient: LinearGradient(
                                    begin: Alignment.topCenter,
                                    end: Alignment.bottomCenter,
                                    colors: [
                                      Color(0x22000000),
                                      Color(0x08000000),
                                      Color(0x88000000),
                                    ],
                                    stops: [0, .45, 1],
                                  ),
                                ),
                              ),
                              Positioned(
                                top: 8,
                                left: 20,
                                right: 20,
                                height: 24,
                                child: _WakeClock(
                                  child: Row(
                                    children: [
                                      widget.connectivity,
                                      const Spacer(),
                                      const MarkUiAsset(
                                        'ui/battery',
                                        size: 16,
                                        color: Colors.white,
                                      ),
                                      if (widget.batteryLabel case final text?)
                                        Padding(
                                          padding: const EdgeInsets.only(
                                            left: 6,
                                          ),
                                          child: Text(
                                            text,
                                            style: const TextStyle(
                                              color: Colors.white,
                                              fontSize: 11,
                                            ),
                                          ),
                                        ),
                                    ],
                                  ),
                                ),
                              ),
                              Positioned(
                                left: 20,
                                right: 20,
                                bottom: math.max(48, height * .115),
                                child: _WakeClock(
                                  child: Column(
                                    crossAxisAlignment:
                                        CrossAxisAlignment.start,
                                    mainAxisSize: MainAxisSize.min,
                                    children: [
                                      Text(
                                        '${MediaQuery.alwaysUse24HourFormatOf(context) ? time.hour : (time.hour % 12 == 0 ? 12 : time.hour % 12)}:${time.minute.toString().padLeft(2, '0')}',
                                        style: TextStyle(
                                          color: Colors.white,
                                          fontSize: 90 * scale,
                                          height: 1.04,
                                          fontWeight: FontWeight.w200,
                                          fontFamily: 'Segoe UI',
                                        ),
                                      ),
                                      Text(
                                        weekdays[time.weekday - 1],
                                        style: TextStyle(
                                          color: Colors.white,
                                          fontSize: 27 * scale,
                                          height: 1.15,
                                          fontWeight: FontWeight.w300,
                                        ),
                                      ),
                                      Text(
                                        date,
                                        style: TextStyle(
                                          color: Colors.white,
                                          fontSize: 27 * scale,
                                          height: 1.15,
                                          fontWeight: FontWeight.w300,
                                        ),
                                      ),
                                      const SizedBox(height: 14),
                                      if (widget.notificationCount > 0)
                                        Padding(
                                          padding: const EdgeInsets.only(
                                            left: 60,
                                          ),
                                          child: Row(
                                            mainAxisSize: MainAxisSize.min,
                                            children: [
                                              const MarkUiAsset(
                                                'ui/notifications',
                                                size: 18,
                                                color: Colors.white,
                                              ),
                                              const SizedBox(width: 5),
                                              Text(
                                                '${widget.notificationCount}',
                                                style: const TextStyle(
                                                  color: Colors.white,
                                                  fontSize: 16,
                                                ),
                                              ),
                                            ],
                                          ),
                                        ),
                                    ],
                                  ),
                                ),
                              ),
                            ],
                          ),
                        ),
                      ),
                    ),
                  ),
              ],
            ),
          ),
        ),
      );
    },
  );
}

class _WakeWallpaper extends StatelessWidget {
  const _WakeWallpaper({required this.child});
  final Widget child;
  @override
  Widget build(BuildContext context) => ValueListenableBuilder<double>(
    valueListenable: ScreenMotion.progress,
    builder: (context, t, _) {
      final p = WpMotion.enter.transform(ScreenMotion.wallpaper(t));
      if (p == 0) return const ColoredBox(color: Color(0xff000000));
      if (p == 1) return child;
      return ShaderMask(
        blendMode: BlendMode.dstIn,
        shaderCallback: (rect) => LinearGradient(
          begin: Alignment.topCenter,
          end: Alignment.bottomCenter,
          colors: const [Color(0x00ffffff), Color(0xffffffff)],
          stops: [(1 - p - .16).clamp(0, 1), (1 - p + .16).clamp(0, 1)],
        ).createShader(rect),
        child: Opacity(opacity: p, child: child),
      );
    },
  );
}

class _WakeClock extends StatelessWidget {
  const _WakeClock({required this.child});
  final Widget child;
  @override
  Widget build(BuildContext context) => ValueListenableBuilder<double>(
    valueListenable: ScreenMotion.progress,
    builder: (context, t, _) {
      final p = WpMotion.enter.transform(ScreenMotion.chrome(t));
      return Opacity(
        opacity: p,
        child: Transform.translate(
          offset: Offset(0, -72 * (1 - p)),
          child: child,
        ),
      );
    },
  );
}
