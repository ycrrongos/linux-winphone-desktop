// SPDX-License-Identifier: GPL-3.0-only
import 'package:test/test.dart';
import 'package:win10x_shell/src/catalog_index.dart';
import 'package:win10x_shell/src/phone_tasks_model.dart';

void main() {
  test(
    'alphabet targets include actual padding and headers, not a guessed offset',
    () {
      final index = CatalogIndex({
        'Z': 2,
        'A': 3,
        '#': 1,
        'C': 5,
      }, rowHeight: 56);
      expect(index.letters, ['#', 'A', 'C', 'Z']);
      expect(index.offsets['A'], 112);
      expect(index.offsets['C'], 328);
      expect(index.offsets['Z'], 656);
      expect(index.contentExtent, 824);
      expect(index.letterAt(328), 'C');
      expect(index.letterAt(0), '#');
      expect(index.letterAtRail(-50, 400), '#');
      expect(index.letterAtRail(101, 400), 'A');
      expect(index.letterAtRail(900, 400), 'Z');
      expect(CatalogIndex({}, rowHeight: 56).letterAtRail(2, 100), isNull);
    },
  );
  test('filtering catalog rebuilds offsets and drops absent sections', () {
    final index = CatalogIndex({'C': 1, 'F': 1}, rowHeight: 56);
    expect(index.letters, ['C', 'F']);
    expect(index.offsets['F'], 112);
    expect(index.offsets['A'], isNull);
  });
  test('task order focuses active once and preserves live cards through focus changes', () {
    final order = PhoneTaskOrder()..sync([1, 2, 3], 2);
    expect(order.ids, [2, 1, 3]);
    order.sync([1, 2, 3], 3);
    expect(order.ids, [2, 1, 3]);
    order.sync([1, 3, 4], 4);
    expect(order.ids, [1, 3, 4]);
    order.sync([], null);
    expect(order.ids, isEmpty);
  });
  test(
    'slow deliberate dismissal closes; short pull or upward reversal cancels',
    () {
      expect(dismissPhoneTask(distance: 90, height: 400, velocity: 0), isTrue);
      expect(
        dismissPhoneTask(distance: 20, height: 400, velocity: 700),
        isTrue,
      );
      expect(
        dismissPhoneTask(distance: 20, height: 400, velocity: 100),
        isFalse,
      );
      expect(
        dismissPhoneTask(distance: 100, height: 400, velocity: -500),
        isFalse,
      );
      expect(phoneTaskPageCapacity(446, 992), 4);
      expect(phoneTaskPageCapacity(390, 780), 1);
      expect(phoneTaskPageCapacity(780, 390), 1);
    },
  );
}
