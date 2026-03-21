#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(CDPATH="" cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# shellcheck source=./common.sh
# shellcheck disable=SC1091
source "$SCRIPT_DIR/common.sh"

usage() {
    cat <<'EOF'
Usage: ./scripts/release/finalize-release.sh [--push] <version>

Finalize a validated release branch by:
  - verifying the repository is on main and synchronized with origin/main
  - confirming the version has not already been published
  - re-running release validation
  - creating the stable annotated tag

Use --push to push the stable tag after it is created.
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

release_log "Finalizing release $TAG"
require_branch "main" "$REPO_ROOT"
require_clean_worktree "$REPO_ROOT"
ensure_main_synced "$REPO_ROOT"

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

release_log "Verifying release-managed versions match $VERSION"
if ! verify_release_versions "$VERSION" "$REPO_ROOT"; then
    release_error "Release-managed files do not declare version $VERSION"
    exit 1
fi

release_log "Running release validation"
run_release_validation "$REPO_ROOT"

release_log "Creating annotated tag $TAG"
git -C "$REPO_ROOT" tag -a "$TAG" -m "Release $TAG"

if [[ $PUSH -eq 1 ]]; then
    release_log "Pushing tag $TAG"
    git -C "$REPO_ROOT" push origin "$TAG"
fi

cat <<EOF
Release finalization complete.
  Version: $VERSION
  Tag: $TAG
  Commit: $(git -C "$REPO_ROOT" rev-parse HEAD)
  Pushed: $([[ $PUSH -eq 1 ]] && echo yes || echo no)
EOF
