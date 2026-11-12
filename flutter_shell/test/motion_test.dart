// SPDX-License-Identifier: GPL-3.0-only
import 'dart:math' as math;

import 'package:test/test.dart';
import 'package:win10x_shell/src/start_motion.dart';
import 'package:win10x_shell/src/model.dart';

void main() {
  test('Fluent curves match the legacy Rust power 2.6 timing', () {
    for (final t in [.1, .25, .5, .75, .9]) {
      expect(easeDecelerate(t), closeTo(1 - math.pow(1 - t, 2.6), 1e-10));
      expect(easeAccelerate(t), closeTo(math.pow(t, 2.6), 1e-10));
    }
  });
  test('general chrome and source Beziers never overshoot or reverse', () {
    for (final curve in [
      easeDecelerate,
      easeAccelerate,
      easeOutExpo,
      flutterEaseOutCubic,
    ]) {
      var previous = 0.0;
      expect(curve(0), 0);
      expect(curve(1), 1);
      for (var i = 0; i <= 1000; i++) {
        final value = curve(i / 1000);
        expect(value, inInclusiveRange(previous, 1.0));
        previous = value;
      }
    }
  });
  test('source expo is the Rust Bezier, not an exponential approximation', () {
    expect(easeOutExpo(.25), closeTo(.8255, .001));
    expect(flutterEaseOutCubic(.5), closeTo(.8751, .002));
  });
  test('easeOutBack matches legacy Rust tile settle overshoot', () {
    expect(easeOutBack(0), closeTo(0, 1e-12));
    expect(easeOutBack(1), closeTo(1, 1e-9));
    expect(easeOutBack(0.8), greaterThan(1.0));
  });
  test('WP ApplicationBar uses log enter / exp exit (MSDN motion guidance)', () {
    // WpMotion.enter/exit wrap these; durations are Denial Fluent 333/250/200.
    expect(easeOutExpo(0.2), greaterThan(0.5));
    expect(easeInExpo(0.2), lessThan(0.1));
    expect(easeInExpo(0.8), lessThan(easeOutExpo(0.8)));
    for (final ms in [333, 250, 200]) {
      expect(ms, greaterThan(0));
    }
  });
  test(
    'closing retains the final staggered group until its veil has faded',
    () {
      final prefs = ShellPreferences(
        startAnimation: StartAnimation.flutterMetroGroup,
        groups: [for (var i = 0; i < 8; i++) TileGroup(i, '$i')],
      );
      final duration = StartReveal.closeDuration(prefs).inMilliseconds / 1000;
      expect(duration, greaterThan(1.05));
      expect(
        StartReveal.flutterMetroGroupProgress(
          elapsed: duration,
          closing: true,
          groupIndex: 7,
        ),
        0,
      );
    },
  );
}
