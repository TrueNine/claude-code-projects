---
argument-hint: [ locale_markdown_file ] [ translation_description ]
allowed-tools: Read, Write, Glob, Grep, Bash
description: Translate Chinese localization memory prompt file to English memory prompt file, maintaining terminology consistency and quality standards
---

Translate Chinese localization memory prompt file #$1 (.locale.md) to English memory prompt file, while maintaining established quality standards and terminology consistency.

# Task Execution Workflow
## [STEP-1] **Parse Output Path**
**Priority match special paths**, generate target files according to the following table:

| Source file path                            | Output file path                                                                 |
|---------------------------------------------|---------------------------------------------------------------------------------|
| `.ai/locale/**/*.locale.md`                 | `**/*.md`                                                                       |
| `.ai/locale/**/AGENTS.locale.md`            | `<relative_path>/AGENTS.md`, `<relative_path>/CLAUDE.md`                         |
| `.ai/locale/AGENTS.locale.md`              | `AGENTS.md`, `CLAUDE.md`                                                        |
| `.ai/locale/README.locale.md`              | `README.md`                                                                     |
| `.ai/cmd/**/*.locale.md`                   | `.claude/commands/**/*.md`, `.ai/out/.claude/commands/**/*.md`                 |
| `.ai/sa/**/*.locale.md`                    | `.claude/subagents/**/*.md`, `.ai/out/.claude/subagents/**/*.md`               |
| `.ai/user/**/*.locale.md`                  | `~/.claude/CLAUDE.md`, `~/.codex/AGENTS.md`, `.ai/out/global/**/*.md`           |
| `.ai/locale/meta/**/*.locale.md`           | `.ai/meta/**/*.md`                                                              |

**When special paths don't match**, apply general rule: `filename.locale.extension` -> `filename.extension`.

Where `<relative_path>` represents the directory structure after removing the `.ai/locale/` prefix from the source file.
```xml
<Example description="">.ai/locale/templates/AGENTS.locale.md -> [templates/AGENTS.md, templates/CLAUDE.md]</Example>
```

## [STEP-2] **Check Target File**
- Use `Glob(pattern: "<target_file>")` to verify if target file exists
- Use `Bash(command: "mkdir -p <target_directory>")` to create all required target directories

## [STEP-3] **Delete Old Files**
- If target file exists, call `Bash(command: "rm <target_file>")` to clean up, ensuring clean subsequent writes

## [STEP-4] **Read Source File**
- Call `Read($1)` to get source file content

## [STEP-5] **Execute Translation**
- Preserve Markdown structure and formatting
- Keep code block content unchanged, only translate Chinese comments or descriptions within them

## [STEP-6] **Write Target File**
- Create new target file and write translation results
- If multiple target paths exist, write to first file first, then use `Bash(command: "cp -R <first_file> <target_file>")` to copy to remaining paths, avoiding discrepancies

## [STEP-7] **Error Handling**
- If `Write` call fails, immediately execute `Bash(command: "rm <target_file>")` to clean up
- Restart from step 1 after cleanup, refusing partial fixes

# Quality Standards
- **Terminology consistency**: Compare with glossary item by item, ensure capitalization and punctuation match exactly
- **Technical accuracy**: Verify commands, parameters, file paths and other technical details, prevent introducing new meanings
- **Format preservation**: Preserve heading hierarchy, list indentation, tables and inline code, prohibit adding or removing blank lines
- **Whitespace fidelity**: Strictly preserve original spaces and blank lines, they are part of the prompt
- **Link synchronization**: Update relative/absolute paths according to target document structure, ensure anchors are synchronized with filenames
- **Code integrity**: Keep code block content unchanged except translating comments, verify statement indentation is correct

```xml
<Examples description="File path conversion">
  <Example>.ai/cmd/pe/translate.locale.md -> [.claude/commands/pe/translate.md, .ai/out/.claude/commands/pe/translate.md]</Example>
  <Example>.ai/cmd/pe/setup.locale.md -> [.claude/commands/pe/setup.md, .ai/out/.claude/commands/pe/setup.md]</Example>
  <Example>.ai/user/cc.locale.md -> [.ai/out/GLOBAL/cc.md, ~/.claude/CLAUDE.md, ~/.codex/AGENTS.md]</Example>
  <Example>.ai/user/USER_AGENTS.locale.md -> [.ai/out/GLOBAL/USER_AGENTS.md, ~/.claude/CLAUDE.md, ~/.codex/AGENTS.md]</Example>
  <Example>.ai/locale/AGENTS.locale.md -> [AGENTS.md, CLAUDE.md]</Example>
  <Example>.ai/locale/templates/AGENTS.locale.md -> [templates/AGENTS.md, templates/CLAUDE.md]</Example>
  <Example>.ai/locale/README.locale.md -> README.md</Example>
  <Example>.ai/locale/.ai/cmd/AGENTS.locale.md -> [.ai/cmd/AGENTS.md, .ai/cmd/CLAUDE.md]</Example>
  <Example>.ai/locale/meta/examples.locale.md -> .ai/meta/examples.md</Example>
  <Example>.ai/locale/meta/prompt.locale.md -> .ai/meta/prompt.md</Example>
  <Example>.ai/locale/meta/AGENTS.locale.md -> [.ai/meta/AGENTS.md, .ai/meta/CLAUDE.md]</Example>
</Examples>
```