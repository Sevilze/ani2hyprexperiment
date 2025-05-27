use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::{
    theme_config::{create_theme_files, create_hyprcursor_manifest},
    CursorTheme, FileUtils, CommandUtils, get_icons_dir,
};

/// Arguments for the create-hyprcursor command
#[derive(Debug)]
pub struct CreateHyprcursorArgs {
    pub source_theme: String,
    pub dest_theme: String,
}

/// Create a hyprcursor theme from an existing cursor theme
pub fn create_hyprcursor_theme(args: CreateHyprcursorArgs) -> Result<()> {
    println!("Creating hyprcursor theme from {}...", args.source_theme);
    
    // Define working directories
    let extract_dir = PathBuf::from("koosh_extract");
    let output_dir = PathBuf::from("koosh_hyprcursor");
    
    // Step 1: Extract the source theme
    extract_source_theme(&args.source_theme, &extract_dir)?;
    
    // Step 2: Update the manifest file
    update_manifest(&extract_dir, &args.source_theme, &args.dest_theme)?;
    
    // Step 3: Create the hyprcursor theme
    create_hyprcursor(&extract_dir, &args.source_theme, &output_dir, &args.dest_theme)?;
    
    // Step 4: Install the theme
    install_hyprcursor_theme(&output_dir, &args.dest_theme)?;
    
    // Step 5: Copy X11 cursors for compatibility
    copy_x11_cursors(&args.source_theme, &args.dest_theme)?;
    
    // Step 6: Create theme configuration files
    create_hyprcursor_config(&args.dest_theme)?;
    
    // Step 7: Update icon cache
    update_icon_cache(&args.dest_theme)?;
    
    // Step 8: Clean up
    cleanup(&extract_dir, &output_dir)?;
    
    println!("Done! Created hyprcursor theme: {}", args.dest_theme);
    
    Ok(())
}

/// Extract the source theme using hyprcursor-util
fn extract_source_theme(source_theme: &str, extract_dir: &Path) -> Result<()> {
    println!("Step 1: Extracting source theme...");
    
    // Remove existing extract directory
    if extract_dir.exists() {
        fs::remove_dir_all(extract_dir)?;
    }
    fs::create_dir_all(extract_dir)?;
    
    let source_path = get_icons_dir()?.join(source_theme);
    
    if !source_path.exists() {
        return Err(anyhow::anyhow!(
            "Source theme not found: {:?}",
            source_path
        ));
    }
    
    // Run hyprcursor-util extract
    CommandUtils::run_command(
        "hyprcursor-util",
        &[
            "--extract",
            source_path.to_str().unwrap(),
            "--output",
            extract_dir.to_str().unwrap(),
        ],
    ).context("Failed to extract source theme with hyprcursor-util")?;
    
    Ok(())
}

/// Update the manifest file with new theme information
fn update_manifest(
    extract_dir: &Path,
    source_theme: &str,
    dest_theme: &str,
) -> Result<()> {
    println!("Step 2: Updating manifest file...");
    
    let manifest_path = extract_dir
        .join(format!("extracted_{}", source_theme))
        .join("manifest.hl");
    
    if !manifest_path.exists() {
        return Err(anyhow::anyhow!(
            "Manifest file not found: {:?}",
            manifest_path
        ));
    }
    
    // Read the existing manifest
    let manifest_content = fs::read_to_string(&manifest_path)?;
    
    // Update the manifest content
    let updated_content = manifest_content
        .lines()
        .map(|line| {
            if line.starts_with("name = ") {
                format!("name = {}", dest_theme)
            } else if line.starts_with("description = ") {
                "description = Koosh cursor theme with hyprcursor support for Wayland".to_string()
            } else if line.starts_with("version = ") {
                "version = 1.0".to_string()
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    
    // Write the updated manifest
    fs::write(&manifest_path, updated_content)?;
    
    Ok(())
}

/// Create the hyprcursor theme using hyprcursor-util
fn create_hyprcursor(
    extract_dir: &Path,
    source_theme: &str,
    output_dir: &Path,
    dest_theme: &str,
) -> Result<()> {
    println!("Step 3: Creating hyprcursor theme...");
    
    // Remove existing output directory
    if output_dir.exists() {
        fs::remove_dir_all(output_dir)?;
    }
    fs::create_dir_all(output_dir)?;
    
    let extracted_theme_dir = extract_dir.join(format!("extracted_{}", source_theme));
    
    // Run hyprcursor-util create
    CommandUtils::run_command(
        "hyprcursor-util",
        &[
            "--create",
            extracted_theme_dir.to_str().unwrap(),
            "--output",
            output_dir.to_str().unwrap(),
        ],
    ).context("Failed to create hyprcursor theme")?;
    
    Ok(())
}

/// Install the hyprcursor theme to user's .icons directory
fn install_hyprcursor_theme(output_dir: &Path, dest_theme: &str) -> Result<()> {
    println!("Step 4: Installing theme to ~/.icons/{}...", dest_theme);
    
    let user_icons_dir = get_icons_dir()?;
    let user_theme_dir = user_icons_dir.join(dest_theme);
    
    // Remove existing installation
    if user_theme_dir.exists() {
        fs::remove_dir_all(&user_theme_dir)?;
    }
    fs::create_dir_all(&user_theme_dir)?;
    
    // Copy the generated theme
    let theme_output_dir = output_dir.join(format!("theme_{}", dest_theme));
    if theme_output_dir.exists() {
        FileUtils::copy_dir_recursive(&theme_output_dir, &user_theme_dir)?;
    } else {
        return Err(anyhow::anyhow!(
            "Generated theme directory not found: {:?}",
            theme_output_dir
        ));
    }
    
    Ok(())
}

/// Copy X11 cursors for compatibility
fn copy_x11_cursors(source_theme: &str, dest_theme: &str) -> Result<()> {
    println!("Step 5: Copying X11 cursors for compatibility...");
    
    let source_cursors = get_icons_dir()?.join(source_theme).join("cursors");
    let dest_cursors = get_icons_dir()?.join(dest_theme).join("cursors");
    
    if source_cursors.exists() {
        fs::create_dir_all(&dest_cursors)?;
        FileUtils::copy_dir_recursive(&source_cursors, &dest_cursors)?;
    }
    
    Ok(())
}

/// Create theme configuration files
fn create_hyprcursor_config(dest_theme: &str) -> Result<()> {
    println!("Step 6: Creating theme configuration files...");
    
    let user_theme_dir = get_icons_dir()?.join(dest_theme);
    
    // Create index.theme
    let index_content = format!(
        r#"[Icon Theme]
Name={}
Comment=Koosh cursor theme with hyprcursor support for Wayland
Inherits=hicolor

# Directory list
Directories=cursors hyprcursors

[cursors]
Context=Cursors
Type=Fixed

[hyprcursors]
Context=Cursors
Type=Fixed
"#,
        dest_theme
    );
    
    fs::write(user_theme_dir.join("index.theme"), index_content)?;
    
    // Create cursor.theme
    let cursor_content = format!(
        r#"[Icon Theme]
Name={}
Comment=Koosh cursor theme with hyprcursor support for Wayland
Inherits={}
"#,
        dest_theme, dest_theme
    );
    
    fs::write(user_theme_dir.join("cursor.theme"), cursor_content)?;
    
    Ok(())
}

/// Update GTK icon cache
fn update_icon_cache(dest_theme: &str) -> Result<()> {
    if CommandUtils::command_exists("gtk-update-icon-cache") {
        println!("Step 7: Updating icon cache...");
        let user_theme_dir = get_icons_dir()?.join(dest_theme);
        let _ = CommandUtils::run_command(
            "gtk-update-icon-cache",
            &["-f", "-t", user_theme_dir.to_str().unwrap()],
        );
        // Ignore errors as this is optional
    }
    Ok(())
}

/// Clean up temporary directories
fn cleanup(extract_dir: &Path, output_dir: &Path) -> Result<()> {
    println!("Step 8: Cleaning up...");
    
    if extract_dir.exists() {
        fs::remove_dir_all(extract_dir)?;
    }
    
    if output_dir.exists() {
        fs::remove_dir_all(output_dir)?;
    }
    
    Ok(())
}
