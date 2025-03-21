#!/bin/bash
set -e

# Build extension
cargo build --release

# Create extension directories
mkdir -p ~/.config/zed/extensions/dev/coolify

# Copy extension files
cp target/release/libzed_coolify.so ~/.config/zed/extensions/dev/coolify/
cp extension.toml ~/.config/zed/extensions/dev/coolify/

# Create vsix package
cd ~/.config/zed/extensions/dev
tar -czf coolify.vsix coolify/

# Copy to extensions directory
cp coolify.vsix ~/.config/zed/extensions/

echo "Extension package created at ~/.config/zed/extensions/coolify.vsix"
