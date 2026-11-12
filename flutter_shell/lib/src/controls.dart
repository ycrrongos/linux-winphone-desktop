// SPDX-License-Identifier: GPL-3.0-only
import 'package:flutter/material.dart';

import 'fluent.dart';

/// Full-height taskbar hit target, including the bottom edge.
class ShellButton extends StatelessWidget {
  const ShellButton({
    super.key,
    required this.label,
    required this.child,
    this.onPressed,
    this.onSecondaryTap,
    this.running = false,
    this.active = false,
    this.width = 56,
  });
  final String label;
  final Widget child;
  final VoidCallback? onPressed, onSecondaryTap;
  final bool running, active;
  final double width;

  @override
  Widget build(BuildContext context) => Semantics(
    button: true,
    label: label,
    selected: active,
    child: Tooltip(
      message: label,
      child: SizedBox(
        width: width,
        child: Material(
          color: active ? Win10xFluent.taskbarActive : Colors.transparent,
          child: InkWell(
            borderRadius: Win10xFluent.zero,
            onTap: onPressed,
            onSecondaryTap: onSecondaryTap,
            hoverColor: Win10xFluent.reveal,
            splashColor: Win10xFluent.revealStrong,
            child: Stack(
              fit: StackFit.expand,
              children: [
                Center(child: SizedBox(width: 30, height: 30, child: child)),
                if (running)
                  const Positioned(
                    bottom: 0,
                    left: 12,
                    right: 12,
                    child: ColoredBox(
                      color: Win10xFluent.accent,
                      child: SizedBox(height: 2),
                    ),
                  ),
              ],
            ),
          ),
        ),
      ),
    ),
  );
}

/// Fluent command target: square hover/press fill instead of Material's circle.
class FluentIconButton extends StatelessWidget {
  const FluentIconButton({
    super.key,
    required this.icon,
    required this.onPressed,
    this.tooltip,
  });
  final Widget icon;
  final VoidCallback? onPressed;
  final String? tooltip;
  @override
  Widget build(BuildContext context) {
    final button = Semantics(
      button: true,
      label: tooltip,
      enabled: onPressed != null,
      child: SizedBox(
        width: 44,
        height: 44,
        child: Material(
          color: Colors.transparent,
          child: InkWell(
            onTap: onPressed,
            borderRadius: BorderRadius.zero,
            splashFactory: NoSplash.splashFactory,
            hoverColor: Win10xFluent.reveal,
            highlightColor: Win10xFluent.revealStrong,
            child: Center(
              child: IconTheme.merge(
                data: IconThemeData(
                  size: 20,
                  color: onPressed == null
                      ? Win10xFluent.textTertiary
                      : Theme.of(context).colorScheme.onSurface,
                ),
                child: icon,
              ),
            ),
          ),
        ),
      ),
    );
    return tooltip == null ? button : Tooltip(message: tooltip!, child: button);
  }
}
