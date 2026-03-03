#!/bin/sh
set -eu

# Variables
APPIMAGE_TOOL_URL="https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-x86_64.AppImage"
APPIMAGE_TOOL="appimagetool-x86_64.AppImage"
APP_NAME="compressr"
VERSION="${1:-1.0.0}"  # Default to "1.0.0" if no argument is provided
BASEDIR="$(dirname "$0")"
PARENT_DIR="$(dirname "$BASEDIR")"
TARGET_DIR="${PARENT_DIR}/target/release/AppImage"
APP_DIR="${TARGET_DIR}/${APP_NAME}.AppDir"
OUTPUT_APPIMAGE="${TARGET_DIR}/${APP_NAME}-x86_64-${VERSION}.AppImage"

# Clear the target directory
rm -rf "$TARGET_DIR"

# Download appimagetool
if command -v wget >/dev/null 2>&1; then
    wget -q "$APPIMAGE_TOOL_URL" -O "$APPIMAGE_TOOL"
elif command -v curl >/dev/null 2>&1; then
    curl -LsS "$APPIMAGE_TOOL_URL" -o "$APPIMAGE_TOOL"
else
    echo "Error: neither 'wget' nor 'curl' is installed; cannot download appimagetool." >&2
    exit 1
fi
chmod a+x "$APPIMAGE_TOOL"

# Prepare target directory
mkdir -p "$TARGET_DIR"
mkdir -p "$APP_DIR/usr/bin"

# Copy resources
cp -r "${BASEDIR}/.AppDir/." "$APP_DIR/"
cp "${PARENT_DIR}/target/release/compressr-app" "$APP_DIR/usr/bin/"

# Build AppImage
./"$APPIMAGE_TOOL" -n "$APP_DIR" "$OUTPUT_APPIMAGE"

# Cleanup
rm "$APPIMAGE_TOOL"
rm -rf "$APP_DIR"
