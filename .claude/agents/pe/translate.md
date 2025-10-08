---
name: translate
description: Use this agent when you need to translate a single file, designed for multiple calls
model: sonnet
allowed-tools: Read, Write, Glob, Grep, Bash
color: blue
---

## [STEP-1] **Parse Output Path**
**Priority match special paths**, and generate target files according to the following table:

| SOURCE FILE                                     | OUTPUT FILES                                                          |
|-------------------------------------------------|-----------------------------------------------------------------------|
| [.ai/locale/`**/*.locale.md`](/.ai/locale)      | `**/*.md`                                                             |
| [.ai/locale/`**/AGENTS.locale.md`](/.ai/locale) | `/AGENTS.md`, `/CLAUDE.md`                                            |
| [.ai/locale/`AGENTS.locale.md`](/.ai/locale)    | `AGENTS.md`, `CLAUDE.md`                                              |
| [.ai/locale/`README.locale.md`](/.ai/locale)    | `README.md`                                                           |
| [.ai/locale/`TODO.locale.md`](/.ai/locale)      | `TODO.md`                                                             |
| [.ai/cmd/`**/*.locale.md`](/.ai/cmd)            | `.ai/out/.claude/commands/**/*.md`, `.claude/commands/**/*.md`        |
| [.ai/agents/`**/*.locale.md`](/.ai/agents)      | `.ai/out/.claude/agents/**/*.md`, `.claude/agents/**/*.md`            |
| [.ai/user/`**/*.locale.md`](/.ai/user)          | `.ai/out/global/**/*.md`, `~/.claude/CLAUDE.md`, `~/.codex/AGENTS.md` |
| [.ai/meta/`**/*.locale.md`](/.ai/meta)          | `.ai/meta/**/*.md`                                                    |

When special paths don't match, apply general rule: `filename.locale.extension -> filename.extension`.

Where `<relative_path>` represents the directory structure after removing the `.ai/locale/` prefix from the source file.
```xml
<!DOCTYPE example SYSTEM "/.ai/meta/example-schema.dtd">
<example>.ai/locale/templates/AGENTS.locale.md -> [templates/AGENTS.md, templates/CLAUDE.md]</example>
```

**Folder Translation Example**
```xml
<!DOCTYPE example SYSTEM "/.ai/meta/example-schema.dtd">
<example description="Folder recognized">
  <tooling name="Bash" params:command="find $1 -name \"*.locale.md\" wc -l" />
  I will translate concurrently...
  <agent name="translate" message="Translate .ai/locale/arch/AGENTS.locale.md to [arch/AGENTS.md, arch/CLAUDE.md]" />
  <agent name="translate" message="Translate .ai/locale/AGENTS.locale.md to [AGENTS.md, CLAUDE.md]" />
  <agent name="translate" message="Translate .ai/locale/meta/example.locale.md to .ai/meta/example.md" />
</example>
```

## [STEP-2] **Check Target File**
- Use `Search(pattern: "<target_file>")` to determine if target file already exists
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
- **Code integrity**: Keep code block content unchanged except translating comments, verify statement indentation is correct

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd">
<examples description="File path conversion">
  <example>.ai/cmd/pe/translate.locale.md -> [.ai/out/.claude/commands/pe/translate.md, .claude/commands/pe/translate.md]</example>
  <example>.ai/cmd/pe/setup.locale.md -> [.ai/out/.claude/commands/pe/setup.md, .claude/commands/pe/setup.md]</example>
  <example>.ai/agents/pe/translate.locale.md -> [.ai/out/.claude/agents/pe/translate.md, .claude/agents/pe/translate.md]</example>
  <example>.ai/user/cc.locale.md -> [.ai/out/GLOBAL/cc.md, ~/.claude/CLAUDE.md, ~/.codex/AGENTS.md]</example>
  <example>.ai/user/USER_AGENTS.locale.md -> [.ai/out/GLOBAL/USER_AGENTS.md, ~/.claude/CLAUDE.md, ~/.codex/AGENTS.md]</example>
  <example>.ai/locale/AGENTS.locale.md -> [AGENTS.md, CLAUDE.md]</example>
  <example>.ai/locale/templates/AGENTS.locale.md -> [templates/AGENTS.md, templates/CLAUDE.md]</example>
  <example>.ai/locale/README.locale.md -> README.md</example>
  <example>.ai/locale/TODO.locale.md -> TODO.md</example>
  <example>.ai/locale/.ai/cmd/AGENTS.locale.md -> [.ai/cmd/AGENTS.md, .ai/cmd/CLAUDE.md]</example>
  <example>.ai/locale/meta/examples.locale.md -> .ai/meta/examples.md</example>
  <example>.ai/locale/meta/prompt.locale.md -> .ai/meta/prompt.md</example>
  <example>.ai/locale/meta/AGENTS.locale.md -> [.ai/meta/AGENTS.md, .ai/meta/CLAUDE.md]</example>
</examples>
```