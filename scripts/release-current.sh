#!/bin/bash
set -e

# Get current version
VERSION=$(grep -m1 '^version =' Cargo.toml | cut -d'"' -f2)
echo "Tagging current version v$VERSION..."

# Create and push tag
git tag "v$VERSION"
git push origin "v$VERSION"

echo ""
echo "Tagged v$VERSION!"
echo "GitHub Actions is building..."