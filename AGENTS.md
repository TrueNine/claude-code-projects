# Repository Guidelines

## Project Structure & Module Organization
This repository curates Claude Code configuration, reference prompts, and localisation assets. English-first docs live in `docs/` (notably `docs/references/` and `docs/other/` for technology guides); their Chinese counterparts use the same name with a `.locale.md` suffix, often sourced from `.docs/`. Automation collateral resides in `.claude/`: `agents/` hosts persona briefs, `commands/` stores slash-command specs, and `hooks/` contains the TypeScript hook that lints/formats prospective `frontend/` contributions. Keep new material co-located with its translated pair and mirror the existing folder taxonomy.

## Build, Test, and Development Commands
Most updates are Markdown-only and require no build. When touching the hook, run `cd .claude/hooks && pnpm install` once, then pipe sample payloads into `pnpm tsx main.ts`, for example:
`echo '{"session_id":"dev","tool_input":{"file_path":"frontend/src/App.tsx"}}' | pnpm tsx main.ts`.
If you do add a `frontend/` workspace, rely on its local `pnpm prettier --write` and `pnpm eslint --fix` scripts, because the hook will call them automatically.

## Coding Style & Naming Conventions
Follow `.editorconfig`: UTF-8, LF endings, two-space indentation, final newline. Use sentence case headings, concise bullet lists, and keep Markdown lines under 160 characters. Name bilingual files as `<slug>.md` (English) and `<slug>.locale.md` (Chinese). TypeScript within `.claude/hooks` prefers ES modules, async/await utilities, and explicit return types; keep logging consistent with `logger.ts`.

## Testing Guidelines
No automated suite exists yet, so validate changes manually. For documentation edits, render Markdown locally and cross-check both language variants. For hook updates, craft fixture JSON inputs that cover success, failure, and missing-path scenarios, run them through `pnpm tsx main.ts`, and record observed output in the PR description. When adding new automation, include lightweight assertions or CLI smoke checks.

## Commit & Pull Request Guidelines
Adopt the existing `type(scope): summary` pattern (`docs(cmd): refine translate workflow`). Keep subjects under 72 characters and expand context in the body when necessary. PRs should link any tracking issues, list affected documents or commands, describe validation steps, and attach before/after snippets or screenshots when formatting changes are user-visible. Flag untranslated copies so reviewers can sync locales before merging.

## Agent-Specific Notes
Claude-facing guides (`CLAUDE*.md` and `.docs/*`) inform downstream agents, so update both the operational instructions and their locale variants together. When introducing new automations, document invocation expectations inside `.claude/commands` to keep tools discoverable.
