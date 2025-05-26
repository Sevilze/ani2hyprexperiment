#!/bin/bash
# Script to create a hyprcursor theme from Koosh-Fast-Animated

# Define source and destination themes
SOURCE_THEME="Koosh-Animated"
DEST_THEME="Koosh-Hyprcursor2"

# Create working directories
EXTRACT_DIR="koosh_extract"
OUTPUT_DIR="koosh_hyprcursor"

echo "Creating hyprcursor theme from $SOURCE_THEME..."

# Step 1: Extract the source theme
echo "Step 1: Extracting source theme..."
rm -rf "$EXTRACT_DIR"
mkdir -p "$EXTRACT_DIR"
hyprcursor-util --extract "$HOME/.icons/$SOURCE_THEME" --output "$EXTRACT_DIR"

# Step 2: Update the manifest.hl file
echo "Step 2: Updating manifest file..."
MANIFEST_FILE="$EXTRACT_DIR/extracted_$SOURCE_THEME/manifest.hl"
if [ -f "$MANIFEST_FILE" ]; then
    sed -i "s/name = .*/name = $DEST_THEME/" "$MANIFEST_FILE"
    sed -i "s/description = .*/description = Koosh cursor theme with hyprcursor support for Wayland/" "$MANIFEST_FILE"
    sed -i "s/version = .*/version = 1.0/" "$MANIFEST_FILE"
else
    echo "Error: Manifest file not found: $MANIFEST_FILE"
    exit 1
fi

# Step 3: Create the hyprcursor theme
echo "Step 3: Creating hyprcursor theme..."
rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"
hyprcursor-util --create "$EXTRACT_DIR/extracted_$SOURCE_THEME" --output "$OUTPUT_DIR"

# Step 4: Install the theme to the user's .icons directory
echo "Step 4: Installing theme to ~/.icons/$DEST_THEME..."
rm -rf "$HOME/.icons/$DEST_THEME"
mkdir -p "$HOME/.icons/$DEST_THEME"
cp -r "$OUTPUT_DIR/theme_$DEST_THEME"/* "$HOME/.icons/$DEST_THEME/"

# Step 5: Copy X11 cursors for compatibility
echo "Step 5: Copying X11 cursors for compatibility..."
mkdir -p "$HOME/.icons/$DEST_THEME/cursors"
cp -r "$HOME/.icons/$SOURCE_THEME/cursors"/* "$HOME/.icons/$DEST_THEME/cursors/"

# Step 6: Create index.theme and cursor.theme files
echo "Step 6: Creating theme configuration files..."
cat > "$HOME/.icons/$DEST_THEME/index.theme" << EOF
[Icon Theme]
Name=$DEST_THEME
Comment=Koosh cursor theme with hyprcursor support for Wayland
Inherits="hicolor"

# Directory list
Directories=cursors hyprcursors

[cursors]
Context=Cursors
Type=Fixed

[hyprcursors]
Context=Cursors
Type=Fixed
EOF

cat > "$HOME/.icons/$DEST_THEME/cursor.theme" << EOF
[Icon Theme]
Name=$DEST_THEME
Comment=Koosh cursor theme with hyprcursor support for Wayland
Inherits="$DEST_THEME"
EOF

# Step 7: Update icon cache
if command -v gtk-update-icon-cache &> /dev/null; then
    echo "Step 7: Updating icon cache..."
    gtk-update-icon-cache -f -t "$HOME/.icons/$DEST_THEME" 2>/dev/null || echo "Icon cache update failed (this is normal if gtk-update-icon-cache is not installed)"
fi

# Step 8: Clean up
echo "Step 8: Cleaning up..."
rm -rf "$EXTRACT_DIR" "$OUTPUT_DIR"
