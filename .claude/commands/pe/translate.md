---
argument-hint: [ locale_markdown_file ] [ translation_description ]
allowed-tools: Read, Write, Glob, Grep, Bash
description: Translate Chinese localization memory prompt file to English memory prompt file while maintaining terminology and quality standards consistency
---

Translate Chinese localization memory prompt file `$1` (.locale.md) to English memory prompt file while maintaining established quality standards and terminology consistency.

# Task Execution Workflow
## [STEP-0] **Handle Directory Input**
- WHEN `$1` points to a directory, first count the files in that directory that meet translation rules to ensure the translation scope is clear
- WHEN `$1` ends with `/`, it can be directly recognized as a directory
- Divide tasks by file and call `pe:translate` agent concurrently in a multi-threaded manner to avoid context pollution and reduce overall time consumption
- IF `$1` starts with `@`, directly ignore the `@` symbol and use the remaining part as the actual path


## [STEP-1] **Parse Output Path**
**Priority match special paths**, and generate target files according to the following table:

| SOURCE FILE                                           | OUTPUT FILES                                                   |
|-------------------------------------------------------|----------------------------------------------------------------|
| [.ai/locale/`**/*.locale.md`](/.ai/locale/)           | `**/*.md`                                                      |
| [.ai/locale/`**/AGENTS.locale.md`](/.ai/locale/)      | `/AGENTS.md`, `/CLAUDE.md`                                     |
| [AGENTS.locale.md](/.ai/locale/AGENTS.locale.md)      | `AGENTS.md`, `CLAUDE.md`, `.cursor/rules/all.mdc`              |
| [README.locale.md](/.ai/locale/README.locale.md)      | `README.md`                                                    |
| [TODO.locale.md](/.ai/locale/)                        | `TODO.md`                                                      |
| [.ai/cmd/`**/*.locale.md`](/.ai/cmd/)                 | `.ai/out/.claude/commands/**/*.md`, `.claude/commands/**/*.md` |
| [.ai/agents/`**/*.locale.md`](/.ai/agents/)           | `.ai/out/.claude/agents/**/*.md`, `.claude/agents/**/*.md`     |
| [.ai/user/`**/*.locale.md`](/.ai/user/)               | `.ai/out/global/**/*.md`,                                      |
| [.ai/locale/meta/`**/*.locale.md`](/.ai/locale/meta/) | `.ai/meta/**/*.md`                                             |

WHEN special path mappings don't match, apply the general rule: `filename.locale.extension -> filename.extension`.

Where `<relative_path>` represents the directory structure after removing the `.ai/locale/` prefix from the source file.
```xml
<!DOCTYPE example SYSTEM "/.ai/meta/example-schema.dtd">
<example>.ai/locale/templates/AGENTS.locale.md -> [templates/AGENTS.md, templates/CLAUDE.md]</example>
```

**Directory translation example**
```xml
<!DOCTYPE example SYSTEM "/.ai/meta/example-schema.dtd">
<example description="Directory detected">
  <tooling name="Bash" params:command="find `$1` -name \"*.locale.md\" wc -l" />
  I will translate concurrently...
  <agent name="translate" message="Translate .ai/locale/arch/AGENTS.locale.md to [arch/AGENTS.md, arch/CLAUDE.md]" />
  <agent name="translate" message="Translate .ai/locale/AGENTS.locale.md to [AGENTS.md, CLAUDE.md]" />
  <agent name="translate" message="Translate .ai/locale/meta/example.locale.md to .ai/meta/example.md" />
</example>
```

## [STEP-2] **Check Target Files**
- Use `Search(pattern: "<target_file>")` to determine IF the target file already exists
- Use `Bash(command: "mkdir -p <target_directory>")` to create all required target directories

## [STEP-3] **Delete Old Files**
- IF the target file exists, THEN call `Bash(command: "rm <target_file>")` to clean up, ensuring clean subsequent writes

## [STEP-4] **Read Source File**
- Call `Read($1)` to get the source file content

## [STEP-5] **Execute Translation**
- Preserve Markdown structure and formatting
- Keep code block content unchanged, only translate Chinese comments or descriptions within them

## [STEP-6] **Write Target File**
- Create new target file and write translation results
- IF multiple target paths exist, THEN write to the first file first, THEN use `Bash(command: "cp -R <first_file> <target_file>")` to copy to remaining paths, avoiding discrepancies

## [STEP-7] **Error Handling**
- IF `Write` call fails, THEN immediately execute `Bash(command: "rm <target_file>")` to clean up
- Restart from step 1 after cleanup, refusing partial fixes

# Quality Standards
- **Terminology consistency**: Compare with the glossary item by item, ensure capitalization and punctuation match exactly
- **Technical accuracy**: Verify commands, parameters, file paths and other technical details, prevent introducing new meanings
- **Format preservation**: Preserve heading hierarchy, list indentation, tables and inline code, prohibit adding or removing blank lines
- **Whitespace fidelity**: Strictly preserve original spaces and blank lines, they are part of the prompt
- **Code integrity**: Keep code block content unchanged except translating comments, verify statement indentation is correct
- **Capitalization rules**: Certain keywords must be translated to all-uppercase form:
  - IF: conditional judgment keyword
  - WHEN: timing or condition keyword
  - THEN: result or execution keyword
  - ELSE: otherwise or alternative keyword

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd">
<examples description="Uppercase keyword translation">
  <good-example description="Correctly use all-uppercase keywords">
    IF the user provides a file path, THEN read the file content.
    WHEN validation fails, THEN return an error message.
    IF the condition is met, THEN execute the action, ELSE skip it.
  </good-example>

  <bad-example description="Incorrectly use lowercase or title case">
    If the user provides a file path, then read the file content.
    When validation fails, then return an error message.
    If the condition is met, Then execute the action, else skip it.
  </bad-example>
</examples>
```

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd">
<examples description="File path conversion">
  <example>.ai/cmd/pe/translate.locale.md -> [.ai/out/.claude/commands/pe/translate.md, .claude/commands/pe/translate.md]</example>
  <example>.ai/cmd/pe/setup.locale.md -> [.ai/out/.claude/commands/pe/setup.md, .claude/commands/pe/setup.md]</example>
  <example>.ai/agents/pe/translate.locale.md -> [.ai/out/.claude/agents/pe/translate.md, .claude/agents/pe/translate.md]</example>
  <example>.ai/user/cc.locale.md -> .ai/out/global/cc.md</example>
  <example>.ai/user/USER_AGENTS.locale.md -> .ai/out/global/USER_AGENTS.md</example>
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
