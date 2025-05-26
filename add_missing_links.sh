#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
NEW_THEME="Koosh-Complete"
THEME_PATH="$ROOT_DIR/$NEW_THEME"
rm -rf "$THEME_PATH"
mkdir -p "$THEME_PATH/cursors"

if [ -d "cursors" ]; then
    cp -r cursors/* "$THEME_PATH/cursors/"
elif [ -d "Koosh/cursors" ]; then
    cp -r Koosh/cursors/* "$THEME_PATH/cursors/"
else
    if [ -d "$HOME/.icons/Koosh/cursors" ]; then
        cp -r "$HOME/.icons/Koosh/cursors/"* "$THEME_PATH/cursors/"
    else
        echo "Error: Could not find Koosh cursor theme."
        echo "Please run this script from the Koosh directory or a directory containing Koosh."
        exit 1
    fi
fi

cd "$THEME_PATH/cursors"

# Add missing symlinks
ln -sf left_ptr arrow
ln -sf left_ptr default
ln -sf left_ptr top_left_arrow
ln -sf pointer hand1
ln -sf pointer hand2
ln -sf pointer pointing_hand
ln -sf pointer openhand
ln -sf pointer grab
ln -sf move fleur
ln -sf move all-scroll
ln -sf move size_all
ln -sf wait watch
ln -sf progress left_ptr_watch
ln -sf crosshair cross
ln -sf text xterm
ln -sf text ibeam
ln -sf pencil draft
ln -sf question_arrow help
ln -sf question_arrow whats_this
ln -sf question_arrow left_ptr_help
ln -sf not-allowed crossed_circle
ln -sf not-allowed forbidden
ln -sf not-allowed no_drop
ln -sf not-allowed dnd_no_drop
ln -sf size_hor sb_h_double_arrow
ln -sf size_hor h_double_arrow
ln -sf size_hor ew-resize
ln -sf size_hor col-resize
ln -sf size_hor split_h
ln -sf size_ver sb_v_double_arrow
ln -sf size_ver v_double_arrow
ln -sf size_ver ns-resize
ln -sf size_ver row-resize
ln -sf size_ver split_v
ln -sf size_bdiag fd_double_arrow
ln -sf size_bdiag nesw-resize
ln -sf size_fdiag bd_double_arrow
ln -sf size_fdiag nwse-resize

# Additional common cursor IDs
ln -sf left_ptr_watch 00000000000000020006000e7e9ffc3f
ln -sf left_ptr_watch 08e8e1c95fe2fc01f976f1e063a24ccd
ln -sf left_ptr_watch 3ecb610c1bf2410f44200f48c40d3599
ln -sf sb_v_double_arrow 00008160000006810000408080010102
ln -sf sb_v_double_arrow 2870a09082c103050810ffdffffe0204
ln -sf sb_h_double_arrow 028006030e0e7ebffc7f7070c0600140
ln -sf sb_h_double_arrow 14fef782d02440884392942c1120523
ln -sf crossed_circle 03b6e0fcb3499374a867c041f52298f0
ln -sf question_arrow 5c6cd98b3f3ebcb1f9c7f1c204630408
ln -sf question_arrow d9ce0ab605698f320427677b458ad60b
ln -sf move 4498f0e0c1937ffe01fd06f973665830
ln -sf move 9081237383d90e509aa00f00170e968f
ln -sf link 3085a0e285430894940527032f8b26df
ln -sf link 640fb0e74195791501fd1ed57b41487f
ln -sf link a2a266d0498c3104214a47bd64ab0fc8
ln -sf hand2 9d800788f1b08800ae810202380a0822
ln -sf hand2 e29285e634086352946a0e7090d73106
ln -sf size_fdiag c7088f0f3e6c8088236ef8e1e3e70000
ln -sf size_bdiag fcf1c3c7cd4491d801f1e1c78f100000
ln -sf copy 1081e37283d90000800003c07f3ef6bf
ln -sf copy 6407b0e94181790501fd1e167b474872
ln -sf copy b66166c04f8c3109214a4fbd64a50fc8
ln -sf dnd-link alias
ln -sf plus cell
ln -sf grabbing closedhand
ln -sf tcross color-picker
ln -sf cross cross_reverse
ln -sf cross diamond_cross
ln -sf grabbing dnd-move
ln -sf grabbing dnd-none
ln -sf dotbox dot_box_mask
ln -sf sb_v_double_arrow double_arrow
ln -sf sb_down_arrow down-arrow
ln -sf right_ptr draft_large
ln -sf right_ptr draft_small
ln -sf dotbox draped_box
ln -sf right_side e-resize
ln -sf grabbing fcf21c00b30f7e3f83fe0dfd12e71cff
ln -sf dotbox icon
ln -sf sb_left_arrow left-arrow
ln -sf top_right_corner ne-resize
ln -sf dnd_no_drop no-drop
# Removed duplicate symlink
# ln -sf crossed_circle not-allowed
ln -sf top_side n-resize
ln -sf top_left_corner nw-resize
ln -sf X_cursor pirate
ln -sf sb_right_arrow right-arrow
ln -sf bottom_right_corner se-resize
# Removed circular references
# ln -sf fd_double_arrow size_fdiag
# ln -sf bd_double_arrow size_bdiag
ln -sf sb_h_double_arrow size-hor
ln -sf sb_v_double_arrow size-ver
ln -sf bottom_side s-resize
ln -sf bottom_left_corner sw-resize
ln -sf dotbox target
# Removed duplicate symlink
# ln -sf xterm text
ln -sf sb_up_arrow up-arrow
ln -sf left_side w-resize
ln -sf X_cursor x-cursor

cd ../..

# Create index.theme file
cat > "$THEME_PATH/index.theme" << EOF
[Icon Theme]
Name=$NEW_THEME
Comment=Koosh cursor theme with all necessary symlinks
Inherits=hicolor

# Directory list
Directories=cursors

[cursors]
Context=Cursors
Type=Fixed
EOF

# Create cursor.theme file
cat > "$THEME_PATH/cursor.theme" << EOF
[Icon Theme]
Name=$NEW_THEME
Comment=Koosh cursor theme with all necessary symlinks
Inherits=hicolor
EOF

# Also install to user's .icons directory for better GTK compatibility
USER_ICONS="$HOME/.icons/$NEW_THEME"
rm -rf "$USER_ICONS"
mkdir -p "$USER_ICONS/cursors"

# Copy files to user's .icons directory
cp -r "$THEME_PATH/cursors" "$USER_ICONS/"

# Copy theme files if they exist, otherwise create them
if [ -f "$THEME_PATH/index.theme" ]; then
    cp "$THEME_PATH/index.theme" "$USER_ICONS/"
else
    # Create index.theme file directly in the user's .icons directory
    cat > "$USER_ICONS/index.theme" << EOF
[Icon Theme]
Name=$NEW_THEME
Comment=Koosh cursor theme with all necessary symlinks
Inherits=hicolor

# Directory list
Directories=cursors

[cursors]
Context=Cursors
Type=Fixed
EOF
fi

if [ -f "$THEME_PATH/cursor.theme" ]; then
    cp "$THEME_PATH/cursor.theme" "$USER_ICONS/"
else
    # Create cursor.theme file directly in the user's .icons directory
    cat > "$USER_ICONS/cursor.theme" << EOF
[Icon Theme]
Name=$NEW_THEME
Comment=Koosh cursor theme with all necessary symlinks
Inherits=hicolor
EOF
fi

# Set proper permissions
chmod -R 755 "$THEME_PATH"
chmod -R 755 "$USER_ICONS"

# Update icon cache
if command -v gtk-update-icon-cache &> /dev/null; then
    gtk-update-icon-cache -f -t "$USER_ICONS" 2>/dev/null
fi

echo "Done! Created new cursor theme: $THEME_PATH"
echo "Also installed to: $USER_ICONS"
echo ""
echo "To use with Hyprland, add to your config:"
echo "env = XCURSOR_THEME,$NEW_THEME"
echo "env = XCURSOR_SIZE,24"
echo ""
echo "cursor {"
echo "    size = 24"
echo "}"
echo ""
echo "Note: Since your cursor files don't support multiple sizes yet,"
echo "it's best to use size 24 which is their native size."
