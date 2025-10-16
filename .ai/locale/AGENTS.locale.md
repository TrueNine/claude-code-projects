
本文件为 `AI Agent` 在此仓库中工作时提供指导。





# 项目概述

这是一个用于 `AI Agent` 工作流程的文档和配置仓库，专注于：
- 提示词工程和模板库
- 开发工作流程的问答文档
- 技术参考和最佳实践资料


# 适配目标

- `OpenAI Codex` - CLI 工具
- `Anthropic Claude Code` - CLI 工具
- `Cursor` - Vscode 魔改IDE
- `Factory Droid` - CLI 工具


# 项目结构

```md
- [.ai/](/.ai/) - 类似于 src 一样的源提示词工作目录
  - [locale/](/.ai/locale/) - 当前项目映射记忆提示词
    - [.ai/](/.ai/locale/.ai/) - 针对于 `.ai/` 的映射记忆提示词
    - [meta/](/.ai/locale/meta/) - 中文版的元信息以及解释标准
  - [user/](/.ai/user/) - 全局用户记忆提示词
  - [project/](/.ai/project/) - 项目级记忆提示词
  - [cmd/](/.ai/cmd/) - 自定义命令提示词
  - [agents/](/.ai/agents/) - 子代理提示词
  - [meta/](/.ai/meta/) - 可参考提示词编写标准
- [cli/](/cli/) - 命令行工具
  - [rust/](/cli/rust/) - rust 核心工具
- [hooks/](/hooks/) - 钩子脚本
```
