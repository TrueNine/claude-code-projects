# Agent CLI 提示管理方案

## 项目定位
- 提供 `npx -y @truenine/agent-cli` 入口, 快速初始化或更新 `.ai/` 提示词仓, 体验类似 `pnpm create vite`。
- 所有核心能力由 Rust 实现, TypeScript 仅负责参数解析与调度, 确保性能与多平台一致性。
- 覆盖记忆提示词、子代理提示词、快捷命令提示词三类内容, 保持 `.ai/` 目录内的提示体系集中可控。

## 用户场景
- 初始化: 一次性生成 `.ai/` 目录结构, 创建示例 memory、sub-agent、cmd 提示词, 配置多平台可用脚本。
- 更新: 检测仓库状态, 应用最新模板或片段, 替换过期文件, 保留备份。
- 清理: 识别失效提示词并安全删除, 避免陈旧内容干扰团队协作。
- 同步: 执行 `npx -y @truenine/agent-cli sync` 将最新提示词推送到 Codex 与 Claude Code 使用场景。

## 技术栈
- TypeScript (入口层): 使用 `pnpm` 管理, 提供最小 CLI 启动与参数收集。
- Rust (核心引擎): 负责文件扫描、差异分析、写入、回滚、平台探测, 通过 `napi-rs` 暴露给 TypeScript。
- 目标平台: macOS (universal), Linux (x64, arm64), Windows (x64)。

## 架构
- CLI Frontend (TS): 解析 `init`, `update`, `compose`, `prune`, `sync` 等子命令, 将请求转交 Rust。
- Orchestrator (Rust): 根据命令执行模板合成、校验、清理, 输出统一事件日志。
- Storage Manager (Rust): 维护 `.ai/` 目录树与提示词索引, 支持快照、回滚、原子写入。
- Template Registry (Rust): 内置标准片段与元数据, 便于未来引入远程仓库。

## 命令设计
- `agent-cli init`
  - 生成 `.ai/locale`, `.ai/user`, `.ai/project`, `.ai/cmd`, `.ai/sa` 等目录
  - 注入示例 memory 提示词、子代理脚本、快捷命令模板
  - 创建 `agents.prompts.json` 记录结构版本
- `agent-cli compose <type>`
  - `type` 支持 `memory | sub-agent | cmd`
  - Rust 读取片段仓库, 交互式组合并写入目标文件
- `agent-cli update`
  - 对比当前版本与模板仓, 应用增量更新, 自动替换旧提示词并写入 changelog
- `agent-cli prune`
  - 扫描 `.ai/` 中的孤立提示词, 提示确认后删除
- `agent-cli sync`
  - 核心命令: 支持 `npx -y @truenine/agent-cli sync` 一次性拉起 CLI, 合并本地更改并同步到共享路径

## 文件规范
- Memory 提示词: `.ai/locale/**/*.locale.md`, `.ai/user/**/*.md`, `.ai/project/**/*.md`
- 子代理提示词: `.ai/sa/**/*.md`
- 快捷命令提示词: `.ai/cmd/**/*.md`
- 配置文件: `agents.prompts.json` 描述目录映射、模板版本、平台支持
- 备份: `.agents/backups/<timestamp>/` 保存更新前状态, 支持 `agent-cli rollback <timestamp>`

## 工作流
- 安装依赖: `pnpm install`, Rust 依赖通过 `cargo` 管理
- 开发模式: `pnpm dev` 启动 TypeScript watcher, `cargo watch -x "test" -x "build"` 监听核心引擎
- 质量保证: `pnpm lint`, `pnpm test`, `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`
- 发布: Rust 编译出跨平台二进制, TypeScript 保留轻量入口, 通过 npm 发布 `@truenine/agent-cli`

## 多平台策略
- Rust 使用 `napi-rs` 或 `tauri bundler` 生成目标平台二进制, 随 npm 包分发
- CLI 运行时自动检测平台, 选择对应二进制, 确保 Windows/macOS/Linux 行为一致

## 安全与回滚
- 所有写入采用临时文件 + rename 保障原子性
- 删除默认移动至 `.agents/trash`, 使用 `--force` 时才真正清除
- 回滚命令在多平台使用统一语义, 保证团队成员体验一致

## 未来方向
- 远程模板仓: 支持从私有 Git 仓库同步片段
- 片段评分: 基于使用频率推荐常用组合
- GUI 包装: 依托 Rust 核心 + Tauri 提供轻量界面
- DevOps 集成: 提供 `agent-cli verify` 检查 `.ai/` 结构是否符合最新规范
