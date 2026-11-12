// SPDX-License-Identifier: GPL-3.0-only
import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:flutter/foundation.dart';

import 'model.dart';

class PreferenceController extends ChangeNotifier {
  PreferenceController({Directory? directory})
    : directory =
          directory ??
          Directory(
            '${Platform.environment['XDG_CONFIG_HOME'] ?? '${Platform.environment['HOME']}/.config'}/win10x',
          );

  static final instance = PreferenceController();
  final Directory directory;
  ShellPreferences value = ShellPreferences();
  bool loaded = false;
  String? error;
  Future<void>? _loading;
  Future<void> _writes = Future.value();
  File get file => File('${directory.path}/flutter-shell.json');

  Future<Map<String, dynamic>> _read(File input) async => await input.exists()
      ? Map<String, dynamic>.from(jsonDecode(await input.readAsString()) as Map)
      : {};

  Future<void> load() => _loading ??= _load();
  Future<void> _load() async {
    try {
      if (await file.exists()) {
        final json = await _read(file);
        if (json['version'] != 1) {
          throw const FormatException('Unsupported Win10X settings version');
        }
        value = ShellPreferences.fromJson(json);
      } else {
        // The previous Rust shell used HOME even with a custom XDG_CONFIG_HOME.
        final legacy = Directory(
          '${Platform.environment['HOME']}/.config/win10x',
        );
        value = ShellPreferences.fromJson(
          await _read(File('${legacy.path}/shell.json')),
          legacyTiles: await _read(File('${legacy.path}/start_tiles.json')),
        );
      }
      loaded = true;
    } catch (failure) {
      // Refuse to overwrite an unreadable/future document with fresh defaults.
      error = failure.toString();
    }
    notifyListeners();
  }

  void change(void Function(ShellPreferences) edit) {
    if (!loaded) return;
    edit(value);
    final content =
        '${const JsonEncoder.withIndent('  ').convert(value.toJson())}\n';
    notifyListeners();
    // Serialize writes: an older asynchronous save must never win a race.
    _writes = _writes.then((_) async {
      try {
        await directory.create(recursive: true);
        final temporary = File('${file.path}.tmp');
        await temporary.writeAsString(content, flush: true);
        await temporary.rename(file.path);
        error = null;
      } catch (failure) {
        error = failure.toString();
      }
      notifyListeners();
    });
  }

  Future<void> flush() => _writes;
}
