
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
- [__ai/](/__ai/) - 类似于 src 一样的源提示词工作目录
  - [locale/](/__ai/locale/) - 当前项目映射记忆提示词
    - [__ai/](/__ai/locale/__ai/) - 针对于 `__ai/` 的映射记忆提示词
    - [meta/](/__ai/locale/meta/) - 中文版的元信息以及解释标准
  - [user/](/__ai/GLOCAL.locale.md) - 全局用户记忆提示词
  - [project/](/__ai/project/) - 项目级记忆提示词
  - [cmd/](/__ai/cmd/) - 自定义命令提示词
  - [agents/](/__ai/agents/) - 子代理提示词
  - [meta/](/__ai/meta/) - 可参考提示词编写标准
- [cli/](/cli/) - 命令行工具
  - [rust/](/cli/rust/) - rust 核心工具
- [hooks/](/hooks/) - 钩子脚本
```
