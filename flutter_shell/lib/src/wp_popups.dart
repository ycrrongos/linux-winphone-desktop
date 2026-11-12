// SPDX-License-Identifier: GPL-3.0-only
import 'markui_icons.dart';

import 'dart:async';
import 'dart:math' as math;

import 'package:denial_flutter_sdk/popups.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:denial_flutter_sdk/state.dart';

import 'fluent.dart';
import 'preferences.dart';
import 'wp_motion.dart';
import 'phone_dialog_motion.dart';
import 'phone_chrome.dart';
import 'shell_navigation.dart';

/// Controlls enter/exit of a WinPhone-style popup before the host dismisses.
class WpPopupController {
  WpPopupController(this._closeHost);
  final VoidCallback _closeHost;
  VoidCallback? _requestExit;
  bool _closing = false;

  void bindExit(VoidCallback requestExit) => _requestExit = requestExit;

  void close() {
    if (_closing) return;
    _closing = true;
    final exit = _requestExit;
    if (exit != null) {
      exit();
    } else {
      _closeHost();
    }
  }

  void finishHostClose() => _closeHost();
}

/// Shows a phone ApplicationBar menu or MessageBox with full enter/exit motion.
abstract final class WpPopups {
  static final _controllers = <int, WpPopupController>{};

  static bool dismiss(int id) {
    final controller = _controllers[id];
    if (controller == null) return false;
    controller.close();
    return true;
  }

  static WpPopupController applicationBarMenu(
    WidgetRef ref, {
    required List<Widget> children,
    List<Widget> icons = const [],
    String debugLabel = 'Win10X WP application bar',
  }) {
    late final ShellPopupHandle handle;
    final controller = WpPopupController(() => handle.close());
    handle = ref
        .read(shellPopupControllerProvider.notifier)
        .show(
          debugLabel: debugLabel,
          transitionDuration: Duration.zero,
          dismissPolicy: ShellDismissPolicy.none,
          barrierColor: Colors.transparent,
          builder: (context, host) {
            final prefs = PreferenceController.instance.value;
            return Win10xFluent.wrapPhone(
              _PopupLifetime(
                onDispose: () => _controllers.remove(host.id),
                onClose: controller.close,
                child: _WpApplicationBar(
                  controller: controller,
                  icons: icons,
                  bottomInset: prefs.bottomInset,
                  dark: prefs.phoneThemeDark,
                  accent: Color(prefs.phoneAccentArgbValue),
                  children: children,
                ),
              ),
              dark: prefs.phoneThemeDark,
              accentColor: Color(prefs.phoneAccentArgbValue),
            );
          },
        );
    _controllers[handle.id] = controller;
    return controller;
  }

  static WpPopupController contextMenu(
    WidgetRef ref, {
    required Offset anchor,
    required List<Widget> children,
  }) {
    late final ShellPopupHandle handle;
    final controller = WpPopupController(() => handle.close());
    handle = ref
        .read(shellPopupControllerProvider.notifier)
        .show(
          debugLabel: 'Win10X WP context menu',
          transitionDuration: Duration.zero,
          dismissPolicy: ShellDismissPolicy.none,
          barrierColor: Colors.transparent,
          builder: (context, host) {
            final prefs = PreferenceController.instance.value;
            return Win10xFluent.wrapPhone(
              _PopupLifetime(
                onDispose: () => _controllers.remove(host.id),
                onClose: controller.close,
                child: _WpContextMenu(
                  controller: controller,
                  anchor: anchor,
                  dark: prefs.phoneThemeDark,
                  children: children,
                ),
              ),
              dark: prefs.phoneThemeDark,
              accentColor: Color(prefs.phoneAccentArgbValue),
            );
          },
        );
    _controllers[handle.id] = controller;
    return controller;
  }

  static Future<bool?> messageBox(
    WidgetRef ref, {
    required String title,
    required String body,
    String primary = '确定',
    String secondary = '取消',
    bool destructive = false,
  }) {
    final completer = Completer<bool?>();
    late final ShellPopupHandle handle;
    final controller = WpPopupController(() => handle.close());
    handle = ref
        .read(shellPopupControllerProvider.notifier)
        .show(
          debugLabel: 'Win10X WP message box',
          transitionDuration: Duration.zero,
          dismissPolicy: ShellDismissPolicy.none,
          barrierColor: Colors.transparent,
          builder: (context, host) {
            final prefs = PreferenceController.instance.value;
            return Win10xFluent.wrapPhone(
              _PopupLifetime(
                onDispose: () {
                  _controllers.remove(host.id);
                  if (!completer.isCompleted) completer.complete(null);
                },
                onClose: controller.close,
                child: _WpMessageBox(
                  controller: controller,
                  title: title,
                  body: body,
                  primary: primary,
                  secondary: secondary,
                  destructive: destructive,
                  dark: prefs.phoneThemeDark,
                  accent: Color(prefs.phoneAccentArgbValue),
                  onResult: (value) {
                    if (!completer.isCompleted) completer.complete(value);
                  },
                ),
              ),
              dark: prefs.phoneThemeDark,
              accentColor: Color(prefs.phoneAccentArgbValue),
            );
          },
        );
    _controllers[handle.id] = controller;
    // A popup can be removed before its builder ever mounts (Start/mode/lock
    // changes). Dispose of the body alone cannot complete that pending result.
    final subscription = ref.listenManual<List<ManagedShellPopup>>(
      shellPopupControllerProvider,
      (_, surfaces) {
        if (!surfaces.any((surface) => surface.id == handle.id) &&
            !completer.isCompleted) {
          _controllers.remove(handle.id);
          completer.complete(null);
        }
      },
    );
    return completer.future.whenComplete(subscription.close);
  }
}

class _PopupLifetime extends ConsumerStatefulWidget {
  const _PopupLifetime({
    required this.child,
    required this.onDispose,
    required this.onClose,
  });
  final Widget child;
  final VoidCallback onDispose, onClose;
  @override
  ConsumerState<_PopupLifetime> createState() => _PopupLifetimeState();
}

class _PopupLifetimeState extends ConsumerState<_PopupLifetime> {
  @override
  void dispose() {
    widget.onDispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => Focus(
    autofocus: true,
    onKeyEvent: (_, event) {
      if (event is KeyDownEvent &&
          event.logicalKey == LogicalKeyboardKey.escape) {
        widget.onClose();
        return KeyEventResult.handled;
      }
      return KeyEventResult.ignored;
    },
    child: Stack(
      fit: StackFit.expand,
      children: [
        widget.child,
        if (PreferenceController.instance.value.isPhone)
          if (ShellNavigation.instance.services case final services?)
            Positioned(
              left: 0,
              right: 0,
              bottom: 0,
              height: 48,
              child: PhoneNavigationBar(
                services: services,
                monitorId:
                    ref.watch(displayLayoutProvider)?.mainOutput?.monitorId ??
                    0,
              ),
            ),
      ],
    ),
  );
}

class _WpApplicationBar extends StatefulWidget {
  const _WpApplicationBar({
    required this.controller,
    required this.children,
    required this.icons,
    required this.bottomInset,
    required this.dark,
    required this.accent,
  });
  final WpPopupController controller;
  final List<Widget> children;
  final List<Widget> icons;
  final double bottomInset;
  final bool dark;
  final Color accent;

  @override
  State<_WpApplicationBar> createState() => _WpApplicationBarState();
}

class _WpApplicationBarState extends State<_WpApplicationBar>
    with TickerProviderStateMixin {
  late final AnimationController _ctrl;
  late final AnimationController _expand;

  /// MSDN: mini bar (icons) first; ellipsis expands the menu list.
  late bool _expanded;

  @override
  void initState() {
    super.initState();
    // No icon strip → open already expanded (text-only menus from Start).
    _expanded = widget.icons.isEmpty;
    _ctrl = AnimationController(vsync: this, duration: WpMotion.barEnter)
      ..forward();
    _expand = AnimationController(
      vsync: this,
      duration: WpMotion.barEnter,
      value: _expanded ? 1 : 0,
    );
    widget.controller.bindExit(_exit);
  }

  Future<void> _exit() async {
    _expand.reverseDuration = WpMotion.barExit;
    _ctrl.reverseDuration = WpMotion.barExit;
    try {
      await Future.wait([_expand.reverse().orCancel, _ctrl.reverse().orCancel]);
    } on TickerCanceled {
      return;
    }
    if (mounted) widget.controller.finishHostClose();
  }

  void _toggleExpand() {
    setState(() => _expanded = !_expanded);
    _expand.duration = WpMotion.barEnter;
    _expand.reverseDuration = WpMotion.barExit;
    if (_expanded) {
      unawaited(_expand.forward());
    } else {
      unawaited(_expand.reverse());
    }
  }

  @override
  void dispose() {
    _ctrl.dispose();
    _expand.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final surface = widget.dark ? const Color(0xff1f1f1f) : Win10xFluent.chrome;
    final onSurface = widget.dark ? Colors.white : Win10xFluent.text;
    final size = MediaQuery.sizeOf(context);
    // MSDN App bar: in landscape the bar docks to the short edge.
    final landscape = size.width > size.height;
    return AnimatedBuilder(
      animation: Listenable.merge([_ctrl, _expand]),
      builder: (context, _) {
        final progress = _progress(_ctrl);
        final expandProgress = _expand.status == AnimationStatus.reverse
            ? 1 - WpMotion.exit.transform(1 - _expand.value)
            : WpMotion.enter.transform(_expand.value);
        final menu = ClipRect(
          child: Align(
            alignment: Alignment.bottomCenter,
            heightFactor: expandProgress.clamp(0.0, 1.0),
            child: Opacity(
              opacity: expandProgress.clamp(0.0, 1.0),
              child: ConstrainedBox(
                constraints: BoxConstraints(
                  maxHeight: size.height * (landscape ? 0.7 : 0.45),
                ),
                child: SingleChildScrollView(
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      for (final child in widget.children)
                        DefaultTextStyle.merge(
                          style: TextStyle(
                            color: onSurface,
                            fontSize: 18,
                            fontFamily: 'Segoe UI',
                          ),
                          child: child,
                        ),
                    ],
                  ),
                ),
              ),
            ),
          ),
        );
        final iconRow = SizedBox(
          height: 64,
          child: Row(
            children: [
              for (final icon in widget.icons.take(4)) Expanded(child: icon),
              Expanded(
                child: IconButton(
                  tooltip: _expanded ? '收起' : '更多',
                  onPressed: widget.icons.isEmpty
                      ? widget.controller.close
                      : _toggleExpand,
                  icon: MarkUiIcon(
                    widget.icons.isEmpty
                        ? Icons.more_horiz
                        : (_expanded ? Icons.expand_more : Icons.more_horiz),
                    color: onSurface,
                  ),
                ),
              ),
            ],
          ),
        );
        final bar = Material(
          color: surface,
          child: ConstrainedBox(
            constraints: BoxConstraints(
              maxWidth: landscape
                  ? math.min(360, size.width * 0.45)
                  : size.width,
            ),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                if (widget.icons.isNotEmpty) iconRow,
                menu,
                if (widget.icons.isEmpty && expandProgress > 0.99)
                  SizedBox(
                    height: 48,
                    child: IconButton(
                      tooltip: '关闭',
                      onPressed: widget.controller.close,
                      icon: MarkUiIcon(Icons.more_horiz, color: onSurface),
                    ),
                  ),
              ],
            ),
          ),
        );
        return Stack(
          fit: StackFit.expand,
          children: [
            GestureDetector(
              behavior: HitTestBehavior.opaque,
              onTap: widget.controller.close,
              child: ColoredBox(color: Colors.transparent),
            ),
            Align(
              alignment: landscape
                  ? Alignment.centerRight
                  : Alignment.bottomCenter,
              child: Padding(
                padding: EdgeInsets.only(
                  bottom: landscape ? 0 : widget.bottomInset,
                ),
                child: Transform.translate(
                  offset: landscape
                      ? Offset((1 - progress) * 120, 0)
                      : Offset(0, (1 - progress) * 120),
                  child: Opacity(opacity: progress.clamp(0.0, 1.0), child: bar),
                ),
              ),
            ),
          ],
        );
      },
    );
  }
}

class _WpMessageBox extends StatefulWidget {
  const _WpMessageBox({
    required this.controller,
    required this.title,
    required this.body,
    required this.primary,
    required this.secondary,
    required this.destructive,
    required this.dark,
    required this.accent,
    required this.onResult,
  });
  final WpPopupController controller;
  final String title;
  final String body;
  final String primary;
  final String secondary;
  final bool destructive;
  final bool dark;
  final Color accent;
  final ValueChanged<bool?> onResult;

  @override
  State<_WpMessageBox> createState() => _WpMessageBoxState();
}

class _WpMessageBoxState extends State<_WpMessageBox>
    with SingleTickerProviderStateMixin {
  late final AnimationController _ctrl;
  bool? _result;

  @override
  void initState() {
    super.initState();
    _ctrl = AnimationController(vsync: this, duration: WpMotion.dialogEnter)
      ..forward();
    widget.controller.bindExit(_exit);
  }

  Future<void> _exit() async {
    _ctrl.reverseDuration = WpMotion.dialogExit;
    await _ctrl.reverse();
    widget.onResult(_result);
    if (mounted) widget.controller.finishHostClose();
  }

  void _choose(bool? value) {
    _result = value;
    widget.controller.close();
  }

  @override
  void dispose() {
    widget.onResult(null);
    _ctrl.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final surface = widget.dark ? const Color(0xff1f1f1f) : Win10xFluent.chrome;
    final onSurface = widget.dark ? Colors.white : Win10xFluent.text;
    return AnimatedBuilder(
      animation: _ctrl,
      builder: (context, _) {
        final progress = _progress(_ctrl);
        final phone = PreferenceController.instance.value.isPhone;
        return Stack(
          fit: StackFit.expand,
          children: [
            GestureDetector(
              behavior: HitTestBehavior.opaque,
              onTap: () {},
              child: ColoredBox(
                color: Color.fromRGBO(0, 0, 0, 0.55 * progress),
              ),
            ),
            Center(
              child: Opacity(
                opacity: progress.clamp(0.0, 1.0),
                child: PhoneDialogFlight(
                  progress: progress,
                  viewport: phone
                      ? MediaQuery.sizeOf(context)
                      : const Size(420, 280),
                  child: ConstrainedBox(
                    constraints: BoxConstraints(
                      maxWidth: phone
                          ? math.min(420, MediaQuery.sizeOf(context).width - 24)
                          : 420,
                      maxHeight: math.max(
                        0,
                        MediaQuery.sizeOf(context).height -
                            MediaQuery.viewInsetsOf(context).bottom -
                            96,
                      ),
                    ),
                    child: Material(
                      color: surface,
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        crossAxisAlignment: CrossAxisAlignment.stretch,
                        children: [
                          Padding(
                            padding: const EdgeInsets.fromLTRB(20, 18, 20, 8),
                            child: Text(
                              widget.title,
                              style: TextStyle(
                                color: onSurface,
                                fontSize: 24,
                                fontFamily: 'Segoe UI',
                                fontWeight: FontWeight.w600,
                              ),
                            ),
                          ),
                          Flexible(
                            child: SingleChildScrollView(
                              padding: const EdgeInsets.fromLTRB(20, 0, 20, 16),
                              child: Text(
                                widget.body,
                                style: TextStyle(
                                  color: onSurface.withValues(alpha: 0.85),
                                  fontSize: 18,
                                  fontFamily: 'Segoe UI',
                                  height: 1.35,
                                ),
                              ),
                            ),
                          ),
                          Padding(
                            padding: const EdgeInsets.fromLTRB(20, 4, 20, 20),
                            child: Row(
                              children: [
                                Expanded(
                                  child: OutlinedButton(
                                    style: OutlinedButton.styleFrom(
                                      foregroundColor: onSurface,
                                      side: BorderSide(
                                        color: onSurface.withValues(alpha: .65),
                                      ),
                                      shape: const RoundedRectangleBorder(),
                                      minimumSize: const Size(0, 40),
                                    ),
                                    onPressed: () => _choose(false),
                                    child: Text(widget.secondary),
                                  ),
                                ),
                                const SizedBox(width: 12),
                                Expanded(
                                  child: FilledButton(
                                    style: FilledButton.styleFrom(
                                      backgroundColor: widget.destructive
                                          ? const Color(0xffc42b1c)
                                          : widget.accent,
                                      foregroundColor: Colors.white,
                                      shape: const RoundedRectangleBorder(),
                                      minimumSize: const Size(0, 40),
                                    ),
                                    onPressed: () => _choose(true),
                                    child: Text(widget.primary),
                                  ),
                                ),
                              ],
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),
                ),
              ),
            ),
          ],
        );
      },
    );
  }
}

double _progress(AnimationController ctrl) {
  if (ctrl.status == AnimationStatus.reverse) {
    return 1 - WpMotion.exit.transform(1 - ctrl.value);
  }
  return WpMotion.enter.transform(ctrl.value);
}

/// Separate from the edge CommandBar: anchored phone flyouts open across, then
/// down (the WP elevator/escalator construction), with eased stage boundaries.
class _WpContextMenu extends StatefulWidget {
  const _WpContextMenu({
    required this.controller,
    required this.anchor,
    required this.dark,
    required this.children,
  });
  final WpPopupController controller;
  final Offset anchor;
  final bool dark;
  final List<Widget> children;
  @override
  State<_WpContextMenu> createState() => _WpContextMenuState();
}

class _WpContextMenuState extends State<_WpContextMenu>
    with SingleTickerProviderStateMixin {
  late final _motion = AnimationController(
    vsync: this,
    duration: const Duration(milliseconds: 500),
    reverseDuration: WpMotion.barExit,
  );
  @override
  void initState() {
    super.initState();
    widget.controller.bindExit(_exit);
    _motion.forward();
  }

  Future<void> _exit() async {
    try {
      await _motion.reverse().orCancel;
    } on TickerCanceled {
      return;
    }
    if (mounted) widget.controller.finishHostClose();
  }

  @override
  void dispose() {
    _motion.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => AnimatedBuilder(
    animation: _motion,
    builder: (context, _) {
      final t = _motion.value;
      final across = WpMotion.enter.transform((t / .76).clamp(0, 1));
      final down = WpMotion.enter.transform(((t - .76) / .24).clamp(0, 1));
      final color = widget.dark ? Colors.white : Colors.black;
      return ClipRect(
        child: Stack(
          fit: StackFit.expand,
          children: [
            GestureDetector(
              behavior: HitTestBehavior.opaque,
              onTap: widget.controller.close,
              child: const ColoredBox(color: Colors.transparent),
            ),
            CustomSingleChildLayout(
              delegate: _PhoneMenuPlacement(widget.anchor),
              child: IgnorePointer(
                ignoring: _motion.status == AnimationStatus.reverse,
                child: Transform.scale(
                  scaleX: across,
                  scaleY: math.max(.004, down),
                  alignment: Alignment.topCenter,
                  child: DecoratedBox(
                    decoration: BoxDecoration(
                      color: widget.dark ? Colors.black : Colors.white,
                      border: Border.all(color: color, width: 1.5),
                    ),
                    child: IntrinsicWidth(
                      child: SingleChildScrollView(
                        padding: const EdgeInsets.symmetric(vertical: 8),
                        child: Column(
                          mainAxisSize: MainAxisSize.min,
                          crossAxisAlignment: CrossAxisAlignment.stretch,
                          children: widget.children,
                        ),
                      ),
                    ),
                  ),
                ),
              ),
            ),
          ],
        ),
      );
    },
  );
}

class _PhoneMenuPlacement extends SingleChildLayoutDelegate {
  const _PhoneMenuPlacement(this.anchor);
  final Offset anchor;
  @override
  BoxConstraints getConstraintsForChild(BoxConstraints c) => BoxConstraints(
    maxWidth: math.max(0, math.min(340, c.maxWidth - 24)),
    maxHeight: math.max(0, c.maxHeight - 96),
  );
  @override
  Offset getPositionForChild(Size size, Size child) {
    final x = (anchor.dx - child.width / 2).clamp(
      12.0,
      math.max(12.0, size.width - child.width - 12),
    );
    final below = anchor.dy + 12;
    final y =
        (below + child.height <= size.height - 60
                ? below
                : anchor.dy - child.height - 12)
            .clamp(32.0, math.max(32.0, size.height - child.height - 60));
    return Offset(x.toDouble(), y.toDouble());
  }

  @override
  bool shouldRelayout(covariant _PhoneMenuPlacement old) =>
      anchor != old.anchor;
}
