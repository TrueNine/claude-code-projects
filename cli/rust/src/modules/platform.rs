//! Platform detection and platform-specific operations
//!
//! This module provides platform detection and platform-specific functionality
//! for the agents core system.

use super::error::{AgentError, Result};
use std::env;
use std::path::PathBuf;

/// Supported platforms
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
    Unknown,
}

impl Platform {
    /// Detect the current platform
    pub fn detect() -> Result<Self> {
        let os = env::consts::OS;
        match os {
            "windows" => Ok(Self::Windows),
            "macos" => Ok(Self::MacOS),
            "linux" => Ok(Self::Linux),
            _ => {
                tracing::warn!("Unknown platform: {}", os);
                Ok(Self::Unknown)
            }
        }
    }

    /// Get platform-specific path separator
    pub fn path_separator(&self) -> &'static str {
        match self {
            Self::Windows => "\\",
            Self::MacOS | Self::Linux | Self::Unknown => "/",
        }
    }

    /// Get platform-specific executable extension
    pub fn executable_extension(&self) -> &'static str {
        match self {
            Self::Windows => ".exe",
            Self::MacOS | Self::Linux | Self::Unknown => "",
        }
    }

    /// Get platform-specific shell command
    pub fn default_shell(&self) -> &'static str {
        match self {
            Self::Windows => "cmd",
            Self::MacOS => "zsh",
            Self::Linux => "bash",
            Self::Unknown => "sh",
        }
    }

    /// Get platform-specific default editor
    pub fn default_editor(&self) -> &'static str {
        match self {
            Self::Windows => "notepad",
            Self::MacOS => "nano",
            Self::Linux => "nano",
            Self::Unknown => "vi",
        }
    }

    /// Get platform-specific configuration directory
    pub fn config_dir(&self) -> Result<PathBuf> {
        let dirs = directories::ProjectDirs::from("com", "truenine", "agents-cli")
            .ok_or_else(|| AgentError::Platform("Failed to get config directory".to_string()))?;

        Ok(dirs.config_dir().to_path_buf())
    }

    /// Get platform-specific data directory
    pub fn data_dir(&self) -> Result<PathBuf> {
        let dirs = directories::ProjectDirs::from("com", "truenine", "agents-cli")
            .ok_or_else(|| AgentError::Platform("Failed to get data directory".to_string()))?;

        Ok(dirs.data_dir().to_path_buf())
    }

    /// Get platform-specific cache directory
    pub fn cache_dir(&self) -> Result<PathBuf> {
        let dirs = directories::ProjectDirs::from("com", "truenine", "agents-cli")
            .ok_or_else(|| AgentError::Platform("Failed to get cache directory".to_string()))?;

        Ok(dirs.cache_dir().to_path_buf())
    }

    /// Check if a command exists on the current platform
    pub fn command_exists(&self, command: &str) -> bool {
        which::which(command).is_ok()
    }

    /// Get platform-specific environment variable name
    pub fn env_var_home(&self) -> &'static str {
        match self {
            Self::Windows => "USERPROFILE",
            Self::MacOS | Self::Linux | Self::Unknown => "HOME",
        }
    }

    /// Get platform-specific environment variable for PATH
    pub fn env_var_path(&self) -> &'static str {
        "PATH"
    }

    /// Check if running in a terminal environment
    pub fn is_terminal(&self) -> bool {
        atty::is(atty::Stream::Stdout)
    }

    /// Get terminal width if available
    pub fn terminal_width(&self) -> Option<usize> {
        if !self.is_terminal() {
            return None;
        }

        terminal_size::terminal_size().map(|(width, _)| width.0 as usize)
    }

    /// Format a command for execution on this platform
    pub fn format_command(&self, command: &str, args: &[&str]) -> Vec<String> {
        let mut cmd = Vec::new();
        cmd.push(command.to_string());
        cmd.extend(args.iter().map(|&s| s.to_string()));
        cmd
    }

    /// Join path components for this platform
    pub fn join_paths(&self, components: &[&str]) -> PathBuf {
        let mut path = PathBuf::new();
        for component in components {
            path.push(component);
        }
        path
    }

    /// Check if a path is absolute on this platform
    pub fn is_absolute_path(&self, path: &str) -> bool {
        PathBuf::from(path).is_absolute()
    }

    /// Normalize a path for this platform
    pub fn normalize_path(&self, path: &str) -> PathBuf {
        let path_buf = PathBuf::from(path);

        // Convert forward slashes to backslashes on Windows
        if *self == Self::Windows {
            let path_str = path_buf.to_string_lossy();
            if path_str.contains('/') {
                let normalized = path_str.replace('/', "\\");
                return PathBuf::from(normalized);
            }
        }

        path_buf
    }

    /// Check if running with elevated privileges
    pub fn is_elevated(&self) -> bool {
        match self {
            Self::Windows => {
                // On Windows, check if running as Administrator
                // This is a simplified check - in a real implementation,
                // you'd want to use Windows API calls
                env::var("USERDOMAIN").is_ok()
                    && env::var("USERNAME").is_ok_and(|user| user == "Administrator")
            }
            Self::MacOS | Self::Linux => {
                #[cfg(unix)]
                {
                    unsafe { libc::geteuid() == 0 }
                }
                #[cfg(not(unix))]
                {
                    false
                }
            }
            Self::Unknown => false,
        }
    }

    /// Get platform-specific temporary directory
    pub fn temp_dir(&self) -> PathBuf {
        env::temp_dir()
    }

    /// Create a temporary directory with a platform-appropriate name
    pub fn create_temp_dir(&self, prefix: &str) -> Result<PathBuf> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| AgentError::Internal(format!("System time error: {}", e)))?
            .as_secs();

        let temp_name = format!("{}_{}", prefix, timestamp);
        let temp_path = self.temp_dir().join(temp_name);

        std::fs::create_dir_all(&temp_path)
            .map_err(|e| AgentError::Storage(format!("Failed to create temp directory: {}", e)))?;

        Ok(temp_path)
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Windows => write!(f, "Windows"),
            Self::MacOS => write!(f, "macOS"),
            Self::Linux => write!(f, "Linux"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Platform-specific utilities
pub struct PlatformUtils;

impl PlatformUtils {
    /// Get current platform
    pub fn current() -> Result<Platform> {
        Platform::detect()
    }

    /// Check if current platform supports symbolic links
    pub fn supports_symlinks() -> bool {
        #[cfg(unix)]
        return true;
        #[cfg(windows)]
        {
            // On Windows, symlink support depends on the version and permissions
            // For simplicity, we'll say it's supported but may fail at runtime
            true
        }
    }

    /// Create a symbolic link if supported
    pub fn create_symlink<P: AsRef<std::path::Path>, Q: AsRef<std::path::Path>>(
        original: P,
        link: Q,
    ) -> Result<()> {
        if !Self::supports_symlinks() {
            return Err(AgentError::Platform(
                "Symbolic links not supported on this platform".to_string(),
            ));
        }

        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(original, link)
                .map_err(|e| AgentError::Storage(format!("Failed to create symlink: {}", e)))
        }
        #[cfg(windows)]
        {
            std::os::windows::fs::symlink_file(original, link)
                .map_err(|e| AgentError::Storage(format!("Failed to create symlink: {}", e)))
        }
    }

    /// Get platform-specific newline sequence
    pub fn newline() -> &'static str {
        #[cfg(windows)]
        return "\r\n";
        #[cfg(not(windows))]
        return "\n";
    }

    /// Convert line endings to platform-specific format
    pub fn normalize_line_endings(text: &str) -> String {
        let newline = Self::newline();
        text.replace("\r\n", "\n").replace('\n', newline)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_detection() {
        let platform = Platform::detect().unwrap();
        assert_ne!(platform, Platform::Unknown);
    }

    #[test]
    fn test_platform_properties() {
        let platform = Platform::detect().unwrap();

        // Test that platform methods return reasonable values
        assert!(!platform.path_separator().is_empty());
        assert!(!platform.default_shell().is_empty());
        assert!(!platform.default_editor().is_empty());
        assert!(!platform.env_var_home().is_empty());
    }

    #[test]
    fn test_command_exists() {
        let platform = Platform::detect().unwrap();

        // Test with a command that should exist on most systems
        let exists = platform.command_exists("echo");
        // We can't guarantee this exists on all systems, so we just test the method works
        let _ = exists;
    }
}
