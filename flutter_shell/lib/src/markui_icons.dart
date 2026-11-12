// SPDX-License-Identifier: GPL-3.0-only
import 'dart:math' as math;

import 'package:flutter/material.dart';

import 'markui_catalog.dart';
import 'markui_metrics.dart';
import 'icon_geometry.dart';

/// Uses the original MarkUI bitmap alpha, with theme tint on every surface.
class MarkUiAsset extends StatelessWidget {
  const MarkUiAsset(this.asset, {super.key, this.size, this.color});
  final String asset;
  final double? size;
  final Color? color;
  @override
  Widget build(BuildContext context) {
    final theme = IconTheme.of(context);
    final extent = size ?? theme.size ?? 24;
    final tint =
        color ?? theme.color ?? Theme.of(context).colorScheme.onSurface;
    return SizedBox.square(
      dimension: extent,
      child: LayoutBuilder(
        builder: (context, constraints) {
          // A tile or taskbar can impose a tighter/larger slot than IconTheme.
          // Normalize inside the resolved slot, never the ignored requested size.
          final slot = constraints.biggest.shortestSide;
          final resolved = slot.isFinite ? slot : extent;
          final bounds = markUiBounds[asset];
          final dpr = MediaQuery.devicePixelRatioOf(context);
          final fit = bounds == null
              ? null
              : fitIconBounds(bounds, resolved, dpr);
          final width = fit?.width ?? resolved;
          final height = fit?.height ?? resolved;
          final offset = fit == null ? Offset.zero : Offset(fit.dx, fit.dy);
          return ClipRect(
            child: OverflowBox(
              minWidth: width,
              maxWidth: width,
              minHeight: height,
              maxHeight: height,
              child: Transform.translate(
                offset: offset,
                child: Image.asset(
                  'assets/wp_icons/$asset.png',
                  package: 'win10x_shell',
                  width: width,
                  height: height,
                  color: tint.withValues(alpha: tint.a * (theme.opacity ?? 1)),
                  colorBlendMode: BlendMode.srcIn,
                  fit: BoxFit.fill,
                  filterQuality: FilterQuality.medium,
                ),
              ),
            ),
          );
        },
      ),
    );
  }
}

Widget buildMarkUiApplicationIcon(
  BuildContext context,
  String id,
  Widget Function(BuildContext, String) fallback, {
  Color? color,
}) {
  final asset = markUiApplicationAsset(id);
  return asset == null
      ? fallback(context, id)
      : MarkUiAsset('apps/$asset', color: color);
}

/// The old IconData call sites select semantics only; Material glyphs are not
/// painted. Shapes absent from the supplied pack use explicit WP line drawings.
class MarkUiIcon extends StatelessWidget {
  const MarkUiIcon(
    this.icon, {
    super.key,
    this.size,
    this.color,
    this.semanticLabel,
  });
  final IconData? icon;
  final double? size;
  final Color? color;
  final String? semanticLabel;
  static final _assets = <IconData, String>{
    Icons.wifi: 'wifi',
    Icons.wifi_outlined: 'wifi_off',
    Icons.bluetooth: 'bluetooth',
    Icons.bluetooth_rounded: 'bluetooth',
    Icons.brightness_6_outlined: 'brightness',
    Icons.volume_up_outlined: 'volume',
    Icons.do_not_disturb_on_outlined: 'mute',
    Icons.settings_outlined: 'settings',
    Icons.lock_outline_rounded: 'lock',
    Icons.travel_explore_rounded: 'data',
    Icons.check: 'check',
    Icons.close: 'close',
    Icons.more_horiz: 'more',
    Icons.expand_more: 'expand',
    Icons.keyboard_arrow_down: 'expand',
    Icons.keyboard_arrow_up: 'collapse',
    Icons.chevron_right: 'chevron',
    Icons.refresh: 'refresh',
    Icons.apps: 'apps',
    Icons.insert_drive_file_rounded: 'file',
    Icons.folder_outlined: 'folder',
    Icons.play_arrow: 'play',
    Icons.tune: 'theme',
    Icons.delete_outline_rounded: 'delete',
    Icons.battery_std: 'battery',
    Icons.battery_charging_full: 'charging',
    Icons.signal_cellular_4_bar: 'signal',
    Icons.signal_cellular_0_bar: 'no_signal',
  };
  @override
  Widget build(BuildContext context) {
    final asset = _assets[icon];
    final theme = IconTheme.of(context);
    final extent = size ?? theme.size ?? 24;
    final tint =
        color ?? theme.color ?? Theme.of(context).colorScheme.onSurface;
    final child = asset != null
        ? MarkUiAsset('ui/$asset', size: extent, color: tint)
        : SizedBox.square(
            dimension: extent,
            child: CustomPaint(painter: _WpLineIcon(icon, tint)),
          );
    return Semantics(label: semanticLabel, child: child);
  }
}

class _WpLineIcon extends CustomPainter {
  const _WpLineIcon(this.icon, this.color);
  final IconData? icon;
  final Color color;
  @override
  void paint(Canvas canvas, Size size) {
    canvas.scale(size.width / 24, size.height / 24);
    final stroke = Paint()
      ..color = color
      ..style = PaintingStyle.stroke
      ..strokeWidth = 1.5;
    final fill = Paint()..color = color;
    void line(double x, double y, double a, double b) =>
        canvas.drawLine(Offset(x, y), Offset(a, b), stroke);
    if (icon == Icons.tablet_mac) {
      canvas.drawRect(const Rect.fromLTWH(5, 2, 14, 20), stroke);
      canvas.drawCircle(const Offset(12, 19), .8, fill);
    } else if (icon == Icons.desktop_windows_outlined) {
      canvas.drawRect(const Rect.fromLTWH(2, 3, 20, 14), stroke);
      line(12, 17, 12, 22);
      line(6, 22, 18, 22);
    } else if (icon == Icons.speed) {
      canvas.drawArc(
        const Rect.fromLTWH(3, 4, 18, 18),
        math.pi,
        math.pi,
        false,
        stroke,
      );
      line(3, 13, 3, 18);
      line(21, 13, 21, 18);
      line(3, 18, 21, 18);
      line(12, 14, 17, 7);
      canvas.drawCircle(const Offset(12, 14), 1.5, fill);
    } else if (icon == Icons.search) {
      canvas.drawCircle(const Offset(10, 10), 6.5, stroke);
      line(15, 15, 22, 22);
    } else if (icon == Icons.arrow_back || icon == Icons.arrow_forward) {
      if (icon == Icons.arrow_forward) {
        canvas.translate(24, 0);
        canvas.scale(-1, 1);
      }
      line(3, 12, 21, 12);
      line(3, 12, 11, 4);
      line(3, 12, 11, 20);
    } else if (icon == Icons.power_settings_new) {
      canvas.drawArc(
        const Rect.fromLTWH(4, 4, 16, 16),
        -math.pi / 3,
        math.pi * 5 / 3,
        false,
        stroke,
      );
      line(12, 2, 12, 12);
    } else if (icon == Icons.pause) {
      canvas.drawRect(const Rect.fromLTWH(6, 4, 4, 16), fill);
      canvas.drawRect(const Rect.fromLTWH(14, 4, 4, 16), fill);
    } else if (icon == Icons.skip_next || icon == Icons.skip_previous) {
      if (icon == Icons.skip_previous) {
        canvas.translate(24, 0);
        canvas.scale(-1, 1);
      }
      canvas.drawPath(
        Path()
          ..moveTo(5, 4)
          ..lineTo(17, 12)
          ..lineTo(5, 20)
          ..close(),
        fill,
      );
      line(19, 4, 19, 20);
    } else if (icon == Icons.push_pin_outlined ||
        icon == Icons.push_pin_rounded) {
      canvas.drawPath(
        Path()
          ..moveTo(7, 3)
          ..lineTo(17, 3)
          ..lineTo(15, 6)
          ..lineTo(15, 12)
          ..lineTo(19, 15)
          ..lineTo(5, 15)
          ..lineTo(9, 12)
          ..lineTo(9, 6)
          ..close(),
        stroke,
      );
      line(12, 15, 12, 23);
    } else if (icon == Icons.open_in_full) {
      line(3, 3, 10, 3);
      line(3, 3, 3, 10);
      line(3, 3, 10, 10);
      line(21, 21, 14, 21);
      line(21, 21, 21, 14);
      line(21, 21, 14, 14);
    } else {
      // Explicit neutral unknown-command shape, never an unrelated branded glyph.
      canvas.drawRect(const Rect.fromLTWH(4, 4, 16, 16), stroke);
    }
  }

  @override
  bool shouldRepaint(_WpLineIcon old) => old.icon != icon || old.color != color;
}
