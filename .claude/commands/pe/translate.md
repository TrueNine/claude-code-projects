---
argument-hint: [ locale_markdown_file ] [ translation_description ]
allowed-tools: Read, Write, Glob, Grep, Bash
description: Translate Chinese localization memory prompt file to English memory prompt file while maintaining quality standards and terminology consistency
---

Translate Chinese localization memory prompt file #$1 (.locale.md) to English memory prompt file while maintaining quality standards and terminology consistency.

# Task Execution Workflow

1. **Parse filename**:
  - **Special location rules** (Check first):
    - `.docs/cmd/**/*.locale.md` -> `.claude/commands/**/*.md`
    - `.docs/sa/**/*.locale.md` -> `.claude/agents/***/*.md`
    - `.docs/CLAUDE-cmd.locale.md` -> `.docs/cmd/CLAUDE.md`
    - `.docs/CLAUDE-sa.locale.md` -> `.docs/sa/CLAUDE.md`
    - `.docs/CLAUDE-user.locale.md` -> `.docs/user/CLAUDE.md`
    - `.docs/CLAUDE-project.locale.md` -> `.docs/project/CLAUDE.md`
    - `.docs/CLAUDE.locale.md` -> `.docs/CLAUDE.md`
  - **Standard rule**: `filename.locale.extension` -> `filename.extension`

2. **Check target file**:
  - Use `Glob(pattern: "target_file")` to verify if target file exists
  - Pattern: Based on target path determined in step 1

3. **Delete existing file**:
  - If target file exists, use Bash tool to delete
  - Command: `Bash(rm filename.extension)` (Linux/Mac) or equivalent (Windows) command

4. **Read source file**: `Read($1)`

5. **Execute translation**:
  - Preserve Markdown structure and formatting
  - Apply consistent terminology from glossary
  - Keep code blocks unchanged and translate all comment content

6. **Write target file**:
  - Create new target file and write translated content
  - No need to read existing target file (deleted in step 3)

7. **Error handling**:
  - If `Write` fails, immediately delete target file
  - Use `Bash(rm target_file)` to execute deletion
  - Restart process without attempting to fix




## Quality Standards

- **Terminology consistency**: Strictly follow glossary
- **Technical accuracy**: Maintain precision of technical concepts
- **Format preservation**: Preserve all Markdown formatting
- **Link handling**: Update documentation links appropriately
- **Code integrity**: Keep code examples unchanged

```xml
<Examples description="Filename conversion">
  <Example>
    translate.locale.md -> translate.md
  </Example>
  <Example>
    setup.locale.md` -> setup.md
  </Example>
  <Example>
    config.locale.yaml -> config.yaml
  </Example>
</Examples>
```

```xml
<Examples description="Translation examples">
  <GoodExample userInput="请参阅文档">
    See documentation
  </GoodExample>
  <GoodExample userInput="配置文件">
    Configuration file
  </GoodExample>
  <GoodExample userInput="命令行工具">
    Command-line tool
  </GoodExample>
</Examples>
```