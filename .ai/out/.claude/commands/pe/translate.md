---
argument-hint: [ locale_markdown_file ] [ translation_description ]
allowed-tools: Read, Write, Glob, Grep, Bash
description: Translate Chinese localization memory prompt file `$1` (.locale.md) to English memory prompt file while maintaining established quality standards and terminology consistency
---

Translate Chinese localization memory prompt file `$1` (.locale.md) to English memory prompt file while maintaining established quality standards and terminology consistency.

# Task Execution Workflow
## [STEP-0] **Handle Directory Input**
- Extract parameter `$1` and optional parameter `$2` from `$ARGUMENTS`
- WHEN `$1` points to a directory, first count the files in that directory that meet translation rules to ensure the translation scope is clear
- WHEN `$1` ends with `/`, it can be directly recognized as a directory
- Divide tasks by file and CALL `pe:translate` agent concurrently in a multi-threaded manner to avoid context pollution and reduce overall time consumption
- IF `$1` starts with `@`, directly ignore the `@` symbol and use the remaining part as the actual path


## [STEP-1] **Parse Output Path**
Standardize the source path before table lookup:
- IF `$1` is located under `.ai/locale/`, MUST first remove the `.ai/locale/` prefix to get `<relative_path>`. Any target path is prohibited from carrying the `.ai/locale/` prefix.
- Use `<relative_path>` to match with the table below; IF special mapping is hit, generate files according to the corresponding output list.
- `AGENTS.locale.md` series always generates paired files based on `<relative_path>`, for example `.ai/locale/deployment/AGENTS.locale.md -> [deployment/AGENTS.md, deployment/CLAUDE.md]`.

**Priority match special paths**, and generate target files according to the following table:

| SOURCE FILE                                           | OUTPUT FILES                                                   |
|-------------------------------------------------------|----------------------------------------------------------------|
| [.ai/locale/`**/*.locale.md`](/.ai/locale/)           | `**/*.md`                                                      |
| [.ai/locale/`**/AGENTS.locale.md`](/.ai/locale/)      | `/**/AGENTS.md`, `/**/CLAUDE.md`                               |
| [AGENTS.locale.md](/.ai/locale/AGENTS.locale.md)      | `AGENTS.md`, `CLAUDE.md`, `.cursor/rules/project.mdc`          |
| [README.locale.md](/.ai/locale/README.locale.md)      | `README.md`                                                    |
| [TODO.locale.md](/.ai/locale/)                        | `TODO.md`                                                      |
| [.ai/cmd/`**/*.locale.md`](/.ai/cmd/)                 | `.ai/out/.claude/commands/**/*.md`, `.claude/commands/**/*.md` |
| [.ai/agents/`**/*.locale.md`](/.ai/agents/)           | `.ai/out/.claude/agents/**/*.md`, `.claude/agents/**/*.md`     |
| [.ai/GLOBAL.locale.md](/.ai/GLOBAL.locale.md)         | `.ai/out/GLOBAL.md`                                            |
| [.ai/locale/meta/`**/*.locale.md`](/.ai/locale/meta/) | `.ai/meta/**/*.md`                                             |

WHEN special path mappings don't match, apply the general rule: `filename.locale.extension -> filename.extension`.

Where `<relative_path>` represents the directory structure after removing the `.ai/locale/` prefix from the source file.
````xml
<!DOCTYPE example SYSTEM "/.ai/meta/example-schema.dtd">
<example>.ai/locale/templates/AGENTS.locale.md -> [templates/AGENTS.md, templates/CLAUDE.md]</example>
````

**Directory translation example**
````xml
<!DOCTYPE example SYSTEM "/.ai/meta/example-schema.dtd">
<example description="Directory detected">
  <tooling name="Bash" params:command="find $1 -name \"*.locale.md\" wc -l" />
  I will translate concurrently...
  <agent name="translate" message="Translate .ai/locale/arch/AGENTS.locale.md to [arch/AGENTS.md, arch/CLAUDE.md]" />
  <agent name="translate" message="Translate .ai/locale/AGENTS.locale.md to [AGENTS.md, CLAUDE.md]" />
  <agent name="translate" message="Translate .ai/locale/meta/example.locale.md to .ai/meta/example.md" />
</example>
````

## [STEP-2] **Prepare Target Path**
- ALWAYS CALL `Bash(command: "mkdir -p <target_directory>")`, even if the directory already exists
- ALWAYS CALL `Bash(command: "rm -f <target_file>")` to clean up old files, ignoring any deletion errors
- ALWAYS CALL `TodoWrite` tool to record complete todo list and maintain status synchronization during task execution

## [STEP-3] **Read and Translate Source File**
- CALL `Read($1)` to get the source file content
- Preserve Markdown structure and formatting
- Keep code block content unchanged, only translate Chinese comments or descriptions within them

## [STEP-4] **Write Target File**
- Create new target file and write translation results
- IF multiple target paths exist, THEN write to the first file first, THEN use `Bash(command: "cp -R <first_file> <target_file>")` to copy to remaining paths, avoiding discrepancies
- IF output target is located under `.cursor/rules/` directory (i.e., `.cursor/rules/**/*.mdc`), MUST insert the following YAML header at the beginning of the file before writing, and omission is prohibited:

  ```
  ---
  alwaysApply: true
  ---
  ```

## [STEP-5] **Error Handling**
- IF `Write` call fails, THEN immediately EXECUTE `Bash(command: "rm <target_file>")` to clean up
- Restart from step 1 after cleanup, refusing partial fixes




# Quality Standards
- **Terminology consistency**: Compare with the glossary item by item, ensure capitalization and punctuation match exactly
- **Technical accuracy**: Verify commands, parameters, file paths and other technical details, prevent introducing new meanings
- **Format preservation**: Preserve heading hierarchy, list indentation, tables and inline code, prohibit adding or removing blank lines
- **Whitespace fidelity**: Strictly preserve original spaces and blank lines, they are part of the prompt
- **Code integrity**: Keep code block content unchanged except translating comments, verify statement indentation is correct
- **Capitalization rules**: Certain keywords MUST be translated to all-uppercase form:
  - IF: conditional judgment keyword
  - WHEN: timing or condition keyword
  - THEN: result or execution keyword
  - ELSE: otherwise or alternative keyword
  - CALL: invocation keyword
  - EXECUTE: execution keyword
  - RUN: running keyword
  - MUST: mandatory keyword
  - SHOULD: recommendation keyword
  - MAY: permission keyword
  - NEVER: prohibition keyword
  - ALWAYS: absolute keyword

````xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd">
<examples description="Uppercase keyword translation">
  <good-example description="Correctly use all-uppercase keywords">
    IF the user provides a file path, THEN read the file content.
    WHEN validation fails, THEN return an error message.
    IF the condition is met, THEN execute the action, ELSE skip it.
    CALL the translation tool to process the file.
    EXECUTE the command and RUN the tests.
    You MUST follow these rules and NEVER skip validation.
    You SHOULD use the standard format and MAY add comments.
    ALWAYS check the file existence before writing.
  </good-example>

  <bad-example description="Incorrectly use lowercase or title case">
    If the user provides a file path, then read the file content.
    When validation fails, then return an error message.
    If the condition is met, Then execute the action, else skip it.
    Call the translation tool to process the file.
    Execute the command and run the tests.
    You must follow these rules and never skip validation.
    You should use the standard format and may add comments.
    Always check the file existence before writing.
  </bad-example>
</examples>
````

````xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd">
<examples description="File path conversion">
  <example>.ai/cmd/pe/translate.locale.md -> [.ai/out/.claude/commands/pe/translate.md, .claude/commands/pe/translate.md]</example>
  <example>.ai/cmd/pe/setup.locale.md -> [.ai/out/.claude/commands/pe/setup.md, .claude/commands/pe/setup.md]</example>
  <example>.ai/agents/pe/translate.locale.md -> [.ai/out/.claude/agents/pe/translate.md, .claude/agents/pe/translate.md]</example>
  <example>.ai/GLOBAL.locale.md -> .ai/out/GLOBAL.md</example>
  <example>.ai/locale/AGENTS.locale.md -> [AGENTS.md, CLAUDE.md]</example>
  <example>.ai/locale/templates/AGENTS.locale.md -> [templates/AGENTS.md, templates/CLAUDE.md]</example>
  <example>.ai/locale/README.locale.md -> README.md</example>
  <example>.ai/locale/TODO.locale.md -> TODO.md</example>
  <example>.ai/locale/.ai/cmd/AGENTS.locale.md -> [.ai/cmd/AGENTS.md, .ai/cmd/CLAUDE.md]</example>
  <example>.ai/locale/meta/examples.locale.md -> .ai/meta/examples.md</example>
  <example>.ai/locale/meta/prompt.locale.md -> .ai/meta/prompt.md</example>
  <example>.ai/locale/meta/AGENTS.locale.md -> [.ai/meta/AGENTS.md, .ai/meta/CLAUDE.md]</example>
</examples>
````