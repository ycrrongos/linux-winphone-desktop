// SPDX-License-Identifier: GPL-3.0-only
import 'dart:math' as math;

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import 'fluent.dart';
import 'fluent_motion.dart';
import 'preferences.dart';

/// Win10 radio: 20px ring, 10px accent disc. Never uses Material Radio.
class FluentRadio<T> extends StatelessWidget {
  const FluentRadio({
    super.key,
    required this.value,
    required this.groupValue,
    required this.onChanged,
    required this.label,
    this.subtitle,
  });
  final T value;
  final T? groupValue;
  final ValueChanged<T>? onChanged;
  final String label;
  final String? subtitle;

  bool get _selected => value == groupValue;

  @override
  Widget build(BuildContext context) {
    return FluentMenuItem(
      onTap: onChanged == null ? null : () => onChanged!(value),
      checked: _selected,
      leading: CustomPaint(
        size: const Size(20, 20),
        painter: _RadioPainter(selected: _selected),
      ),
      label: label,
      subtitle: subtitle,
    );
  }
}

/// Win10 toggle switch from Win10UI tokens (accent track when on).
class FluentToggle extends StatelessWidget {
  const FluentToggle({
    super.key,
    required this.value,
    required this.onChanged,
    required this.label,
  });
  final bool value;
  final ValueChanged<bool>? onChanged;
  final String label;

  @override
  Widget build(BuildContext context) {
    return FluentMenuItem(
      onTap: onChanged == null ? null : () => onChanged!(!value),
      label: label,
      trailing: TweenAnimationBuilder<double>(
        tween: Tween(end: value ? 1.0 : 0.0),
        duration: MediaQuery.disableAnimationsOf(context)
            ? Duration.zero
            : FluentMotion.normal,
        curve: FluentMotion.standard,
        builder: (context, progress, _) => CustomPaint(
          size: const Size(44, 20),
          painter: _TogglePainter(progress: progress),
        ),
      ),
      toggled: value,
    );
  }
}

/// Flat Win10 menu/command-bar row with reveal hover.
class FluentMenuItem extends StatefulWidget {
  const FluentMenuItem({
    super.key,
    required this.label,
    this.subtitle,
    this.leading,
    this.trailing,
    this.onTap,
    this.danger = false,
    this.checked,
    this.toggled,
  });
  final String label;
  final String? subtitle;
  final Widget? leading;
  final Widget? trailing;
  final VoidCallback? onTap;
  final bool danger;
  final bool? checked, toggled;

  @override
  State<FluentMenuItem> createState() => _FluentMenuItemState();
}

class _FluentMenuItemState extends State<FluentMenuItem> {
  bool _hover = false, _pressed = false, _focused = false;

  @override
  Widget build(BuildContext context) {
    final enabled = widget.onTap != null;
    final color = !enabled
        ? Win10xFluent.textTertiary
        : widget.danger
        ? const Color(0xffc42b1c)
        : Theme.of(context).colorScheme.onSurface;
    return Semantics(
      button: widget.checked == null && widget.toggled == null,
      checked: widget.checked,
      toggled: widget.toggled,
      enabled: enabled,
      inMutuallyExclusiveGroup: widget.checked == null ? null : true,
      child: FocusableActionDetector(
        enabled: enabled,
        mouseCursor: enabled
            ? SystemMouseCursors.click
            : SystemMouseCursors.basic,
        onShowHoverHighlight: (value) => setState(() => _hover = value),
        onShowFocusHighlight: (value) => setState(() => _focused = value),
        actions: {
          ActivateIntent: CallbackAction<ActivateIntent>(
            onInvoke: (_) {
              widget.onTap?.call();
              return null;
            },
          ),
        },
        shortcuts: const {
          SingleActivator(LogicalKeyboardKey.enter): ActivateIntent(),
          SingleActivator(LogicalKeyboardKey.space): ActivateIntent(),
        },
        child: GestureDetector(
          behavior: HitTestBehavior.opaque,
          onTap: widget.onTap,
          onTapDown: enabled ? (_) => setState(() => _pressed = true) : null,
          onTapUp: (_) => setState(() => _pressed = false),
          onTapCancel: () => setState(() => _pressed = false),
          child: AnimatedContainer(
            duration: MediaQuery.disableAnimationsOf(context)
                ? Duration.zero
                : FluentMotion.fast,
            curve: FluentMotion.standard,
            constraints: const BoxConstraints(minHeight: 44),
            decoration: BoxDecoration(
              color: _pressed && enabled
                  ? Win10xFluent.revealStrong
                  : _hover && enabled
                  ? Win10xFluent.reveal
                  : Colors.transparent,
              border: Border.all(
                color: _focused
                    ? Theme.of(context).colorScheme.onSurface
                    : Colors.transparent,
              ),
            ),
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
              child: Row(
                children: [
                  if (widget.leading != null) ...[
                    widget.leading!,
                    const SizedBox(width: 10),
                  ],
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        Text(
                          widget.label,
                          style: TextStyle(
                            color: color,
                            fontSize: 14,
                            fontFamily: 'Segoe UI',
                            height: 1.3,
                          ),
                        ),
                        if (widget.subtitle != null)
                          Padding(
                            padding: const EdgeInsets.only(top: 2),
                            child: Text(
                              widget.subtitle!,
                              style: const TextStyle(
                                color: Win10xFluent.textSecondary,
                                fontSize: 12,
                                fontFamily: 'Segoe UI',
                                height: 1.3,
                              ),
                            ),
                          ),
                      ],
                    ),
                  ),
                  if (widget.trailing != null) ...[
                    const SizedBox(width: 12),
                    widget.trailing!,
                  ],
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}

class FluentSeparator extends StatelessWidget {
  const FluentSeparator({super.key});
  @override
  Widget build(BuildContext context) => const ColoredBox(
    color: Win10xFluent.strokeCard,
    child: SizedBox(height: 1, width: double.infinity),
  );
}

enum FluentMenuPlacement { nearPointer, bottomBar }

/// Opaque Win10 chrome panel. Popups must wrap with [Win10xFluent.wrap]
/// because Denial's overlay sits outside the launcher Theme.
class FluentMenuSurface extends StatelessWidget {
  const FluentMenuSurface({
    super.key,
    required this.placement,
    required this.children,
    this.anchor,
    this.maxWidth = 320,
  });
  final FluentMenuPlacement placement;
  final List<Widget> children;
  final Offset? anchor;
  final double maxWidth;

  @override
  Widget build(BuildContext context) {
    final size = MediaQuery.sizeOf(context);
    final taskbar = PreferenceController.instance.value.bottomInset;
    final status = PreferenceController.instance.value.topInset;
    final panel = Material(
      color: Theme.of(context).colorScheme.surface,
      elevation: 0,
      shape: Win10xFluent.square,
      child: DecoratedBox(
        decoration: BoxDecoration(
          color: Theme.of(context).colorScheme.surface,
          border: Border.all(color: Win10xFluent.strokeCard),
          boxShadow: const [
            BoxShadow(
              color: Color(0x24000000),
              blurRadius: 8,
              offset: Offset(0, 4),
            ),
          ],
        ),
        child: ConstrainedBox(
          constraints: BoxConstraints(
            maxWidth: placement == FluentMenuPlacement.bottomBar
                ? size.width
                : math.min(maxWidth, math.max(0, size.width - 16)),
            maxHeight: math.max(0, size.height - status - taskbar - 16),
          ),
          child: SingleChildScrollView(
            child: Padding(
              padding: const EdgeInsets.symmetric(vertical: 2),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: children,
              ),
            ),
          ),
        ),
      ),
    );

    if (placement == FluentMenuPlacement.bottomBar) {
      // Windows Phone application-bar style: full-width strip slides up.
      return Align(
        alignment: Alignment.bottomCenter,
        child: Padding(
          padding: EdgeInsets.only(bottom: taskbar),
          child: TweenAnimationBuilder<double>(
            tween: Tween(begin: 1, end: 0),
            duration: const Duration(milliseconds: 220),
            curve: Curves.easeOutCubic,
            builder: (context, t, child) => Transform.translate(
              offset: Offset(0, 48 * t),
              child: Opacity(opacity: 1 - t * .35, child: child),
            ),
            child: SizedBox(width: size.width, child: panel),
          ),
        ),
      );
    }

    final point = anchor ?? Offset(size.width / 2, size.height / 2);
    return CustomSingleChildLayout(
      delegate: _MenuPosition(point, status + 8, taskbar + 8),
      child: panel,
    );
  }
}

class _RadioPainter extends CustomPainter {
  const _RadioPainter({required this.selected});
  final bool selected;

  @override
  void paint(Canvas canvas, Size size) {
    final center = Offset(size.width / 2, size.height / 2);
    final ring = Paint()
      ..style = PaintingStyle.stroke
      ..strokeWidth = 1
      ..color = selected ? Win10xFluent.accent : Win10xFluent.stroke;
    canvas.drawCircle(center, 9.5, ring);
    if (selected) {
      canvas.drawCircle(center, 5, Paint()..color = Win10xFluent.accent);
    }
  }

  @override
  bool shouldRepaint(covariant _RadioPainter oldDelegate) =>
      oldDelegate.selected != selected;
}

class _TogglePainter extends CustomPainter {
  const _TogglePainter({required this.progress});
  final double progress;
  bool get on => progress >= .5;

  @override
  void paint(Canvas canvas, Size size) {
    final track = RRect.fromRectAndRadius(
      Offset.zero & size,
      const Radius.circular(10),
    );
    canvas.drawRRect(
      track,
      Paint()
        ..color = on ? Win10xFluent.accent : Win10xFluent.bg
        ..style = PaintingStyle.fill,
    );
    canvas.drawRRect(
      track,
      Paint()
        ..color = on ? Win10xFluent.accent : Win10xFluent.stroke
        ..style = PaintingStyle.stroke
        ..strokeWidth = 1,
    );
    final thumb = Offset(10 + (size.width - 20) * progress, size.height / 2);
    canvas.drawCircle(
      thumb,
      7,
      Paint()..color = on ? Colors.white : Win10xFluent.textSecondary,
    );
  }

  @override
  bool shouldRepaint(covariant _TogglePainter oldDelegate) =>
      oldDelegate.progress != progress;
}

class _MenuPosition extends SingleChildLayoutDelegate {
  const _MenuPosition(this.anchor, this.topInset, this.bottomInset);
  final Offset anchor;
  final double topInset, bottomInset;
  @override
  BoxConstraints getConstraintsForChild(BoxConstraints constraints) =>
      BoxConstraints(
        maxWidth: math.max(0, constraints.maxWidth - 16),
        maxHeight: math.max(0, constraints.maxHeight - topInset - bottomInset),
      );
  @override
  Offset getPositionForChild(Size size, Size childSize) => Offset(
    anchor.dx.clamp(8, math.max(8, size.width - childSize.width - 8)),
    anchor.dy.clamp(
      topInset,
      math.max(topInset, size.height - bottomInset - childSize.height),
    ),
  );
  @override
  bool shouldRelayout(covariant _MenuPosition old) =>
      old.anchor != anchor ||
      old.topInset != topInset ||
      old.bottomInset != bottomInset;
}
