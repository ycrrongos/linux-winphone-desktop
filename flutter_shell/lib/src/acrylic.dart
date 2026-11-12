// SPDX-License-Identifier: GPL-3.0-only
// New acrylic baseline: Denial compositor ShellBackdropBlur + accent tint.
// Forbidden: old wallpaper sampler, CPU blur cache, HostBackdrop, GLES shader.
import 'package:denial_flutter_sdk/effects.dart';
import 'package:flutter/material.dart';

/// Clean acrylic layer for Start / panels. Falls back to alpha tint when the
/// host has backdrop blur disabled.
class Win10xAcrylic extends StatelessWidget {
  const Win10xAcrylic({
    super.key,
    required this.dark,
    required this.accent,
    this.strength = 0.65,
    this.child,
  });

  final bool dark;
  final Color accent;
  final double strength;
  final Widget? child;

  Color get _tint {
    if (dark) {
      return Color.alphaBlend(
        accent.withValues(alpha: 0.18),
        const Color(0x99000000),
      );
    }
    return Color.alphaBlend(
      accent.withValues(alpha: 0.10),
      const Color(0xb3ffffff),
    );
  }

  @override
  Widget build(BuildContext context) {
    final content = ColoredBox(
      color: _tint,
      child: child ?? const SizedBox.expand(),
    );
    return ShellBackdropBlur(
      strength: strength.clamp(0.0, 1.0),
      separateChild: true,
      child: content,
    );
  }
}
