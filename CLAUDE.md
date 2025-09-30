# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a documentation and configuration repository for Claude Code workflows, focusing on:
- Claude Code configuration files and MCP server settings
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

docs/
  - `prompts/` - Prompt templates
    - `user/` - Global user prompts
    - `project/` - Project-level prompts
    - `slashcommands/` - Slash command prompts
    - `subagents/` - Sub-agent prompts
  - `metaprompts/` - Prompt writing standards
  - `qa/` - Q&A documentation
  - `references/` - Technical reference documentation
  - `other/` - Other documentation (build, Git, database, etc.)

## Important Configuration

### Claude Code Settings (.claude/settings.json)
- **Model**: `opusplan` - Dedicated planning model
- **Permission policy**:
  - Disabled: WebFetch, WebSearch, git/npm/docker commands
  - Allowed: MCP servers, basic Bash and file operations
- **MCP servers**:
  - `context7` - Get latest library documentation
  - `sequential-thinking` - Structured thinking
  - `memory` - Memory management
  - `deepwiki` - GitHub repository documentation
  - `grep` - Code search

### Project Conventions
- **File naming conventions**:
  - Chinese documentation: `*.locale.md`
  - English documentation: `*.md`
  - Maintain bilingual correspondence
- **Document organization principles**:
  - Categorize by functional modules
  - Maintain clear hierarchy
  - Facilitate quick lookup

## Main Functional Modules

### 1. Prompt Template System (docs/prompts/)
- **Global prompts**: General guidance applicable to all projects
- **Project templates**: Standard configuration for new projects
- **Slash commands**: Predefined tasks for quick execution
  - `/translate` - Chinese document translation
  - `/setup-project` - Project initialization
  - `/organize-docs` - Document organization and optimization
  - `/optimize-prompt` - Prompt optimization
  - `/doc-cc` - Claude Code official documentation
  - `/child-use` - Sub-project configuration

### 2. Q&A Documentation (docs/qa/)
- Frequently asked questions for development workflows
- Problem handling in real-world scenarios
- Best practices and experience summaries

### 3. Technical References (docs/references/)
- Reference documentation for various technologies and tools
- Practical guides and operation manuals
- Common solutions

## Workflows

### Documentation Management
1. **Create documentation**: Choose appropriate location based on content
2. **Localization**: Chinese content uses `.locale.md` suffix
3. **Translation**: Corresponding English version placed in same directory
4. **Update**: Keep bilingual content synchronized

### Prompt Development
1. **Design template**: Determine purpose and scope of prompts
2. **Write content**: Follow best practices
3. **Test and optimize**: Use in practice and collect feedback
4. **Version management**: Record change history

### Configuration Management
- **Claude Code configuration**: Modify `.claude/settings.json`
- **MCP servers**: Enable or disable as needed
- **Permission settings**: Follow security principles

## Usage Guide

### For New Users
1. Read `README.locale.md` to understand project overview
2. Check `docs/qa/qa.md` for common questions
3. Use slash commands as needed

### For Developers
1. Reference global configuration in `docs/prompts/user/`
2. Use `/setup-project` to initialize new projects
3. Regularly update and optimize prompts

### For Documentation Authors
1. Follow existing document structure
2. Maintain consistency in bilingual content
3. Use clear headings and categories

## Important Reminders

- **Focus on core**: Execute strictly as required, no more, no less
- **File management**: Do not create new files unless absolutely necessary
- **Prioritize editing**: Always prioritize editing existing files over creating new ones
- **Documentation principle**: Only create documentation files when explicitly requested