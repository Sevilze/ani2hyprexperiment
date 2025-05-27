use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::{
    cursor_mapping::get_cursor_symlinks,
    theme_config::create_theme_files,
    CursorTheme, FileUtils, CommandUtils, get_icons_dir,
};

/// Arguments for the add-links command
#[derive(Debug)]
pub struct AddLinksArgs {
    pub theme_name: String,
    pub source_dir: Option<PathBuf>,
}

/// Add missing symlinks to a cursor theme
pub fn add_missing_links(args: AddLinksArgs) -> Result<()> {
    println!("Adding missing links to cursor theme...");
    
    let script_dir = std::env::current_dir()
        .context("Failed to get current directory")?;
    let root_dir = script_dir.parent()
        .unwrap_or(&script_dir);
    
    let theme_path = root_dir.join(&args.theme_name);
    let theme = CursorTheme::new(args.theme_name.clone(), theme_path);
    
    // Remove existing theme and create new one
    if theme.path.exists() {
        fs::remove_dir_all(&theme.path)
            .context("Failed to remove existing theme directory")?;
    }
    theme.create_directories()?;
    
    // Find and copy cursor files
    let source_cursors = find_cursor_source(&args.source_dir)?;
    copy_cursor_files(&source_cursors, &theme.cursors_dir)?;
    
    // Create symlinks
    create_cursor_symlinks(&theme.cursors_dir)?;
    
    // Create theme configuration files
    create_theme_files(
        &theme.path,
        &args.theme_name,
        "Koosh cursor theme with all necessary symlinks",
        None,
    )?;
    
    // Install to user's .icons directory
    install_to_user_icons(&theme)?;
    
    // Set permissions
    FileUtils::set_permissions_recursive(&theme.path, 0o755)?;
    
    // Update icon cache
    update_icon_cache(&theme.name)?;
    
    println!("Done! Created new cursor theme: {:?}", theme.path);
    println!("Also installed to: {:?}", get_icons_dir()?.join(&theme.name));
    println!();
    println!("To use with Hyprland, add to your config:");
    println!("env = XCURSOR_THEME,{}", args.theme_name);
    println!("env = XCURSOR_SIZE,24");
    println!();
    println!("cursor {{");
    println!("    size = 24");
    println!("}}");
    println!();
    println!("Note: Since your cursor files don't support multiple sizes yet,");
    println!("it's best to use size 24 which is their native size.");
    
    Ok(())
}

/// Find the source directory for cursor files
fn find_cursor_source(source_dir: &Option<PathBuf>) -> Result<PathBuf> {
    if let Some(dir) = source_dir {
        if dir.exists() {
            return Ok(dir.clone());
        }
    }
    
    // Try different possible locations
    let current_dir = std::env::current_dir()?;
    
    // Check for cursors directory in current directory
    let cursors_dir = current_dir.join("cursors");
    if cursors_dir.exists() {
        return Ok(cursors_dir);
    }
    
    // Check for Koosh/cursors
    let koosh_cursors = current_dir.join("Koosh").join("cursors");
    if koosh_cursors.exists() {
        return Ok(koosh_cursors);
    }
    
    // Check user's .icons directory
    let user_koosh = get_icons_dir()?.join("Koosh").join("cursors");
    if user_koosh.exists() {
        return Ok(user_koosh);
    }
    
    Err(anyhow::anyhow!(
        "Error: Could not find Koosh cursor theme.\n\
         Please run this command from the Koosh directory or specify the source directory."
    ))
}

/// Copy cursor files from source to destination
fn copy_cursor_files(source: &Path, dest: &Path) -> Result<()> {
    println!("Copying cursor files from {:?} to {:?}", source, dest);
    
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            let file_name = path.file_name()
                .ok_or_else(|| anyhow::anyhow!("Invalid file name"))?;
            let dest_path = dest.join(file_name);
            
            fs::copy(&path, &dest_path)
                .with_context(|| format!("Failed to copy {:?} to {:?}", path, dest_path))?;
        }
    }
    
    Ok(())
}

/// Create cursor symlinks for compatibility
fn create_cursor_symlinks(cursors_dir: &Path) -> Result<()> {
    println!("Creating cursor symlinks...");
    
    let symlinks = get_cursor_symlinks();
    
    for (target, link_name) in symlinks {
        let target_path = cursors_dir.join(target);
        let link_path = cursors_dir.join(link_name);
        
        // Only create symlink if target exists and link doesn't exist
        if target_path.exists() && !link_path.exists() {
            FileUtils::create_symlink(target, &link_path)
                .with_context(|| format!("Failed to create symlink: {} -> {}", link_name, target))?;
        }
    }
    
    Ok(())
}

/// Install theme to user's .icons directory
fn install_to_user_icons(theme: &CursorTheme) -> Result<()> {
    let user_icons_dir = get_icons_dir()?;
    let user_theme_dir = user_icons_dir.join(&theme.name);
    
    // Remove existing installation
    if user_theme_dir.exists() {
        fs::remove_dir_all(&user_theme_dir)?;
    }
    
    // Copy theme to user directory
    FileUtils::copy_dir_recursive(&theme.path, &user_theme_dir)?;
    
    // Set permissions
    FileUtils::set_permissions_recursive(&user_theme_dir, 0o755)?;
    
    Ok(())
}

/// Update GTK icon cache
fn update_icon_cache(theme_name: &str) -> Result<()> {
    if CommandUtils::command_exists("gtk-update-icon-cache") {
        let user_theme_dir = get_icons_dir()?.join(theme_name);
        let _ = CommandUtils::run_command(
            "gtk-update-icon-cache",
            &["-f", "-t", user_theme_dir.to_str().unwrap()],
        );
        // Ignore errors as this is optional
    }
    Ok(())
}
