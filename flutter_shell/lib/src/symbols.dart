// SPDX-License-Identifier: GPL-3.0-only
import 'package:flutter/widgets.dart';

import 'markui_icons.dart';

enum ShellSymbol {
  start,
  back,
  search,
  tasks,
  notifications,
  menu,
  apps,
  tiles,
}

/// Windows Phone / 马克笔UI nav glyphs with vector fallbacks.
class ShellGlyph extends StatelessWidget {
  const ShellGlyph(this.symbol, {super.key, this.size = 20, this.color});
  final ShellSymbol symbol;
  final double size;
  final Color? color;

  static const _asset = {
    ShellSymbol.start: 'assets/wp_icons/nav/start.png',
    ShellSymbol.back: 'assets/wp_icons/nav/back.png',
    ShellSymbol.search: 'assets/wp_icons/nav/search.png',
    ShellSymbol.tasks: 'assets/wp_icons/nav/tasks.png',
    ShellSymbol.notifications: 'assets/wp_icons/ui/notifications.png',
    ShellSymbol.apps: 'assets/wp_icons/ui/apps.png',
  };

  @override
  Widget build(BuildContext context) {
    final tint =
        color ?? IconTheme.of(context).color ?? const Color(0xff111111);
    final asset = _asset[symbol];
    if (asset != null) {
      return SizedBox.square(
        dimension: size,
        child: MarkUiAsset(
          asset.replaceFirst('assets/wp_icons/', '').replaceFirst('.png', ''),
          size: size,
          color: tint,
        ),
      );
    }
    return CustomPaint(
      size: Size.square(size),
      painter: _GlyphPainter(symbol, tint),
    );
  }
}

class _GlyphPainter extends CustomPainter {
  const _GlyphPainter(this.symbol, this.color);
  final ShellSymbol symbol;
  final Color color;
  @override
  void paint(Canvas canvas, Size size) {
    canvas.scale(size.width / 24, size.height / 24);
    final stroke = Paint()
      ..color = color
      ..style = PaintingStyle.stroke
      ..strokeWidth = 1.4;
    final fill = Paint()..color = color;
    void line(double x, double y, double a, double b) =>
        canvas.drawLine(Offset(x, y), Offset(a, b), stroke);
    switch (symbol) {
      case ShellSymbol.start:
        for (final x in [2.0, 13.0]) {
          for (final y in [2.0, 13.0]) {
            canvas.drawRect(Rect.fromLTWH(x, y, 9, 9), fill);
          }
        }
      case ShellSymbol.back:
        line(3, 12, 21, 12);
        line(3, 12, 11, 4);
        line(3, 12, 11, 20);
      case ShellSymbol.search:
        canvas.drawCircle(const Offset(10, 10), 7, stroke);
        line(15, 15, 22, 22);
      case ShellSymbol.tasks:
        canvas.drawRect(const Rect.fromLTWH(5, 4, 14, 16), stroke);
        line(1, 6, 1, 18);
        line(23, 6, 23, 18);
      case ShellSymbol.notifications:
        canvas.drawPath(
          Path()
            ..moveTo(2, 3)
            ..lineTo(22, 3)
            ..lineTo(22, 18)
            ..lineTo(10, 18)
            ..lineTo(5, 22)
            ..lineTo(5, 18)
            ..lineTo(2, 18)
            ..close(),
          stroke,
        );
        line(6, 7, 18, 7);
        line(6, 11, 18, 11);
      case ShellSymbol.menu:
        for (final y in [5.0, 12.0, 19.0]) {
          line(2, y, 22, y);
        }
      case ShellSymbol.apps:
        for (final x in [3.0, 10.0, 17.0]) {
          for (final y in [3.0, 10.0, 17.0]) {
            canvas.drawRect(Rect.fromLTWH(x, y, 4, 4), fill);
          }
        }
      case ShellSymbol.tiles:
        canvas.drawRect(const Rect.fromLTWH(2, 2, 12, 12), fill);
        canvas.drawRect(const Rect.fromLTWH(16, 2, 6, 6), fill);
        canvas.drawRect(const Rect.fromLTWH(16, 10, 6, 12), fill);
        canvas.drawRect(const Rect.fromLTWH(2, 16, 12, 6), fill);
    }
  }

  @override
  bool shouldRepaint(covariant _GlyphPainter oldDelegate) =>
      oldDelegate.symbol != symbol || oldDelegate.color != color;
}
