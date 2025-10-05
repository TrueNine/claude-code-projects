# Agent CLI Prompt Management Solution

## Project Positioning
- Provide `npx -y @truenine/agent-cli` entry point for quick initialization or updates of `.ai/` prompt repository, with experience similar to `pnpm create vite`.
- All core capabilities implemented in Rust, TypeScript only handles parameter parsing and scheduling, ensuring performance and multi-platform consistency.
- Cover three types of content: memory prompts, sub-agent prompts, and shortcut command prompts, maintaining centralized and controllable prompt system within `.ai/` directory.

## User Scenarios
- Initialization: One-time generation of `.ai/` directory structure, creation of example memory, sub-agent, cmd prompts, configuration of multi-platform available scripts.
- Update: Detect repository status, apply latest templates or fragments, replace outdated files, keep backups.
- Cleanup: Identify invalid prompts and safely delete them, avoiding stale content interfering with team collaboration.
- Sync: Execute `npx -y @truenine/agent-cli sync` to push latest prompts to Codex and Claude Code usage scenarios.

## Tech Stack
- TypeScript (Entry Layer): Managed with `pnpm`, provides minimal CLI startup and parameter collection.
- Rust (Core Engine): Responsible for file scanning, diff analysis, writing, rollback, platform detection, exposed to TypeScript via `napi-rs`.
- Target Platforms: macOS (universal), Linux (x64, arm64), Windows (x64).

## Architecture
- CLI Frontend (TS): Parses subcommands like `init`, `update`, `compose`, `prune`, `sync`, forwards requests to Rust.
- Orchestrator (Rust): Executes template composition, validation, cleanup based on commands, outputs unified event logs.
- Storage Manager (Rust): Maintains `.ai/` directory tree and prompt index, supports snapshots, rollbacks, atomic writes.
- Template Registry (Rust): Built-in standard fragments and metadata, facilitating future introduction of remote repositories.

## Command Design
- `agent-cli init`
  - Generate `.ai/locale`, `.ai/user`, `.ai/project`, `.ai/cmd`, `.ai/sa` directories
  - Inject example memory prompts, sub-agent scripts, shortcut command templates
  - Create `agents.prompts.json` to record structure version
- `agent-cli compose <type>`
  - `type` supports `memory | sub-agent | cmd`
  - Rust reads fragment repository, interactively composes and writes to target files
- `agent-cli update`
  - Compare current version with template repository, apply incremental updates, automatically replace old prompts and write changelog
- `agent-cli prune`
  - Scan for orphaned prompts in `.ai/`, delete after confirmation
- `agent-cli sync`
  - Core command: Support `npx -y @truenine/agent-cli sync` to one-time startup CLI, merge local changes and sync to shared paths

## File Specifications
- Memory prompts: `.ai/locale/**/*.locale.md`, `.ai/user/**/*.md`, `.ai/project/**/*.md`
- Sub-agent prompts: `.ai/sa/**/*.md`
- Shortcut command prompts: `.ai/cmd/**/*.md`
- Configuration file: `agents.prompts.json` describes directory mapping, template versions, platform support
- Backups: `.agents/backups/<timestamp>/` saves pre-update state, supports `agent-cli rollback <timestamp>`

## Workflow
- Install dependencies: `pnpm install`, Rust dependencies managed via `cargo`
- Development mode: `pnpm dev` starts TypeScript watcher, `cargo watch -x "test" -x "build"` monitors core engine
- Quality assurance: `pnpm lint`, `pnpm test`, `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`
- Release: Rust compiles cross-platform binaries, TypeScript retains lightweight entry, published via npm as `@truenine/agent-cli`

## Multi-platform Strategy
- Rust uses `napi-rs` or `tauri bundler` to generate target platform binaries, distributed with npm package
- CLI runtime automatically detects platform, selects corresponding binary, ensures consistent Windows/macOS/Linux behavior

## Security and Rollback
- All writes use temporary file + rename to ensure atomicity
- Deletions default to moving to `.agents/trash`, only actually cleared when using `--force`
- Rollback commands use unified semantics across platforms, ensuring consistent team member experience

## Future Directions
- Remote template repositories: Support fragment synchronization from private Git repositories
- Fragment scoring: Recommend common combinations based on usage frequency
- GUI wrapper: Lightweight interface based on Rust core + Tauri
- DevOps integration: Provide `agent-cli verify` to check if `.ai/` structure complies with latest specifications