// SPDX-License-Identifier: GPL-3.0-only
/// A complete, balanced native chord. Modifiers are held only for this stroke.
class PcKeyStroke {
  const PcKeyStroke(this.key, [this.modifiers = const []]);
  final String key;
  final List<String> modifiers;
}

class PcKey {
  const PcKey(this.label, this.key, [this.width = 1]);
  final String label, key;
  final double width;
}

enum KeyboardView { typing, editing, computer, numpad }

class KeyboardTools {
  KeyboardView view = KeyboardView.typing;
  final Map<String, bool> modifiers = {};
  bool selecting = false, byWord = false, fn = false, caps = false;
  void reset() {
    view = KeyboardView.typing;
    modifiers.clear();
    selecting = byWord = fn = caps = false;
  }

  void modifier(String key, {bool lock = false}) {
    if (lock) {
      modifiers[key] = true;
    } else if (modifiers.containsKey(key)) {
      modifiers.remove(key);
    } else {
      modifiers[key] = false;
    }
  }

  PcKeyStroke stroke(String key, {bool direction = false}) {
    if (fn) {
      key =
          const {
            '1': 'F1',
            '2': 'F2',
            '3': 'F3',
            '4': 'F4',
            '5': 'F5',
            '6': 'F6',
            '7': 'F7',
            '8': 'F8',
            '9': 'F9',
            '0': 'F10',
            'minus': 'F11',
            'equal': 'F12',
            'Left': 'Home',
            'Right': 'End',
            'Up': 'Page_Up',
            'Down': 'Page_Down',
            'BackSpace': 'Delete',
          }[key] ??
          key;
    }
    final held = modifiers.keys.toSet();
    if (direction && selecting) held.add('Shift_L');
    if (direction && byWord) held.add('Control_L');
    if (caps && RegExp(r'^[a-z]$').hasMatch(key)) {
      if (held.contains('Shift_L') || held.contains('Shift_R')) {
        held.remove('Shift_L');
        held.remove('Shift_R');
      } else {
        held.add('Shift_L');
      }
    }
    modifiers.removeWhere((_, locked) => !locked);
    return PcKeyStroke(key, List.unmodifiable(held));
  }

  PcKeyStroke command(String command) {
    final key = const {
      '全选': 'a',
      '剪切': 'x',
      '复制': 'c',
      '粘贴': 'v',
      '撤销': 'z',
      '重做': 'y',
    }[command]!;
    modifiers.clear();
    selecting = command == '全选';
    return command == '重做'
        ? const PcKeyStroke('z', ['Control_L', 'Shift_L'])
        : PcKeyStroke(key, const ['Control_L']);
  }

  static const rows = <List<PcKey>>[
    [
      PcKey('Esc', 'Escape'),
      PcKey('F1', 'F1'),
      PcKey('F2', 'F2'),
      PcKey('F3', 'F3'),
      PcKey('F4', 'F4'),
      PcKey('F5', 'F5'),
      PcKey('F6', 'F6'),
      PcKey('F7', 'F7'),
      PcKey('F8', 'F8'),
      PcKey('F9', 'F9'),
      PcKey('F10', 'F10'),
      PcKey('F11', 'F11'),
      PcKey('F12', 'F12'),
      PcKey('PrtSc', 'Print'),
      PcKey('ScrLk', 'Scroll_Lock'),
      PcKey('Pause', 'Pause'),
    ],
    [
      PcKey('`', 'grave'),
      PcKey('1', '1'),
      PcKey('2', '2'),
      PcKey('3', '3'),
      PcKey('4', '4'),
      PcKey('5', '5'),
      PcKey('6', '6'),
      PcKey('7', '7'),
      PcKey('8', '8'),
      PcKey('9', '9'),
      PcKey('0', '0'),
      PcKey('-', 'minus'),
      PcKey('=', 'equal'),
      PcKey('⌫', 'BackSpace', 2),
      PcKey('Ins', 'Insert'),
      PcKey('Home', 'Home'),
      PcKey('PgUp', 'Page_Up'),
    ],
    [
      PcKey('Tab', 'Tab', 1.5),
      PcKey('q', 'q'),
      PcKey('w', 'w'),
      PcKey('e', 'e'),
      PcKey('r', 'r'),
      PcKey('t', 't'),
      PcKey('y', 'y'),
      PcKey('u', 'u'),
      PcKey('i', 'i'),
      PcKey('o', 'o'),
      PcKey('p', 'p'),
      PcKey('[', 'bracketleft'),
      PcKey(']', 'bracketright'),
      PcKey('\\', 'backslash', 1.5),
      PcKey('Del', 'Delete'),
      PcKey('End', 'End'),
      PcKey('PgDn', 'Page_Down'),
    ],
    [
      PcKey('Caps', 'caps', 1.8),
      PcKey('a', 'a'),
      PcKey('s', 's'),
      PcKey('d', 'd'),
      PcKey('f', 'f'),
      PcKey('g', 'g'),
      PcKey('h', 'h'),
      PcKey('j', 'j'),
      PcKey('k', 'k'),
      PcKey('l', 'l'),
      PcKey(';', 'semicolon'),
      PcKey("'", 'apostrophe'),
      PcKey('Enter', 'Return', 2.2),
    ],
    [
      PcKey('Shift', 'Shift_L', 2.3),
      PcKey('z', 'z'),
      PcKey('x', 'x'),
      PcKey('c', 'c'),
      PcKey('v', 'v'),
      PcKey('b', 'b'),
      PcKey('n', 'n'),
      PcKey('m', 'm'),
      PcKey(',', 'comma'),
      PcKey('.', 'period'),
      PcKey('/', 'slash'),
      PcKey('Shift', 'Shift_R', 2.3),
      PcKey('↑', 'Up'),
    ],
    [
      PcKey('Fn', 'fn'),
      PcKey('Ctrl', 'Control_L', 1.3),
      PcKey('Win', 'Super_L'),
      PcKey('Alt', 'Alt_L'),
      PcKey('空格', 'space', 6),
      PcKey('AltGr', 'Alt_R', 1.3),
      PcKey('Ctrl', 'Control_R', 1.3),
      PcKey('←', 'Left'),
      PcKey('↓', 'Down'),
      PcKey('→', 'Right'),
    ],
  ];
  static const numpad = <List<PcKey>>[
    [
      PcKey('Num', 'Num_Lock'),
      PcKey('/', 'KP_Divide'),
      PcKey('*', 'KP_Multiply'),
      PcKey('-', 'KP_Subtract'),
    ],
    [
      PcKey('7', 'KP_7'),
      PcKey('8', 'KP_8'),
      PcKey('9', 'KP_9'),
      PcKey('+', 'KP_Add'),
    ],
    [
      PcKey('4', 'KP_4'),
      PcKey('5', 'KP_5'),
      PcKey('6', 'KP_6'),
      PcKey('+', 'KP_Add'),
    ],
    [
      PcKey('1', 'KP_1'),
      PcKey('2', 'KP_2'),
      PcKey('3', 'KP_3'),
      PcKey('Enter', 'KP_Enter'),
    ],
    [PcKey('0', 'KP_0', 2), PcKey('.', 'KP_Decimal'), PcKey('电脑', 'computer')],
  ];
}

/// Fractional distance survives direction reversals without snapping or a
/// release-generated Space. One arrow step per 12 logical pixels.
class SpaceCursorDrag {
  double distance = 0;
  bool active = false;
  int update(double delta) {
    distance += delta;
    if (!active && distance.abs() < 12) return 0;
    active = true;
    final steps = (distance / 12).truncate();
    distance -= steps * 12;
    return steps;
  }
}
