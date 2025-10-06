//! Error handling for the agents core system
//!
//! This module provides centralized error types and handling for the agents core.

use thiserror::Error;

/// Main error type for the agents core system
#[derive(Error, Debug)]
pub enum AgentError {
  #[error("I/O error: {0}")]
  Io(#[from] std::io::Error),

  #[error("Serialization error: {0}")]
  Serialization(#[from] serde_json::Error),

  #[error("TOML parsing error: {0}")]
  TomlParsing(#[from] toml::de::Error),

  #[error("Configuration error: {0}")]
  Configuration(String),

  #[error("Template error: {0}")]
  Template(String),

  #[error("Storage error: {0}")]
  Storage(String),

  #[error("Platform detection error: {0}")]
  Platform(String),

  #[error("Invalid path: {0}")]
  InvalidPath(String),

  #[error("File not found: {0}")]
  FileNotFound(String),

  #[error("Permission denied: {0}")]
  PermissionDenied(String),

  #[error("Operation cancelled")]
  Cancelled,

  #[error("Internal error: {0}")]
  Internal(String),
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, AgentError>;

impl AgentError {
  /// Create a new configuration error
  pub fn config<S: Into<String>>(msg: S) -> Self {
    Self::Configuration(msg.into())
  }

  /// Create a new template error
  pub fn template<S: Into<String>>(msg: S) -> Self {
    Self::Template(msg.into())
  }

  /// Create a new storage error
  pub fn storage<S: Into<String>>(msg: S) -> Self {
    Self::Storage(msg.into())
  }

  /// Create a new platform error
  pub fn platform<S: Into<String>>(msg: S) -> Self {
    Self::Platform(msg.into())
  }

  /// Create a new internal error
  pub fn internal<S: Into<String>>(msg: S) -> Self {
    Self::Internal(msg.into())
  }
}
