#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(CDPATH="" cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# shellcheck source=./common.sh
# shellcheck disable=SC1091
source "$SCRIPT_DIR/common.sh"

usage() {
    cat <<'EOF'
Usage: ./scripts/release/prerelease.sh <version>

Update the release-managed version set across Rust, npm, Python, and lockfile
artifacts, then verify the repository is internally consistent.
EOF
}

VERSION="${1:-}"

if [[ -z "$VERSION" || "$VERSION" == "--help" || "$VERSION" == "-h" ]]; then
    usage
    [[ -n "$VERSION" ]] && exit 0
    exit 1
fi

if ! validate_stable_semver "$VERSION"; then
    release_error "Version must be stable semantic version format MAJOR.MINOR.PATCH"
    exit 1
fi

require_clean_worktree "$REPO_ROOT"

release_log "Updating release-managed versions for $VERSION"
update_release_versions "$VERSION" "$REPO_ROOT"

release_log "Verifying release-managed versions for $VERSION"
verify_release_versions "$VERSION" "$REPO_ROOT"

release_log "Prerelease version bump complete"
