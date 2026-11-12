// SPDX-License-Identifier: GPL-3.0-only
import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:isolate';

import 'pinyin_engine.dart';

void _decoder(List args) {
  final reply = args[0] as SendPort;
  final engine = PinyinEngine(args[1] as String);
  engine.personal.addAll(Map<String, int>.from(args[2] as Map));
  final inbox = ReceivePort();
  reply.send(inbox.sendPort);
  inbox.listen((dynamic message) {
    final m = message as List;
    if (m[0] == 'query') {
      reply.send([
        m[1],
        engine.lookup(m[2] as String).map((c) => c.toWire()).toList(),
      ]);
    }
    if (m[0] == 'learn') engine.learn(PinyinChoice.fromWire(m[1] as List));
    if (m[0] == 'clear') engine.personal.clear();
  });
}

/// Decoder and mutable dictionary indexes live on a single bounded worker.
class PinyinSession {
  ReceivePort? _receive;
  Isolate? _isolate;
  SendPort? _send;
  final _pending = <int, Completer<List<PinyinChoice>>>{};
  final _learned = <String, int>{};
  int _serial = 0;
  Timer? _save;
  Future<void> _writes = Future.value();
  bool chinese = false, learning = true, _closed = false, _restored = false;
  final ready = Completer<void>();
  File get file => File(
    '${Platform.environment['XDG_STATE_HOME'] ?? '${Platform.environment['HOME']}/.local/state'}/win10x/ime/personal.json',
  );
  Future<void> initialize(String dictionary) async {
    try {
      if (await file.exists() && await file.length() < 1024 * 1024) {
        final json = jsonDecode(await file.readAsString()) as Map;
        chinese = json['chinese'] == true;
        learning = json['learning'] != false;
        final words = json['words'];
        if (words is Map) {
          for (final e in words.entries.take(4000)) {
            if (e.key is String && e.value is int && e.value > 0) {
              _learned[e.key as String] = (e.value as int).clamp(1, 10000);
            }
          }
        }
      }
    } on Object {
      /* A corrupt user dictionary must not disable typing. */
    }
    _restored = true;
    if (_closed) return;
    _receive = ReceivePort();
    _receive!.listen((dynamic message) {
      if (message is SendPort) {
        _send = message;
        if (!ready.isCompleted) ready.complete();
      } else if (message is List) {
        final task = _pending.remove(message[0]);
        task?.complete(
          (message[1] as List)
              .map((c) => PinyinChoice.fromWire(c as List))
              .toList(),
        );
      }
    });
    _isolate = await Isolate.spawn(_decoder, [
      _receive!.sendPort,
      dictionary,
      _learned,
    ]);
    if (_closed) _isolate?.kill(priority: Isolate.immediate);
  }

  Future<List<PinyinChoice>> query(String value) async {
    await ready.future;
    if (_closed) return [];
    // The UI debounces; discard older awaiting results instead of building a queue.
    for (final c in _pending.values) {
      if (!c.isCompleted) c.complete([]);
    }
    _pending.clear();
    final id = ++_serial, task = Completer<List<PinyinChoice>>();
    _pending[id] = task;
    _send!.send(['query', id, value]);
    return task.future;
  }

  void learn(PinyinChoice choice) {
    if (!learning || _closed) return;
    final key = '${choice.code}\t${choice.text}';
    _learned[key] = ((_learned[key] ?? 0) + 1).clamp(1, 10000);
    if (_learned.length > 4000) _learned.remove(_learned.keys.first);
    _send?.send(['learn', choice.toWire()]);
    save();
  }

  void save() {
    _save?.cancel();
    _save = Timer(const Duration(milliseconds: 600), () => _persist());
  }

  void _persist() {
    if (!_restored) return;
    final payload = jsonEncode({
      'version': 1,
      'chinese': chinese,
      'learning': learning,
      'words': _learned,
    });
    _writes = _writes.then((_) async {
      try {
        await file.parent.create(recursive: true);
        if ((await Process.run('chmod', ['700', file.parent.path])).exitCode !=
            0) {
          return;
        }
        final temp = File('${file.path}.tmp');
        await temp.writeAsString(payload, flush: true);
        if ((await Process.run('chmod', ['600', temp.path])).exitCode != 0) {
          await temp.delete();
          return;
        }
        await temp.rename(file.path);
      } on Object {
        /* Do not log composing/learned text. */
      }
    });
  }

  void dispose() {
    _closed = true;
    if (!ready.isCompleted) ready.complete();
    _save?.cancel();
    _persist();
    _receive?.close();
    _isolate?.kill(priority: Isolate.immediate);
    for (final c in _pending.values) {
      if (!c.isCompleted) c.complete([]);
    }
    _pending.clear();
  }
}
