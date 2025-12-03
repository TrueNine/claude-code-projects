//! Agents Core CLI - Standalone Rust binary
//!
//! This binary provides the standalone Rust implementation of the agents CLI,
//! which can be used independently or as the core engine for the TypeScript CLI.

use agents_core::Result;
use clap::{Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(name = "agents-core")]
#[command(about = "Agents Core - Rust engine for prompt management")]
#[command(version)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  /// Initialize a new __ai/ repository
  Init {
    /// Force overwrite existing files
    #[arg(short, long)]
    force: bool,
    /// Use specific template
    #[arg(short, long)]
    template: Option<String>,
  },
  /// Update existing repository
  Update {
    /// Create backup before updating
    #[arg(short, long)]
    backup: bool,
    /// Target version to update to
    #[arg(short, long)]
    version: Option<String>,
  },
  /// Compose new prompts
  Compose {
    /// Type of prompt to compose
    #[arg(value_enum)]
    r#type: PromptType,
    /// Interactive mode
    #[arg(short, long)]
    interactive: bool,
  },
  /// Prune old prompts
  Prune {
    /// Force deletion without confirmation
    #[arg(short, long)]
    force: bool,
    /// Dry run (show what would be deleted)
    #[arg(short, long)]
    dry_run: bool,
  },
  /// Sync with remote repository
  Sync {
    /// Remote repository URL
    #[arg(short, long)]
    remote: Option<String>,
    /// Branch to sync with
    #[arg(short, long)]
    branch: Option<String>,
  },
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum PromptType {
  Memory,
  SubAgent,
  Cmd,
}

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  let cli = Cli::parse();

  if let Err(e) = run(cli.command).await {
    eprintln!("Error: {}", e);
    process::exit(1);
  }
}

async fn run(command: Commands) -> Result<()> {
  match command {
    Commands::Init { force, template } => {
      println!("Initializing __ai/ repository...");
      println!("Force: {}, Template: {:?}", force, template);
      // TODO: Implement init logic
      Ok(())
    }
    Commands::Update { backup, version } => {
      println!("Updating repository...");
      println!("Backup: {}, Version: {:?}", backup, version);
      // TODO: Implement update logic
      Ok(())
    }
    Commands::Compose {
      r#type,
      interactive,
    } => {
      println!("Composing {:?} prompt...", r#type);
      println!("Interactive: {}", interactive);
      // TODO: Implement compose logic
      Ok(())
    }
    Commands::Prune { force, dry_run } => {
      println!("Pruning prompts...");
      println!("Force: {}, Dry run: {}", force, dry_run);
      // TODO: Implement prune logic
      Ok(())
    }
    Commands::Sync { remote, branch } => {
      println!("Syncing with remote...");
      println!("Remote: {:?}, Branch: {:?}", remote, branch);
      // TODO: Implement sync logic
      Ok(())
    }
  }
}
