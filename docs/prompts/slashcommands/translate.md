---
allowed-tools: Read, Write, Edit, MultiEdit, Glob, Grep, Bash, TodoWrite, Task
description: Translate Chinese locale documentation files to English, following consistent terminology and quality standards
---

# Translation Command

## Usage
```
/translate @path/to/file.locale.md
```

## Description

Translate Chinese localization documentation files (`.locale.md`) to corresponding English documentation files, removing the `.locale.` part from the filename.

## Workflow

1. **Read source file**: `$1`
2. **Parse filename**:
  - Input: `filename.locale.extension`
  - Output: `filename.extension`
3. **Check target file**:
  - Use Glob tool to check if target file exists
  - Pattern: `filename.extension`
4. **Delete existing file**:
  - If target file exists, use Bash tool to delete
  - Command: `rm filename.extension` (Linux/Mac) or `del filename.extension` (Windows)
5. **Perform translation**:
  - Preserve Markdown formatting and structure
  - Apply consistent terminology from glossary
  - Keep code blocks unchanged
  - Translate code comments appropriately
6. **Write target file**:
  - Create new target file and write translated content
  - No need to read existing target file content (deleted in step 4)

## Quality Standards

- **Terminology consistency**: Strictly follow glossary
- **Technical accuracy**: Maintain accuracy of technical concepts
- **Format preservation**: Preserve all Markdown formatting
- **Link handling**: Update documentation links appropriately
- **Code integrity**: Keep code examples unchanged

## Examples

### Filename Conversion
- `translate.locale.md` → `translate.md`
- `setup.locale.md` → `setup.md`
- `config.locale.yaml` → `config.yaml`

### Translation Examples
- `请参阅文档` → `See documentation`
- `配置文件` → `Configuration file`
- `命令行工具` → `Command-line tool`

## References

See [slash_commands](https://docs.claude.com/en/docs/claude-code/slash-commands)

## Glossary

### Common Terms
- 请参阅 / 参见 - see, refer to
- 配置 - configuration, config
- 设置 - settings
- 文档 - documentation, docs
- 指南 - guide
- 教程 - tutorial
- 示例 - example
- 工具 - tool
- 命令 - command
- 脚本 - script
- 文件 - file
- 目录 - directory
- 路径 - path

### Claude Code Specific Terms
- 钩子 - hook
- 斜杠命令 - slash command
- 工作区 - workspace
- 代理 - agent
- 模型 - model
- 提示 - prompt
- 上下文 - context
- 会话 - session
- 任务 - task
- 工作流 - workflow

### Technical Terms
- 版本控制 - version control
- 构建 - build
- 测试 - test
- 部署 - deployment
- 调试 - debug
- 日志 - log
- 错误 - error
- 警告 - warning
- 依赖 - dependency
- 包管理器 - package manager
- 环境变量 - environment variable
- 接口 - interface, API
- 端点 - endpoint
- 请求 - request
- 响应 - response
- 数据库 - database
- 查询 - query
- 架构 - architecture
- 框架 - framework
- 库 - library
- 模块 - module
- 组件 - component

### Files and Directories
- 根目录 - root directory
- 源代码 - source code
- 资源 - assets, resources
- 公共 - public
- 静态 - static
- 构建输出 - build output
- 缓存 - cache
- 临时文件 - temporary files
- 备份 - backup
- 归档 - archive

### Git Related
- 仓库 - repository, repo
- 分支 - branch
- 提交 - commit
- 合并 - merge
- 拉取请求 - pull request
- 问题 - issue
- 标签 - tag
- 发布 - release
- 克隆 - clone
- 推送 - push
- 拉取 - pull
- 获取 - fetch
- 暂存 - stage
- 变基 - rebase