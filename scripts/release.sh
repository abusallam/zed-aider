#!/bin/bash
set -e

VERSION=$1

if [ -z "$VERSION" ]; then
    echo "Usage: ./scripts/release.sh <version>"
    exit 1
fi

# Update version in Cargo.toml
sed -i "s/^version = .*/version = \"$VERSION\"/" Cargo.toml

# Commit changes
git add Cargo.toml
git commit -m "Release v$VERSION"
git tag "v$VERSION"

# Push to GitHub
git push origin main
git push origin "v$VERSION"

echo "Released version $VERSION"
