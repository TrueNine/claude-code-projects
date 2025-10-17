
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
- [__aiissues](/__aiissues/): 用户对于 AI Agent 工作问题的反馈
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

# 反馈驱动改进

用户会在 [__aiissues](/__aiissues/) 下记录提示词或 AI 协作相关的问题与改进建议，通常以 `.md` 文件形式存在（不处理纯代码缺陷）。

处理流程：
- 使用 `List` 工具按时间或主题浏览待处理反馈，并通过 `Read` 快速获取详情。
- 在输出前归纳问题根因与期望结果，必要时制定可执行的修复或流程调整方案。
- 若需修改记忆提示词或脚本，直接在相关文件中实现并说明改动如何回应反馈。
- 反馈无法立即解决时，告知限制、建议后续步骤，并记录在当前会话或待办中确保后续追踪。
