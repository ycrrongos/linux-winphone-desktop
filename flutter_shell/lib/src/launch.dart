// SPDX-License-Identifier: GPL-3.0-only
import 'dart:math' as math;

import 'package:flutter/material.dart';

/// Selected Windows 8 tile: lift, reach 90° in the air, then expand the
/// unmirrored reverse face until the 180° turn completes at 720 ms.
class Windows8FlyingTile extends StatelessWidget {
  const Windows8FlyingTile({
    super.key,
    required this.source,
    required this.output,
    required this.progress,
    required this.face,
    required this.color,
  });
  final Rect source, output;
  final double progress;
  final Widget face;
  final Color color;

  @override
  Widget build(BuildContext context) {
    final p = progress.clamp(0.0, 1.0);
    // One continuous trajectory: no zero-velocity seams between lift,
    // edge-on turn and expansion. Start slowly, carry momentum through 90°,
    // and decelerate only at the final full-output face.
    final t = p * p * (3 - 2 * p);
    final growth = t * t;
    final center =
        Offset.lerp(source.center, output.center, t)! +
        Offset(0, -source.height * .22 * math.sin(math.pi * p));
    final size = Size.lerp(
      source.size,
      output.size + const Offset(4, 4),
      growth,
    )!;
    final rect = Rect.fromCenter(
      center: center,
      width: size.width,
      height: size.height,
    );
    final angle = math.pi * t;
    final reverse = angle >= math.pi / 2;
    final matrix = Matrix4.identity()
      ..setEntry(3, 2, .0007)
      ..rotateY(angle);
    final unmirror = Matrix4.identity()..rotateY(math.pi);
    return Positioned.fromRect(
      rect: rect,
      child: IgnorePointer(
        child: Transform(
          transform: matrix,
          alignment: Alignment.center,
          child: reverse
              ? Transform(
                  transform: unmirror,
                  alignment: Alignment.center,
                  child: ColoredBox(
                    color: color,
                    child: Center(
                      child: SizedBox(
                        width: 48 + 24 * t,
                        height: 48 + 24 * t,
                        child: face,
                      ),
                    ),
                  ),
                )
              : ColoredBox(
                  color: color,
                  child: Center(
                    child: SizedBox(
                      width: 48 + 24 * t,
                      height: 48 + 24 * t,
                      child: face,
                    ),
                  ),
                ),
        ),
      ),
    );
  }
}
