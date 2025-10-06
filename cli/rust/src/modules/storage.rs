//! Storage management for the agents core system
//!
//! This module provides file system operations, backup management, and atomic writes.

use super::error::{AgentError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Backup information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
  /// Backup timestamp
  pub timestamp: u64,
  /// Backup directory path
  pub path: String,
  /// Original directory that was backed up
  pub original_path: String,
  /// Files included in this backup
  pub files: Vec<String>,
  /// Backup description
  pub description: Option<String>,
}

/// Storage manager for handling file operations
pub struct StorageManager {
  base_path: PathBuf,
  backup_path: PathBuf,
}

impl StorageManager {
  /// Create a new storage manager
  pub fn new<P: AsRef<Path>>(base_path: P) -> Result<Self> {
    let base_path = base_path.as_ref().to_path_buf();
    let backup_path = base_path.join(".agents").join("backups");

    // Ensure directories exist
    fs::create_dir_all(&base_path)
      .map_err(|e| AgentError::Storage(format!("Failed to create base directory: {}", e)))?;
    fs::create_dir_all(&backup_path)
      .map_err(|e| AgentError::Storage(format!("Failed to create backup directory: {}", e)))?;

    Ok(Self {
      base_path,
      backup_path,
    })
  }

  /// Get the base path
  pub fn base_path(&self) -> &Path {
    &self.base_path
  }

  /// Get the backup path
  pub fn backup_path(&self) -> &Path {
    &self.backup_path
  }

  /// Write content to a file atomically
  pub fn write_file_atomic<P: AsRef<Path>, C: AsRef<[u8]>>(
    &self,
    path: P,
    content: C,
  ) -> Result<()> {
    let path = path.as_ref();
    let full_path = self.base_path.join(path);

    // Ensure parent directory exists
    if let Some(parent) = full_path.parent() {
      fs::create_dir_all(parent)
        .map_err(|e| AgentError::Storage(format!("Failed to create parent directory: {}", e)))?;
    }

    // Write to temporary file first
    let temp_path = full_path.with_extension(format!(
      "tmp.{}",
      SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| AgentError::Internal(format!("System time error: {}", e)))?
        .as_nanos()
    ));

    fs::write(&temp_path, content)
      .map_err(|e| AgentError::Storage(format!("Failed to write temporary file: {}", e)))?;

    // Atomically rename
    fs::rename(&temp_path, &full_path).map_err(|e| {
      // Try to clean up temp file if rename fails
      let _ = fs::remove_file(&temp_path);
      AgentError::Storage(format!("Failed to rename temporary file: {}", e))
    })?;

    Ok(())
  }

  /// Read file content
  pub fn read_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>> {
    let path = path.as_ref();
    let full_path = self.base_path.join(path);

    fs::read(&full_path).map_err(|e| AgentError::Storage(format!("Failed to read file: {}", e)))
  }

  /// Read file content as string
  pub fn read_file_to_string<P: AsRef<Path>>(&self, path: P) -> Result<String> {
    let path = path.as_ref();
    let full_path = self.base_path.join(path);

    fs::read_to_string(&full_path)
      .map_err(|e| AgentError::Storage(format!("Failed to read file to string: {}", e)))
  }

  /// Check if a file exists
  pub fn file_exists<P: AsRef<Path>>(&self, path: P) -> bool {
    let path = path.as_ref();
    let full_path = self.base_path.join(path);
    full_path.exists()
  }

  /// Delete a file
  pub fn delete_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
    let path = path.as_ref();
    let full_path = self.base_path.join(path);

    fs::remove_file(&full_path)
      .map_err(|e| AgentError::Storage(format!("Failed to delete file: {}", e)))
  }

  /// Move a file to trash (backup directory)
  pub fn move_to_trash<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf> {
    let path = path.as_ref();
    let full_path = self.base_path.join(path);
    let trash_path = self.base_path.join(".agents").join("trash");

    // Ensure trash directory exists
    fs::create_dir_all(&trash_path)
      .map_err(|e| AgentError::Storage(format!("Failed to create trash directory: {}", e)))?;

    let timestamp = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .map_err(|e| AgentError::Internal(format!("System time error: {}", e)))?
      .as_secs();

    let file_name = path
      .file_name()
      .and_then(|n| n.to_str())
      .ok_or_else(|| AgentError::InvalidPath("Invalid file name".to_string()))?;

    let trash_file_name = format!(
      "{}.{}.{}",
      timestamp,
      file_name,
      path.extension().and_then(|e| e.to_str()).unwrap_or("bak")
    );

    let trash_file_path = trash_path.join(trash_file_name);

    fs::rename(&full_path, &trash_file_path)
      .map_err(|e| AgentError::Storage(format!("Failed to move file to trash: {}", e)))?;

    Ok(trash_file_path)
  }

  /// Create a backup of the current state
  pub fn create_backup(&self, description: Option<String>) -> Result<BackupInfo> {
    let timestamp = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .map_err(|e| AgentError::Internal(format!("System time error: {}", e)))?
      .as_secs();

    let backup_dir = self.backup_path.join(format!("backup_{}", timestamp));
    fs::create_dir_all(&backup_dir)
      .map_err(|e| AgentError::Storage(format!("Failed to create backup directory: {}", e)))?;

    // Find all files to backup
    let mut files = Vec::new();
    self.collect_files(&self.base_path, &mut files)?;

    // Copy files to backup directory
    for file in &files {
      if let Ok(relative_path) = file.strip_prefix(&self.base_path) {
        let backup_file_path = backup_dir.join(relative_path);
        if let Some(parent) = backup_file_path.parent() {
          fs::create_dir_all(parent).map_err(|e| {
            AgentError::Storage(format!("Failed to create backup subdirectory: {}", e))
          })?;
        }
        fs::copy(file, &backup_file_path)
          .map_err(|e| AgentError::Storage(format!("Failed to copy file to backup: {}", e)))?;
      }
    }

    Ok(BackupInfo {
      timestamp,
      path: backup_dir.to_string_lossy().to_string(),
      original_path: self.base_path.to_string_lossy().to_string(),
      files: files
        .iter()
        .filter_map(|f| {
          f.strip_prefix(&self.base_path)
            .ok()
            .and_then(|p| p.to_str())
            .map(|s| s.to_string())
        })
        .collect(),
      description,
    })
  }

  /// Restore from a backup
  pub fn restore_from_backup(&self, backup: &BackupInfo) -> Result<()> {
    let backup_path = Path::new(&backup.path);
    if !backup_path.exists() {
      return Err(AgentError::Storage(format!(
        "Backup directory not found: {}",
        backup.path
      )));
    }

    // Create a backup of current state before restoring
    self.create_backup(Some(format!(
      "Pre-restore backup before restoring from {}",
      backup.timestamp
    )))?;

    // Remove existing files and restore from backup
    for file in &backup.files {
      let current_file_path = self.base_path.join(file);
      let backup_file_path = backup_path.join(file);

      // Remove existing file if it exists
      if current_file_path.exists() {
        fs::remove_file(&current_file_path)
          .map_err(|e| AgentError::Storage(format!("Failed to remove existing file: {}", e)))?;
      }

      // Ensure parent directory exists
      if let Some(parent) = current_file_path.parent() {
        fs::create_dir_all(parent)
          .map_err(|e| AgentError::Storage(format!("Failed to create parent directory: {}", e)))?;
      }

      // Copy from backup
      fs::copy(&backup_file_path, &current_file_path)
        .map_err(|e| AgentError::Storage(format!("Failed to restore file: {}", e)))?;
    }

    Ok(())
  }

  /// List available backups
  pub fn list_backups(&self) -> Result<Vec<BackupInfo>> {
    let mut backups = Vec::new();

    if !self.backup_path.exists() {
      return Ok(backups);
    }

    for entry in fs::read_dir(&self.backup_path)
      .map_err(|e| AgentError::Storage(format!("Failed to read backup directory: {}", e)))?
    {
      let entry =
        entry.map_err(|e| AgentError::Storage(format!("Failed to read backup entry: {}", e)))?;
      let path = entry.path();

      if path.is_dir() {
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        if let Some(timestamp_str) = file_name.strip_prefix("backup_")
          && let Ok(timestamp) = timestamp_str.parse::<u64>()
        {
          // Try to read backup metadata
          let metadata_path = path.join("backup_info.json");
          let backup_info = if metadata_path.exists() {
            let content = fs::read_to_string(&metadata_path)
              .map_err(|e| AgentError::Storage(format!("Failed to read backup metadata: {}", e)))?;
            serde_json::from_str(&content)
              .map_err(|e| AgentError::Storage(format!("Failed to parse backup metadata: {}", e)))?
          } else {
            // Create basic backup info from directory structure
            let mut files = Vec::new();
            self.collect_files(&path, &mut files)?;
            BackupInfo {
              timestamp,
              path: path.to_string_lossy().to_string(),
              original_path: self.base_path.to_string_lossy().to_string(),
              files: files
                .iter()
                .filter_map(|f| {
                  f.strip_prefix(&path)
                    .ok()
                    .and_then(|p| p.to_str())
                    .map(|s| s.to_string())
                })
                .collect(),
              description: None,
            }
          };
          backups.push(backup_info);
        }
      }
    }

    // Sort by timestamp (newest first)
    backups.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    Ok(backups)
  }

  /// Delete a backup
  pub fn delete_backup(&self, backup: &BackupInfo) -> Result<()> {
    let backup_path = Path::new(&backup.path);
    fs::remove_dir_all(backup_path)
      .map_err(|e| AgentError::Storage(format!("Failed to delete backup: {}", e)))
  }

  /// Collect all files in a directory recursively
  #[allow(clippy::only_used_in_recursion)]
  fn collect_files(&self, dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    if dir.is_dir() {
      for entry in fs::read_dir(dir)
        .map_err(|e| AgentError::Storage(format!("Failed to read directory: {}", e)))?
      {
        let entry = entry
          .map_err(|e| AgentError::Storage(format!("Failed to read directory entry: {}", e)))?;
        let path = entry.path();

        // Skip .agents directory
        if let Some(name) = path.file_name()
          && name == ".agents"
        {
          continue;
        }

        if path.is_dir() {
          self.collect_files(&path, files)?;
        } else {
          files.push(path);
        }
      }
    }
    Ok(())
  }
}
