import 'package:test/test.dart';
import 'package:win10x_shell/src/ime/keyboard_tools.dart';

void main() {
  test(
    'Fn remaps function and navigation keys, never sends fictional Fn key',
    () {
      final t = KeyboardTools()..fn = true;
      expect(t.stroke('1').key, 'F1');
      expect(t.stroke('equal').key, 'F12');
      expect(t.stroke('Left').key, 'Home');
      expect(t.stroke('Down').key, 'Page_Down');
    },
  );
  test('one shot and locked modifiers produce complete balanced chord specifications', () {
    final t = KeyboardTools();
    t.modifier('Control_L');
    expect(t.stroke('c').modifiers, ['Control_L']);
    expect(t.stroke('v').modifiers, isEmpty);
    t.modifier('Alt_L', lock: true);
    expect(t.stroke('Tab').modifiers, ['Alt_L']);
    expect(t.stroke('Tab').modifiers, ['Alt_L']);
    t.modifier('Alt_L');
    expect(t.stroke('Tab').modifiers, isEmpty);
  });
  test(
    'selection and word navigation combine Shift and Ctrl only for directions',
    () {
      final t = KeyboardTools()
        ..selecting = true
        ..byWord = true;
      expect(
        t.stroke('Left', direction: true).modifiers,
        containsAll(['Control_L', 'Shift_L']),
      );
      expect(t.stroke('a').modifiers, isEmpty);
      expect(t.command('复制').key, 'c');
      expect(t.selecting, isFalse);
      expect(t.command('全选').modifiers, ['Control_L']);
      expect(t.selecting, isTrue);
      expect(t.command('重做').key, 'z');
      expect(t.command('重做').modifiers, containsAll(['Control_L', 'Shift_L']));
    },
  );
  test(
    'caps with either shift lowers letters, does not change punctuation',
    () {
      final t = KeyboardTools()..caps = true;
      expect(t.stroke('a').modifiers, ['Shift_L']);
      t.modifier('Shift_R');
      expect(t.stroke('a').modifiers, isEmpty);
      expect(t.stroke('1').modifiers, isEmpty);
      t.reset();
      expect(t.view, KeyboardView.typing);
      expect(t.modifiers, isEmpty);
      expect(t.caps, isFalse);
    },
  );
  test('space drag keeps fractional distance and consumes release after a round trip', () {
    final d = SpaceCursorDrag();
    expect(d.update(5), 0);
    expect(d.active, isFalse);
    expect(d.update(8), 1);
    expect(d.active, isTrue);
    expect(d.update(12), 1);
    expect(d.update(-24), -1);
    expect(d.update(-2), -1);
    expect(d.active, isTrue);
  });
  test('computer layout includes function keys and actual editing/navigation/numpad keys', () {
    final keys = KeyboardTools.rows.expand((r) => r).map((s) => s.key).toSet();
    expect(
      keys,
      containsAll([
        'F1',
        'F12',
        'Control_L',
        'Alt_L',
        'Super_L',
        'Home',
        'End',
        'Page_Up',
        'Page_Down',
        'Insert',
        'Delete',
        'Escape',
        'Tab',
        'fn',
      ]),
    );
    expect(
      KeyboardTools.numpad.expand((r) => r).map((s) => s.key),
      containsAll(['KP_0', 'KP_Enter', 'KP_Add']),
    );
  });
}
