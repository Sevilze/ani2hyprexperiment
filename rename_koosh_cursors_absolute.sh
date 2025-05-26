#!/bin/bash
# Script to rename Koosh cursor files from output directory to X11 cursor names using absolute paths

# Define absolute paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INPUT_DIR="$SCRIPT_DIR/output"
OUTPUT_DIR="$SCRIPT_DIR/Koosh-X11"

echo "Script directory: $SCRIPT_DIR"
echo "Input directory: $INPUT_DIR"
echo "Output directory: $OUTPUT_DIR"

# Create output directory
rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR/cursors"

# Define cursor mappings (Windows cursor name to X11 cursor name)
declare -A CURSOR_MAP
CURSOR_MAP["Normal"]="left_ptr"
CURSOR_MAP["Link"]="link"
CURSOR_MAP["Person"]="pointer"
CURSOR_MAP["Handwriting"]="pencil"
CURSOR_MAP["Text"]="text"
CURSOR_MAP["Unavailable"]="not-allowed"
CURSOR_MAP["Busy"]="wait"
CURSOR_MAP["Working"]="progress"
CURSOR_MAP["Precision"]="crosshair"
CURSOR_MAP["Move"]="move"
CURSOR_MAP["Alternate"]="question_arrow"
CURSOR_MAP["Help"]="help"
CURSOR_MAP["Pin"]="pin"
CURSOR_MAP["Horizontal"]="size_hor"
CURSOR_MAP["Vertical"]="size_ver"
CURSOR_MAP["Diagonal1"]="size_bdiag"
CURSOR_MAP["Diagonal2"]="size_fdiag"

# Process each cursor file
echo "Processing cursor files..."
for cursor_file in "$INPUT_DIR"/*; do
    base_name=$(basename "$cursor_file")
    
    if [[ -n "${CURSOR_MAP[$base_name]}" ]]; then
        x11_name="${CURSOR_MAP[$base_name]}"
        echo "  Copying $base_name to $x11_name"
        
        # Simply copy the file with the new name
        cp "$cursor_file" "$OUTPUT_DIR/cursors/$x11_name"
        
        if [ $? -eq 0 ]; then
            echo "    Successfully copied cursor"
            # Verify file exists
            if [ -f "$OUTPUT_DIR/cursors/$x11_name" ]; then
                echo "    Verified: File exists at destination"
            else
                echo "    Error: File does not exist at destination"
            fi
        else
            echo "    Failed to copy cursor"
        fi
    else
        echo "  Skipping $base_name (no mapping defined)"
    fi
done

# Create symlinks for compatibility
echo "Creating symlinks..."
pushd "$OUTPUT_DIR/cursors" > /dev/null || {
    echo "Error: Failed to change directory to $OUTPUT_DIR/cursors"
    exit 1
}

# Function to create a symlink if the target exists
create_link() {
    if [ -e "$1" ] && [ ! -e "$2" ]; then
        ln -sf "$1" "$2"
        echo "  Created symlink: $2 -> $1"
    fi
}

# Basic cursor symlinks
create_link "left_ptr" "arrow"
create_link "left_ptr" "default"
create_link "left_ptr" "top_left_arrow"
create_link "pointer" "hand1"
create_link "pointer" "hand2"
create_link "pointer" "pointing_hand"
create_link "pointer" "openhand"
create_link "pointer" "grab"
create_link "move" "fleur"
create_link "move" "all-scroll"
create_link "move" "size_all"
create_link "wait" "watch"
create_link "progress" "left_ptr_watch"
create_link "crosshair" "cross"
create_link "text" "xterm"
create_link "text" "ibeam"
create_link "pencil" "draft"
create_link "question_arrow" "help"
create_link "question_arrow" "whats_this"
create_link "question_arrow" "left_ptr_help"
create_link "not-allowed" "crossed_circle"
create_link "not-allowed" "forbidden"
create_link "not-allowed" "no_drop"
create_link "not-allowed" "dnd_no_drop"
create_link "size_hor" "sb_h_double_arrow"
create_link "size_hor" "h_double_arrow"
create_link "size_hor" "ew-resize"
create_link "size_hor" "col-resize"
create_link "size_hor" "split_h"
create_link "size_ver" "sb_v_double_arrow"
create_link "size_ver" "v_double_arrow"
create_link "size_ver" "ns-resize"
create_link "size_ver" "row-resize"
create_link "size_ver" "split_v"
create_link "size_fdiag" "fd_double_arrow"
create_link "size_fdiag" "nesw-resize"
create_link "size_bdiag" "bd_double_arrow"
create_link "size_bdiag" "nwse-resize"
create_link "left_ptr" "wayland-cursor"

popd > /dev/null || {
    echo "Error: Failed to return to original directory"
    exit 1
}

# Create index.theme file
echo "Creating theme files..."
cat > "$OUTPUT_DIR/index.theme" << EOF
[Icon Theme]
Name=Koosh-X11
Comment=Koosh cursor theme
Inherits=hicolor

# Directory list
Directories=cursors

[cursors]
Context=Cursors
Type=Fixed
EOF

# Create cursor.theme file
cat > "$OUTPUT_DIR/cursor.theme" << EOF
[Icon Theme]
Name=Koosh-X11
Comment=Koosh cursor theme
Inherits=hicolor
EOF

# Also install to user's .icons directory
USER_ICONS="$HOME/.icons/Koosh-X11"
if [ "$OUTPUT_DIR" != "$USER_ICONS" ]; then
    echo "Installing to $USER_ICONS"
    rm -rf "$USER_ICONS"
    mkdir -p "$USER_ICONS"
    
    # Copy files to user's .icons directory
    if [ -d "$OUTPUT_DIR/cursors" ]; then
        cp -r "$OUTPUT_DIR/cursors" "$USER_ICONS/"
    fi
    if [ -f "$OUTPUT_DIR/index.theme" ]; then
        cp "$OUTPUT_DIR/index.theme" "$USER_ICONS/"
    fi
    if [ -f "$OUTPUT_DIR/cursor.theme" ]; then
        cp "$OUTPUT_DIR/cursor.theme" "$USER_ICONS/"
    fi
fi

# Set proper permissions
chmod -R 755 "$OUTPUT_DIR"
chmod -R 755 "$USER_ICONS"

# Update icon cache
if command -v gtk-update-icon-cache &> /dev/null; then
    gtk-update-icon-cache -f -t "$USER_ICONS" 2>/dev/null
fi

echo "Done! Created X11 cursor theme: Koosh-X11"
echo "Listing files in $OUTPUT_DIR/cursors:"
ls -la "$OUTPUT_DIR/cursors"
