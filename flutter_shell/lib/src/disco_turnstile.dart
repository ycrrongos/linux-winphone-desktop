// SPDX-License-Identifier: GPL-3.0-only
// Disco Launcher turnstile projection matching shell/src/start_tiles.rs.
import 'dart:math' as math;

import 'package:flutter/material.dart';

import 'motion_math.dart';
import 'disco_projection.dart';

const _perspective = 2000.0;
const _outerStartAngle = 70.0;
const _innerStartAngle = 45.0;
const _innerStartX = 60.0;

class DiscoTurnstileFrame {
  const DiscoTurnstileFrame({
    required this.outer,
    required this.inner,
    required this.closing,
  });
  final double outer;
  final double inner;
  final bool closing;
}

/// Projects a tile face through Disco's outer/inner turnstile.
class DiscoTurnstileTile extends StatelessWidget {
  const DiscoTurnstileTile({
    super.key,
    required this.rect,
    required this.frame,
    required this.color,
    required this.child,
    this.exitDistance = 120,
  });
  final Rect rect;
  final DiscoTurnstileFrame frame;
  final Color color;
  final Widget child;
  final double exitDistance;

  @override
  Widget build(BuildContext context) {
    final closing = frame.closing;
    final outerAngle = closing
        ? -40.0 * frame.outer
        : _outerStartAngle * (1 - frame.outer);
    final translateX = closing ? -exitDistance * frame.outer : 0.0;
    final innerAngle = closing ? 0.0 : _innerStartAngle * (1 - frame.inner);
    final innerX = closing ? 0.0 : _innerStartX * (1 - frame.inner);
    final quad = _projectContent(rect, rect, outerAngle, 0, 0, translateX);
    final projection = DiscoProjection(
      width: rect.width,
      height: rect.height,
      outerDegrees: outerAngle,
      innerDegrees: innerAngle,
      innerX: innerX,
      translateX: translateX,
    );
    final matrix = Matrix4.zero()
      ..setEntry(0, 0, projection.xx)
      ..setEntry(0, 3, projection.xt)
      ..setEntry(1, 0, projection.yx)
      ..setEntry(1, 1, 1)
      ..setEntry(1, 3, projection.yt)
      ..setEntry(2, 2, 1)
      ..setEntry(3, 0, projection.wx)
      ..setEntry(3, 3, projection.ww);
    return Stack(
      clipBehavior: Clip.none,
      children: [
        CustomPaint(
          size: Size(rect.width, rect.height),
          painter: _DiscoQuadPainter(quad: quad, color: color),
        ),
        Transform(
          alignment: Alignment.topLeft,
          transform: matrix,
          child: SizedBox(
            width: rect.width,
            height: rect.height,
            child: Opacity(
              opacity: frame.closing
                  ? (1 - frame.outer).clamp(0.0, 1.0)
                  : frame.outer.clamp(0.15, 1.0),
              child: child,
            ),
          ),
        ),
      ],
    );
  }
}

class _ProjectedQuad {
  const _ProjectedQuad(this.lt, this.rt, this.lb, this.rb);
  final Offset lt, rt, lb, rb;
}

_ProjectedQuad _projectContent(
  Rect tile,
  Rect content,
  double outerDeg,
  double innerDeg,
  double innerX,
  double translateX,
) {
  Offset project(Offset point) {
    final outer = outerDeg * math.pi / 180;
    final inner = innerDeg * math.pi / 180;
    final centerX = tile.center.dx;
    final innerLocalX = point.dx - centerX;
    final contentX =
        centerX - tile.left + innerLocalX * math.cos(inner) + innerX;
    final contentZ = -innerLocalX * math.sin(inner);
    final x = contentX * math.cos(outer) + contentZ * math.sin(outer);
    final z = -contentX * math.sin(outer) + contentZ * math.cos(outer);
    final scale = _perspective / (_perspective - z);
    return Offset(
      translateX + x * scale,
      (tile.height / 2) + (point.dy - tile.center.dy) * scale,
    );
  }

  return _ProjectedQuad(
    project(content.topLeft),
    project(content.topRight),
    project(content.bottomLeft),
    project(content.bottomRight),
  );
}

class _DiscoQuadPainter extends CustomPainter {
  const _DiscoQuadPainter({required this.quad, required this.color});
  final _ProjectedQuad quad;
  final Color color;

  @override
  void paint(Canvas canvas, Size size) {
    final path = Path()
      ..moveTo(quad.lt.dx, quad.lt.dy)
      ..lineTo(quad.rt.dx, quad.rt.dy)
      ..lineTo(quad.rb.dx, quad.rb.dy)
      ..lineTo(quad.lb.dx, quad.lb.dy)
      ..close();
    canvas.drawPath(path, Paint()..color = color);
  }

  @override
  bool shouldRepaint(covariant _DiscoQuadPainter oldDelegate) =>
      oldDelegate.quad != quad || oldDelegate.color != color;
}

/// Factor Launcher source-rectangle clip reveal for application launch.
class FactorClipReveal extends StatelessWidget {
  const FactorClipReveal({
    super.key,
    required this.source,
    required this.output,
    required this.progress,
    required this.child,
  });
  final Rect source;
  final Rect output;
  final double progress;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    final t = easeDecelerate(progress.clamp(0.0, 1.0));
    final rect = Rect.lerp(source, output, t)!;
    return IgnorePointer(
      child: Stack(
        fit: StackFit.expand,
        children: [
          Positioned.fromRect(
            rect: rect,
            child: ClipRect(
              child: OverflowBox(
                alignment: Alignment.topLeft,
                minWidth: output.width,
                maxWidth: output.width,
                minHeight: output.height,
                maxHeight: output.height,
                child: Transform.translate(
                  offset: -rect.topLeft,
                  child: SizedBox(
                    width: output.width,
                    height: output.height,
                    child: child,
                  ),
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
