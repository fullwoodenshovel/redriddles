#!/bin/bash
set -e

echo "Bumping patch version..."

# Get and bump version
CURRENT=$(grep -m1 '^version =' Cargo.toml | cut -d'"' -f2)
IFS='.' read -r major minor patch <<< "$CURRENT"
NEW_VERSION="$major.$minor.$((patch + 1))"

echo "$CURRENT → $NEW_VERSION"

# Update Cargo.toml
sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml

# Commit and tag
git add Cargo.toml
git commit -m "chore: release v$NEW_VERSION"
git tag "v$NEW_VERSION"
git push origin main
git push origin "v$NEW_VERSION"

echo ""
echo "Released v$NEW_VERSION!"
echo "GitHub Actions is building..."