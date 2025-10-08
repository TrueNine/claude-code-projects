# UIControl 桌面 Shell 指南

## 项目定位
- 构建在 Tauri 2 + React 19 上的跨平台桌面壳, 通过 Web 前端与 Rust 后端组合形成快速原型环境。
- 目标是为 UI 自动化或控制台工具提供可扩展基础, 默认示例演示如何使用 `invoke` 触发 Rust 命令并回传数据。
- 引入 `@tauri-apps/plugin-opener` 统一外部链接行为, 方便在桌面环境中打开文档或系统资源。

## 用户场景
- 桌面调试: 使用 `pnpm tauri dev` 启动 Tauri DevServer, 同时拉起 Rust 后端与 React 前端, 适合端到端联调。
- 前端迭代: 使用 `pnpm dev` 单独运行 Vite, 快速验证 UI 组件或状态逻辑后再回到 Tauri。
- 打包发布: 通过 `pnpm tauri build` 生成 Windows/macOS/Linux 安装包, 构建过程中自动调用 `pnpm build` 产出前端静态资源。
- API 验证: 借助 `invoke("greet", { name })` 示例验证 Rust Command 通路, 为后续扩展自定义命令提供参照。

## 技术栈
- 前端: React 19 + TypeScript 5.9 + Vite 7, 状态管理目前依赖 React Hooks。
- 桌面壳: Tauri 2, 默认窗口尺寸及 CSP 在 `src-tauri/tauri.conf.json` 中配置。
- 后端: Rust 2021 Edition, 命令在 `src-tauri/src/lib.rs` 内注册, `tauri-plugin-opener` 作为内建插件。
- 包管理: `pnpm@10.17.1`, Rust 依赖通过 `cargo` 维护。

## 架构
- React SPA 位于 `src/`, 入口 `main.tsx` 绑定到根节点, `App.tsx` 展示欢迎界面并调用 Rust 命令。
- `src-tauri/src/lib.rs` 暴露 `greet` 命令, 使用 `tauri::Builder` 注册插件与命令后运行。`main.rs` 仅代理到 `uicontrol_lib::run()`。
- `tauri.conf.json` 定义 `beforeDevCommand`/`beforeBuildCommand`, 控制开发与发布阶段调用 `pnpm` 指令, 并声明窗口元数据与打包目标。
- 静态资源输出目录为 `dist`, 通过 `frontendDist` 与 Rust 侧集成。

## 命令设计
- `pnpm dev`: 独立运行 Vite, 带有热更新, 不启动 Rust Runtime。
- `pnpm build`: 先执行 `tsc` 类型检查, 再执行 `vite build` 生成生产资源。
- `pnpm tauri`: 透传给 Tauri CLI, 常用子命令 `pnpm tauri dev`、`pnpm tauri build`。
- `cargo fmt --check`: 保证 Rust 代码风格。
- `cargo clippy -- -D warnings`: 静态诊断 Rust 代码。
- `cargo test`: 当前无测试但命令可用于回归检测, 后续补充单元测试时沿用。

## 文件规范
- 前端入口: `src/main.tsx`, 根组件 `src/App.tsx`, 样式集中在 `src/App.css`。
- 资源管理: SVG 等静态资源存放于 `src/assets/`, 通过 Vite 资源管线加载。
- Rust 配置: `src-tauri/Cargo.toml` 声明 crate 名称及插件依赖, `src-tauri/src` 存放实际实现。
- 桌面配置: `src-tauri/tauri.conf.json` 负责窗口、打包、DevServer 钩子。
- TypeScript 配置: `tsconfig.json` + `tsconfig.node.json` + `vite.config.ts` 管理别名与编译目标。

## 工作流
- 安装依赖: 在 `uicontrol/` 目录执行 `pnpm install`, 同时确保本机已安装 Rust 稳定工具链和 `cargo`.
- 开发调试: 推荐 `pnpm tauri dev`, 如需单前端迭代可切换 `pnpm dev`, Rust 改动支持 `cargo watch` 自行配置。
- 打包发布: 使用 `pnpm tauri build`, 输出位于 `src-tauri/target` 子目录, 三平台安装包默认全部生成。
- 质量校验: 前端通过 `pnpm build` 覆盖类型检查, Rust 使用 `cargo fmt --check` 与 `cargo clippy -- -D warnings`, 必要时追加 `cargo test`.
- 版本控制: 变更前确认 `dist/` 与 `node_modules/` 等构建产物未加入 Git 追踪, 新增 Rust 命令需同步更新 TypeScript 调用层。

## 扩展方向
- 用实际业务命令替换 `greet`, 在 `src-tauri/src/lib.rs` 内追加 `#[tauri::command]` 并更新 `generate_handler`.
- 建立共享状态或数据层 (如 Zustand/Redux) 以管理复杂 UI, 同时确保与 Tauri 命令通信保持类型安全。
- 加入端到端测试 (Playwright) 或 Rust 集成测试, 完成后在本指南补充对应命令。
- 评估引入 CI (例如 GitHub Actions) 执行 `pnpm build`、`cargo fmt --check`、`cargo clippy` 以确保提交质量。
