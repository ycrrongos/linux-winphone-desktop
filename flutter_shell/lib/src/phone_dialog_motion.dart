// SPDX-License-Identifier: GPL-3.0-only
import 'dart:math' as math;

import 'package:flutter/rendering.dart';
import 'package:flutter/widgets.dart';

/// Product-requested full-viewport→dialog flight. Uniformly scales the real
/// content; no layout/text reflow and no independently stretched axes.
double phoneDialogScale(Size viewport, Size dialog, double progress) {
  if (dialog.isEmpty) return 1;
  final from = math.max(
    1.0,
    math.max(viewport.width / dialog.width, viewport.height / dialog.height),
  );
  return from + (1 - from) * progress.clamp(0, 1);
}

class PhoneDialogFlight extends SingleChildRenderObjectWidget {
  const PhoneDialogFlight({
    super.key,
    required this.progress,
    required this.viewport,
    required super.child,
  });
  final double progress;
  final Size viewport;
  @override
  RenderObject createRenderObject(BuildContext context) =>
      RenderPhoneDialogFlight(progress, viewport);
  @override
  void updateRenderObject(
    BuildContext context,
    covariant RenderPhoneDialogFlight renderObject,
  ) => renderObject.update(progress, viewport);
}

class RenderPhoneDialogFlight extends RenderTransform {
  RenderPhoneDialogFlight(this._progress, this._viewport)
    : super(transform: Matrix4.identity(), alignment: Alignment.center);
  double _progress;
  Size _viewport;
  void update(double progress, Size viewport) {
    _progress = progress;
    _viewport = viewport;
    if (hasSize) _matrix();
  }

  void _matrix() {
    final scale = phoneDialogScale(_viewport, size, _progress);
    transform = Matrix4.diagonal3Values(scale, scale, 1);
  }

  @override
  void performLayout() {
    super.performLayout();
    _matrix();
  }
}
