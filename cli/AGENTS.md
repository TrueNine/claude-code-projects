# Agent CLI Prompt Management Plan

## Project Positioning
- Provide the `npx -y @truenine/agent-cli` entry to quickly initialize or update the `__ai/` prompt repository with an experience similar to `pnpm create vite`.
- Implement all core capabilities in Rust while TypeScript only handles argument parsing and orchestration to ensure performance and cross-platform consistency.
- Cover memory prompts, sub-agent prompts, and shortcut command prompts so the prompt system under the `__ai/` directory stays centralized and controllable.

## User Scenarios
- Initialization: Generate the `__ai/` directory structure in one go, create sample memory, sub-agent, and cmd prompts, and configure scripts usable across platforms.
- Update: Detect repository status, apply the latest templates or snippets, replace outdated files, and keep backups.
- Cleanup: Identify invalid prompts and remove them safely to keep outdated content from disrupting team collaboration.
- Synchronization: RUN `npx -y @truenine/agent-cli sync` to push the latest prompts to Codex and Claude Code usage scenarios.

## Tech Stack
- TypeScript (entry layer): Managed with `pnpm`, provides the minimal CLI bootstrap and parameter collection.
- Rust (core engine): Handles file scanning, diff analysis, writing, rollback, and platform detection, exposed to TypeScript through `napi-rs`.
- Target platforms: macOS (universal), Linux (x64, arm64), Windows (x64).

## Architecture
- CLI Frontend (TS): Parses subcommands such as `init`, `update`, `compose`, `prune`, and `sync`, then forwards requests to Rust.
- Orchestrator (Rust): Executes template composition, validation, and cleanup based on the command and outputs unified event logs.
- Storage Manager (Rust): Maintains the `__ai/` directory tree and prompt index, supporting snapshots, rollback, and atomic writes.
- Template Registry (Rust): Bundles standard snippets and metadata, making it easier to introduce remote repositories later.

## Command Design
- `agent-cli init`
  - Generate the `__ai/locale`, `__ai/user`, `__ai/project`, `__ai/cmd`, `__ai/sa`, and other directories.
  - Inject sample memory prompts, sub-agent scripts, and shortcut command templates.
  - Create `agents.prompts.json` to record the structure version.
- `agent-cli compose <type>`
  - `type` supports `memory | sub-agent | cmd`.
  - Rust reads the snippet repository, composes content interactively, and writes it to the target file.
- `agent-cli update`
  - Compare the current version with the template repository, apply incremental updates, automatically replace outdated prompts, and write a changelog.
- `agent-cli prune`
  - Scan for orphaned prompts under `__ai/`, prompt for confirmation, and delete them.
- `agent-cli sync`
  - Core command: RUN `npx -y @truenine/agent-cli sync` to start the CLI, merge local changes, and synchronize them to the shared path.

## File Conventions
- Memory prompts: `__ai/locale/**/*.locale.md`, `__ai/user/**/*.md`, `__ai/project/**/*.md`
- Sub-agent prompts: `__ai/sa/**/*.md`
- Shortcut command prompts: `__ai/cmd/**/*.md`
- Configuration file: `agents.prompts.json` describes directory mappings, template versions, and platform support.
- Backups: `.agents/backups/<timestamp>/` saves the state before updates and supports `agent-cli rollback <timestamp>`.

## Workflow
- Install dependencies: `pnpm install`; Rust dependencies are managed through `cargo`.
- Development mode: RUN `pnpm dev` to start the TypeScript watcher and RUN `cargo watch -x "test" -x "build"` to monitor the core engine.
- Quality assurance: RUN `pnpm lint`, RUN `pnpm test`, RUN `cargo fmt --check`, RUN `cargo clippy -- -D warnings`, RUN `cargo test`.
- Release: Rust compiles cross-platform binaries, TypeScript keeps the lightweight entry point, and the package is published to npm as `@truenine/agent-cli`.

## Multi-platform Strategy
- Rust uses `napi-rs` or `tauri bundler` to produce binaries for target platforms, distributed with the npm package.
- The CLI runtime detects the platform automatically, selects the corresponding binary, and ensures consistent behavior across Windows, macOS, and Linux.

## Safety and Rollback
- All writes use temporary files plus rename to guarantee atomicity.
- Deletions default to moving content into `.agents/trash`, and only remove it permanently when `--force` is specified.
- The rollback command keeps unified semantics across platforms to give team members a consistent experience.

## Future Directions
- Remote template repository: Support synchronizing snippets from private Git repositories.
- Snippet scoring: Recommend frequently used combinations based on usage frequency.
- GUI wrapper: Provide a lightweight interface via the Rust core plus Tauri.
- DevOps integration: Offer `agent-cli verify` to check whether the `__ai/` structure satisfies the latest specifications.
