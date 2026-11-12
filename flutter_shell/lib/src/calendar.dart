// SPDX-License-Identifier: GPL-3.0-only
import 'markui_icons.dart';

import 'package:flutter/material.dart';

import 'controls.dart';
import 'fluent.dart';

class Win10xCalendar extends StatefulWidget {
  const Win10xCalendar({super.key});
  @override
  State<Win10xCalendar> createState() => _CalendarState();
}

class _CalendarState extends State<Win10xCalendar> {
  DateTime _month = DateTime(DateTime.now().year, DateTime.now().month);
  DateTime _selected = DateTime.now();
  @override
  Widget build(BuildContext context) {
    final today = DateTime.now();
    final first = DateTime(_month.year, _month.month, 1);
    final start = first.subtract(Duration(days: first.weekday - 1));
    return Material(
      color: Win10xFluent.chrome,
      child: SizedBox(
        width: 340,
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Align(
                alignment: Alignment.centerLeft,
                child: Text(
                  '${today.hour.toString().padLeft(2, '0')}:${today.minute.toString().padLeft(2, '0')}',
                  style: const TextStyle(
                    fontSize: 36,
                    fontWeight: FontWeight.w300,
                  ),
                ),
              ),
              Align(
                alignment: Alignment.centerLeft,
                child: TextButton(
                  onPressed: () => setState(
                    () => _month = DateTime(today.year, today.month),
                  ),
                  child: Text('${today.year}年${today.month}月${today.day}日'),
                ),
              ),
              const Divider(),
              Row(
                children: [
                  Expanded(child: Text('${_month.year}年${_month.month}月')),
                  FluentIconButton(
                    tooltip: '上个月',
                    onPressed: () => setState(
                      () => _month = DateTime(_month.year, _month.month - 1),
                    ),
                    icon: const MarkUiIcon(Icons.keyboard_arrow_up),
                  ),
                  FluentIconButton(
                    tooltip: '下个月',
                    onPressed: () => setState(
                      () => _month = DateTime(_month.year, _month.month + 1),
                    ),
                    icon: const MarkUiIcon(Icons.keyboard_arrow_down),
                  ),
                ],
              ),
              Row(
                children: [
                  for (final label in ['一', '二', '三', '四', '五', '六', '日'])
                    Expanded(child: Center(child: Text(label))),
                ],
              ),
              const SizedBox(height: 8),
              for (var row = 0; row < 6; row++)
                Row(
                  children: [
                    for (var col = 0; col < 7; col++)
                      Expanded(
                        child: Builder(
                          builder: (context) {
                            final date = start.add(
                              Duration(days: row * 7 + col),
                            );
                            final current =
                                date.year == today.year &&
                                date.month == today.month &&
                                date.day == today.day;
                            final selected =
                                date.year == _selected.year &&
                                date.month == _selected.month &&
                                date.day == _selected.day;
                            return SizedBox(
                              height: 38,
                              child: InkWell(
                                onTap: () => setState(() => _selected = date),
                                child: Container(
                                  alignment: Alignment.center,
                                  decoration: BoxDecoration(
                                    color: current ? Win10xFluent.accent : null,
                                    border: Border.all(
                                      color: selected
                                          ? Win10xFluent.accent
                                          : Colors.transparent,
                                    ),
                                  ),
                                  child: Text(
                                    '${date.day}',
                                    style: TextStyle(
                                      color: current
                                          ? Colors.white
                                          : date.month == _month.month
                                          ? Win10xFluent.text
                                          : Win10xFluent.textTertiary,
                                    ),
                                  ),
                                ),
                              ),
                            );
                          },
                        ),
                      ),
                  ],
                ),
            ],
          ),
        ),
      ),
    );
  }
}
