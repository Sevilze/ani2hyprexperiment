#!/bin/bash
# Create a new theme with proper animation support

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Define input and output theme names
INPUT_THEME="${1:-Koosh-X11}"
NEW_THEME="${2:-Koosh-Animated}"

echo "=== Koosh Cursor Theme Creator ==="
echo "This script will create a new cursor theme with:"
echo "- Multi-size support (24, 32, 48, 64, 72, 96)"
echo "- Proper hotspots for all cursors"
echo "- 'not-allowed' and 'unavailable' cursors using the same hotspot as 'left_ptr'"
echo "- All temporary .conf files will be removed after completion"
echo "==============================="
echo "Input theme: $INPUT_THEME"
echo "Output theme: $NEW_THEME"
echo "==============================="

# Check if input theme exists
if [ ! -d "$INPUT_THEME" ]; then
    echo "Error: Input theme directory '$INPUT_THEME' not found!"
    echo "Usage: $0 [input_theme] [output_theme]"
    echo "Example: $0 Koosh-X11 Koosh-Animated"
    exit 1
fi

# Check if input theme has a cursors directory
if [ ! -d "$INPUT_THEME/cursors" ]; then
    echo "Error: Input theme cursors directory '$INPUT_THEME/cursors' not found!"
    exit 1
fi

# Create theme directories
rm -rf "$NEW_THEME"
mkdir -p "$NEW_THEME/cursors"

# Also create user's .icons directory
USER_ICONS="$HOME/.icons/$NEW_THEME"
rm -rf "$USER_ICONS"
mkdir -p "$USER_ICONS/cursors"

# Create temporary directory
TEMP_DIR="koosh_animated_temp"
rm -rf "$TEMP_DIR"
mkdir -p "$TEMP_DIR"

# Define sizes similar to Bibata
SIZES=(24 32 48 64 72 96)

# Process each cursor file
echo "Processing cursor files..."
for cursor_file in "$INPUT_THEME/cursors"/*; do
    if [ -f "$cursor_file" ] && [ ! -L "$cursor_file" ]; then
        cursor_name=$(basename "$cursor_file")
        echo "  Processing: $cursor_name"

        # Create directory for this cursor
        mkdir -p "$TEMP_DIR/$cursor_name"

        # Extract original cursor
        xcur2png "$cursor_file" -d "$TEMP_DIR/$cursor_name" > /dev/null 2>&1

        # Count how many frames were extracted
        frame_count=$(ls "$TEMP_DIR/$cursor_name/${cursor_name}_"*.png 2>/dev/null | wc -l)

        if [ "$frame_count" -eq 0 ]; then
            echo "    Failed to extract cursor, copying original"
            cp "$cursor_file" "$NEW_THEME/cursors/$cursor_name"
            continue
        fi

        echo "    Found $frame_count animation frames"

        # Get original size from first frame
        orig_png="$TEMP_DIR/$cursor_name/${cursor_name}_000.png"
        orig_size=$(identify -format "%w" "$orig_png" 2>/dev/null || echo "48")
        echo "    Original size: ${orig_size}x${orig_size}"

        # For other cursors, we'll use a reasonable default
        if [[ "$cursor_name" == "left_ptr" || "$cursor_name" == "not-allowed" || "$cursor_name" == "unavailable" ]]; then
            # For left_ptr, not-allowed, and unavailable, hotspot is at the tip of the arrow (top-left)
            hotspot_x_ratio=0.125  # 1/8 of the width from the left
            hotspot_y_ratio=0.125  # 1/8 of the height from the top
        elif [[ "$cursor_name" == "text" || "$cursor_name" == "xterm" || "$cursor_name" == "ibeam" ]]; then
            hotspot_x_ratio=0.5
            hotspot_y_ratio=0.5
        elif [[ "$cursor_name" == "pointer" || "$cursor_name" == "hand"* ]]; then
            # For pointer/hand, hotspot is at the fingertip
            hotspot_x_ratio=0.3    # 30% from the left
            hotspot_y_ratio=0.125  # 1/8 of the height from the top
        elif [[ "$cursor_name" == "pencil" ]]; then
            # For pencil, hotspot is at the tip
            hotspot_x_ratio=0.125  # 1/8 of the width from the left
            hotspot_y_ratio=0.125  # 1/8 of the height from the top
        elif [[ "$cursor_name" == "move" ]]; then
            # For move, hotspot is in the center
            hotspot_x_ratio=0.5    # middle
            hotspot_y_ratio=0.5    # middle
        elif [[ "$cursor_name" == "size_"* ]]; then
            # For sizing cursors, hotspot is in the center
            hotspot_x_ratio=0.5    # middle
            hotspot_y_ratio=0.5    # middle
        else
            # Default: center hotspot
            hotspot_x_ratio=0.5    # middle
            hotspot_y_ratio=0.5    # middle
        fi

        # Create a working directory for this cursor
        working_dir="$TEMP_DIR/$cursor_name/working"
        mkdir -p "$working_dir"

        # Create a config file for xcursorgen
        config_file="$working_dir/cursor.config"
        > "$config_file"

        # Process each size
        for size in "${SIZES[@]}"; do
            # Calculate hotspot coordinates for this size
            hotspot_x=$(printf "%.0f" $(echo "$size * $hotspot_x_ratio" | bc -l))
            hotspot_y=$(printf "%.0f" $(echo "$size * $hotspot_y_ratio" | bc -l))

            # Ensure hotspot is at least 1 pixel
            [ "$hotspot_x" -lt 1 ] && hotspot_x=1
            [ "$hotspot_y" -lt 1 ] && hotspot_y=1

            # Process each frame
            for frame in $(seq 0 $(($frame_count - 1))); do
                # Format frame number with leading zeros
                frame_num=$(printf "%03d" $frame)
                src_png="$TEMP_DIR/$cursor_name/${cursor_name}_${frame_num}.png"

                if [ ! -f "$src_png" ]; then
                    echo "    Warning: Missing frame $frame_num"
                    continue
                fi

                # Create scaled version of this frame
                dst_png="$working_dir/${size}_${frame_num}.png"

                if [ "$size" -eq "$orig_size" ]; then
                    # Use original for original size
                    cp "$src_png" "$dst_png"
                else
                    echo "    Creating ${size}x${size} version of frame $frame_num"
                    # Use magick command if available, otherwise use convert
                    if command -v magick &> /dev/null; then
                        magick "$src_png" -resize ${size}x${size} "$dst_png"
                    else
                        convert "$src_png" -resize ${size}x${size} "$dst_png" 2>/dev/null
                    fi
                fi

                # Add to config file with delay (100ms per frame)
                # Format: <size> <hotx> <hoty> <filename> <delay>
                echo "$size $hotspot_x $hotspot_y ${size}_${frame_num}.png 100" >> "$config_file"
            done
        done

        # Debug: show config file contents
        echo "    Config file contents (first few lines):"
        head -n 5 "$config_file"
        echo "    ... (total $(wc -l < "$config_file") lines)"

        # Change to the working directory
        pushd "$working_dir" > /dev/null

        # Generate cursor file
        echo "    Generating cursor file"
        xcursorgen "cursor.config" "cursor"

        # Check if cursor was created successfully
        if [ -f "cursor" ]; then
            # Copy the cursor to the theme directory
            cp "cursor" "../../../$NEW_THEME/cursors/$cursor_name"
            echo "    Successfully created multi-size animated cursor"

            # Verify the cursor
            echo "    Verifying cursor..."
            mkdir -p "verify"
            xcur2png "cursor" -d "verify" > /dev/null 2>&1
            frame_count_new=$(ls "verify/${cursor_name}_"*.png 2>/dev/null | wc -l)
            echo "    New cursor has $frame_count_new frames/sizes"

            # Show sizes
            sizes=$(identify -format "%w x %h\n" "verify"/*.png | sort | uniq)
            echo "    Sizes: $sizes"

            rm -rf "verify"
        else
            echo "    Failed to create cursor, copying original"
            cp "$cursor_file" "../../../$NEW_THEME/cursors/$cursor_name"
        fi

        # Return to the original directory
        popd > /dev/null
    elif [ -L "$cursor_file" ]; then
        # Copy symlinks
        target=$(readlink "$cursor_file")
        link_name=$(basename "$cursor_file")
        echo "  Copying symlink: $link_name -> $target"
        ln -sf "$target" "$NEW_THEME/cursors/$link_name"
    fi
done

# Create additional symlinks for compatibility
echo "Creating additional symlinks..."
cd "$NEW_THEME/cursors"

# Common cursor symlinks
create_link() {
    if [ ! -e "$2" ] && [ -e "$1" ]; then
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

echo "Creating hex-encoded symlinks..."
create_link "link" "a2a266d0498c3104214a47bd64ab0fc8"
create_link "link" "640fb0e74195791501fd1ed57b41487f"
create_link "link" "3085a0e285430894940527032f8b26df"
create_link "left_ptr_watch" "3ecb610c1bf2410f44200f48c40d3599"
create_link "left_ptr_watch" "08e8e1c95fe2fc01f976f1e063a24ccd"
create_link "left_ptr_watch" "00000000000000020006000e7e9ffc3f"
create_link "question_arrow" "5c6cd98b3f3ebcb1f9c7f1c204630408"
create_link "question_arrow" "d9ce0ab605698f320427677b458ad60b"
create_link "hand2" "9d800788f1b08800ae810202380a0822"
create_link "hand2" "e29285e634086352946a0e7090d73106"
create_link "sb_h_double_arrow" "028006030e0e7ebffc7f7070c0600140"
create_link "sb_h_double_arrow" "14fef782d02440884392942c1120523"
create_link "sb_v_double_arrow" "00008160000006810000408080010102"
create_link "sb_v_double_arrow" "2870a09082c103050810ffdffffe0204"
create_link "fd_double_arrow" "fcf1c3c7cd4491d801f1e1c78f100000"
create_link "bd_double_arrow" "c7088f0f3e6c8088236ef8e1e3e70000"
create_link "move" "9081237383d90e509aa00f00170e968f"
create_link "move" "4498f0e0c1937ffe01fd06f973665830"
create_link "crossed_circle" "03b6e0fcb3499374a867c041f52298f0"

cd ../..

echo "Creating theme files..."
cat > "$NEW_THEME/index.theme" << EOF
[Icon Theme]
Name=$NEW_THEME
Comment=Koosh cursor theme with proper animation support
Inherits="hicolor"

# Directory list
Directories=cursors

[cursors]
Context=Cursors
Type=Fixed

# Add size information
[cursors/24]
Size=24
Context=Cursors
Type=Fixed

[cursors/32]
Size=32
Context=Cursors
Type=Fixed

[cursors/48]
Size=48
Context=Cursors
Type=Fixed

[cursors/64]
Size=64
Context=Cursors
Type=Fixed

[cursors/72]
Size=72
Context=Cursors
Type=Fixed

[cursors/96]
Size=96
Context=Cursors
Type=Fixed
EOF

# Create cursor.theme file
cat > "$NEW_THEME/cursor.theme" << EOF
[Icon Theme]
Name=$NEW_THEME
Comment=Koosh cursor theme with proper animation support
Inherits="$NEW_THEME"
EOF

# Copy to user's .icons directory
echo "Installing to ~/.icons..."
cp -r "$NEW_THEME/cursors" "$USER_ICONS/"
cp "$NEW_THEME/index.theme" "$USER_ICONS/"
cp "$NEW_THEME/cursor.theme" "$USER_ICONS/"

# Set proper permissions
chmod -R 755 "$NEW_THEME"
chmod -R 755 "$USER_ICONS"

# Update icon cache
if command -v gtk-update-icon-cache &> /dev/null; then
    gtk-update-icon-cache -f -t "$USER_ICONS" 2>/dev/null
fi

# Clean up
rm -rf "$TEMP_DIR"

# Remove .conf files
echo "Removing .conf files..."
rm -f *.conf
