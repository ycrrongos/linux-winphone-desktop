// SPDX-License-Identifier: GPL-3.0-only
// Copied beside client_geometry.dart in the prepared reference plugin.
import 'package:flutter/widgets.dart';

import 'client_geometry.dart';

Rect win10xClientRect(Rect target, Size source) {
  final fitted = fitClientRect(
    (
      left: target.left,
      top: target.top,
      width: target.width,
      height: target.height,
    ),
    source.width,
    source.height,
  );
  return Rect.fromLTWH(fitted.left, fitted.top, fitted.width, fitted.height);
}

/// Preserve the committed client's proportions while awaiting its new buffer.
/// The publisher uses win10xClientRect for the same source→destination mapping.
class Win10xClientSurface extends StatelessWidget {
  const Win10xClientSurface({
    super.key,
    required this.sourceSize,
    required this.frameWidth,
    required this.child,
  });
  final Size sourceSize;
  final double frameWidth;
  final Widget child;
  @override
  Widget build(BuildContext context) => LayoutBuilder(
    builder: (context, constraints) {
      final size = constraints.biggest;
      if (!size.isFinite || size.isEmpty || sourceSize.isEmpty) return child;
      final inset = frameWidth.clamp(0.0, size.shortestSide / 2).toDouble();
      final target = (Offset.zero & size).deflate(inset);
      final fitted = win10xClientRect(target, sourceSize).inflate(inset);
      return ClipRect(
        child: Stack(
          fit: StackFit.expand,
          children: [Positioned.fromRect(rect: fitted, child: child)],
        ),
      );
    },
  );
}
