This file provides guidance for `AI Agent` working in this repository.




# Project Overview

This is a documentation and configuration repository for `AI Agent` workflows, focusing on:
- Prompt engineering and template libraries
- Q&A documentation for development workflows
- Technical references and best practice materials
- Multilingual documentation management (Chinese localization + English translation)




# Project Features

- **Pure Documentation Repository**: No source code or build processes
- **Bilingual Documentation**: Chinese content uses `.locale.md` suffix, English content uses `.md` suffix
- **Modular Organization**: Documents are organized hierarchically by function and use case
- **Practical Toolset**: Includes various practical slash commands and prompt templates




# Documentation Structure

```md
`.jiumate_ai/` - Similar to src, a source prompt working directory
  - `.locale/` - Current project mapping memory prompts
  - `user/` - Global user memory prompts
  - `project/` - Project-level memory prompts
  - `cmd/` - Custom command prompts
  - `sa/` - Sub-agent prompts
  - `metaprompts/` - Prompt writing standards
```




# Workflow

## Document Management
1. **Create Documents**: Choose appropriate location based on content
2. **Localization**: Use `**/*.locale.md` suffix for Chinese content
3. **Translation**: Place corresponding English versions in the same directory
4. **Updates**: Keep bilingual content synchronized

## Prompt Development
1. **Design Templates**: Determine prompt purpose and scope
2. **Write Content**: Follow best practices
3. **Test and Optimize**: Use in practice and collect feedback
4. **Version Management**: Record change history