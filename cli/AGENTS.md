# Agent CLI Prompt Management Solution

## Project Positioning
- Provide `npx -y @truenine/agent-cli` entry point for quick initialization or updates of `.ai/` prompt repositories, with user experience similar to `pnpm create vite`.
- All core capabilities are implemented in Rust, with TypeScript responsible only for parameter parsing and scheduling, ensuring performance and multi-platform consistency.
- Cover three types of content: memory prompts, sub-agent prompts, and shortcut command prompts, keeping the `.ai/` directory prompt system centralized and controllable.

## User Scenarios
- **Initialization**: One-time generation of `.ai/` directory structure, creating example memory, sub-agent, and cmd prompts, configuring multi-platform usable scripts.
- **Update**: Detect repository status, apply latest templates or snippets, replace outdated files, preserve backups.
- **Cleanup**: Identify invalid prompts and safely delete them, avoiding interference with team collaboration from stale content.
- **Sync**: Execute `npx -y @truenine/agent-cli sync` to push latest prompts to Codex and Claude Code usage scenarios.

## Technology Stack
- **TypeScript (Entry Layer)**: Managed using `pnpm`, providing minimal CLI startup and parameter collection.
- **Rust (Core Engine)**: Responsible for file scanning, difference analysis, writing, rollback, platform detection, exposed to TypeScript through `napi-rs`.
- **Target Platforms**: macOS (universal), Linux (x64, arm64), Windows (x64).

## Architecture
- **CLI Frontend (TS)**: Parse subcommands like `init`, `update`, `compose`, `prune`, `sync`, forwarding requests to Rust.
- **Orchestrator (Rust)**: Execute template composition, validation, cleanup according to commands, output unified event logs.
- **Storage Manager (Rust)**: Maintain `.ai/` directory tree and prompt index, supporting snapshots, rollbacks, atomic writes.
- **Template Registry (Rust)**: Built-in standard snippets and metadata, facilitating future remote repository integration.

## Command Design
- `agent-cli init`
  - Generate `.ai/locale`, `.ai/user`, `.ai/project`, `.ai/cmd`, `.ai/sa` and other directories
  - Inject example memory prompts, sub-agent scripts, shortcut command templates
  - Create `agents.prompts.json` to record structure version
- `agent-cli compose <type>`
  - `type` supports `memory | sub-agent | cmd`
  - Rust reads snippet repository, interactively composes and writes to target files
- `agent-cli update`
  - Compare current version with template repository, apply incremental updates, automatically replace old prompts and write changelog
- `agent-cli prune`
  - Scan `.ai/` for orphaned prompts, delete after confirmation
- `agent-cli sync`
  - Core command: Supports `npx -y @truenine/agent-cli sync` to one-time launch CLI, merge local changes and sync to shared paths

## File Specifications
- **Memory Prompts**: `.ai/locale/**/*.locale.md`, `.ai/user/**/*.md`, `.ai/project/**/*.md`
- **Sub-agent Prompts**: `.ai/sa/**/*.md`
- **Shortcut Command Prompts**: `.ai/cmd/**/*.md`
- **Configuration File**: `agents.prompts.json` describes directory mapping, template versions, platform support
- **Backups**: `.agents/backups/<timestamp>/` saves pre-update state, supports `agent-cli rollback <timestamp>`

## Workflow
- **Install Dependencies**: `pnpm install`, Rust dependencies managed through `cargo`
- **Development Mode**: `pnpm dev` starts TypeScript watcher, `cargo watch -x "test" -x "build"` monitors core engine
- **Quality Assurance**: `pnpm lint`, `pnpm test`, `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`
- **Release**: Rust compiles cross-platform binaries, TypeScript retains lightweight entry, published through npm as `@truenine/agent-cli`

## Multi-platform Strategy
- Rust uses `napi-rs` or `tauri bundler` to generate target platform binaries, distributed with npm package
- CLI runtime automatically detects platform, selects corresponding binary, ensuring consistent Windows/macOS/Linux behavior

## Security and Rollback
- All writes use temporary files + rename to guarantee atomicity
- Deletion defaults to moving to `.agents/trash`, only truly cleared when using `--force`
- Rollback commands use unified semantics across multiple platforms, ensuring consistent team member experience

## Future Directions
- **Remote Template Repository**: Support syncing snippets from private Git repositories
- **Snippet Scoring**: Recommend commonly used combinations based on usage frequency
- **GUI Wrapper**: Leverage Rust core + Tauri to provide lightweight interface
- **DevOps Integration**: Provide `agent-cli verify` to check if `.ai/` structure complies with latest specifications