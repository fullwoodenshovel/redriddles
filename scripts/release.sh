#!/bin/bash
set -e

echo "Release script"
echo "=============================="

# Get current version
CURRENT=$(grep -m1 '^version =' Cargo.toml | cut -d'"' -f2)
echo "Current version: $CURRENT"

# Show options
echo ""
echo "Chose one:"
echo "1) Tag current version ($CURRENT)"
echo "2) Bump patch version ($CURRENT → x.x.$(($(echo $CURRENT | cut -d. -f3) + 1)))"
echo "3) Bump minor version ($CURRENT → x.$(($(echo $CURRENT | cut -d. -f2) + 1)).0)"
echo "4) Bump major version ($CURRENT → $(($(echo $CURRENT | cut -d. -f1) + 1)).0.0)"
echo "5) Enter custom version"
echo ""

read -p "Choose [1-5]: " choice

case $choice in
    1)
        NEW_VERSION="$CURRENT"
        ACTION="Tagging"
        ;;
    2)
        IFS='.' read -r major minor patch <<< "$CURRENT"
        NEW_VERSION="$major.$minor.$((patch + 1))"
        ACTION="Bumping patch"
        sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
        git add Cargo.toml
        git commit -m "chore: bump version to $NEW_VERSION"
        git push origin main
        ;;
    3)
        IFS='.' read -r major minor patch <<< "$CURRENT"
        NEW_VERSION="$major.$((minor + 1)).0"
        ACTION="Bumping minor"
        sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
        git add Cargo.toml
        git commit -m "chore: bump version to $NEW_VERSION"
        git push origin main
        ;;
    4)
        IFS='.' read -r major minor patch <<< "$CURRENT"
        NEW_VERSION="$((major + 1)).0.0"
        ACTION="Bumping major"
        sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
        git add Cargo.toml
        git commit -m "chore: bump version to $NEW_VERSION"
        git push origin main
        ;;
    5)
        read -p "Enter new version (e.g., 1.2.3): " NEW_VERSION
        ACTION="Setting custom"
        sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
        git add Cargo.toml
        git commit -m "chore: bump version to $NEW_VERSION"
        git push origin main
        ;;
    *)
        echo "Invalid choice"
        exit 1
        ;;
esac

echo ""
echo "$ACTION v$NEW_VERSION..."

# Create and push tag
git tag "v$NEW_VERSION"
git push origin "v$NEW_VERSION"

echo ""
echo "Successfully released v$NEW_VERSION!"
echo "GitHub Actions is now building the release..."
echo "Watch build: https://github.com/$(git remote get-url origin | sed 's/.*github.com[:/]\(.*\)\.git/\1/')/actions"