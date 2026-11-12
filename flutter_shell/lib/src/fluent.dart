// SPDX-License-Identifier: GPL-3.0-only
import 'package:flutter/material.dart';

/// Windows 10 / Fluent 1 tokens from `/data/Win10UI` (not Win11 Sun Valley).
/// Chrome stays square; surfaces are opaque. No acrylic stack here.
abstract final class Win10xFluent {
  static const accent = Color(0xff0078d4);
  static const accentLight = Color(0xff429ce3);
  static const accentDark = Color(0xff005a9e);
  static const chrome = Color(0xfff3f3f3);
  static const bg = Color(0xffffffff);
  static const bgSmoke = Color(0xfff9f9f9);
  static const text = Color(0xff1a1a1a);
  static const textSecondary = Color(0xff5d5d5d);
  static const textTertiary = Color(0xff8a8a8a);
  static const stroke = Color(0xff8a8a8a);
  static const strokeCard = Color(0xffe5e5e5);
  static const reveal = Color(0x0f000000);
  static const revealStrong = Color(0x1a000000);
  static const taskbarActive = Color(0xffdcebf5);
  static const metroStart = Color(0xff1d1d1d);

  static const zero = BorderRadius.zero;
  static const square = RoundedRectangleBorder(borderRadius: zero);

  static ThemeData light() {
    const scheme = ColorScheme.light(
      primary: accent,
      onPrimary: Color(0xffffffff),
      secondary: accent,
      onSecondary: Color(0xffffffff),
      surface: chrome,
      onSurface: text,
      surfaceContainerHighest: bg,
      outline: stroke,
      error: Color(0xffc42b1c),
    );
    final base = ThemeData(
      useMaterial3: false,
      brightness: Brightness.light,
      colorScheme: scheme,
      primaryColor: accent,
      scaffoldBackgroundColor: chrome,
      canvasColor: chrome,
      cardColor: bg,
      dividerColor: strokeCard,
      highlightColor: reveal,
      splashColor: revealStrong,
      hoverColor: reveal,
      focusColor: revealStrong,
      visualDensity: VisualDensity.standard,
      materialTapTargetSize: MaterialTapTargetSize.shrinkWrap,
      splashFactory: NoSplash.splashFactory,
      pageTransitionsTheme: const PageTransitionsTheme(),
    );
    return base.copyWith(
      textSelectionTheme: const TextSelectionThemeData(
        cursorColor: accent,
        selectionColor: Color(0x660078d4),
        selectionHandleColor: accent,
      ),
      textTheme: base.textTheme.apply(
        bodyColor: text,
        displayColor: text,
        fontFamily: 'Segoe UI',
        fontFamilyFallback: const ['Noto Sans', 'Noto Sans CJK SC'],
      ),
      primaryTextTheme: base.primaryTextTheme.apply(
        bodyColor: text,
        displayColor: text,
        fontFamily: 'Segoe UI',
        fontFamilyFallback: const ['Noto Sans', 'Noto Sans CJK SC'],
      ),
      appBarTheme: const AppBarTheme(
        backgroundColor: chrome,
        foregroundColor: text,
        elevation: 0,
        shape: square,
        titleTextStyle: TextStyle(
          color: text,
          fontSize: 16,
          fontFamily: 'Segoe UI',
        ),
      ),
      cardTheme: const CardThemeData(
        color: bg,
        elevation: 0,
        margin: EdgeInsets.zero,
        shape: square,
      ),
      dialogTheme: const DialogThemeData(
        backgroundColor: chrome,
        shape: square,
        elevation: 8,
      ),
      popupMenuTheme: const PopupMenuThemeData(
        color: chrome,
        shape: square,
        textStyle: TextStyle(color: text, fontFamily: 'Segoe UI'),
      ),
      listTileTheme: const ListTileThemeData(
        iconColor: text,
        textColor: text,
        selectedColor: accent,
        selectedTileColor: taskbarActive,
        shape: square,
        contentPadding: EdgeInsets.symmetric(horizontal: 16),
      ),
      iconTheme: const IconThemeData(color: text, size: 20),
      iconButtonTheme: IconButtonThemeData(
        style: IconButton.styleFrom(
          foregroundColor: text,
          shape: square,
          hoverColor: reveal,
          highlightColor: revealStrong,
        ),
      ),
      textButtonTheme: TextButtonThemeData(
        style: TextButton.styleFrom(
          foregroundColor: accent,
          shape: square,
          textStyle: const TextStyle(fontFamily: 'Segoe UI'),
        ),
      ),
      elevatedButtonTheme: ElevatedButtonThemeData(
        style: ElevatedButton.styleFrom(
          backgroundColor: accent,
          foregroundColor: Colors.white,
          elevation: 0,
          shape: square,
          textStyle: const TextStyle(fontFamily: 'Segoe UI'),
        ),
      ),
      outlinedButtonTheme: OutlinedButtonThemeData(
        style: OutlinedButton.styleFrom(
          foregroundColor: text,
          side: const BorderSide(color: stroke),
          shape: square,
          textStyle: const TextStyle(fontFamily: 'Segoe UI'),
        ),
      ),
      filledButtonTheme: FilledButtonThemeData(
        style: FilledButton.styleFrom(
          backgroundColor: accent,
          foregroundColor: Colors.white,
          shape: square,
        ),
      ),
      inputDecorationTheme: const InputDecorationTheme(
        filled: true,
        fillColor: bg,
        hintStyle: TextStyle(color: textTertiary, fontFamily: 'Segoe UI'),
        contentPadding: EdgeInsets.all(8),
        border: OutlineInputBorder(
          borderRadius: zero,
          borderSide: BorderSide(color: stroke),
        ),
        enabledBorder: OutlineInputBorder(
          borderRadius: zero,
          borderSide: BorderSide(color: stroke),
        ),
        focusedBorder: OutlineInputBorder(
          borderRadius: zero,
          borderSide: BorderSide(color: accent, width: 2),
        ),
      ),
      sliderTheme: SliderThemeData(
        trackHeight: 2,
        activeTrackColor: accent,
        inactiveTrackColor: const Color(0xffc8c8c8),
        thumbColor: accent,
        overlayColor: Colors.transparent,
        thumbShape: const _FluentSliderThumb(),
        overlayShape: SliderComponentShape.noOverlay,
        trackShape: const RectangularSliderTrackShape(),
        showValueIndicator: ShowValueIndicator.never,
      ),
      switchTheme: SwitchThemeData(
        thumbColor: WidgetStateProperty.resolveWith(
          (states) => states.contains(WidgetState.selected)
              ? accent
              : const Color(0xff5d5d5d),
        ),
        trackColor: WidgetStateProperty.resolveWith(
          (states) => states.contains(WidgetState.selected)
              ? accentLight
              : const Color(0xffc8c8c8),
        ),
        trackOutlineColor: const WidgetStatePropertyAll(Colors.transparent),
      ),
      checkboxTheme: CheckboxThemeData(
        fillColor: WidgetStateProperty.resolveWith(
          (states) => states.contains(WidgetState.selected)
              ? accent
              : Colors.transparent,
        ),
        checkColor: const WidgetStatePropertyAll(Colors.white),
        side: const BorderSide(color: stroke, width: 1.5),
        shape: square,
      ),
      radioTheme: RadioThemeData(
        fillColor: WidgetStateProperty.resolveWith(
          (states) => states.contains(WidgetState.selected) ? accent : stroke,
        ),
      ),
      tooltipTheme: const TooltipThemeData(
        preferBelow: false,
        waitDuration: Duration(milliseconds: 400),
        decoration: BoxDecoration(color: Color(0xff2b2b2b)),
        textStyle: TextStyle(
          color: Colors.white,
          fontSize: 12,
          fontFamily: 'Segoe UI',
        ),
      ),
      dividerTheme: const DividerThemeData(
        color: strokeCard,
        thickness: 1,
        space: 1,
      ),
      snackBarTheme: const SnackBarThemeData(
        backgroundColor: Color(0xff2b2b2b),
        contentTextStyle: TextStyle(
          color: Colors.white,
          fontFamily: 'Segoe UI',
        ),
        shape: square,
      ),
      bannerTheme: const MaterialBannerThemeData(
        backgroundColor: bgSmoke,
        contentTextStyle: TextStyle(color: text, fontFamily: 'Segoe UI'),
      ),
    );
  }

  static ThemeData phone({bool dark = true, Color accentColor = accent}) {
    final base = light();
    final on = dark ? Colors.white : text;
    final surface = dark ? const Color(0xff101010) : chrome;
    return base.copyWith(
      brightness: dark ? Brightness.dark : Brightness.light,
      colorScheme: ColorScheme(
        brightness: dark ? Brightness.dark : Brightness.light,
        primary: accentColor,
        onPrimary: Colors.white,
        secondary: accentColor,
        onSecondary: Colors.white,
        surface: surface,
        onSurface: on,
        error: const Color(0xffc42b1c),
        onError: Colors.white,
      ),
      primaryColor: accentColor,
      scaffoldBackgroundColor: dark ? Colors.black : bg,
      canvasColor: dark ? Colors.black : chrome,
      textTheme: base.textTheme.apply(bodyColor: on, displayColor: on),
      iconTheme: IconThemeData(color: on, size: 20),
      listTileTheme: base.listTileTheme.copyWith(textColor: on, iconColor: on),
      dividerColor: dark ? const Color(0xff333333) : strokeCard,
      inputDecorationTheme: base.inputDecorationTheme.copyWith(
        fillColor: dark ? const Color(0xff202020) : bg,
        prefixIconColor: on,
        suffixIconColor: on,
      ),
      popupMenuTheme: PopupMenuThemeData(
        color: dark ? const Color(0xff1f1f1f) : chrome,
        shape: square,
        textStyle: TextStyle(color: on, fontFamily: 'Segoe UI'),
      ),
      dialogTheme: DialogThemeData(
        backgroundColor: dark ? const Color(0xff1f1f1f) : chrome,
        shape: square,
        elevation: 8,
        contentTextStyle: TextStyle(color: on, fontFamily: 'Segoe UI'),
      ),
    );
  }

  static Widget wrapPhone(
    Widget child, {
    bool dark = true,
    Color accentColor = accent,
  }) => Theme(
    data: phone(dark: dark, accentColor: accentColor),
    child: ScrollConfiguration(
      behavior: const _FluentScrollBehavior(),
      child: child,
    ),
  );

  static Widget wrap(Widget child) => Theme(
    data: light(),
    child: ScrollConfiguration(
      behavior: const _FluentScrollBehavior(),
      child: child,
    ),
  );
}

class _FluentScrollBehavior extends MaterialScrollBehavior {
  const _FluentScrollBehavior();
  @override
  ScrollPhysics getScrollPhysics(BuildContext context) =>
      const ClampingScrollPhysics();
  @override
  Widget buildOverscrollIndicator(
    BuildContext context,
    Widget child,
    ScrollableDetails details,
  ) => child;
}

class _FluentSliderThumb extends SliderComponentShape {
  const _FluentSliderThumb();
  @override
  Size getPreferredSize(bool isEnabled, bool isDiscrete) => const Size(8, 24);
  @override
  void paint(
    PaintingContext context,
    Offset center, {
    required Animation<double> activationAnimation,
    required Animation<double> enableAnimation,
    required bool isDiscrete,
    required TextPainter labelPainter,
    required RenderBox parentBox,
    required SliderThemeData sliderTheme,
    required TextDirection textDirection,
    required double value,
    required double textScaleFactor,
    required Size sizeWithOverflow,
  }) {
    final color =
        Color.lerp(
          sliderTheme.disabledThumbColor,
          sliderTheme.thumbColor,
          enableAnimation.value,
        ) ??
        Win10xFluent.accent;
    context.canvas.drawRRect(
      RRect.fromRectAndRadius(
        Rect.fromCenter(center: center, width: 8, height: 24),
        const Radius.circular(1),
      ),
      Paint()..color = color,
    );
  }
}
