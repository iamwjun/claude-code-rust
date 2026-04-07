#!/bin/bash

# Update CHANGELOG.md incrementally based on commits since last-commit marker.
#
# Usage:
#   ./scripts/update-changelog.sh              # auto-detect version from Cargo.toml
#   ./scripts/update-changelog.sh v1.2.0       # specify version explicitly

set -e

CHANGELOG="CHANGELOG.md"

# --- Version ---
if [ -n "$1" ]; then
    VERSION="$1"
else
    RAW=$(grep '^version' Cargo.toml | head -1 | sed 's/.*= *//' | tr -d '"')
    VERSION="v${RAW}"
fi
DATE=$(date +%Y-%m-%d)

# --- Find last synced commit ---
LAST_COMMIT=$(grep 'last-commit:' "$CHANGELOG" | sed 's/.*last-commit: *//' | tr -d ' -->' | head -1)

if [ -z "$LAST_COMMIT" ]; then
    echo "❌ No '<!-- last-commit: SHA -->' marker found in $CHANGELOG"
    exit 1
fi

echo "📌 Last synced commit: $LAST_COMMIT"

# --- Get new commits since last sync ---
NEW_COMMITS=$(git log --pretty=format:"%h %s" "${LAST_COMMIT}..HEAD")

if [ -z "$NEW_COMMITS" ]; then
    echo "✅ No new commits since $LAST_COMMIT — CHANGELOG is up to date"
    exit 0
fi

echo "📝 New commits:"
echo "$NEW_COMMITS" | while IFS= read -r line; do echo "  $line"; done
echo ""

# --- Build new section ---
LATEST_SHA=$(git rev-parse --short HEAD)

COMMIT_ROWS=""
while IFS= read -r line; do
    SHA=$(echo "$line" | cut -d' ' -f1)
    MSG=$(echo "$line" | cut -d' ' -f2-)
    COMMIT_ROWS="${COMMIT_ROWS}| \`${SHA}\` | ${MSG} |\n"
done <<< "$NEW_COMMITS"

NEW_SECTION="## ${VERSION} - ${DATE}\n\n| Commit | Description |\n|--------|-------------|\n${COMMIT_ROWS}"

# --- Prepend new section after the last-commit marker line ---
MARKER_LINE=$(grep -n 'last-commit:' "$CHANGELOG" | head -1 | cut -d: -f1)
BEFORE=$(head -n "$MARKER_LINE" "$CHANGELOG")
AFTER=$(tail -n +"$((MARKER_LINE + 1))" "$CHANGELOG")

printf '%s\n\n%b\n%s\n' "$BEFORE" "$NEW_SECTION" "$AFTER" > "$CHANGELOG"

# --- Update last-commit marker ---
sed -i '' "s/last-commit: ${LAST_COMMIT}/last-commit: ${LATEST_SHA}/" "$CHANGELOG"

echo "✅ CHANGELOG.md updated"
echo "   Version: $VERSION"
echo "   Marker:  $LAST_COMMIT → $LATEST_SHA"
