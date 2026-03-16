#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(CDPATH="" cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# shellcheck source=./common.sh
source "$SCRIPT_DIR/common.sh"

TAG="${1:-${GITHUB_REF_NAME:-}}"
COMMIT_SHA="${2:-${GITHUB_SHA:-}}"

if [[ -z "$TAG" || -z "$COMMIT_SHA" ]]; then
    release_error "Usage: verify-tag.sh <tag> <commit-sha>"
    exit 1
fi

if [[ ! "$TAG" =~ ^v([0-9]+\.[0-9]+\.[0-9]+)$ ]]; then
    release_error "Tag must be stable semantic version format vMAJOR.MINOR.PATCH"
    exit 1
fi

VERSION="${BASH_REMATCH[1]}"

if ! git -C "$REPO_ROOT" fetch origin main --tags; then
    release_error "Failed to fetch origin/main and tags"
    exit 1
fi

if ! COMMIT_SHA="$(git -C "$REPO_ROOT" rev-parse "${COMMIT_SHA}^{commit}" 2>/dev/null)"; then
    release_error "Could not resolve $COMMIT_SHA to a commit"
    exit 1
fi

if ! git -C "$REPO_ROOT" merge-base --is-ancestor "$COMMIT_SHA" "refs/remotes/origin/main"; then
    release_error "Tag commit $COMMIT_SHA is not contained in origin/main"
    exit 1
fi

verify_release_versions "$VERSION" "$REPO_ROOT"

if [[ -n "${GITHUB_OUTPUT:-}" ]]; then
    {
        echo "version=$VERSION"
        echo "tag=$TAG"
    } >> "$GITHUB_OUTPUT"
fi

release_log "Verified $TAG at $COMMIT_SHA on origin/main"
