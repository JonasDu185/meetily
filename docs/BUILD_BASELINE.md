# 原版构建基线记录

记录日期：2026-07-27
源码基线：Meetily Community v0.4.0
提交：`0281737d87d26352fb0adc78c8c0975f691b23d1`

## 本机工具

- macOS 26.5.2，Apple Silicon
- Node.js 20.20.2（keg-only，不替换系统现有 Node 22）
- pnpm 10.34.5（通过 Corepack 临时指定）
- Rust/Cargo 1.97.1
- CMake 4.4.0
- Xcode 26.6（许可已接受）

系统全局 `xcode-select` 仍指向 Command Line Tools。项目构建通过临时设置
`DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer` 使用完整 Xcode，
没有修改全局开发环境。

## 依赖安装

上游 CI 配置使用 pnpm 8，但仓库提交的 `pnpm-lock.yaml` 是 v9 格式。`package.json` 还包含 5 个 ProseMirror `overrides`，原锁文件没有记录，导致 frozen 安装失败。

使用 pnpm 10.34.5 重新生成锁文件后：

- 增加 5 个既有 ProseMirror 固定版本约束。
- 未改变依赖版本或完整性哈希。
- 移除 9 个只用于 Linux 预编译包的冗余 `libc` 标记。
- 锁文件共 23 行新增、25 行删除。

随后使用 `--frozen-lockfile` 成功安装 637 个包。`node_modules` 约 813 MB，属于可重新生成且已被 Git 忽略的构建目录。

## 构建修正

- 将 Google 在线字体替换为 macOS 系统字体栈，不再依赖构建时联网下载字体。
- 在前端开发和构建命令中关闭 Next.js 匿名遥测。
- 使用修正后的锁文件进行 frozen 安装。

修正后，前端生产构建成功：

- 编译成功。
- TypeScript 与 lint 检查通过。
- 11 个静态页面生成成功。

## 原生构建

完整 Xcode 配置完成后，Apple Silicon release 构建通过：

- Whisper：Core ML
- llama-helper：Metal
- 应用版本：0.1.0
- 架构：arm64
- Bundle ID：`com.jonasdu.meetingrecorder`
- 签名：ad-hoc

应用产物：

`target/release/bundle/macos/会议录音.app`

`codesign --verify --deep --strict` 校验通过。没有 Apple Developer 付费账号，
因此不做公证，首次打开时可能需要用户在 macOS 安全设置中确认。

## 安装盘

上游 `create-dmg` 美化脚本在 macOS 26 的 Finder/磁盘镜像自动化阶段退出，
但应用本体已成功构建。为避免把这个偶发问题隐藏起来，本版改用标准 DMG 结构：

- `会议录音.app`
- `Applications` 快捷方式

安装盘：

`target/release/bundle/dmg/会议录音_0.1.0_aarch64.dmg`

SHA-256：

`1289053316ff3827294ecde38901b4349e2dfb97530f8ea3231960294beefb1f`

已实际只读挂载 DMG，并验证内部应用签名、版本、Bundle ID 和中文权限说明。

## 个人隐私基线

- 后端分析初始化固定使用空密钥、禁用状态，不创建 PostHog 客户端。
- 启动时覆盖旧版可能遗留的统计选择，固定写回关闭状态。
- 设置界面只显示“使用情况统计已永久关闭”，不再提供启用开关。
- 移除启动更新检查、托盘更新入口、Tauri 更新插件注册、权限和官方更新地址。
- 保留模型下载功能；它与应用自动更新是两条独立链路。

应用已启动到欢迎页进行冒烟测试：

- 进程保持运行，没有异常退出。
- `analyticsOptedIn` 为 `false`。
- 未发现网络连接。
- 未下载模型或产生大体积模型文件。

## 首次启动与模型

- 保留上游内置 Whisper/Parakeet 模型管理。
- 首次启动不自动下载转写或摘要模型。
- 中文转写默认选择 Whisper `large-v3-turbo-q5_0`，下载仍需用户确认。
- 当前未下载任何转写模型。
- 当前版本不启用会议摘要模型。

## 中文搜索安全

- 搜索结果上下文改为按完整 Unicode 字符边界截取，不再按任意字节位置切割。
- 保持 SQLite 一致的 ASCII 大小写不敏感语义。
- 新增纯中文、中英混合、表情符号和长文本四项单元测试。

测试结果：

- 中文搜索 4 项通过。
- 首次启动兼容测试 1 项通过。

## 当前结论

- 本地 `v0.1.0` 候选应用和 DMG 已构建、签名并验证。
- 尚未安装到 `/Applications`。
- 尚未下载转写模型。
- 尚未进行真实录音、录音导出、Markdown/PDF 导出或会议提醒验收。
- 代码已按个人 fork 流程提交；发布状态以 GitHub Releases 为准。

下一步先由用户确认安装候选应用；随后下载 Whisper 模型并完成 5 分钟真实录音
基线，再进入录音导出和文本导出开发。
