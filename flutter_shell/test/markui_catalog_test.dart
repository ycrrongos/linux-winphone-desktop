// SPDX-License-Identifier: GPL-3.0-only
import 'package:test/test.dart';
import 'package:win10x_shell/src/markui_catalog.dart';

void main() {
  test(
    'desktop paths, case and Flatpak ids resolve the same MarkUI artwork',
    () {
      expect(
        markUiApplicationAsset('/usr/share/applications/Firefox-ESR.desktop'),
        'browser',
      );
      expect(markUiApplicationAsset('org.mozilla.firefox'), 'browser');
      expect(markUiApplicationAsset('org.kde.dolphin.desktop'), 'files');
      expect(markUiApplicationAsset('kcm_networkmanagement'), 'settings');
    },
  );
  test(
    'Waydroid applications map by identity rather than generic Waydroid icon',
    () {
      expect(
        markUiApplicationAsset('waydroid.org.lineageos.aperture.desktop'),
        'camera',
      );
      expect(
        markUiApplicationAsset('waydroid.com.google.android.apps.messaging'),
        'messages',
      );
      expect(
        markUiApplicationAsset('waydroid.com.android.documentsui'),
        'files',
      );
    },
  );
  test('real Denial opaque desktop/local IDs resolve like window app IDs', () {
    expect(markUiApplicationAsset('desktop:firefox.desktop'), 'browser');
    expect(markUiApplicationAsset('desktop:org.kde.dolphin.desktop'), 'files');
    expect(markUiApplicationAsset('local:dev.denial.Settings'), 'settings');
    expect(
      markUiApplicationAsset(
        'desktop:/usr/share/applications/org.kde.kate.desktop',
      ),
      'notes',
    );
  });
  test('unrecognized third party identity keeps its own icon', () {
    expect(markUiApplicationAsset('com.example.notfirefox'), isNull);
    expect(markUiApplicationAsset('unknown.settings.clone'), isNull);
  });
}
