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
- [.ai/](/.ai/) - Source prompt working directory similar to src
  - [locale/](/.ai/locale/) - Current project mapping memory prompts
    - [.ai/](/.ai/locale/.ai/) - Mapping memory prompts for `.ai/`
    - [meta/](/.ai/locale/meta/) - Chinese version meta information and explanation standards
  - [user/](/.ai/user/) - Global user memory prompts
  - [project/](/.ai/project/) - Project-level memory prompts
  - [cmd/](/.ai/cmd/) - Custom command prompts
  - [agents/](/.ai/agents/) - Sub-agent prompts
  - [meta/](/.ai/meta/) - Reference prompt writing standards
- [cli/](/cli/) - Command line tools
  - [rust/](/cli/rust/) - Rust core tools
- [hooks/](/hooks/) - Hook scripts
```
