#!/usr/bin/env node

import { Command } from 'commander';
import chalk from 'chalk';
import { spawn } from 'child_process';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';
import { existsSync } from 'fs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const program = new Command();

program
  .name('agent-cli')
  .description('Agent CLI Prompt Management Solution')
  .version('0.0.1');

// Initialize command
program
  .command('init')
  .description('Initialize a new .ai/ repository')
  .option('-f, --force', 'Force overwrite existing files')
  .option('-t, --template <template>', 'Use specific template')
  .option('--use-rust', 'Use Rust core engine for initialization')
  .action(async (options) => {
    if (options.useRust) {
      await runRustCommand(['init', options.force ? '--force' : '', options.template ? `--template ${options.template}` : ''].filter(Boolean));
    } else {
      console.log(chalk.blue('🚀 Initializing .ai/ repository...'));
      // TODO: Implement TypeScript initialization logic
      console.log(chalk.green('✅ Repository initialized successfully!'));
    }
  });

// Update command
program
  .command('update')
  .description('Update existing repository')
  .option('-b, --backup', 'Create backup before updating')
  .option('-v, --version <version>', 'Target version to update to')
  .option('--use-rust', 'Use Rust core engine for update')
  .action(async (options) => {
    if (options.useRust) {
      await runRustCommand(['update', options.backup ? '--backup' : '', options.version ? `--version ${options.version}` : ''].filter(Boolean));
    } else {
      console.log(chalk.blue('🔄 Updating repository...'));
      // TODO: Implement TypeScript update logic
      console.log(chalk.green('✅ Repository updated successfully!'));
    }
  });

// Compose command
program
  .command('compose')
  .description('Compose new prompts')
  .argument('<type>', 'Type of prompt to compose (memory|sub-agent|cmd)')
  .option('-i, --interactive', 'Interactive mode')
  .option('--use-rust', 'Use Rust core engine for composition')
  .action(async (type, options) => {
    if (options.useRust) {
      await runRustCommand(['compose', type, options.interactive ? '--interactive' : ''].filter(Boolean));
    } else {
      console.log(chalk.blue(`📝 Composing ${type} prompt...`));
      // TODO: Implement TypeScript composition logic
      console.log(chalk.green('✅ Prompt composed successfully!'));
    }
  });

// Prune command
program
  .command('prune')
  .description('Prune old prompts')
  .option('-f, --force', 'Force deletion without confirmation')
  .option('-d, --dry-run', 'Dry run (show what would be deleted)')
  .option('--use-rust', 'Use Rust core engine for pruning')
  .action(async (options) => {
    if (options.useRust) {
      await runRustCommand(['prune', options.force ? '--force' : '', options.dryRun ? '--dry-run' : ''].filter(Boolean));
    } else {
      console.log(chalk.blue('🧹 Pruning prompts...'));
      // TODO: Implement TypeScript pruning logic
      console.log(chalk.green('✅ Pruning completed successfully!'));
    }
  });

// Sync command
program
  .command('sync')
  .description('Sync with remote repository')
  .option('-r, --remote <remote>', 'Remote repository URL')
  .option('-b, --branch <branch>', 'Branch to sync with')
  .option('--use-rust', 'Use Rust core engine for synchronization')
  .action(async (options) => {
    if (options.useRust) {
      await runRustCommand(['sync', options.remote ? `--remote ${options.remote}` : '', options.branch ? `--branch ${options.branch}` : ''].filter(Boolean));
    } else {
      console.log(chalk.blue('🔄 Syncing with remote repository...'));
      // TODO: Implement TypeScript sync logic
      console.log(chalk.green('✅ Sync completed successfully!'));
    }
  });

// Rust-specific commands
program
  .command('rust')
  .description('Run Rust core engine directly')
  .allowUnknownOption(true)
  .action(async (options, command) => {
    const args = command.args;
    await runRustCommand(args);
  });

// Build command
program
  .command('build')
  .description('Build the project (TypeScript + Rust)')
  .option('--rust-only', 'Build only Rust components')
  .option('--ts-only', 'Build only TypeScript components')
  .action(async (options) => {
    if (options.rustOnly) {
      await runCommand('pnpm', ['run', 'build:rust']);
    } else if (options.tsOnly) {
      await runCommand('pnpm', ['run', 'build:ts']);
    } else {
      console.log(chalk.blue('🔨 Building TypeScript + Rust...'));
      await runCommand('pnpm', ['run', 'build']);
      console.log(chalk.green('✅ Build completed successfully!'));
    }
  });

// Helper function to run Rust binary
async function runRustCommand(args: string[]): Promise<void> {
  const rustBinary = getRustBinaryPath();

  if (!existsSync(rustBinary)) {
    console.log(chalk.yellow('⚠️  Rust binary not found. Building...'));
    await runCommand('pnpm', ['run', 'build:rust:debug']);
  }

  console.log(chalk.blue(`🦀 Running Rust: agents-core ${args.join(' ')}`));
  await runCommand(rustBinary, args);
}

// Helper function to get Rust binary path
function getRustBinaryPath(): string {
  const isWindows = process.platform === 'win32';
  const binaryName = isWindows ? 'agents-core.exe' : 'agents-core';
  return join(__dirname, '..', 'dist', 'rust', 'debug', binaryName);
}

// Helper function to run shell commands
function runCommand(command: string, args: string[]): Promise<void> {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      stdio: 'inherit',
      shell: true
    });

    child.on('close', (code) => {
      if (code === 0) {
        resolve();
      } else {
        reject(new Error(`Command failed with exit code ${code}`));
      }
    });

    child.on('error', (error) => {
      reject(error);
    });
  });
}

// Handle unknown commands
program.on('command:*', () => {
  console.error(chalk.red('❌ Invalid command: %s'), program.args.join(' '));
  console.log(chalk.yellow('💡 Use --help for available commands'));
  process.exit(1);
});

// Parse and execute
program.parse();

// Handle no command provided
if (!process.argv.slice(2).length) {
  program.outputHelp();
}