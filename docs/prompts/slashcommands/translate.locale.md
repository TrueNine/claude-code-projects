---
allowed-tools: Read, Write, Edit, MultiEdit, Glob, Grep, Bash, TodoWrite, Task
description: Translate Chinese locale documentation files to English, following consistent terminology and quality standards
---

# 翻译指令 / Translation Command

## 使用方法 / Usage
```
/translate @path/to/file.locale.md
```

## 功能说明 / Description

将本地化文档文件（`.locale.md`）翻译为对应的英文文档文件，去除文件名中的 `.locale.` 部分。

Translate Chinese localization documentation files (`.locale.md`) to corresponding English documentation files, removing the `.locale.` part from the filename.

## 工作流程 / Workflow

1. **读取源文件** / Read source file: `$1`
2. **解析文件名** / Parse filename:
  - Input: `filename.locale.extension`
  - Output: `filename.extension`
3. **执行翻译** / Perform translation:
  - 保持 Markdown 格式和结构 / Preserve Markdown formatting and structure
  - 应用术语表中的一致翻译 / Apply consistent terminology from glossary
  - 保留代码块不变 / Keep code blocks unchanged
  - 适当翻译代码注释 / Translate code comments appropriately
4. **写入目标文件** / Write target file:
  - 如果目标文件已存在，直接替换全部内容 / If target file exists, replace all content
  - 不要读取现有目标文件内容 / Do not read existing target file content

## 质量标准 / Quality Standards

- **术语一致性** / Terminology consistency: 严格遵循词汇表 / Strictly follow glossary
- **技术准确性** / Technical accuracy: 保持技术概念的准确性 / Maintain accuracy of technical concepts
- **格式保持** / Format preservation: 保留所有 Markdown 格式 / Preserve all Markdown formatting
- **链接处理** / Link handling: 适当更新文档链接 / Update documentation links appropriately
- **代码完整性** / Code integrity: 代码示例保持不变 / Keep code examples unchanged

## 示例 / Examples

### 文件名转换 / Filename Conversion
- `translate.locale.md` → `translate.md`
- `setup.locale.md` → `setup.md`
- `config.locale.yaml` → `config.yaml`

### 翻译示例 / Translation Examples
- `请参阅文档` → `See documentation`
- `配置文件` → `Configuration file`
- `命令行工具` → `Command-line tool`

## 参考资料 / References

请参阅 [slash_commands](https://docs.claude.com/zh-CN/docs/claude-code/slash-commands)

See [slash_commands](https://docs.claude.com/en/docs/claude-code/slash-commands)

## 词汇表 / Glossary

### 常用术语 / Common Terms
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

### Claude Code 专用术语 / Claude Code Specific Terms
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

### 技术术语 / Technical Terms
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

### 文件和目录 / Files and Directories
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

### Git 相关 / Git Related
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
