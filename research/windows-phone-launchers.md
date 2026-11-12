# Windows Phone 启动器与磁贴动画调研

最后核对：2026-08-18。

用途：集中保存可重复使用的 Windows Phone / Windows 10 Mobile 启动器、
动画源码位置、适用范围和许可证注意事项。这里记录的是“可借鉴什么”，
不表示第三方项目等同于微软原版；最终视觉判断仍应以原机录像逐帧为准。

## 结论速览

| 优先级 | 项目 | 类型 | 最适合借鉴 | 不适合直接当作什么 |
|---|---|---|---|---|
| 1 | [Disco Launcher](https://github.com/cherryhoax/DiscoLauncher) | Android 外壳 + HTML/CSS/JS，MIT | WP8/8.1 turnstile、磁贴内容层动画、Live Tile 动画词汇 | Windows 10 Mobile 原生开始屏幕的逐帧证明 |
| 2 | [MetroLauncherAndroid](https://github.com/lorddie/MetroLauncherAndroid) | Kotlin Android Launcher | W10M 风格布局、磁贴尺寸、通知/媒体 Live Tile | 开始屏幕入场动画；其实现只是向上位移淡入 |
| 3 | [Factor Launcher Reboot](https://github.com/Valkriaine/Factor_Launcher_Reboot) | Java/Kotlin Android Launcher | 简单、低成本的磁贴列表入场和通知磁贴 | 3D turnstile 或 W10M 复杂动效 |
| 补充 | [Retiled](https://github.com/DrewNaylor/Retiled) | Linux/QML 手机桌面 | Linux 手机形态、WP8.x 导航与布局方向 | 已验证的 Disco/W10M 磁贴入场参数 |
| Win8 补充 | [flutter_metro_ui](https://github.com/coldshineb/flutter_metro_ui) | Flutter 桌面演示，未声明许可证 | 点击磁贴矩形到全屏的插值、Y 轴翻转与按压倾斜 | Windows 8 原版完整启动动效；缺少开始屏幕后撤且飞出阶段不准确 |

目前没有找到开源项目完整实现“整行磁贴黏连、下方像给上方推力”的
Windows 10 Mobile 式波动入场。就能直接读到的源码而言，Disco 的实现是
逐磁贴 turnstile，并不是按整行作为一个刚体缩放。

## 1. Disco Launcher（当前试验基准）

- 仓库：[cherryhoax/DiscoLauncher](https://github.com/cherryhoax/DiscoLauncher)
- 本次核对提交：[`4907389`](https://github.com/cherryhoax/DiscoLauncher/commit/4907389358708754521bf62a5b2048823edcd12e)
- 许可证：[MIT](https://github.com/cherryhoax/DiscoLauncher/blob/4907389358708754521bf62a5b2048823edcd12e/LICENSE)
- 形态：Android Launcher，但主要界面和动效写在 WebView 的
  HTML/CSS/JavaScript 中；仓库也提供 `npm run debug:web`，因此它同时是
  当前找到的最好开源网页版动画参考。

### 返回/首次进入磁贴页

控制逻辑在
[`appTransition.js`](https://github.com/cherryhoax/DiscoLauncher/blob/4907389358708754521bf62a5b2048823edcd12e/src/scripts/appTransition.js)，
关键帧在
[`appTransition.scss`](https://github.com/cherryhoax/DiscoLauncher/blob/4907389358708754521bf62a5b2048823edcd12e/src/styles/appTransition.scss)：

1. 仅取可见磁贴，并把原布局顺序 `reverse()`；通常表现为右下角先动，
   然后逐块向左上扩散。
2. 索引被归一化到 `0..1`，全部错峰延迟压在约 `0.2s` 的窗口里；
   不是每增加一行就固定再等一段时间。
3. 外层磁贴以左边缘为旋转原点，持续 `0.5s`，缓动
   `cubic-bezier(.3, 1, .2, 1)`。源码起点包含 `rotateY(60deg)` 和
   额外 `rotateY(10deg)`；在前向入场的距离变量为 0 时可视作约 70°
   翻到 0°。
4. 内层内容独立持续 `0.35s`，由
   `translateX(60px) rotateY(45deg)` 回到原位，缓动
   `cubic-bezier(.2, .25, .25, 1)`。因此图标/文字不是焊死在磁贴平面
   上做一次简单缩放，而是更早、更快地二次回正。
5. 磁贴页父容器透视为 `calc(--flow-perspective * 2)`；默认
   `--flow-perspective: 1000px`，即约 2000px。见
   [`tileList.scss`](https://github.com/cherryhoax/DiscoLauncher/blob/4907389358708754521bf62a5b2048823edcd12e/src/styles/pages/tileList.scss)
   和
   [`flowTouch.scss`](https://github.com/cherryhoax/DiscoLauncher/blob/4907389358708754521bf62a5b2048823edcd12e/src/styles/flowTouch.scss)。

### 从磁贴启动应用

- 普通磁贴约 `0.175s`，所选磁贴约 `0.3s`。
- 普通磁贴仍使用归一化索引形成最多 `0.2s` 的错峰；所选磁贴不用该索引，
  固定延迟 `0.2s` 后再播放 `0.3s`，所以完整交接约 `0.5s`。
- 旋转原点在左侧，磁贴向屏幕左方移动约 `25vw`，并组合
  `rotateY(-30deg)` / `rotateY(-10deg)`；直到结束前基本保持不透明，
  最后一刻才隐藏。
- 当前项目按这套时序区分普通磁贴与所选磁贴，但没有复制 Disco 的 DOM、
  GridStack 或 Android 应用生命周期代码；Linux 应用进程在点击当帧启动，
  约 `0.5s` 的可见交接与冷启动并行。

### 从应用列表启动应用

- Disco 对应用列表使用另一套 `appListAppTransitionAnim0`，不能复用磁贴
  的约 -40° 退场。可见项目仍反向归一化到 `0..0.2s` 的延迟窗口，每项
  动画持续 `0.2s`，以共同的屏幕左侧轴心转到 `rotateY(-90deg)`，并向左
  移动约 `flow-perspective / 3`。
- 被点击的应用行固定延迟 `1.5 × 0.2s = 0.3s`，随后播放 `0.2s`，因此
  仍在约 `0.5s` 完成交接；字母项目还额外向左移出一整屏。
- 本项目现在记录点击来源：从磁贴点击才使用所选磁贴时序，从左侧应用
  列表点击则让列表行和字母绕屏幕左侧错峰转出，避免已固定应用从列表
  启动时错误地把右侧磁贴当成所选元素。

### 其他值得以后复用的动效

[`tileList.scss`](https://github.com/cherryhoax/DiscoLauncher/blob/4907389358708754521bf62a5b2048823edcd12e/src/styles/pages/tileList.scss)
还包含：Live Tile 纵向换页、整块翻转、3×3/3×6 小块矩阵随机
`rotateX`、按压透视倾斜、编辑态缩小与晃动。需要增加 Live Tile 内容
轮播时，应先从这里挑一种语义，避免把“进入 Start”和“磁贴内容更新”
混成同一种动画。

## 2. MetroLauncherAndroid（功能参考强，入场动画弱）

- 仓库：[lorddie/MetroLauncherAndroid](https://github.com/lorddie/MetroLauncherAndroid)
- 本次核对提交：[`29ad1c1`](https://github.com/lorddie/MetroLauncherAndroid/commit/29ad1c10a2ddc4bc020ffd4203f4b2c49d5f20f7)
- 许可证注意：仓库没有独立 `LICENSE` 文件；README 只写
  “Personal / free to adapt”。在许可证澄清前，只把它当行为参考，
  不直接复制代码。

它的 W10M 功能覆盖较完整：1×1、2×2、4×2、4×4 磁贴，4/6 列布局，
拖动重排，透明磁贴，通知文本/计数，媒体封面和控制等。但开始屏幕入口
[`playEntranceAnimation()`](https://github.com/lorddie/MetroLauncherAndroid/blob/29ad1c10a2ddc4bc020ffd4203f4b2c49d5f20f7/app/src/main/java/com/metrolauncher/view/TileGridLayout.kt#L256-L261)
只是每块磁贴从 `translationY = 120dp`、`alpha = 0` 在 `350ms` 内回到
原位，并按 `row * 40ms` 延迟。因此适合借布局与 Live Tile 功能，不适合
解决当前 3D 入场问题。

它的
[`TileView.kt`](https://github.com/lorddie/MetroLauncherAndroid/blob/29ad1c10a2ddc4bc020ffd4203f4b2c49d5f20f7/app/src/main/java/com/metrolauncher/view/TileView.kt)
还可作为内容动画索引：Live Tile 纵向滑动约 350ms；整块 `rotationX`
翻出/翻入各约 350ms；编辑态约 ±1.5° 晃动；按压时 `rotationX` 约 6°、
缩到 97%。

启动应用时，
[`MainActivity.launchApp()`](https://github.com/lorddie/MetroLauncherAndroid/blob/29ad1c10a2ddc4bc020ffd4203f4b2c49d5f20f7/app/src/main/java/com/metrolauncher/activity/MainActivity.kt#L506-L514)
和
[`AllAppsActivity.launchApp()`](https://github.com/lorddie/MetroLauncherAndroid/blob/29ad1c10a2ddc4bc020ffd4203f4b2c49d5f20f7/app/src/main/java/com/metrolauncher/activity/AllAppsActivity.kt#L143-L151)
都只是调用 `startActivity()`，没有 `ActivityOptions` 或自定义 transition。
所以本项目的 Metro 样式只反向播放上浮磁贴作为 shell 退场，再交给系统；
不能把额外的窗口特效冒充成 MetroLauncherAndroid 源码行为。

## 3. Factor Launcher Reboot（简化动效参考）

- 仓库：[Valkriaine/Factor_Launcher_Reboot](https://github.com/Valkriaine/Factor_Launcher_Reboot)
- 本次核对提交：[`000a5d8`](https://github.com/Valkriaine/Factor_Launcher_Reboot/commit/000a5d828f8c0b4ea64d9114592bbd872e1d9c3e)
- 许可证注意：本次核对没有找到独立许可证文件，不能默认可复制代码。

其布局动画
[`tile_animation.xml`](https://github.com/Valkriaine/Factor_Launcher_Reboot/blob/000a5d828f8c0b4ea64d9114592bbd872e1d9c3e/app/src/main/res/anim/tile_animation.xml)
使用 normal 顺序和 15% 延迟；单块动画
[`tile_animation_slide_in.xml`](https://github.com/Valkriaine/Factor_Launcher_Reboot/blob/000a5d828f8c0b4ea64d9114592bbd872e1d9c3e/app/src/main/res/anim/tile_animation_slide_in.xml)
为 200ms：Y 方向 20%→0、透明度 0→1、中心缩放 90%→100%。它比
MetroLauncherAndroid 更轻，但仍不是 turnstile。

它的应用启动与上述布局入场不同：磁贴路径在
[`FactorsAdapter`](https://github.com/Valkriaine/Factor_Launcher_Reboot/blob/000a5d828f8c0b4ea64d9114592bbd872e1d9c3e/app/src/main/java/com/factor/launcher/adapters/FactorsAdapter.java#L441-L449)，
应用列表路径在
[`AppListAdapter`](https://github.com/Valkriaine/Factor_Launcher_Reboot/blob/000a5d828f8c0b4ea64d9114592bbd872e1d9c3e/app/src/main/java/com/factor/launcher/adapters/AppListAdapter.java#L507-L515)；
两者都把被点击 `itemView` 的 `0,0,width,height` 交给
`ActivityOptionsCompat.makeClipRevealAnimation()`，让目标应用从来源矩形
裁剪展开。本项目在真实 Linux 客户端窗口尚未创建时，先用应用图标和中性
表面绘制 0.30s 的同几何代理展开；真实进程从点击当帧就与代理动画并行
启动，而不是等代理完成。

## 4. flutter_metro_ui（Windows 8 启动结构参考）

- 仓库：[coldshineb/flutter_metro_ui](https://github.com/coldshineb/flutter_metro_ui)
- 本次核对提交：[`bad0f8a`](https://github.com/coldshineb/flutter_metro_ui/commit/bad0f8a184b154ff259c6d561eec2e858ab3c390)
- 许可证：仓库在该提交没有 `LICENSE` / `COPYING`，因此只记录行为和参数，
  不复制实现代码。
- 关键路径：[`Home.dart`](https://github.com/coldshineb/flutter_metro_ui/blob/bad0f8a184b154ff259c6d561eec2e858ab3c390/lib/Home.dart#L91-L109)
  用 850ms 控制器、`easeOutExpo` 旋转曲线和 `easeOutQuart` 缩放曲线；
  `_buildFlyingTileLayer()` 将点击磁贴的中心、宽高分别插值到屏幕中心和
  全屏尺寸，再以 0.001 透视项绕 Y 轴转到 ±180°。按压反馈在
  [`MetroTile.dart`](https://github.com/coldshineb/flutter_metro_ui/blob/bad0f8a184b154ff259c6d561eec2e858ab3c390/lib/Theme/MetroUI/MetroTile.dart#L79-L176)
  中使用 150ms、0.96 缩放和最多约 ±0.12rad 的 X/Y 倾斜。

布局方面，`Home.dart` 使用 `#1D1D1D` 背景和横向
`SingleChildScrollView`；基础磁贴为 120px、间隙 10px，组间距 60px，
按可用高度计算行数并最多限制为 6 行，再按列优先把磁贴放入各组。鼠标纵向
滚轮会转成横向滚动。该仓库没有完整的“全部应用”页或 Start↔应用列表
导航，因此本项目只把这些可核对的 Metro 布局参数作为 Windows 8 样式依据，
应用列表和页面切换属于本项目补充实现。

与用户提供的 Windows 8 连续帧相比，源码有两项明显缺失：一是未选中的
开始磁贴没有作为整个场景向后缩退和变暗；二是所选磁贴从第一帧就把尺寸
直接插值到全屏，视觉上更像“边变成页面边翻面”，没有先脱离网格、悬空
放大、到中段侧立，再由背面铺满屏幕的飞出阶段。本项目因此只复用“源矩形
＋透视 Y 轴双面翻转”的结构，运动分段和后撤效果依据用户截图重新建立。

这个仓库还实现了两段与飞出磁贴相互独立的动画：

- **Start 启动：** `_startupController` 总时基为 3000ms；每个磁贴组使用
  `Interval(groupIndex × 0.04, start + 0.35, easeOutExpo)`。组内磁贴不是
  各自动，而是整个组从 `translateX(100)`、80% 缩放和 0 透明度一起回到
  原位。第一个组的可见运动因此为 1050ms，后续组每组晚 120ms。标题和
  用户信息另用 `Interval(0.2, 0.5, easeOutCubic)`，从右侧 50px 淡入。
  本项目现在为所有 Start 布局共用一份持久化分组数据。每个真实组以自己的
  共同中心进行 100epx、80%→100% 和透明度的整体变换，并严格按组索引每组
  晚 120ms；普通关闭使用该运动的反向版本。旧版没有分组字段的数据会迁移到
  默认“开始”组。
- **从 Start/任务栏进入已打开应用：** `_taskbarZoomController` 为 200ms、
  `easeOutQuad`，用 `Rect.lerp()` 把任务栏图标的全局矩形插值到“全屏减去
  任务栏”的应用区域，内容是事先捕获的页面快照并按 `BoxFit.cover` 裁切。
- **已打开应用之间切换：** `AnimatedSwitcher` 为 500ms。新页面先从任务栏
  顺序对应方向的 ±1.5 屏宽进入，在时间轴 0.4..0.7 内用
  `easeOutCubic` 滑到中心；随后在 0.7..1.0 内由 50% 放大到 100%。旧页面
  同时由 100% 后撤到 80%。本项目由合成器定期捕获真实 xdg-toplevel
  缩略图，点击当帧先激活目标窗口，再让这些快照在 shell 顶层完成相同时序，
  任务栏保持在动画层上方。

## 5. 其他可观察项目

这些项目可以用于截图、交互或产品范围对照，但当前没有可直接采用的
完整动画源码：

- [Lumetro / 原 MetroPhoneLauncher](https://github.com/queuejw/LumetroLauncher)：
  曾以 MIT 的实验项目出现，强调不是精确复刻；本次核对时仓库地址无法
  克隆，后续若恢复再重新确认源码和许可证，不把旧搜索摘要当事实来源。
- [Grid — Windows Phone launcher](https://play.google.com/store/apps/details?id=io.karn.grid)：
  Play 商店成品，适合看四种尺寸、纵向网格、文件夹、组件和 Live 内容；
  没有找到对应的公开源码仓库。
- [METROV](https://play.google.com/store/apps/details?id=com.tuzkituan.metrov)：
  闭源 Android 启动器，偏 Windows Phone 8.1；可观察动态天气、日历、
  图片磁贴和回到主页的动效，不作为代码来源。另见
  [Windows Central 介绍](https://www.windowscentral.com/hardware/windows-phone/metrov-windows-phone-launcher)。
- [sqTile 开发帖](https://www.reddit.com/r/wplaunchers/comments/1v79lon/i_missed_the_metro_home_screen_so_much_i_rebuilt/)：
  闭源测试项目，展示照片慢速平移、Live Tile 和 turnstile，可作为新实现
  的视觉样本。

## 6. 网页实现和相邻项目

- Disco 本身就是目前最有价值的开源网页实现，可用
  `npm run debug:web` 在浏览器里单独调动画。
- [Droptiles](https://github.com/oazabir/Droptiles) 是 Windows 8 风格网页
  仪表盘，不是 Windows Phone shell；可借磁贴内容组件，不宜借开始屏幕
  入场逻辑。
- [Metro UI CSS](https://github.com/olton/Metro-UI-CSS) 是通用 Metro 组件
  库，不是操作系统交互复刻。
- [Retiled](https://github.com/DrewNaylor/Retiled) 是面向 Linux 手机/平板的
  QML 桌面，平台方向和本项目接近；后续做手机导航、状态栏和应用生命周期
  时值得单独审查。

## 7. 本项目当前映射

实现位于 `shell/src/start_tiles.rs`，设置入口和持久化位于
`shell/src/main.rs`。设置页把 Start 屏幕动画和应用启动动画拆成两个独立
选择器。

### Start 屏幕动画

| 设置名称 | 行为 | 来源/性质 |
|---|---|---|
| Windows 10 Mobile | 底行向顶行传播；同一行以共享水平中心作为黏合层整体缩放，91%→100% 并从上方轻微向下落位，8%→100% 不透明度；上层逐层展开并靠向下方已稳定的行；各层错峰限制在前 0.16s，每层运动 0.24s，整段不超过 0.40s | 本项目依据用户提供的原机连续帧建立的默认样式；91% 是把旧版 82% 缩小量减半后的校正值，0.40s 为用户确认的整段时长 |
| Disco turnstile | 右下→左上归一化错峰；外层 0.50s、约 70°→0°；内容层 0.35s、X 偏移 60epx、45°→0°；2000epx 透视 | Disco 源码参数 |
| Metro Launcher rise | 按行从顶部开始，Y 偏移 120epx→0、透明度 0→1，0.35s、每行延迟 0.04s | MetroLauncherAndroid 源码参数 |
| Factor slide + scale | 正常布局顺序，Y 偏移磁贴高度 20%→0、透明度 0→1、90%→100% 缩放；0.20s、每块延迟 0.03s | Factor XML 的 200ms + 15% layout delay |
| flutter_metro_ui group sweep | 每个持久化磁贴组作为一个黏合整体，围绕本组中心由右侧 100epx、80% 缩放、0 透明度以 `easeOutExpo` 回到原位；首组时长 1.05s，后组逐组晚 120ms，普通关闭反向播放 | 严格采用 flutter_metro_ui 3000ms 时基上的 `groupIndex × 0.04 .. +0.35` 组区间 |

### 应用启动动画

| 设置名称 | 行为 | 来源/性质 |
|---|---|---|
| Windows 10 Mobile wave | 点击当帧启动进程；第 0 帧保持完整展开的 100% 磁贴几何，随后所有磁贴按黏合行层向外掀起并淡出，波次由顶行传到底行；被点击磁贴在所属行之后再等 0.08s，但仍使用相同的 0.24s 掀起/消失过程，不生成全屏磁贴代理。与此同时应用列表、侧栏和搜索逐渐淡出；最后一个内容元素消失后，完整的开始屏幕壁纸＋白色遮罩继续保持 0.20s，作为等待并行启动应用窗口的空白交接帧；不是把 91%→100% 的入场简单倒放 | 项目默认启动交接；几何、透明度和波次依据用户提供的原机连续帧校正 |
| Windows 8 flying tile | 点击当帧启动进程；未选中的磁贴围绕同一开始屏幕中心在前 0.24s 缩到 72% 并淡出，列表、侧栏与搜索同步消失。点击磁贴由原位置单独置顶，先悬空放大并向屏幕中心移动，再继续放大到中等页面尺寸并转至 90°侧边，最后切换到不镜像的背面、边展开边完成 180°翻转并覆盖全屏；整段 0.72s | flutter_metro_ui 提供源矩形/全屏插值和透视翻面结构；后撤、三段飞行轨迹和双面切换按用户提供的 Windows 8 连续帧重做 |
| Disco launch turnstile | 点击当帧启动；磁贴来源使用 0.175s 普通项及 0.2s 延迟/0.3s 所选项；列表来源使用 -90°、0.2s 普通项及 0.3s 延迟/0.2s 所选项 | Disco 两套 pause 动画参数 |
| Factor clip reveal | 点击当帧启动，同时从磁贴/列表行裁剪扩展为全屏代理表面 | Factor `makeClipRevealAnimation()` 几何 |
| System default (Metro) | 立即关闭 Start，由系统/合成器决定新窗口动画 | MetroLauncherAndroid 直接 `startActivity()` 的语义 |

### 应用切换动画

flutter_metro_ui 的应用切换不加入“应用启动动画”选择器，因为它只作用于
已经存在的窗口：从 Start 或最小化状态恢复时，目标窗口快照在 200ms 内从
任务栏图标矩形放大到应用区域；两个已打开应用之间切换时，使用 500ms 的
旧页 100%→80% 后撤、新页 ±1.5 屏宽滑入后 50%→100% 放大。目标真实窗口
在点击帧立即获得激活，动画只是覆盖在上面的视觉交接，不增加 200/500ms
的输入前置等待。没有可用缩略图时以应用图标和中性页面色作为回退。
用户可以单独关闭这一动画；关闭时 shell 不生成代理，合成器也停止周期性
GPU 读取和切换图 PNG 编码。为保留多窗口任务栏菜单的真实预览，只有菜单
实际打开期间才按需生成有界缩略图。

### Start 布局和应用列表导航

三种布局复用同一份持久化磁贴数据。Windows 10 保持列表在左、磁贴在右并
沿纵向扩展；Windows Phone 的磁贴页沿纵向扩展，左滑后让整页向左退出、
应用列表从右侧进入，右滑反向返回；Windows 8 按 flutter_metro_ui 的横向
Metro 板组织磁贴：背景 `#1d1d1d`，磁贴可在普通 `120×120`、宽磁贴
`250×120`，间距 10px、左边距 100px、顶部 140px、底部 60px，按列优先
填充且最多六行。所有布局共用持久化的组 ID 和可编辑标题；磁贴菜单可以移入
已有组或建立新组，旧数据自动进入默认“开始”组。Win8 把每组独立横向排布，
各组从右侧 100px、80% 缩放和 0 透明度以 `easeOutExpo` 进入：第一组 1.05s，
后续每组晚 120ms；标题和用户栏在 0.6..1.5s 以 `easeOutCubic` 延迟进入。
Win8 模式固定使用这套动画，但保留用户给其他布局选择的 Start 动画设置，也
不会改写其他布局的自由磁贴坐标。

参考仓库没有全部应用页；本项目补充的应用列表按列优先横向展开，向上滑或点
左下角按钮后让 Start 向上退出、应用列表从下方进入，向下滑反向返回。Phone
和 Win8 的分页切换均为 340ms `easeOutExpo` 推页，前段快速响应、后段明显
减速；动画期间标题和页面内容保持同一位移，不做瞬切。

Disco 和 Windows 8 的纹理图标使用四顶点网格真实跟随透视，文字/字体
图标采用投影位置与缩放近似。这些第三方样式是可选实验，不能覆盖默认 W10M 产品
方向。第三方源码没有定义与本 shell 相同的 Start 关闭流程时，项目使用
相应入场的反向版本；有明确启动路径的 Disco 和 Factor 则优先按源码映射。

两个选择都写入 `~/.config/win10x/shell.json`。旧配置只有
`start_tile_animation` 时，会按 Windows 10 Mobile→同名、
Disco→Disco、Metro→系统默认、Factor→Factor、flutter_metro_ui→系统默认
的映射恢复启动动画，避免
升级后改变现有观感。壁纸 + 半透明白层仍由
`shell/src/main.rs` 绘制，不做亚克力模糊；每种样式分别计算总时长，使
背景在最后一个延迟磁贴完成时同时结束。W10M/Metro 的逐行时长按当前
布局的相对行跨度计算，不受磁贴整体被拖到第几行影响。应用进程和 shell 交接从同一次
点击同时开始，使动画时间与冷启动重叠，而不是额外增加等待。
