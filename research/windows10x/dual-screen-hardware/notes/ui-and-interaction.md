# 双屏硬件 UI 与操作逻辑（后续阶段）

## 资料边界

这部分主要来自 2019 年微软概念演示、2020 年 Windows 10X Emulator 开发者预览，以及后续 Surface Duo 设计资料。它们不能代表正式发布产品，而且不属于当前单屏版本范围。

## 双屏体验的三类价值

- Expansive workspaces：一个应用扩展到更大画布。
- Focused screens：内容和工具分别占据两个明确区域。
- Connected apps：两个应用并排工作、跨应用拖放且不丢上下文。

## 五种常见布局模式

- Extended Canvas：地图、画布、表格等跨越更大区域，但重要内容不能落在铰链或遮挡区。
- List–Detail：一侧列表，一侧详情；窄屏可折叠为“列表 → 详情 → 返回”。
- Two Page：书籍、杂志式双页阅读。
- Companion Pane：主内容与工具或辅助信息分离。
- Dual View：同一数据的两个视角，例如地图与列表、编辑与预览。

## 硬件相关操作

- 用户可主动拖动窗口跨越两块屏幕。
- 系统根据屏幕方向、铰链和设备姿态重排应用。
- 跨应用拖放用于连接两个同时可见的任务上下文。
- 硬件键盘覆盖一部分屏幕后，剩余区域可成为 Wonder Bar，承载虚拟触控板、输入加速器、画中画和媒体控制。

## 图片

- `dual-screen-postures.png`：不同设备姿态。
- `dual-screen-five-patterns.png`：五种常见双屏模式。
- `dual-screen-pattern-navigation.png`：模式间的示例导航流程。
- `responsive-list-detail-phone-tablet-dual.png`：List–Detail 在手机、平板和双屏上的变化。
- `dual-screen-navigation-reachability.png`：跨屏导航组件的触控可达性。

