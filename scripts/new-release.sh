#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(CDPATH="" cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# shellcheck source=./release/common.sh
# shellcheck disable=SC1091
source "$REPO_ROOT/scripts/release/common.sh"

usage() {
    cat <<'EOF'
Usage: ./scripts/new-release.sh [--push] <version>

Prepare a stable release branch from main by:
  - verifying branch, cleanliness, and origin/main sync
  - creating a versioned release branch
  - running the prerelease version bump and verification
  - creating a release commit on the branch

Use --push to push the release branch after preparation succeeds.
EOF
}

PUSH=0
VERSION=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --push)
            PUSH=1
            shift
            ;;
        --help|-h)
            usage
            exit 0
            ;;
        *)
            VERSION="$1"
            shift
            ;;
    esac
done

if [[ -z "$VERSION" ]]; then
    usage
    exit 1
fi

if ! validate_stable_semver "$VERSION"; then
    release_error "Version must be stable semantic version format MAJOR.MINOR.PATCH"
    exit 1
fi

TAG="$(release_tag_for_version "$VERSION")"
BRANCH="$(release_branch_for_version "$VERSION")"

release_log "Preparing release branch $BRANCH for $TAG"
require_branch "main" "$REPO_ROOT"
require_clean_worktree "$REPO_ROOT"
ensure_main_synced "$REPO_ROOT"

if release_branch_exists "$BRANCH" "$REPO_ROOT"; then
    release_error "Branch $BRANCH already exists"
    exit 1
fi

if release_tag_exists "$TAG" "$REPO_ROOT"; then
    release_error "Tag $TAG already exists"
    exit 1
fi

if release_version_published "$VERSION"; then
    release_error "Version $VERSION is already published"
    exit 1
else
    status=$?
    if [[ $status -eq 2 ]]; then
        release_error "Unable to verify whether $VERSION is already published"
        exit 1
    fi
fi

release_log "Creating release branch $BRANCH"
git -C "$REPO_ROOT" checkout -b "$BRANCH"

release_log "Running prerelease version bump"
"$REPO_ROOT/scripts/release/prerelease.sh" "$VERSION"

release_log "Creating release commit"
MANIFESTS=()
while IFS= read -r manifest; do
    MANIFESTS+=("$manifest")
done < <(release_manifests "$REPO_ROOT")
git -C "$REPO_ROOT" add "${MANIFESTS[@]}"
git -C "$REPO_ROOT" commit -m "release: prepare $TAG"

if [[ $PUSH -eq 1 ]]; then
    release_log "Pushing release branch $BRANCH"
    git -C "$REPO_ROOT" push -u origin "$BRANCH"
fi

cat <<EOF
Release preparation complete.
  Version: $VERSION
  Branch: $BRANCH
  Commit: $(git -C "$REPO_ROOT" rev-parse HEAD)
  Pushed: $([[ $PUSH -eq 1 ]] && echo yes || echo no)
EOF
