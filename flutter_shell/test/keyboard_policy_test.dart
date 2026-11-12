import 'package:test/test.dart';
import 'package:win10x_shell/src/keyboard_policy.dart';

void main() {
  test(
    'phone opens enabled editor without a separate show hint and stays open',
    () {
      final p = TouchKeyboardPolicy();
      expect(
        p.update(
          active: true,
          requested: false,
          serial: 1,
          allowAutofocus: true,
        ),
        isTrue,
      );
      expect(
        p.update(
          active: true,
          requested: false,
          serial: 1,
          allowAutofocus: true,
        ),
        isTrue,
      );
      expect(
        p.update(
          active: false,
          requested: false,
          serial: 1,
          allowAutofocus: true,
        ),
        isFalse,
      );
    },
  );
  test('manual hide wins over stale updates; new activation can reopen', () {
    final p = TouchKeyboardPolicy();
    p.update(active: true, requested: true, serial: 1, allowAutofocus: true);
    p.dismiss(1);
    expect(
      p.update(active: true, requested: true, serial: 1, allowAutofocus: true),
      isFalse,
    );
    expect(
      p.update(active: true, requested: false, serial: 2, allowAutofocus: true),
      isTrue,
    );
    p.dismiss(2);
    p.update(active: false, requested: false, serial: 2, allowAutofocus: true);
    expect(
      p.update(active: true, requested: false, serial: 2, allowAutofocus: true),
      isTrue,
    );
  });
  test('desktop respects the native show request', () {
    final p = TouchKeyboardPolicy();
    expect(
      p.update(
        active: true,
        requested: false,
        serial: 1,
        allowAutofocus: false,
      ),
      isFalse,
    );
    expect(
      p.update(active: true, requested: true, serial: 1, allowAutofocus: false),
      isTrue,
    );
  });
}
