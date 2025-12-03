---
inclusion: fileMatch
fileMatchPattern: "**"
---

This file provides guidance for `AI Agent` when working in this repository.




# Project Overview

This is a documentation and configuration repository for `AI Agent` workflows, focusing on:
- Prompt engineering and template libraries
- Q&A documentation for development workflows
- Technical reference and best practice materials


# Adaptation Targets

- `OpenAI Codex` - CLI tool
- `Anthropic Claude Code` - CLI tool
- `Cursor` - Vscode modified IDE
- `Factory Droid` - CLI tool


# Project Structure

```md
- [__ai/](/__ai/) - Source prompt working directory similar to src
  - [locale/](/__ai/locale/) - Current project mapping memory prompts
    - [__ai/](/__ai/locale/__ai/) - Mapping memory prompts for `__ai/`
    - [meta/](/__ai/locale/meta/) - Chinese version meta information and explanation standards
  - [user/](/__ai/GLOBAL.locale.md) - Global user memory prompts
  - [project/](/__ai/project/) - Project-level memory prompts
  - [cmd/](/__ai/cmd/) - Custom command prompts
  - [agents/](/__ai/agents/) - Sub-agent prompts
  - [meta/](/__ai/meta/) - Reference prompt writing standards
- [cli/](/cli/) - Command line tools
  - [rust/](/cli/rust/) - Rust core tools
- [hooks/](/hooks/) - Hook scripts
```
