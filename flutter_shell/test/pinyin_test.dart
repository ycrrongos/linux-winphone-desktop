// SPDX-License-Identifier: GPL-3.0-only
import 'dart:io';

import 'package:test/test.dart';
import 'package:win10x_shell/src/ime/pinyin_engine.dart';

void main() {
  late PinyinEngine engine;
  setUpAll(
    () => engine = PinyinEngine(
      File('assets/ime/pinyin_simp.dict.yaml').readAsStringSync(),
    ),
  );
  test(
    'bundled dictionary is complete and common full pinyin/initials work',
    () {
      expect(engine.entries, greaterThan(60000));
      expect(engine.lookup('nihao').map((c) => c.text), contains('你好'));
      expect(engine.lookup('zhongguo').map((c) => c.text), contains('中国'));
      expect(engine.lookup('nh').map((c) => c.text), contains('你好'));
      expect(engine.lookup('nv').map((c) => c.text), contains('女'));
      expect(engine.lookup('lü').map((c) => c.text), contains('绿'));
    },
  );
  test('sentence composition and explicit syllable boundaries', () {
    expect(engine.lookup('nihaoshijie').first.text, '你好世界');
    expect(engine.lookup("xi'an").map((c) => c.text), contains('西安'));
    expect(engine.lookup("xi'an").map((c) => c.text), isNot(contains('先')));
    expect(engine.lookup('nih').map((c) => c.text), contains('你好'));
  });
  test('candidate prefix consumption and personal learning stay bounded', () {
    final rows = engine.lookup('nihaoma');
    expect(rows.any((c) => c.text == '你好' && c.consumed == 5), isTrue);
    final second = engine.lookup('ni').last;
    for (var i = 0; i < 50; i++) {
      engine.learn(second);
    }
    expect(engine.lookup('ni').any((c) => c.text == second.text), isTrue);
    expect(engine.lookup(''), isEmpty);
    expect(engine.lookup('a' * 100).length, lessThanOrEqualTo(60));
  });
}
