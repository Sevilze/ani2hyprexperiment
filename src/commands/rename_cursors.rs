use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::{
    cursor_mapping::{get_windows_to_x11_mapping, get_cursor_symlinks},
    theme_config::create_theme_files,
    CursorTheme, FileUtils, CommandUtils, get_icons_dir,
};

/// Arguments for the rename-cursors command
#[derive(Debug)]
pub struct RenameCursorsArgs {
    pub input_dir: PathBuf,
    pub output_theme: String,
}

/// Rename cursor files from Windows names to X11 names
pub fn rename_cursors(args: RenameCursorsArgs) -> Result<()> {
    println!("Renaming cursor files from Windows to X11 format...");
    println!("Input directory: {:?}", args.input_dir);
    println!("Output theme: {}", args.output_theme);
    
    // Validate input directory
    if !args.input_dir.exists() {
        return Err(anyhow::anyhow!("Input directory does not exist: {:?}", args.input_dir));
    }
    
    let script_dir = std::env::current_dir()?;
    let output_path = script_dir.join(&args.output_theme);
    let theme = CursorTheme::new(args.output_theme.clone(), output_path);
    
    // Create output directory
    if theme.path.exists() {
        fs::remove_dir_all(&theme.path)?;
    }
    theme.create_directories()?;
    
    // Process cursor files
    process_cursor_files(&args.input_dir, &theme)?;
    
    // Create symlinks
    create_compatibility_symlinks(&theme.cursors_dir)?;
    
    // Create theme files
    create_theme_files(
        &theme.path,
        &args.output_theme,
        "Koosh cursor theme",
        None,
    )?;
    
    // Install to user's .icons directory
    install_to_user_icons(&theme)?;
    
    // Set permissions
    FileUtils::set_permissions_recursive(&theme.path, 0o755)?;
    
    // Update icon cache
    update_icon_cache(&theme.name)?;
    
    println!("Done! Created X11 cursor theme: {}", args.output_theme);
    println!("Listing files in {:?}:", theme.cursors_dir);
    list_cursor_files(&theme.cursors_dir)?;
    
    Ok(())
}

/// Process cursor files and rename them
fn process_cursor_files(input_dir: &Path, theme: &CursorTheme) -> Result<()> {
    let mapping = get_windows_to_x11_mapping();
    
    println!("Processing cursor files...");
    
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            let file_name = path.file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| anyhow::anyhow!("Invalid file name"))?;
            
            if let Some(&x11_name) = mapping.get(file_name) {
                println!("  Copying {} to {}", file_name, x11_name);
                
                let dest_path = theme.cursors_dir.join(x11_name);
                fs::copy(&path, &dest_path)
                    .with_context(|| format!("Failed to copy cursor file: {:?}", path))?;
                
                if dest_path.exists() {
                    println!("    Successfully copied cursor");
                    println!("    Verified: File exists at destination");
                } else {
                    println!("    Error: File does not exist at destination");
                }
            } else {
                println!("  Skipping {} (no mapping defined)", file_name);
            }
        }
    }
    
    Ok(())
}

/// Create compatibility symlinks
fn create_compatibility_symlinks(cursors_dir: &Path) -> Result<()> {
    println!("Creating symlinks...");
    
    let symlinks = get_cursor_symlinks();
    
    for (target, link_name) in symlinks {
        let target_path = cursors_dir.join(target);
        let link_path = cursors_dir.join(link_name);
        
        if target_path.exists() && !link_path.exists() {
            FileUtils::create_symlink(target, &link_path)
                .with_context(|| format!("Failed to create symlink: {} -> {}", link_name, target))?;
            println!("  Created symlink: {} -> {}", link_name, target);
        }
    }
    
    Ok(())
}

/// Install theme to user's .icons directory
fn install_to_user_icons(theme: &CursorTheme) -> Result<()> {
    let user_icons_dir = get_icons_dir()?;
    let user_theme_dir = user_icons_dir.join(&theme.name);
    
    if theme.path != user_theme_dir {
        println!("Installing to {:?}", user_theme_dir);
        
        // Remove existing installation
        if user_theme_dir.exists() {
            fs::remove_dir_all(&user_theme_dir)?;
        }
        fs::create_dir_all(&user_theme_dir)?;
        
        // Copy files
        if theme.cursors_dir.exists() {
            FileUtils::copy_dir_recursive(&theme.cursors_dir, &user_theme_dir.join("cursors"))?;
        }
        
        let index_theme = theme.path.join("index.theme");
        if index_theme.exists() {
            fs::copy(&index_theme, &user_theme_dir.join("index.theme"))?;
        }
        
        let cursor_theme = theme.path.join("cursor.theme");
        if cursor_theme.exists() {
            fs::copy(&cursor_theme, &user_theme_dir.join("cursor.theme"))?;
        }
        
        // Set permissions
        FileUtils::set_permissions_recursive(&user_theme_dir, 0o755)?;
    }
    
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

/// List cursor files in the directory
fn list_cursor_files(cursors_dir: &Path) -> Result<()> {
    for entry in fs::read_dir(cursors_dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;
        
        let file_type = if path.is_symlink() {
            let target = fs::read_link(&path)?;
            format!("-> {:?}", target)
        } else if metadata.is_file() {
            format!("{} bytes", metadata.len())
        } else {
            "directory".to_string()
        };
        
        println!("  {:?} ({})", path.file_name().unwrap(), file_type);
    }
    
    Ok(())
}
