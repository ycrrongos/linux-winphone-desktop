// SPDX-License-Identifier: GPL-3.0-only
import 'dart:async';
import 'dart:math' as math;

import 'package:flutter/material.dart';
import 'package:denial_flutter_sdk/services.dart';

import 'fluent_motion.dart';
import 'markui_icons.dart';
import 'phone_tasks_model.dart';
import 'symbols.dart';

/// Shared presentation, with real compositor data/actions supplied by the host.
/// Synthetic windows are only supplied by the opt-in preview exercise.
class PhoneTaskBoard extends StatefulWidget {
  const PhoneTaskBoard({
    super.key,
    required this.windows,
    required this.accent,
    required this.labelFor,
    required this.buildIcon,
    required this.buildPreview,
    required this.onActivate,
    required this.onClose,
    required this.onHome,
  });
  final List<ApplicationWindow> windows;
  final Color accent;
  final String Function(ApplicationWindow) labelFor;
  final Widget Function(BuildContext, String) buildIcon;
  final Widget Function(BuildContext, int) buildPreview;
  final void Function(ApplicationWindow, Rect) onActivate;
  final ValueChanged<int> onClose;
  final VoidCallback onHome;
  @override
  State<PhoneTaskBoard> createState() => _PhoneTaskBoardState();
}

class _PhoneTaskBoardState extends State<PhoneTaskBoard> {
  final _order = PhoneTaskOrder();
  final _pages = PageController(viewportFraction: .86);
  @override
  void dispose() {
    _pages.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    _order.sync(
      widget.windows.map((w) => w.id),
      widget.windows.where((w) => w.active && !w.minimized).firstOrNull?.id,
    );
    final windows = {for (final w in widget.windows) w.id: w};
    final ids = [
      ..._order.ids,
      0,
    ]; // Start is a shell destination, never a fake native window.
    return Material(
      color: widget.accent,
      child: Padding(
        padding: const EdgeInsets.only(top: 24, bottom: 48),
        child: LayoutBuilder(
          builder: (context, constraints) {
            final capacity = phoneTaskPageCapacity(
              MediaQuery.sizeOf(context).width,
              MediaQuery.sizeOf(context).height,
            );
            final columns = capacity == 4 ? 2 : 1;
            final rows = capacity == 4 ? 2 : 1;
            final pageCount = (ids.length / capacity).ceil();
            if (_pages.hasClients && (_pages.page ?? 0) >= pageCount) {
              WidgetsBinding.instance.addPostFrameCallback((_) {
                if (mounted && _pages.hasClients) {
                  _pages.jumpToPage(pageCount - 1);
                }
              });
            }
            final width = math.max(1.0, constraints.maxWidth * .86 - 16);
            final cardWidth = (width - (columns - 1) * 14) / columns;
            final aspect =
                MediaQuery.sizeOf(context).width /
                math.max(1.0, MediaQuery.sizeOf(context).height - 72);
            final desiredHeight =
                rows * (cardWidth / aspect + 40) + (rows - 1) * 18;
            final height = math.min(constraints.maxHeight - 24, desiredHeight);
            Widget card(int id) {
              final window = windows[id];
              final home = id == 0;
              final label = home ? '开始' : widget.labelFor(window!);
              return _PhoneTaskCard(
                key: ValueKey(id),
                label: label,
                home: home,
                foreground: widget.accent.computeLuminance() > .4
                    ? Colors.black
                    : Colors.white,
                icon: home
                    ? ShellGlyph(
                        ShellSymbol.start,
                        color: widget.accent.computeLuminance() > .4
                            ? Colors.black
                            : Colors.white,
                        size: 18,
                      )
                    : widget.buildIcon(context, window!.appId),
                preview: home
                    ? Center(
                        child: ShellGlyph(
                          ShellSymbol.start,
                          color: Colors.white,
                          size: 52,
                        ),
                      )
                    : widget.buildPreview(context, id),
                onActivate: (rect) {
                  if (home) {
                    widget.onHome();
                  } else {
                    widget.onActivate(window!, rect);
                  }
                },
                onClose: () => widget.onClose(id),
              );
            }

            return Center(
              child: SizedBox(
                height: math.max(1, height),
                child: PageView.builder(
                  controller: _pages,
                  itemCount: pageCount,
                  physics: const _TaskPagePhysics(
                    parent: ClampingScrollPhysics(),
                  ),
                  itemBuilder: (context, page) => Padding(
                    padding: const EdgeInsets.symmetric(horizontal: 8),
                    child: GridView.count(
                      crossAxisCount: columns,
                      mainAxisSpacing: 18,
                      crossAxisSpacing: 14,
                      physics: const NeverScrollableScrollPhysics(),
                      childAspectRatio:
                          cardWidth / ((height - (rows - 1) * 18) / rows),
                      padding: EdgeInsets.zero,
                      children: [
                        for (final id
                            in ids.skip(page * capacity).take(capacity))
                          card(id),
                      ],
                    ),
                  ),
                ),
              ),
            );
          },
        ),
      ),
    );
  }
}

class _TaskPagePhysics extends PageScrollPhysics {
  const _TaskPagePhysics({super.parent});
  @override
  _TaskPagePhysics applyTo(ScrollPhysics? ancestor) =>
      _TaskPagePhysics(parent: buildParent(ancestor));
  @override
  SpringDescription get spring =>
      SpringDescription.withDampingRatio(mass: 1, stiffness: 250, ratio: 1);
}

class _PhoneTaskCard extends StatefulWidget {
  const _PhoneTaskCard({
    super.key,
    required this.label,
    required this.home,
    required this.foreground,
    required this.icon,
    required this.preview,
    required this.onActivate,
    required this.onClose,
  });
  final String label;
  final bool home;
  final Color foreground;
  final Widget icon, preview;
  final ValueChanged<Rect> onActivate;
  final VoidCallback onClose;
  @override
  State<_PhoneTaskCard> createState() => _PhoneTaskCardState();
}

class _PhoneTaskCardState extends State<_PhoneTaskCard>
    with SingleTickerProviderStateMixin {
  late final _controller = AnimationController(
    vsync: this,
    duration: FluentMotion.slow,
  );
  Animation<double>? _motion;
  double _drag = 0;
  bool _closing = false;
  Timer? _restore;
  int _generation = 0;
  final _previewKey = GlobalKey();
  double get _offset => _motion?.value ?? _drag;
  @override
  void dispose() {
    _restore?.cancel();
    _controller.dispose();
    super.dispose();
  }

  Future<bool> _settle(double target, Curve curve) async {
    final generation = ++_generation;
    final from = _offset;
    _controller.stop();
    _motion = Tween<double>(
      begin: from,
      end: target,
    ).chain(CurveTween(curve: curve)).animate(_controller);
    try {
      await _controller.forward(from: 0).orCancel;
    } on TickerCanceled {
      return false;
    }
    if (!mounted || generation != _generation) return false;
    setState(() => _drag = target);
    return true;
  }

  Future<void> _close(double height) async {
    if (_closing || widget.home) return;
    setState(() => _closing = true);
    if (!await _settle(height + 48, FluentMotion.accelerate)) return;
    widget.onClose();
    // Only a real compositor removal removes the card. Restore a still-live
    // client so an unsaved-document confirmation/rejected close stays reachable.
    if (mounted) {
      _restore = Timer(const Duration(milliseconds: 700), () {
        if (!mounted) return;
        setState(() => _closing = false);
        unawaited(_settle(0, FluentMotion.standard));
      });
    }
  }

  @override
  Widget build(BuildContext context) => LayoutBuilder(
    builder: (context, constraints) {
      final height = constraints.maxHeight;
      return AnimatedBuilder(
        animation: _controller,
        builder: (context, _) => Transform.translate(
          offset: Offset(0, _offset),
          child: GestureDetector(
            behavior: HitTestBehavior.opaque,
            onTap: () {
              if (_closing) return;
              final box =
                  _previewKey.currentContext!.findRenderObject()! as RenderBox;
              widget.onActivate(box.localToGlobal(Offset.zero) & box.size);
            },
            onVerticalDragStart: widget.home
                ? null
                : (_) {
                    _restore?.cancel();
                    final offset = _offset;
                    _controller.stop();
                    _generation++;
                    setState(() {
                      _drag = offset;
                      _motion = null;
                      _closing = false;
                    });
                  },
            onVerticalDragUpdate: widget.home
                ? null
                : (d) => setState(
                    () => _drag = math.max(0, _drag + (d.primaryDelta ?? 0)),
                  ),
            onVerticalDragCancel: widget.home
                ? null
                : () => unawaited(_settle(0, FluentMotion.standard)),
            onVerticalDragEnd: widget.home
                ? null
                : (d) {
                    if (dismissPhoneTask(
                      distance: _drag,
                      height: height,
                      velocity: d.primaryVelocity ?? 0,
                    )) {
                      unawaited(_close(height));
                    } else {
                      unawaited(_settle(0, FluentMotion.standard));
                    }
                  },
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                SizedBox(
                  height: 40,
                  child: Row(
                    children: [
                      SizedBox(width: 22, height: 22, child: widget.icon),
                      const SizedBox(width: 6),
                      Expanded(
                        child: Text(
                          widget.label,
                          maxLines: 1,
                          overflow: TextOverflow.ellipsis,
                          style: TextStyle(
                            color: widget.foreground,
                            fontSize: 13,
                          ),
                        ),
                      ),
                      if (!widget.home)
                        SizedBox(
                          width: 40,
                          height: 40,
                          child: IconButton(
                            padding: EdgeInsets.zero,
                            tooltip: '关闭 ${widget.label}',
                            onPressed: _closing
                                ? null
                                : () => unawaited(_close(height)),
                            icon: MarkUiIcon(
                              Icons.close,
                              color: widget.foreground,
                              size: 18,
                            ),
                          ),
                        ),
                    ],
                  ),
                ),
                Expanded(
                  child: Semantics(
                    key: _previewKey,
                    button: true,
                    label: '切换到 ${widget.label}',
                    child: ColoredBox(
                      color: const Color(0xff161616),
                      child: IgnorePointer(child: widget.preview),
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
