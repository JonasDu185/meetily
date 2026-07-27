# 版本与发布

## 版本关系

- 个人版应用版本：从 `0.1.0` 独立演进。
- 当前上游基线：Meetily Community `v0.4.0`。
- GitHub 标签：使用 `v0.1.0`、`v0.2.0` 等个人版标签。
- 每次同步上游后，在 `docs/UPSTREAM.md` 记录上游标签、提交号和冲突决策。

个人版版本号不追随上游编号，避免把“同步了哪个上游版本”和“个人版发布到哪一版”
混为一谈。

## 仓库关系

```text
Meetily 官方仓库
      │ fetch
      ▼
upstream ──> sync/upstream-* ──> 个人 main
                                     │
                                     ▼
                              origin（个人 fork）
```

- `upstream` 只拉取，push URL 固定为 `DISABLED`。
- `origin` 指向用户自己的 GitHub fork。
- `main` 只接收经过验收的改动。
- 日常开发使用 `feature/*`、`fix/*` 和 `chore/*`。
- 不强制推送共享分支。

## 首次建立 GitHub fork

以下操作必须在用户明确确认后执行：

1. 在 GitHub 上 fork `Zackriya-Solutions/meetily`。
2. 核对个人 fork 地址并添加为 `origin`。
3. 将当前基线分支推送到个人 fork。
4. 通过个人仓库内的 Pull Request 合并到 `main`。
5. 创建 `v0.1.0` 标签和 GitHub Release。
6. Release 附上 DMG、SHA-256、更新说明和已知限制。

不把录音、数据库、模型文件、日志、密钥或本机路径上传到 GitHub。

## 发布前检查

- 工作区只包含本版本计划内的变更。
- `LICENSE.md` 和原版权声明保留。
- `CHANGELOG.md`、应用版本、Cargo 版本三处一致。
- 前端生产构建通过。
- 受影响的 Rust 测试通过。
- `.app` 的 Bundle ID、版本、权限说明和 ad-hoc 签名正确。
- DMG 能够挂载，内部应用签名有效。
- 用户完成真实设备验收；未验证项目必须写入 Release Notes。

## 推荐提交拆分

commit message 使用中文，建议按逻辑拆分：

1. `文档：建立个人 fork 维护与验收规范`
2. `构建：建立本机离线构建与独立应用标识`
3. `隐私：关闭遥测和官方自动更新`
4. `转写：调整首次启动与中文模型默认配置`
5. `搜索：修复中文上下文截取`
6. `品牌：更新会议录音应用图标`

提交、标签、推送和创建 Release 前都需要用户再次确认。

## 当前本地产物

- 应用：`target/release/bundle/macos/会议录音.app`
- 安装盘：`target/release/bundle/dmg/会议录音_0.1.0_aarch64.dmg`
- DMG SHA-256：
  `1289053316ff3827294ecde38901b4349e2dfb97530f8ea3231960294beefb1f`

构建产物不进入 Git 历史，只在 GitHub Release 中作为附件发布。
