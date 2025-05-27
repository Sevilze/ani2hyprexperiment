use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use koosh_cursor_tools::commands::{
    add_links::{add_missing_links, AddLinksArgs},
    create_animated::{create_animated_theme, CreateAnimatedArgs},
    create_hyprcursor::{create_hyprcursor_theme, CreateHyprcursorArgs},
    rename_cursors::{rename_cursors, RenameCursorsArgs},
};

#[derive(Parser)]
#[command(name = "koosh-cursor-tools")]
#[command(about = "Rust implementation of Koosh cursor theme management tools")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add missing symlinks to a cursor theme and install it
    AddLinks {
        /// Name of the theme to create (default: Koosh-Complete)
        #[arg(short, long, default_value = "Koosh-Complete")]
        theme_name: String,
        
        /// Source directory containing cursor files
        #[arg(short, long)]
        source_dir: Option<PathBuf>,
    },
    
    /// Create animated cursor theme with multi-size support
    CreateAnimated {
        /// Input theme directory (default: Koosh-X11)
        #[arg(short, long, default_value = "Koosh-X11")]
        input_theme: String,
        
        /// Output theme name (default: Koosh-Animated)
        #[arg(short, long, default_value = "Koosh-Animated")]
        output_theme: String,
    },
    
    /// Create hyprcursor theme from an existing animated theme
    CreateHyprcursor {
        /// Source theme name (default: Koosh-Animated)
        #[arg(short, long, default_value = "Koosh-Animated")]
        source_theme: String,
        
        /// Destination theme name (default: Koosh-Hyprcursor2)
        #[arg(short, long, default_value = "Koosh-Hyprcursor2")]
        dest_theme: String,
    },
    
    /// Rename cursor files from Windows names to X11 names
    RenameCursors {
        /// Input directory containing Windows-named cursor files
        #[arg(short, long, default_value = "output")]
        input_dir: PathBuf,
        
        /// Output theme name (default: Koosh-X11)
        #[arg(short, long, default_value = "Koosh-X11")]
        output_theme: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::AddLinks { theme_name, source_dir } => {
            let args = AddLinksArgs {
                theme_name,
                source_dir,
            };
            add_missing_links(args)
        }
        
        Commands::CreateAnimated { input_theme, output_theme } => {
            let args = CreateAnimatedArgs {
                input_theme,
                output_theme,
            };
            create_animated_theme(args)
        }
        
        Commands::CreateHyprcursor { source_theme, dest_theme } => {
            let args = CreateHyprcursorArgs {
                source_theme,
                dest_theme,
            };
            create_hyprcursor_theme(args)
        }
        
        Commands::RenameCursors { input_dir, output_theme } => {
            let args = RenameCursorsArgs {
                input_dir,
                output_theme,
            };
            rename_cursors(args)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
