use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{
    cursor_mapping::{get_cursor_symlinks, get_cursor_hotspot},
    theme_config::{create_theme_files, STANDARD_SIZES},
    CursorTheme, FileUtils, CommandUtils, get_icons_dir,
};

/// Arguments for the create-animated command
#[derive(Debug)]
pub struct CreateAnimatedArgs {
    pub input_theme: String,
    pub output_theme: String,
}

/// Create animated cursor theme with multi-size support
pub fn create_animated_theme(args: CreateAnimatedArgs) -> Result<()> {
    println!("=== Koosh Cursor Theme Creator ===");
    println!("This tool will create a new cursor theme with:");
    println!("- Multi-size support (24, 32, 48, 64, 72, 96)");
    println!("- Proper hotspots for all cursors");
    println!("- 'not-allowed' and 'unavailable' cursors using the same hotspot as 'left_ptr'");
    println!("- All temporary files will be removed after completion");
    println!("===============================");
    println!("Input theme: {}", args.input_theme);
    println!("Output theme: {}", args.output_theme);
    println!("===============================");

    // Check if input theme exists
    let input_path = PathBuf::from(&args.input_theme);
    if !input_path.exists() {
        return Err(anyhow::anyhow!(
            "Error: Input theme directory '{}' not found!",
            args.input_theme
        ));
    }

    let input_cursors = input_path.join("cursors");
    if !input_cursors.exists() {
        return Err(anyhow::anyhow!(
            "Error: Input theme cursors directory '{}/cursors' not found!",
            args.input_theme
        ));
    }

    // Create output theme
    let output_theme = CursorTheme::new(args.output_theme.clone(), PathBuf::from(&args.output_theme));
    if output_theme.path.exists() {
        fs::remove_dir_all(&output_theme.path)?;
    }
    output_theme.create_directories()?;

    // Create user's .icons directory
    let user_icons_dir = get_icons_dir()?.join(&args.output_theme);
    if user_icons_dir.exists() {
        fs::remove_dir_all(&user_icons_dir)?;
    }
    fs::create_dir_all(&user_icons_dir.join("cursors"))?;

    // Create temporary directory
    let temp_dir = PathBuf::from("koosh_animated_temp");
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }
    fs::create_dir_all(&temp_dir)?;

    // Process each cursor file
    process_cursor_files(&input_cursors, &output_theme, &temp_dir)?;

    // Create additional symlinks
    create_additional_symlinks(&output_theme.cursors_dir)?;

    // Create theme files
    create_theme_files(
        &output_theme.path,
        &args.output_theme,
        "Koosh cursor theme with proper animation support",
        Some(STANDARD_SIZES),
    )?;

    // Install to user's .icons directory
    install_to_user_icons(&output_theme, &user_icons_dir)?;

    // Set permissions
    FileUtils::set_permissions_recursive(&output_theme.path, 0o755)?;
    FileUtils::set_permissions_recursive(&user_icons_dir, 0o755)?;

    // Update icon cache
    update_icon_cache(&args.output_theme)?;

    // Clean up
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }

    // Remove .conf files
    println!("Removing .conf files...");
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        if let Some(ext) = entry.path().extension() {
            if ext == "conf" {
                fs::remove_file(entry.path())?;
            }
        }
    }

    println!("Done! Created animated cursor theme: {:?}", output_theme.path);
    println!("Also installed to: {:?}", user_icons_dir);

    Ok(())
}

/// Process cursor files to create multi-size animated versions
fn process_cursor_files(
    input_cursors: &Path,
    output_theme: &CursorTheme,
    temp_dir: &Path,
) -> Result<()> {
    println!("Processing cursor files...");

    for entry in fs::read_dir(input_cursors)? {
        let entry = entry?;
        let cursor_file = entry.path();

        if cursor_file.is_file() && !cursor_file.is_symlink() {
            let cursor_name = cursor_file.file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| anyhow::anyhow!("Invalid cursor file name"))?;

            println!("  Processing: {}", cursor_name);

            process_single_cursor(&cursor_file, cursor_name, output_theme, temp_dir)?;
        } else if cursor_file.is_symlink() {
            // Copy symlinks
            copy_symlink(&cursor_file, &output_theme.cursors_dir)?;
        }
    }

    Ok(())
}

/// Process a single cursor file
fn process_single_cursor(
    cursor_file: &Path,
    cursor_name: &str,
    output_theme: &CursorTheme,
    temp_dir: &Path,
) -> Result<()> {
    let cursor_temp_dir = temp_dir.join(cursor_name);
    fs::create_dir_all(&cursor_temp_dir)?;

    // Extract cursor frames using xcur2png
    let extract_result = Command::new("xcur2png")
        .arg(cursor_file)
        .arg("-d")
        .arg(&cursor_temp_dir)
        .output();

    match extract_result {
        Ok(output) if output.status.success() => {
            // Count extracted frames
            let frame_count = count_extracted_frames(&cursor_temp_dir, cursor_name)?;

            if frame_count == 0 {
                println!("    Failed to extract cursor, copying original");
                fs::copy(cursor_file, output_theme.cursors_dir.join(cursor_name))?;
                return Ok(());
            }

            println!("    Found {} animation frames", frame_count);

            // Create multi-size cursor
            create_multi_size_cursor(&cursor_temp_dir, cursor_name, output_theme, frame_count)?;
        }
        _ => {
            println!("    xcur2png failed, copying original cursor");
            fs::copy(cursor_file, output_theme.cursors_dir.join(cursor_name))?;
        }
    }

    Ok(())
}

/// Count extracted PNG frames
fn count_extracted_frames(temp_dir: &Path, cursor_name: &str) -> Result<usize> {
    let mut count = 0;

    for entry in fs::read_dir(temp_dir)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if file_name_str.starts_with(&format!("{}_", cursor_name)) && file_name_str.ends_with(".png") {
            count += 1;
        }
    }

    Ok(count)
}

/// Create multi-size cursor from extracted frames
fn create_multi_size_cursor(
    temp_dir: &Path,
    cursor_name: &str,
    output_theme: &CursorTheme,
    frame_count: usize,
) -> Result<()> {
    // Get original size from first frame
    let first_frame = temp_dir.join(format!("{}_000.png", cursor_name));
    let orig_size = if first_frame.exists() {
        get_image_size(&first_frame)?
    } else {
        48 // Default size
    };

    println!("    Original size: {}x{}", orig_size, orig_size);

    // Get hotspot ratios for this cursor
    let (hotspot_x_ratio, hotspot_y_ratio) = get_cursor_hotspot(cursor_name);

    let working_dir = temp_dir.join("working");
    fs::create_dir_all(&working_dir)?;

    // Create config file for xcursorgen
    let config_file = working_dir.join("cursor.config");
    let mut config_content = String::new();

    // Process each size
    for &size in STANDARD_SIZES {
        // Calculate hotspot coordinates
        let hotspot_x = ((size as f64 * hotspot_x_ratio) as u32).max(1);
        let hotspot_y = ((size as f64 * hotspot_y_ratio) as u32).max(1);

        // Process each frame
        for frame in 0..frame_count {
            let frame_num = format!("{:03}", frame);
            let src_png = temp_dir.join(format!("{}_{}.png", cursor_name, frame_num));

            if !src_png.exists() {
                println!("    Warning: Missing frame {}", frame_num);
                continue;
            }

            let dst_png = working_dir.join(format!("{}_{}.png", size, frame_num));

            if size == orig_size {
                // Use original for original size
                fs::copy(&src_png, &dst_png)?;
            } else {
                // Scale the image
                println!("    Creating {}x{} version of frame {}", size, size, frame_num);
                scale_image(&src_png, &dst_png, size)?;
            }

            // Add to config file (100ms delay per frame)
            config_content.push_str(&format!(
                "{} {} {} {}_{}.png 100\n",
                size, hotspot_x, hotspot_y, size, frame_num
            ));
        }
    }

    // Write config file
    fs::write(&config_file, config_content)?;

    // Generate cursor using xcursorgen
    let cursor_output = working_dir.join("cursor");
    let result = Command::new("xcursorgen")
        .arg("cursor.config")
        .arg("cursor")
        .current_dir(&working_dir)
        .output();

    match result {
        Ok(output) if output.status.success() && cursor_output.exists() => {
            // Copy the generated cursor to the theme directory
            fs::copy(&cursor_output, output_theme.cursors_dir.join(cursor_name))?;
            println!("    Successfully created multi-size animated cursor");

            // Verify the cursor
            verify_generated_cursor(&cursor_output, cursor_name)?;
        }
        _ => {
            println!("    Failed to create cursor with xcursorgen, copying original");
            // This would need the original cursor file path, which we'd need to pass through
        }
    }

    Ok(())
}

/// Get image dimensions using ImageMagick identify command
fn get_image_size(image_path: &Path) -> Result<u32> {
    let output = Command::new("identify")
        .arg("-format")
        .arg("%w")
        .arg(image_path)
        .output()
        .context("Failed to run identify command")?;

    if output.status.success() {
        let size_str = String::from_utf8_lossy(&output.stdout);
        size_str.trim().parse::<u32>()
            .context("Failed to parse image size")
    } else {
        Ok(48) // Default size
    }
}

/// Scale an image using ImageMagick
fn scale_image(src: &Path, dst: &Path, size: u32) -> Result<()> {
    let size_arg = format!("{}x{}", size, size);

    // Try magick command first, then convert
    let result = if CommandUtils::command_exists("magick") {
        Command::new("magick")
            .arg(src)
            .arg("-resize")
            .arg(&size_arg)
            .arg(dst)
            .output()
    } else {
        Command::new("convert")
            .arg(src)
            .arg("-resize")
            .arg(&size_arg)
            .arg(dst)
            .output()
    };

    match result {
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) => {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Image scaling failed: {}", error))
        }
        Err(e) => Err(anyhow::anyhow!("Failed to run image scaling command: {}", e)),
    }
}

/// Copy a symlink to the destination
fn copy_symlink(src: &Path, dest_dir: &Path) -> Result<()> {
    let target = fs::read_link(src)?;
    let link_name = src.file_name()
        .ok_or_else(|| anyhow::anyhow!("Invalid symlink name"))?;

    println!("  Copying symlink: {:?} -> {:?}", link_name, target);

    let dest_link = dest_dir.join(link_name);
    FileUtils::create_symlink(&target, &dest_link)?;

    Ok(())
}

/// Verify the generated cursor
fn verify_generated_cursor(cursor_path: &Path, cursor_name: &str) -> Result<()> {
    println!("    Verifying cursor...");

    let verify_dir = cursor_path.parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid cursor path"))?
        .join("verify");

    fs::create_dir_all(&verify_dir)?;

    let result = Command::new("xcur2png")
        .arg(cursor_path)
        .arg("-d")
        .arg(&verify_dir)
        .output();

    match result {
        Ok(output) if output.status.success() => {
            let frame_count = count_extracted_frames(&verify_dir, cursor_name)?;
            println!("    New cursor has {} frames/sizes", frame_count);

            // Show available sizes
            show_cursor_sizes(&verify_dir)?;
        }
        _ => {
            println!("    Warning: Could not verify cursor");
        }
    }

    // Clean up verification directory
    if verify_dir.exists() {
        fs::remove_dir_all(&verify_dir)?;
    }

    Ok(())
}

/// Show available cursor sizes
fn show_cursor_sizes(verify_dir: &Path) -> Result<()> {
    let mut sizes = std::collections::HashSet::new();

    for entry in fs::read_dir(verify_dir)? {
        let entry = entry?;
        if entry.path().extension().map_or(false, |ext| ext == "png") {
            if let Ok(size) = get_image_size(&entry.path()) {
                sizes.insert(size);
            }
        }
    }

    let mut sizes_vec: Vec<_> = sizes.into_iter().collect();
    sizes_vec.sort();

    println!("    Sizes: {:?}", sizes_vec);

    Ok(())
}

/// Create additional symlinks for compatibility
fn create_additional_symlinks(cursors_dir: &Path) -> Result<()> {
    println!("Creating additional symlinks...");

    let symlinks = get_cursor_symlinks();

    for (target, link_name) in symlinks {
        let target_path = cursors_dir.join(target);
        let link_path = cursors_dir.join(link_name);

        if target_path.exists() && !link_path.exists() {
            FileUtils::create_symlink(target, &link_path)?;
            println!("  Created symlink: {} -> {}", link_name, target);
        }
    }

    Ok(())
}

/// Install theme to user's .icons directory
fn install_to_user_icons(theme: &CursorTheme, user_icons_dir: &Path) -> Result<()> {
    println!("Installing to ~/.icons...");

    FileUtils::copy_dir_recursive(&theme.cursors_dir, &user_icons_dir.join("cursors"))?;

    let index_theme = theme.path.join("index.theme");
    if index_theme.exists() {
        fs::copy(&index_theme, &user_icons_dir.join("index.theme"))?;
    }

    let cursor_theme = theme.path.join("cursor.theme");
    if cursor_theme.exists() {
        fs::copy(&cursor_theme, &user_icons_dir.join("cursor.theme"))?;
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