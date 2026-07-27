# 上游与分支维护

## 固定基线

- 上游：`https://github.com/Zackriya-Solutions/meetily.git`
- Community 基线：v0.4.0
- 基线提交：`0281737d87d26352fb0adc78c8c0975f691b23d1`
- 个人版起始版本：v0.1.0
- 初始审计日期：2026-07-27

当前本地官方远程名为 `upstream`，其 push URL 已设为 `DISABLED`，用于防止误推送。在用户创建个人 GitHub fork 前，暂不配置 `origin`。

个人版使用独立版本号。上游版本表示“代码从哪里来”，个人版版本表示“本机应用
具备哪些能力”；两者分别记录，避免混淆。

## 远程关系

- `upstream`：Meetily 官方仓库，只获取更新，不向其推送。
- `origin`：用户未来的个人 fork；只有用户明确创建并授权后才添加。

添加个人 fork 时先核对目标地址，再执行：

```bash
git remote add origin <个人-fork地址>
git remote -v
```

## 分支约定

- `main`：个人版本的稳定集成分支，不直接开发。
- `feature/*`：独立功能。
- `fix/*`：缺陷修复。
- `chore/*`：构建、文档、合规和维护。
- `sync/*`：同步上游和解决冲突。

一个分支只处理一个目的。分支名称使用简短英文；commit message 使用中文。

## 同步上游流程

1. 检查工作区，确认用户改动不会被覆盖。
2. 获取 `upstream` 更新。
3. 从个人 `main` 创建 `sync/upstream-日期或版本`。
4. 比较基线与新上游，重点检查：
   - `LICENSE.md` 和第三方许可证
   - `frontend/package.json` 与锁文件
   - `frontend/src-tauri/Cargo.toml` 与锁文件
   - 数据库迁移
   - 音频采集、混音、录音保存和恢复
   - Whisper/Parakeet 模型目录
   - Tauri 权限、Bundle ID、遥测和更新器
5. 在同步分支合并上游，逐项解决冲突。
6. 执行受影响验收，不用整文件覆盖掩盖冲突。
7. 记录新上游提交号、冲突决策和未验证项目。
8. 经用户确认后再提交、合并和推送。

不对已经共享的 `main` 做变基或强制推送。

## 回滚

- 尚未提交：只撤销本轮明确创建的文件或代码，先确认目标。
- 已提交未推送：创建反向提交，保留历史。
- 已推送：优先 `git revert`，不强制改写共享历史。
- 上游同步失败：保留个人 `main`，删除同步分支前必须获得确认。
