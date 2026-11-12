// SPDX-License-Identifier: GPL-3.0-only
import 'dart:math' as math;

class PinyinChoice {
  const PinyinChoice(this.text, this.code, this.consumed, this.score);
  final String text, code;
  final int consumed;
  final double score;
  List<Object> toWire() => [text, code, consumed, score];
  factory PinyinChoice.fromWire(List value) => PinyinChoice(
    value[0] as String,
    value[1] as String,
    value[2] as int,
    (value[3] as num).toDouble(),
  );
}

String normalizePinyin(String raw) => raw
    .toLowerCase()
    .replaceAll('u:', 'v')
    .replaceAll('ü', 'v')
    .replaceAll(RegExp("[^a-zv']"), '');

class _Entry {
  const _Entry(this.text, this.code, this.weight);
  final String text, code;
  final int weight;
  double get score => math.log(weight + 1);
}

/// Bounded offline decoder: full spelling, initials, incomplete last syllable,
/// prefix selection, and dictionary-word sentence composition. Runs off UI thread.
class PinyinEngine {
  PinyinEngine(String source) {
    for (final line in source.split('\n')) {
      if (line.startsWith('#')) continue;
      final fields = line.split('\t');
      if (fields.length < 2) continue;
      final code = fields[1].trim().replaceAll(' ', "'");
      if (!RegExp("^[a-zv]+('[a-zv]+)*\$").hasMatch(code)) continue;
      final weight = int.tryParse(fields.length > 2 ? fields[2] : '0') ?? 0;
      final e = _Entry(fields[0], code, weight);
      (_full[code.replaceAll("'", '')] ??= []).add(e);
      final syllables = code.split("'");
      if (syllables.length > 1) {
        (_initials[syllables.map((s) => s[0]).join()] ??= []).add(e);
      }
    }
    for (final rows in [..._full.values, ..._initials.values]) {
      rows.sort((a, b) => b.weight.compareTo(a.weight));
    }
    _keys = _full.keys.toList()..sort();
  }
  final _full = <String, List<_Entry>>{}, _initials = <String, List<_Entry>>{};
  late final List<String> _keys;
  final personal = <String, int>{};
  int get entries => _full.values.fold(0, (n, l) => n + l.length);
  double _score(_Entry e) =>
      e.score + math.log(1 + (personal['${e.code}\t${e.text}'] ?? 0)) * 6;
  void learn(PinyinChoice choice) {
    final key = '${choice.code}\t${choice.text}';
    personal[key] = math.min(10000, (personal[key] ?? 0) + 1);
    if (personal.length > 4000) personal.remove(personal.keys.first);
  }

  List<PinyinChoice> lookup(String raw, {int limit = 60}) {
    final marked = normalizePinyin(raw)
        .substring(0, math.min(64, normalizePinyin(raw).length));
    final input = marked.replaceAll("'", '');
    if (input.isEmpty) return [];
    final choices = <PinyinChoice>[];
    void add(_Entry e, int count, double bonus) {
      // Explicit apostrophes disambiguate xi'an from xian, not vice versa.
      if (marked.contains("'") &&
          count == input.length &&
          !e.code.startsWith(marked)) {
        return;
      }
      choices.add(PinyinChoice(e.text, e.code, count, bonus + _score(e)));
    }

    final full = _full[input] ?? const <_Entry>[];
    for (final e in full.take(80)) {
      add(e, input.length, 1000);
    }
    if (!marked.contains("'")) {
      for (final e in (_initials[input] ?? const <_Entry>[]).take(40)) {
        add(e, input.length, 900);
      }
    }
    // At most64 prefix keys, found by binary search instead of scanning65k rows.
    var lo = 0, hi = _keys.length;
    while (lo < hi) {
      final mid = (lo + hi) ~/ 2;
      if (_keys[mid].compareTo(input) < 0) {
        lo = mid + 1;
      } else {
        hi = mid;
      }
    }
    for (
      var k = lo;
      k < _keys.length && k < lo + 64 && _keys[k].startsWith(input);
      k++
    ) {
      if (_keys[k] == input) continue;
      for (final e in _full[_keys[k]]!.take(4)) {
        add(e, input.length, 700);
      }
    }
    // Best segmentation for text not present as one dictionary phrase.
    final score = List<double>.filled(input.length + 1, -double.infinity)
      ..[0] = 0;
    final phrase = List<String>.filled(input.length + 1, ''),
        codes = List<String>.filled(input.length + 1, '');
    for (var end = 1; end <= input.length; end++) {
      for (var start = math.max(0, end - 24); start < end; start++) {
        if (!score[start].isFinite) continue;
        final rows = _full[input.substring(start, end)];
        if (rows == null) continue;
        final best = rows
            .take(8)
            .reduce((a, b) => _score(a) > _score(b) ? a : b);
        final candidate = score[start] + _score(best) - 15;
        if (candidate > score[end]) {
          score[end] = candidate;
          phrase[end] = phrase[start] + best.text;
          codes[end] = codes[start].isEmpty
              ? best.code
              : '${codes[start]}\'${best.code}';
        }
      }
    }
    if (phrase.last.isNotEmpty && !marked.contains("'")) {
      choices.add(
        PinyinChoice(phrase.last, codes.last, input.length, 950 + score.last),
      );
    }
    for (var n = input.length - 1; n > 0; n--) {
      for (final e in (_full[input.substring(0, n)] ?? const <_Entry>[]).take(
        5,
      )) {
        add(e, n, 100 + n * 5);
      }
    }
    choices.sort((a, b) => b.score.compareTo(a.score));
    final seen = <String>{};
    return choices
        .where((c) => seen.add('${c.text}\t${c.consumed}'))
        .take(limit)
        .toList();
  }
}
