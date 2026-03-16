#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(CDPATH="" cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# shellcheck source=./release/common.sh
source "$REPO_ROOT/scripts/release/common.sh"

usage() {
    cat <<'EOF'
Usage: ./scripts/new-release.sh [--push] <version>

Prepare a stable release from main by:
  - verifying branch, cleanliness, and origin/main sync
  - updating release-managed versions
  - running release validation
  - creating a release commit and annotated tag

Use --push to push main and the new tag after preparation succeeds.
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

release_log "Preparing release $TAG"
require_branch "main" "$REPO_ROOT"
require_clean_worktree "$REPO_ROOT"
ensure_main_synced "$REPO_ROOT"

if git -C "$REPO_ROOT" rev-parse "$TAG" >/dev/null 2>&1; then
    release_error "Tag $TAG already exists"
    exit 1
fi

release_log "Updating release-managed versions"
update_release_versions "$VERSION" "$REPO_ROOT"
verify_release_versions "$VERSION" "$REPO_ROOT"

release_log "Running release validation"
run_release_validation "$REPO_ROOT"

release_log "Creating release commit"
MANIFESTS=()
while IFS= read -r manifest; do
    MANIFESTS+=("$manifest")
done < <(release_manifests "$REPO_ROOT")
git -C "$REPO_ROOT" add "${MANIFESTS[@]}"
git -C "$REPO_ROOT" commit -m "release: prepare $TAG"

release_log "Creating annotated tag $TAG"
git -C "$REPO_ROOT" tag -a "$TAG" -m "Release $TAG"

if [[ $PUSH -eq 1 ]]; then
    release_log "Pushing main and tags"
    git -C "$REPO_ROOT" push origin main --follow-tags
fi

cat <<EOF
Release preparation complete.
  Version: $VERSION
  Commit: $(git -C "$REPO_ROOT" rev-parse HEAD)
  Tag: $TAG
  Pushed: $([[ $PUSH -eq 1 ]] && echo yes || echo no)
EOF
