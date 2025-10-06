//! Orchestrator for coordinating agent operations
//!
//! This module provides the main orchestration logic for managing agent operations
//! like initialization, updates, composition, pruning, and synchronization.

use super::config::{AgentConfig, PromptTemplate, TemplateType};
use super::error::{AgentError, Result};
use super::platform::Platform;
use super::storage::{BackupInfo, StorageManager};
use super::template::{TemplateContext, TemplateRegistry};
use std::collections::HashMap;

/// Main orchestrator for agent operations
pub struct Orchestrator {
  config: AgentConfig,
  storage: StorageManager,
  templates: TemplateRegistry,
  platform: Platform,
}

/// Options for initialization operations
#[derive(Debug, Clone)]
pub struct InitOptions {
  pub force: bool,
  pub template: Option<String>,
  pub silent: bool,
  pub verbose: bool,
}

/// Options for update operations
#[derive(Debug, Clone)]
pub struct UpdateOptions {
  pub backup: bool,
  pub version: Option<String>,
  pub silent: bool,
  pub verbose: bool,
}

/// Options for composition operations
#[derive(Debug, Clone)]
pub struct ComposeOptions {
  pub r#type: TemplateType,
  pub interactive: bool,
  pub silent: bool,
  pub verbose: bool,
}

/// Options for pruning operations
#[derive(Debug, Clone)]
pub struct PruneOptions {
  pub force: bool,
  pub dry_run: bool,
  pub silent: bool,
  pub verbose: bool,
}

/// Options for synchronization operations
#[derive(Debug, Clone)]
pub struct SyncOptions {
  pub remote: Option<String>,
  pub branch: Option<String>,
  pub silent: bool,
  pub verbose: bool,
}

/// Result of an operation
#[derive(Debug, Clone)]
pub struct OperationResult {
  pub success: bool,
  pub message: String,
  pub affected_files: Vec<String>,
  pub backup_info: Option<BackupInfo>,
  pub warnings: Vec<String>,
}

impl Orchestrator {
  /// Create a new orchestrator with the given configuration
  pub fn new(config: AgentConfig) -> Result<Self> {
    let storage = StorageManager::new(&config.directories.memory)?;
    let templates = TemplateRegistry::new()?;
    let platform = Platform::detect()?;

    Ok(Self {
      config,
      storage,
      templates,
      platform,
    })
  }

  /// Initialize a new agent repository
  pub async fn initialize(&mut self, options: InitOptions) -> Result<OperationResult> {
    if !options.silent {
      println!("Initializing agent repository...");
    }

    let mut result = OperationResult {
      success: false,
      message: "Initialization completed".to_string(),
      affected_files: Vec::new(),
      backup_info: None,
      warnings: Vec::new(),
    };

    // Check if repository already exists
    if self.storage.file_exists("agents.prompts.json") && !options.force {
      return Err(AgentError::Storage(
        "Agent repository already exists. Use --force to overwrite.".to_string(),
      ));
    }

    // Create backup if overwriting
    if options.force && self.storage.file_exists("agents.prompts.json") {
      let backup = self
        .storage
        .create_backup(Some("Pre-initialization backup".to_string()))?;
      result.backup_info = Some(backup.clone());
      if !options.silent {
        println!("Created backup: {}", backup.path);
      }
    }

    // Create directory structure
    let directories = vec![
      &self.config.directories.memory,
      &self.config.directories.user,
      &self.config.directories.project,
      &self.config.directories.sub_agent,
      &self.config.directories.command,
      &self.config.directories.backups,
    ];

    for dir in directories {
      let dir_path = std::path::Path::new(dir);
      if !dir_path.exists() {
        std::fs::create_dir_all(dir_path)
          .map_err(|e| AgentError::Storage(format!("Failed to create directory {}: {}", dir, e)))?;
        result.affected_files.push(dir.clone());
        if !options.silent {
          println!("Created directory: {}", dir);
        }
      }
    }

    // Load default templates
    let default_templates = TemplateRegistry::get_default_templates();
    for template in default_templates {
      self.templates.register_template(template.clone())?;

      // Save template to appropriate directory
      let template_dir = match template.r#type {
        TemplateType::Memory => &self.config.directories.memory,
        TemplateType::SubAgent => &self.config.directories.sub_agent,
        TemplateType::Command => &self.config.directories.command,
      };

      let template_file = format!("{}/{}.json", template_dir, template.name);
      self
        .templates
        .save_template_to_file(&template, &template_file)?;
      result.affected_files.push(template_file);

      if !options.silent {
        println!("Created template: {}", template.name);
      }
    }

    // Save configuration
    let config_content = serde_json::to_string_pretty(&self.config).map_err(|e| {
      AgentError::Configuration(format!("Failed to serialize configuration: {}", e))
    })?;

    self
      .storage
      .write_file_atomic("agents.prompts.json", config_content.as_bytes())?;
    result
      .affected_files
      .push("agents.prompts.json".to_string());

    result.success = true;
    Ok(result)
  }

  /// Update existing repository
  pub async fn update(&mut self, options: UpdateOptions) -> Result<OperationResult> {
    if !options.silent {
      println!("Updating agent repository...");
    }

    let mut result = OperationResult {
      success: false,
      message: "Update completed".to_string(),
      affected_files: Vec::new(),
      backup_info: None,
      warnings: Vec::new(),
    };

    // Check if repository exists
    if !self.storage.file_exists("agents.prompts.json") {
      return Err(AgentError::Storage(
        "No agent repository found. Initialize first with 'init' command.".to_string(),
      ));
    }

    // Create backup before updating
    if options.backup {
      let backup = self
        .storage
        .create_backup(Some("Pre-update backup".to_string()))?;
      result.backup_info = Some(backup.clone());
      if !options.silent {
        println!("Created backup: {}", backup.path);
      }
    }

    // TODO: Implement actual update logic
    // This would involve:
    // 1. Loading current configuration
    // 2. Comparing with template repository
    // 3. Applying updates
    // 4. Handling conflicts

    result.success = true;
    Ok(result)
  }

  /// Compose new prompts
  pub async fn compose(&mut self, options: ComposeOptions) -> Result<PromptTemplate> {
    if !options.silent {
      println!("Composing {:?} prompt...", options.r#type);
    }

    // Get templates of the specified type
    let templates = self
      .templates
      .list_templates_by_type(options.r#type.clone());
    if templates.is_empty() {
      return Err(AgentError::Template(format!(
        "No templates found for type {:?}",
        options.r#type
      )));
    }

    if options.interactive {
      // TODO: Implement interactive composition
      // This would involve:
      // 1. Presenting template options to user
      // 2. Collecting variable values
      // 3. Generating the prompt
      return Err(AgentError::Internal(
        "Interactive composition not yet implemented".to_string(),
      ));
    }

    // Use first available template
    let template = templates[0].clone();

    // Create context with default values
    let mut variables = HashMap::new();
    for var in &template.variables {
      if let Some(default) = &var.default {
        variables.insert(var.name.clone(), serde_json::Value::String(default.clone()));
      }
    }

    let context = TemplateContext {
      variables,
      metadata: HashMap::new(),
    };

    // Render template
    let rendered = self.templates.render_template(&template.name, &context)?;

    // Create new prompt template with rendered content
    let mut new_template = template.clone();
    new_template.content = rendered;

    Ok(new_template)
  }

  /// Prune old prompts
  pub async fn prune(&mut self, options: PruneOptions) -> Result<OperationResult> {
    if !options.silent {
      println!("Pruning old prompts...");
    }

    let mut result = OperationResult {
      success: false,
      message: "Pruning completed".to_string(),
      affected_files: Vec::new(),
      backup_info: None,
      warnings: Vec::new(),
    };

    // TODO: Implement pruning logic
    // This would involve:
    // 1. Scanning for orphaned prompts
    // 2. Identifying unused templates
    // 3. Moving files to trash or deleting them

    result.success = true;
    Ok(result)
  }

  /// Sync with remote repository
  pub async fn sync(&mut self, options: SyncOptions) -> Result<OperationResult> {
    if !options.silent {
      println!("Syncing with remote repository...");
    }

    let mut result = OperationResult {
      success: false,
      message: "Sync completed".to_string(),
      affected_files: Vec::new(),
      backup_info: None,
      warnings: Vec::new(),
    };

    // TODO: Implement sync logic
    // This would involve:
    // 1. Connecting to remote repository
    // 2. Pulling updates
    // 3. Pushing local changes
    // 4. Handling conflicts

    result.success = true;
    Ok(result)
  }

  /// Get the current configuration
  pub fn config(&self) -> &AgentConfig {
    &self.config
  }

  /// Get the storage manager
  pub fn storage(&self) -> &StorageManager {
    &self.storage
  }

  /// Get the template registry
  pub fn templates(&self) -> &TemplateRegistry {
    &self.templates
  }

  /// Get platform information
  pub fn platform(&self) -> &Platform {
    &self.platform
  }

  /// Load configuration from storage
  pub fn load_config(&mut self) -> Result<()> {
    let config_content = self.storage.read_file_to_string("agents.prompts.json")?;
    self.config = serde_json::from_str(&config_content)
      .map_err(|e| AgentError::Configuration(format!("Failed to parse configuration: {}", e)))?;
    Ok(())
  }

  /// Save configuration to storage
  pub fn save_config(&self) -> Result<()> {
    let config_content = serde_json::to_string_pretty(&self.config).map_err(|e| {
      AgentError::Configuration(format!("Failed to serialize configuration: {}", e))
    })?;

    self
      .storage
      .write_file_atomic("agents.prompts.json", config_content.as_bytes())?;
    Ok(())
  }
}
