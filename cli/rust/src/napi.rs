//! NAPI bindings for Node.js integration
//!
//! This module provides the NAPI bindings that allow the Rust core engine
//! to be called from Node.js/TypeScript code.

use napi::bindgen_prelude::{Result as NapiResult, Status};
use napi_derive::napi;

use crate::{AgentConfig, AgentError, Platform, PromptTemplate};

fn map_agent_error(error: AgentError) -> napi::Error {
    napi::Error::new(Status::GenericFailure, error.to_string())
}

fn map_serde_error(message: &str, error: serde_json::Error) -> napi::Error {
    napi::Error::new(Status::GenericFailure, format!("{}: {}", message, error))
}

/// Initialize the agents core system
#[napi]
pub async fn init(options: Option<InitOptions>) -> NapiResult<()> {
    let _ = options;
    crate::init().map_err(map_agent_error)?;
    Ok(())
}

/// Initialize a new .ai/ repository
#[napi]
pub async fn init_repository(options: InitOptions) -> NapiResult<String> {
    let _ = options;
    serde_json::to_string_pretty(&AgentConfig::default())
        .map_err(|error| map_serde_error("Failed to serialize default config", error))
}

/// Update existing repository
#[napi]
pub async fn update_repository(options: UpdateOptions) -> NapiResult<Vec<String>> {
    let _ = options;
    Ok(vec!["Updated".to_string()])
}

/// Compose new prompts
#[napi]
pub async fn compose_prompt(options: ComposeOptions) -> NapiResult<String> {
    let _ = options;
    serde_json::to_string_pretty(&PromptTemplate::default())
        .map_err(|error| map_serde_error("Failed to serialize template", error))
}

/// Prune old prompts
#[napi]
pub async fn prune_prompts(options: PruneOptions) -> NapiResult<Vec<String>> {
    let _ = options;
    Ok(vec!["Pruned".to_string()])
}

/// Sync with remote repository
#[napi]
pub async fn sync_repository(options: SyncOptions) -> NapiResult<Vec<String>> {
    let _ = options;
    Ok(vec!["Synced".to_string()])
}

/// Get system platform information
#[napi]
pub fn get_platform() -> NapiResult<String> {
    Platform::detect()
        .map(|platform| platform.to_string())
        .map_err(map_agent_error)
}

/// NAPI option types
#[napi(object)]
pub struct InitOptions {
    pub force: Option<bool>,
    pub template: Option<String>,
    pub silent: Option<bool>,
    pub verbose: Option<bool>,
}

#[napi(object)]
pub struct UpdateOptions {
    pub backup: Option<bool>,
    pub version: Option<String>,
    pub silent: Option<bool>,
    pub verbose: Option<bool>,
}

#[napi(object)]
pub struct ComposeOptions {
    pub r#type: String,
    pub interactive: Option<bool>,
    pub silent: Option<bool>,
    pub verbose: Option<bool>,
}

#[napi(object)]
pub struct PruneOptions {
    pub force: Option<bool>,
    pub dry_run: Option<bool>,
    pub silent: Option<bool>,
    pub verbose: Option<bool>,
}

#[napi(object)]
pub struct SyncOptions {
    pub remote: Option<String>,
    pub branch: Option<String>,
    pub silent: Option<bool>,
    pub verbose: Option<bool>,
}
