// SPDX-License-Identifier: GPL-3.0-only
import 'dart:math' as math;

typedef ClientRect = ({double left, double top, double width, double height});

/// One uniform scale shared by pixels, pointer routing and retained transitions.
/// A client can commit its old buffer after a new configure, or enforce a
/// minimum size larger than a phone viewport. Never squash that buffer to fit.
ClientRect fitClientRect(
  ClientRect target,
  double sourceWidth,
  double sourceHeight,
) {
  if (!sourceWidth.isFinite ||
      !sourceHeight.isFinite ||
      sourceWidth <= 0 ||
      sourceHeight <= 0 ||
      target.width <= 0 ||
      target.height <= 0) {
    return target;
  }
  final scale = math.min(
    target.width / sourceWidth,
    target.height / sourceHeight,
  );
  final width = sourceWidth * scale, height = sourceHeight * scale;
  return (
    left: target.left + (target.width - width) / 2,
    top: target.top + (target.height - height) / 2,
    width: width,
    height: height,
  );
}
