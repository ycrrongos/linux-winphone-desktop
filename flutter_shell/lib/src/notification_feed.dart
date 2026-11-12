// SPDX-License-Identifier: GPL-3.0-only
import 'markui_icons.dart';

import 'package:denial_flutter_sdk/state.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'controls.dart';

class NotificationFeed extends ConsumerStatefulWidget {
  const NotificationFeed({super.key});
  @override
  ConsumerState<NotificationFeed> createState() => _NotificationFeedState();
}

class _NotificationFeedState extends ConsumerState<NotificationFeed> {
  final _expanded = <String>{};
  @override
  Widget build(BuildContext context) {
    final state = ref.watch(desktopNotificationsProvider);
    final controller = ref.read(desktopNotificationsProvider.notifier);
    if (state.groups.isEmpty) return const Center(child: Text('没有新通知'));
    return ListView(
      children: [
        for (final group in state.groups)
          Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              ListTile(
                title: Text(
                  group.newest.notification.appName,
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                ),
                trailing: group.clearable
                    ? FluentIconButton(
                        tooltip: '清除此组通知',
                        onPressed: () =>
                            controller.dismissGroupFromHistory(group.key),
                        icon: const MarkUiIcon(Icons.close),
                      )
                    : null,
              ),
              for (final record
                  in _expanded.contains(group.key)
                      ? group.members
                      : group.members.take(3))
                Dismissible(
                  key: ValueKey('notice.${record.notification.id}'),
                  direction: DismissDirection.horizontal,
                  confirmDismiss: (_) async =>
                      controller.dismissFromHistory(record.notification.id),
                  child: ListTile(
                    title: Text(record.notification.summary),
                    subtitle: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        if (record.notification.body.isNotEmpty)
                          Text(
                            record.notification.body,
                            maxLines: 4,
                            overflow: TextOverflow.ellipsis,
                          ),
                        if (record.notification.actions.any(
                          (action) => action.key != 'default',
                        ))
                          Wrap(
                            children: [
                              for (final action
                                  in record.notification.actions.where(
                                    (action) => action.key != 'default',
                                  ))
                                TextButton(
                                  onPressed: () => controller.invokeAction(
                                    record.notification.id,
                                    action.key,
                                  ),
                                  child: Text(action.label),
                                ),
                            ],
                          ),
                      ],
                    ),
                    onTap: () =>
                        controller.invokeDefaultAction(record.notification.id),
                  ),
                ),
              if (group.members.length > 3)
                TextButton(
                  onPressed: () => setState(() {
                    if (!_expanded.remove(group.key)) _expanded.add(group.key);
                  }),
                  child: Text(
                    _expanded.contains(group.key)
                        ? '收起'
                        : '展开 ${group.members.length - 3} 条通知',
                  ),
                ),
              const Divider(),
            ],
          ),
      ],
    );
  }
}
