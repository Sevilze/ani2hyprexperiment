use anyhow::Result;
use std::fs;
use std::path::Path;

/// Create an index.theme file for a cursor theme
pub fn create_index_theme<P: AsRef<Path>>(
    theme_path: P,
    theme_name: &str,
    comment: &str,
    sizes: Option<&[u32]>,
) -> Result<()> {
    let theme_path = theme_path.as_ref();
    let index_path = theme_path.join("index.theme");
    
    let mut content = format!(
        r#"[Icon Theme]
Name={}
Comment={}
Inherits=hicolor

# Directory list
Directories=cursors

[cursors]
Context=Cursors
Type=Fixed
"#,
        theme_name, comment
    );
    
    // Add size-specific sections if sizes are provided
    if let Some(sizes) = sizes {
        for &size in sizes {
            content.push_str(&format!(
                r#"
[cursors/{}]
Size={}
Context=Cursors
Type=Fixed
"#,
                size, size
            ));
        }
    }
    
    fs::write(index_path, content)?;
    Ok(())
}

/// Create a cursor.theme file for a cursor theme
pub fn create_cursor_theme<P: AsRef<Path>>(
    theme_path: P,
    theme_name: &str,
    comment: &str,
) -> Result<()> {
    let theme_path = theme_path.as_ref();
    let cursor_theme_path = theme_path.join("cursor.theme");
    
    let content = format!(
        r#"[Icon Theme]
Name={}
Comment={}
Inherits={}
"#,
        theme_name, comment, theme_name
    );
    
    fs::write(cursor_theme_path, content)?;
    Ok(())
}

/// Create both theme configuration files
pub fn create_theme_files<P: AsRef<Path>>(
    theme_path: P,
    theme_name: &str,
    comment: &str,
    sizes: Option<&[u32]>,
) -> Result<()> {
    let theme_path = theme_path.as_ref();
    
    create_index_theme(theme_path, theme_name, comment, sizes)?;
    create_cursor_theme(theme_path, theme_name, comment)?;
    
    Ok(())
}

/// Standard cursor sizes used by modern themes
pub const STANDARD_SIZES: &[u32] = &[24, 32, 48, 64, 72, 96];

/// Create a hyprcursor manifest file
pub fn create_hyprcursor_manifest<P: AsRef<Path>>(
    theme_path: P,
    theme_name: &str,
    description: &str,
    version: &str,
) -> Result<()> {
    let theme_path = theme_path.as_ref();
    let manifest_path = theme_path.join("manifest.hl");
    
    let content = format!(
        r#"name = {}
description = {}
version = {}
cursors_directory = cursors
"#,
        theme_name, description, version
    );
    
    fs::write(manifest_path, content)?;
    Ok(())
}
