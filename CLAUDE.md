# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a documentation and configuration repository for `AI Agent` workflows, focusing on:
- Prompt engineering and template library
- Q&A documentation for development workflows
- Technical references and best practices
- Multilingual documentation management (Chinese localization + English translation)

## Project Features

- **Documentation-only repository**: No source code or build process
- **Bilingual documentation**: Chinese content uses `.locale.md` suffix, English content uses `.md` suffix
- **Modular organization**: Documents are hierarchically organized by function and use case
- **Utility toolkit**: Contains various practical slash commands and prompt templates

## Documentation Structure

```md
`.docs/`
  - `user/` - Global user prompts
  - `project/` - Project-level prompts
  - `cmd/` - Custom command prompts
  - `sa/` - Sub-agent prompts
  - `metaprompts/` - Prompt writing standards
```

## Important Configuration

### Project Conventions
- **File naming conventions**:
  - Chinese documentation: `*.locale.md`
  - English documentation: `*.md`
  - Maintain bilingual correspondence
- **Document organization principles**:
  - Categorize by functional modules
  - Maintain clear hierarchy
  - Facilitate quick lookup

## Workflows

### Documentation Management
1. **Create documentation**: Choose appropriate location based on content
2. **Localization**: Chinese content uses `**/*.locale.md` suffix
3. **Translation**: Corresponding English version placed in same directory
4. **Update**: Keep bilingual content synchronized

### Prompt Development
1. **Design template**: Determine purpose and scope of prompts
2. **Write content**: Follow best practices
3. **Test and optimize**: Use in practice and collect feedback
4. **Version management**: Record change history
