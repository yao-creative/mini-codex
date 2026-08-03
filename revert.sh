#!/usr/bin/env bash
set -e

BASE="77b398d0e3582d11074be803c5b979aabda2e67d"

echo "Creating backup branch..."
git branch backup-before-secret-removal

echo "Squashing commits since main..."
GIT_SEQUENCE_EDITOR="sed -i '' -e '2,$ s/^pick /squash /'" \
git rebase -i "$BASE"

echo "Removing .env from history snapshot..."
git rm --cached -f .env 2>/dev/null || true

echo "Amending commit..."
git commit --amend --no-edit

echo "Checking if .env is still tracked..."
if git ls-files | grep -q "\.env"; then
    echo "ERROR: .env is still tracked"
    exit 1
fi

echo "Force pushing rewritten history..."
git push origin develop --force-with-lease

echo "Done."