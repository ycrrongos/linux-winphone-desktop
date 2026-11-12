// SPDX-License-Identifier: GPL-3.0-only
/// Geometry shared by the list and its fast index, including real list padding.
class CatalogIndex {
  CatalogIndex(
    Map<String, int> counts, {
    required this.rowHeight,
    this.headerHeight = 48,
    this.padding = 8,
  }) {
    var offset = padding;
    for (final letter in [
      '#',
      ...List.generate(26, (i) => String.fromCharCode(65 + i)),
    ]) {
      final count = counts[letter] ?? 0;
      if (count == 0) continue;
      offsets[letter] = offset;
      offset += headerHeight + count * rowHeight;
    }
    contentExtent = offset + padding;
  }
  late final double contentExtent;
  final double rowHeight, headerHeight, padding;
  final offsets = <String, double>{};
  List<String> get letters => offsets.keys.toList(growable: false);
  String? letterAt(double offset) {
    String? result;
    for (final entry in offsets.entries) {
      if (entry.value > offset + padding) break;
      result = entry.key;
    }
    return result ?? offsets.keys.firstOrNull;
  }

  String? letterAtRail(double y, double height) {
    final keys = letters;
    if (keys.isEmpty || height <= 0) return null;
    return keys[((y / height) * keys.length).floor().clamp(0, keys.length - 1)];
  }
}
