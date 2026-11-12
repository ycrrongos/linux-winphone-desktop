import 'package:test/test.dart';
import 'package:win10x_shell/src/phone_lock_model.dart';

void main() {
  test(
    'swipe reveal uses travel and direction; an upward gesture never unlocks',
    () {
      expect(revealLockCredentials(.4, 0), isTrue);
      expect(revealLockCredentials(.05, -700), isTrue);
      expect(revealLockCredentials(.1, 0), isFalse);
      expect(revealLockCredentials(.4, 700), isFalse);
      expect(revealLockCredentials(.28, 0), isTrue);
    },
  );
}
