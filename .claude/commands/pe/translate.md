---
argument-hint: [ locale_markdown_file ] [ translation_description ]
allowed-tools: Read, Write, Glob, Grep, Bash
description: Translate Chinese localization memory prompt file to English memory prompt file, following consistent terminology and quality standards
---

Translate Chinese localization memory prompt file #$1 (.locale.md) to English memory prompt file while maintaining quality standards and terminology consistency.

# Task Execution Workflow

1. **Parse filename**:
  - **Priority matching special paths**, generate target file according to the following mapping:
    - `.docs/cmd/**/*.locale.md` -> `.claude/commands/**/*.md`
    - `.docs/sa/**/*.locale.md` -> `.claude/agents/***/*.md`
    - `.docs/CLAUDE-cmd.locale.md` -> `.docs/cmd/CLAUDE.md`
    - `.docs/CLAUDE-sa.locale.md` -> `.docs/sa/CLAUDE.md`
    - `.docs/CLAUDE-user.locale.md` -> `.docs/user/CLAUDE.md`
    - `.docs/CLAUDE-project.locale.md` -> `.docs/project/CLAUDE.md`
    - `.docs/CLAUDE.locale.md` -> `.docs/CLAUDE.md`
  - **When special paths don't match**, use general rule: `filename.locale.extension` -> `filename.extension`

2. **Check target file**:
  - Use `Search(pattern: "target_file")` to verify if target file exists
  - Pattern: Based on target path determined in step 2

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
  - No need to read existing target file (deleted in step 4)

7. **Error handling**:
  - If `Write` fails, immediately delete target file
  - Use `Bash(rm target_file)` to execute deletion
  - Restart process without attempting to fix




## Quality Standards

- **Terminology consistency**: Determine translation by comparing with glossary item by item, maintain consistent capitalization and punctuation with terminology table
- **Technical accuracy**: Confirm commands, parameters, file paths and other technical information are correct, avoid introducing new meanings
- **Format preservation**: Preserve title hierarchy, list indentation, tables and inline code as-is, do not add or remove blank lines
- **Whitespace preservation**: Strictly preserve blank lines and spaces, they are also part of the prompt
- **Link handling**: Update relative/absolute paths according to target document structure, ensure anchors are synchronized with filenames
- **Code integrity**: Keep code block content unchanged, only translate comments within blocks and verify statement alignment

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
