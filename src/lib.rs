use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub mod commands;
pub mod cursor_mapping;
pub mod theme_config;

pub use walkdir;

/// Common error types for cursor operations
#[derive(thiserror::Error, Debug)]
pub enum CursorError {
    #[error("Theme directory not found: {0}")]
    ThemeNotFound(PathBuf),
    #[error("Cursor file not found: {0}")]
    CursorNotFound(PathBuf),
    #[error("Command failed: {command} - {error}")]
    CommandFailed { command: String, error: String },
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Represents a cursor theme
#[derive(Debug, Clone)]
pub struct CursorTheme {
    pub name: String,
    pub path: PathBuf,
    pub cursors_dir: PathBuf,
}

impl CursorTheme {
    pub fn new(name: String, path: PathBuf) -> Self {
        let cursors_dir = path.join("cursors");
        Self {
            name,
            path,
            cursors_dir,
        }
    }

    pub fn exists(&self) -> bool {
        self.path.exists() && self.cursors_dir.exists()
    }

    pub fn create_directories(&self) -> Result<()> {
        fs::create_dir_all(&self.cursors_dir)
            .with_context(|| format!("Failed to create cursors directory: {:?}", self.cursors_dir))?;
        Ok(())
    }
}

/// Utility functions for file operations
pub struct FileUtils;

impl FileUtils {
    /// Create a symbolic link
    pub fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(
        target: P,
        link: Q,
    ) -> Result<()> {
        let target = target.as_ref();
        let link = link.as_ref();

        // Remove existing link if it exists
        if link.exists() || link.is_symlink() {
            fs::remove_file(link)
                .with_context(|| format!("Failed to remove existing link: {:?}", link))?;
        }

        #[cfg(unix)]
        std::os::unix::fs::symlink(target, link)
            .with_context(|| format!("Failed to create symlink: {:?} -> {:?}", link, target))?;

        #[cfg(windows)]
        std::os::windows::fs::symlink_file(target, link)
            .with_context(|| format!("Failed to create symlink: {:?} -> {:?}", link, target))?;

        Ok(())
    }

    /// Copy a file
    pub fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
        let from = from.as_ref();
        let to = to.as_ref();

        fs::copy(from, to)
            .with_context(|| format!("Failed to copy file: {:?} -> {:?}", from, to))?;
        Ok(())
    }

    /// Copy a directory recursively
    pub fn copy_dir_recursive<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
        let from = from.as_ref();
        let to = to.as_ref();

        if !from.exists() {
            return Err(anyhow::anyhow!("Source directory does not exist: {:?}", from));
        }

        fs::create_dir_all(to)
            .with_context(|| format!("Failed to create destination directory: {:?}", to))?;

        for entry in walkdir::WalkDir::new(from) {
            let entry = entry?;
            let path = entry.path();
            let relative_path = path.strip_prefix(from)?;
            let dest_path = to.join(relative_path);

            if path.is_dir() {
                fs::create_dir_all(&dest_path)
                    .with_context(|| format!("Failed to create directory: {:?}", dest_path))?;
            } else {
                if let Some(parent) = dest_path.parent() {
                    fs::create_dir_all(parent)
                        .with_context(|| format!("Failed to create parent directory: {:?}", parent))?;
                }
                fs::copy(path, &dest_path)
                    .with_context(|| format!("Failed to copy file: {:?} -> {:?}", path, dest_path))?;
            }
        }

        Ok(())
    }

    /// Set file permissions (Unix only)
    #[cfg(unix)]
    pub fn set_permissions_recursive<P: AsRef<Path>>(path: P, mode: u32) -> Result<()> {
        use std::os::unix::fs::PermissionsExt;

        let path = path.as_ref();
        for entry in walkdir::WalkDir::new(path) {
            let entry = entry?;
            let permissions = fs::Permissions::from_mode(mode);
            fs::set_permissions(entry.path(), permissions)
                .with_context(|| format!("Failed to set permissions for: {:?}", entry.path()))?;
        }
        Ok(())
    }

    #[cfg(not(unix))]
    pub fn set_permissions_recursive<P: AsRef<Path>>(_path: P, _mode: u32) -> Result<()> {
        // No-op on non-Unix systems
        Ok(())
    }
}

/// Utility functions for running external commands
pub struct CommandUtils;

impl CommandUtils {
    /// Run a command and return success/failure
    pub fn run_command(command: &str, args: &[&str]) -> Result<()> {
        let output = Command::new(command)
            .args(args)
            .output()
            .with_context(|| format!("Failed to execute command: {}", command))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(CursorError::CommandFailed {
                command: format!("{} {}", command, args.join(" ")),
                error: error.to_string(),
            }.into());
        }

        Ok(())
    }

    /// Check if a command is available
    pub fn command_exists(command: &str) -> bool {
        Command::new("which")
            .arg(command)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

/// Get the user's home directory
pub fn get_home_dir() -> Result<PathBuf> {
    home::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))
}

/// Get the user's .icons directory
pub fn get_icons_dir() -> Result<PathBuf> {
    Ok(get_home_dir()?.join(".icons"))
}
