//! Configuration management for the agents core system
//!
//! This module provides configuration structures and management functionality.

use super::error::{AgentError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main configuration structure for the agents system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
  /// Version of the configuration format
  pub version: String,
  /// Directory mappings for different prompt types
  pub directories: DirectoryMappings,
  /// Template settings
  pub templates: TemplateSettings,
  /// Platform-specific settings
  pub platform: PlatformSettings,
}

/// Directory mappings for different types of prompts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryMappings {
  /// Memory prompts directory
  pub memory: String,
  /// User prompts directory
  pub user: String,
  /// Project prompts directory
  pub project: String,
  /// Sub-agent prompts directory
  pub sub_agent: String,
  /// Command prompts directory
  pub command: String,
  /// Backups directory
  pub backups: String,
}

/// Template configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSettings {
  /// Default template repository URL
  pub repository: Option<String>,
  /// Local templates directory
  pub local_directory: String,
  /// Template cache settings
  pub cache: CacheSettings,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheSettings {
  /// Enable template caching
  pub enabled: bool,
  /// Cache directory
  pub directory: String,
  /// Maximum cache size in MB
  pub max_size_mb: u64,
  /// Cache TTL in seconds
  pub ttl_seconds: u64,
}

/// Platform-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformSettings {
  /// Windows-specific settings
  pub windows: Option<PlatformSpecific>,
  /// macOS-specific settings
  pub macos: Option<PlatformSpecific>,
  /// Linux-specific settings
  pub linux: Option<PlatformSpecific>,
}

/// Platform-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformSpecific {
  /// Shell to use for command execution
  pub shell: Option<String>,
  /// Default editor
  pub editor: Option<String>,
  /// Path separator
  pub path_separator: String,
}

/// Template structure for prompts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
  /// Template name
  pub name: String,
  /// Template description
  pub description: String,
  /// Template version
  pub version: String,
  /// Template type
  pub r#type: TemplateType,
  /// Template content
  pub content: String,
  /// Template metadata
  pub metadata: HashMap<String, String>,
  /// Required variables for the template
  pub variables: Vec<TemplateVariable>,
}

/// Template variable definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
  /// Variable name
  pub name: String,
  /// Variable description
  pub description: String,
  /// Variable type
  pub r#type: VariableType,
  /// Whether the variable is required
  pub required: bool,
  /// Default value
  pub default: Option<String>,
  /// Possible values (for enums)
  pub values: Option<Vec<String>>,
}

/// Template variable types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum VariableType {
  String,
  Number,
  Boolean,
  Array,
  Object,
}

/// Template types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TemplateType {
  Memory,
  SubAgent,
  Command,
}

impl Default for AgentConfig {
  fn default() -> Self {
    Self {
      version: "1.0.0".to_string(),
      directories: DirectoryMappings::default(),
      templates: TemplateSettings::default(),
      platform: PlatformSettings::default(),
    }
  }
}

impl Default for DirectoryMappings {
  fn default() -> Self {
    Self {
      memory: ".ai/locale".to_string(),
      user: ".ai/user".to_string(),
      project: ".ai/project".to_string(),
      sub_agent: ".ai/sa".to_string(),
      command: ".ai/cmd".to_string(),
      backups: ".agents/backups".to_string(),
    }
  }
}

impl Default for TemplateSettings {
  fn default() -> Self {
    Self {
      repository: None,
      local_directory: ".ai/templates".to_string(),
      cache: CacheSettings::default(),
    }
  }
}

impl Default for CacheSettings {
  fn default() -> Self {
    Self {
      enabled: true,
      directory: ".agents/cache".to_string(),
      max_size_mb: 100,
      ttl_seconds: 3600,
    }
  }
}

impl Default for PlatformSettings {
  fn default() -> Self {
    Self {
      windows: Some(PlatformSpecific {
        shell: Some("cmd".to_string()),
        editor: Some("notepad".to_string()),
        path_separator: "\\".to_string(),
      }),
      macos: Some(PlatformSpecific {
        shell: Some("zsh".to_string()),
        editor: Some("nano".to_string()),
        path_separator: "/".to_string(),
      }),
      linux: Some(PlatformSpecific {
        shell: Some("bash".to_string()),
        editor: Some("nano".to_string()),
        path_separator: "/".to_string(),
      }),
    }
  }
}

impl Default for PromptTemplate {
  fn default() -> Self {
    Self {
      name: "default".to_string(),
      description: "Default template".to_string(),
      version: "1.0.0".to_string(),
      r#type: TemplateType::Memory,
      content: "".to_string(),
      metadata: HashMap::new(),
      variables: Vec::new(),
    }
  }
}

/// Configuration manager
pub struct ConfigManager {
  config_path: String,
}

impl ConfigManager {
  /// Create a new configuration manager
  pub fn new<P: Into<String>>(config_path: P) -> Self {
    Self {
      config_path: config_path.into(),
    }
  }

  /// Load configuration from file
  pub fn load(&self) -> Result<AgentConfig> {
    let content = std::fs::read_to_string(&self.config_path)
      .map_err(|e| AgentError::FileNotFound(format!("{}: {}", self.config_path, e)))?;

    serde_json::from_str(&content)
      .map_err(|e| AgentError::Configuration(format!("Failed to parse config: {}", e)))
  }

  /// Save configuration to file
  pub fn save(&self, config: &AgentConfig) -> Result<()> {
    let content = serde_json::to_string_pretty(config)
      .map_err(|e| AgentError::Configuration(format!("Failed to serialize config: {}", e)))?;

    // Ensure parent directory exists
    if let Some(parent) = std::path::Path::new(&self.config_path).parent() {
      std::fs::create_dir_all(parent)
        .map_err(|e| AgentError::Storage(format!("Failed to create config directory: {}", e)))?;
    }

    std::fs::write(&self.config_path, content)
      .map_err(|e| AgentError::Storage(format!("Failed to write config file: {}", e)))
  }

  /// Get the configuration file path
  pub fn path(&self) -> &str {
    &self.config_path
  }
}
