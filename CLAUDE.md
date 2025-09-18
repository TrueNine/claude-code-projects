# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a documentation and configuration repository for Claude Code workflows, containing:
- Claude Code configuration files and hooks
- Documentation templates and prompts
- QA documentation for development workflows
- Reference materials for development best practices

## Key Commands

This repository doesn't have traditional build/test commands as it's primarily documentation-focused. The main development happens through:

### Hook Development (TypeScript)
- **Location**: `.claude/hooks/`
- **Package Manager**: `pnpm` (specified in package.json)
- **Run hooks**: Hooks are automatically triggered by Claude Code events
- **Development**: Edit TypeScript files in `.claude/hooks/src/`

### Documentation Management
- **No build commands** - Documentation files are managed directly
- **Structure**: Well-organized in `docs/` directory with locale-specific content

## Architecture & Code Organization

### Configuration Structure
- **`.claude/`** - Claude Code configuration and hooks
  - `settings.json` - Main Claude Code settings (model: opusplan, permissions, MCP servers)
  - `hooks/` - TypeScript hooks for file processing automation
  - `hooks/main.ts` - Main hook entry point for frontend file linting/formatting

### Documentation Architecture
- **`docs/`** - Main documentation directory
  - `prompts/` - Prompt templates (both locale and English versions)
    - `locale/` - Chinese locale prompts
    - `output/` - English translated prompts
  - `qa/` - Q&A documentation for development workflows
  - `references/` - Technical reference documentation
  - `other/` - Various documentation (build, Git, database, etc.)

### Hook System
The repository includes a sophisticated TypeScript hook system (`.claude/hooks/main.ts`) that:
- Automatically processes frontend files on save/edit
- Runs prettier, eslint, and TypeScript checks in parallel
- Provides detailed logging to `.claude/hook-debug.log`
- Only processes files in `frontend/` directories

## Important Configuration

### Claude Code Settings
- **Model**: `opusplan` (configured in `.claude/settings.json`)
- **MCP Servers**: context7, sequential-thinking, memory, deepwiki, grep
- **Permissions**: Restricted git/npm/docker commands, allows MCP and basic tools

### Project Conventions
- **Language**: Bilingual (Chinese locale files with English translations)
- **File Naming**: Uses `.locale.md` suffix for Chinese content
- **Documentation Structure**: Hierarchical organization in `docs/` directory

## Development Workflow

Since this is a documentation repository:

1. **Editing Documentation**: Direct file editing, no build step required
2. **Hook Development**: Edit TypeScript in `.claude/hooks/src/`, hooks auto-reload
3. **Configuration Changes**: Update `.claude/settings.json` as needed
4. **Content Management**: Follow existing locale/translation patterns

## Important Instructions

Do what has been asked; nothing more, nothing less.
NEVER create files unless they're absolutely necessary for achieving your goal.
ALWAYS prefer editing an existing file to creating a new one.
NEVER proactively create documentation files (*.md) or README files. Only create documentation files if explicitly requested by the User.