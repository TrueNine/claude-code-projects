//! Agents Core - Rust engine for Agent CLI prompt management
//!
//! This library provides the core functionality for managing AI agent prompts,
//! including initialization, updates, composition, pruning, and synchronization.

pub mod modules;
#[cfg(feature = "napi")]
pub mod napi;

#[cfg(feature = "napi")]
pub use crate::napi::*;

// Re-export main types
pub use modules::{
    config::{AgentConfig, PromptTemplate, TemplateType},
    error::{AgentError, Result},
    orchestrator::Orchestrator,
    platform::Platform,
    storage::{BackupInfo, StorageManager},
    template::TemplateRegistry,
};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the agents core system
pub fn init() -> Result<()> {
    tracing::info!("Initializing Agents Core v{}", VERSION);

    let platform = Platform::detect()?;
    tracing::info!("Detected platform: {}", platform);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        assert!(init().is_ok());
    }
}
