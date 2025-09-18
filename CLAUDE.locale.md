# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

这是一个用于 Claude Code 工作流程的文档和配置仓库，包含：
- Claude Code 配置文件和钩子
- 文档模板和提示词
- 开发工作流程的问答文档
- 开发最佳实践参考资料

## 关键命令

此仓库主要专注于文档，没有传统的构建/测试命令。主要开发工作通过以下方式进行：

### 钩子开发（TypeScript）
- **位置**：`.claude/hooks/`
- **包管理器**：`pnpm`（在 package.json 中指定）
- **运行钩子**：钩子由 Claude Code 事件自动触发
- **开发**：编辑 `.claude/hooks/src/` 中的 TypeScript 文件

### 文档管理
- **无构建命令** - 文档文件直接管理
- **结构**：在 `docs/` 目录中井然有序地组织，包含特定语言环境的内容

## 架构与代码组织

### 配置结构
- **`.claude/`** - Claude Code 配置和钩子
  - `settings.json` - 主要 Claude Code 设置（模型：opusplan，权限，MCP 服务器）
  - `hooks/` - 用于文件处理自动化的 TypeScript 钩子
  - `hooks/main.ts` - 前端文件检查/格式化的主钩子入口点

### 文档架构
- **`docs/`** - 主文档目录
  - `prompts/` - 提示词模板（本地化和英文版本）
    - `locale/` - 中文本地化提示词
    - `output/` - 英文翻译提示词
  - `qa/` - 开发工作流程的问答文档
  - `references/` - 技术参考文档
  - `other/` - 各种文档（构建、Git、数据库等）

### 钩子系统
仓库包含一个复杂的 TypeScript 钩子系统（`.claude/hooks/main.ts`），该系统：
- 在保存/编辑时自动处理前端文件
- 并行运行 prettier、eslint 和 TypeScript 检查
- 提供详细日志记录到 `.claude/hook-debug.log`
- 仅处理 `frontend/` 目录中的文件

## 重要配置

### Claude Code 设置
- **模型**：`opusplan`（在 `.claude/settings.json` 中配置）
- **MCP 服务器**：context7, sequential-thinking, memory, deepwiki, grep
- **权限**：限制 git/npm/docker 命令，允许 MCP 和基本工具

### 项目约定
- **语言**：双语（中文本地化文件和英文翻译）
- **文件命名**：中文内容使用 `.locale.md` 后缀
- **文档结构**：在 `docs/` 目录中分层组织

## 开发工作流程

由于这是一个文档仓库：

1. **编辑文档**：直接编辑文件，无需构建步骤
2. **钩子开发**：编辑 `.claude/hooks/src/` 中的 TypeScript，钩子自动重新加载
3. **配置更改**：根据需要更新 `.claude/settings.json`
4. **内容管理**：遵循现有的本地化/翻译模式

## 重要说明

按要求执行；不多不少。
除非绝对必要才创建文件来实现目标。
始终优先编辑现有文件而不是创建新文件。
绝不主动创建文档文件（*.md）或 README 文件。只有在用户明确要求时才创建文档文件。
