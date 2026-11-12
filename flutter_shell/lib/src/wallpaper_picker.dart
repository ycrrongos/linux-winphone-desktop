// SPDX-License-Identifier: GPL-3.0-only
import 'dart:io';
import 'dart:ui' as ui;

import 'package:flutter/material.dart';

import 'fluent_controls.dart';

class WallpaperPicker extends StatefulWidget {
  const WallpaperPicker({
    super.key,
    required this.onPicked,
    required this.onClose,
  });
  final ValueChanged<String> onPicked;
  final VoidCallback onClose;
  @override
  State<WallpaperPicker> createState() => _WallpaperPickerState();
}

class _WallpaperPickerState extends State<WallpaperPicker> {
  late final path = TextEditingController(
    text: '${Platform.environment['HOME'] ?? '/home'}/Pictures',
  );
  late Future<List<FileSystemEntity>> entries = list(path.text);
  String? error;
  bool loading = false;
  Future<List<FileSystemEntity>> list(String name) async {
    final items = await Directory(name)
        .list(followLinks: false)
        .take(500)
        .toList();
    items.removeWhere(
      (p) =>
          p is! Directory &&
          !RegExp(
            r'\.(png|jpe?g|webp|bmp)$',
            caseSensitive: false,
          ).hasMatch(p.path),
    );
    items.sort(
      (a, b) => a is Directory && b is! Directory
          ? -1
          : b is Directory && a is! Directory
          ? 1
          : a.path.compareTo(b.path),
    );
    return items;
  }

  void open(String name) => setState(() {
    path.text = name;
    error = null;
    entries = list(name);
  });
  Future<void> choose(String name) async {
    if (await Directory(name).exists()) {
      open(name);
      return;
    }
    if (loading) return;
    setState(() {
      loading = true;
      error = null;
    });
    try {
      final file = File(name);
      if (await file.length() > 64 * 1024 * 1024) {
        throw const FormatException('图片文件超过64MB');
      }
      final codec = await ui.instantiateImageCodec(
        await file.readAsBytes(),
        targetWidth: 64,
      );
      final frame = await codec.getNextFrame();
      frame.image.dispose();
      codec.dispose();
      if (mounted) widget.onPicked(file.absolute.path);
    } on Object {
      if (mounted) setState(() => error = '无法读取图片，请选择 PNG、JPEG、WebP 或 BMP 文件');
    } finally {
      if (mounted) setState(() => loading = false);
    }
  }

  @override
  void dispose() {
    path.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => Material(
    color: Theme.of(context).colorScheme.surface,
    child: Padding(
      padding: const EdgeInsets.fromLTRB(20, 36, 20, 48),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Row(
            children: [
              Expanded(
                child: Text(
                  '选择壁纸',
                  style: Theme.of(context).textTheme.headlineSmall,
                ),
              ),
              TextButton(onPressed: widget.onClose, child: const Text('取消')),
            ],
          ),
          const SizedBox(height: 12),
          const Text('目录或图片路径'),
          const SizedBox(height: 6),
          TextField(
            controller: path,
            onSubmitted: choose,
            decoration: const InputDecoration(hintText: '输入目录或图片的完整路径'),
          ),
          Wrap(
            children: [
              TextButton(
                onPressed: () => open(Directory(path.text).parent.path),
                child: const Text('上一级'),
              ),
              TextButton(
                onPressed: () => open(Platform.environment['HOME'] ?? '/home'),
                child: const Text('主目录'),
              ),
              TextButton(
                onPressed: () => open('/usr/share/backgrounds'),
                child: const Text('系统图片'),
              ),
              TextButton(
                onPressed: loading ? null : () => choose(path.text),
                child: Text(loading ? '正在读取…' : '打开 / 使用'),
              ),
            ],
          ),
          if (error != null)
            Text(error!, style: const TextStyle(color: Color(0xffc42b1c))),
          Expanded(
            child: FutureBuilder<List<FileSystemEntity>>(
              future: entries,
              builder: (context, snapshot) {
                if (snapshot.hasError) {
                  return const Text('目录不可用，可输入其它目录或图片的完整路径');
                }
                if (!snapshot.hasData) {
                  return const Center(child: CircularProgressIndicator());
                }
                if (snapshot.data!.isEmpty) return const Text('此目录没有图片');
                return ListView(
                  children: [
                    for (final entry in snapshot.data!)
                      FluentMenuItem(
                        label: entry.path.split('/').last,
                        subtitle: entry is Directory ? '文件夹' : null,
                        onTap: () => choose(entry.path),
                      ),
                  ],
                );
              },
            ),
          ),
        ],
      ),
    ),
  );
}
