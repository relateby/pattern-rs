#!/usr/bin/env bash

# Shared helpers for local release preparation and CI/CD release jobs.

release_repo_root() {
    local script_dir
    script_dir="$(CDPATH="" cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    cd "$script_dir/../.." >/dev/null 2>&1 && pwd
}

release_log() {
    printf '[release] %s\n' "$*"
}

release_error() {
    printf '[release] ERROR: %s\n' "$*" >&2
}

require_command() {
    if ! command -v "$1" >/dev/null 2>&1; then
        release_error "Missing required command: $1"
        return 1
    fi
}

validate_stable_semver() {
    [[ "$1" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]
}

release_tag_for_version() {
    printf 'v%s\n' "$1"
}

require_clean_worktree() {
    local repo_root
    repo_root="${1:-$(release_repo_root)}"
    if [[ -n "$(git -C "$repo_root" status --porcelain)" ]]; then
        release_error "Working tree is not clean"
        return 1
    fi
}

require_branch() {
    local expected repo_root current
    expected="$1"
    repo_root="${2:-$(release_repo_root)}"
    current="$(git -C "$repo_root" rev-parse --abbrev-ref HEAD)"
    if [[ "$current" != "$expected" ]]; then
        release_error "Expected branch '$expected' but found '$current'"
        return 1
    fi
}

ensure_main_synced() {
    local repo_root local_sha remote_sha
    repo_root="${1:-$(release_repo_root)}"
    git -C "$repo_root" fetch origin main >/dev/null 2>&1
    local_sha="$(git -C "$repo_root" rev-parse refs/heads/main)"
    remote_sha="$(git -C "$repo_root" rev-parse refs/remotes/origin/main)"
    if [[ "$local_sha" != "$remote_sha" ]]; then
        release_error "Local main is not synchronized with origin/main"
        return 1
    fi
}

release_manifests() {
    local repo_root
    repo_root="${1:-$(release_repo_root)}"
    cat <<EOF
$repo_root/Cargo.toml
$repo_root/crates/gram-codec/Cargo.toml
$repo_root/typescript/@relateby/pattern/package.json
$repo_root/python/relateby/pyproject.toml
EOF
}

read_release_version() {
    local repo_root
    repo_root="${1:-$(release_repo_root)}"
    python3 - "$repo_root/Cargo.toml" <<'PY'
import re
import sys
text = open(sys.argv[1], encoding="utf-8").read()
m = re.search(r'^\[workspace\.package\]\n(?:.*\n)*?version = "([^"]+)"', text, re.M)
if not m:
    raise SystemExit("workspace version not found")
print(m.group(1))
PY
}

update_release_versions() {
    local version repo_root
    version="$1"
    repo_root="${2:-$(release_repo_root)}"
    python3 - "$repo_root" "$version" <<'PY'
from pathlib import Path
import json
import re
import sys

repo = Path(sys.argv[1])
version = sys.argv[2]

def replace(pattern: str, replacement: str, path: Path) -> None:
    text = path.read_text(encoding="utf-8")
    new_text, count = re.subn(pattern, replacement, text, flags=re.M)
    if count == 0:
        raise SystemExit(f"pattern not found in {path}")
    path.write_text(new_text, encoding="utf-8")

replace(
    r'(^\[workspace\.package\]\n(?:.*\n)*?version = ")[^"]+(")',
    rf'\g<1>{version}\2',
    repo / "Cargo.toml",
)
replace(
    r'(package = "relateby-pattern", path = "\.\./pattern-core", version = ")[^"]+(")',
    rf'\g<1>{version}\2',
    repo / "crates" / "gram-codec" / "Cargo.toml",
)

pkg_path = repo / "typescript" / "@relateby" / "pattern" / "package.json"
pkg = json.loads(pkg_path.read_text(encoding="utf-8"))
pkg["version"] = version
pkg_path.write_text(json.dumps(pkg, indent=2) + "\n", encoding="utf-8")

replace(
    r'(^version = ")[^"]+(")',
    rf'\g<1>{version}\2',
    repo / "python" / "relateby" / "pyproject.toml",
)
PY
}

verify_release_versions() {
    local version repo_root
    version="$1"
    repo_root="${2:-$(release_repo_root)}"
    python3 - "$repo_root" "$version" <<'PY'
from pathlib import Path
import json
import re
import sys

repo = Path(sys.argv[1])
version = sys.argv[2]
errors = []

cargo = (repo / "Cargo.toml").read_text(encoding="utf-8")
if f'version = "{version}"' not in cargo:
    errors.append("workspace Cargo version mismatch")

gram = (repo / "crates" / "gram-codec" / "Cargo.toml").read_text(encoding="utf-8")
if f'version = "{version}"' not in gram:
    errors.append("gram-codec dependency version mismatch")

pkg = json.loads((repo / "typescript" / "@relateby" / "pattern" / "package.json").read_text(encoding="utf-8"))
if pkg.get("version") != version:
    errors.append("@relateby/pattern package version mismatch")

pyproject = (repo / "python" / "relateby" / "pyproject.toml").read_text(encoding="utf-8")
m = re.search(r'^version = "([^"]+)"', pyproject, re.M)
if not m or m.group(1) != version:
    errors.append("python combined package version mismatch")

if errors:
    raise SystemExit("\n".join(errors))
PY
}

run_release_validation() {
    local repo_root
    repo_root="${1:-$(release_repo_root)}"
    "$repo_root/scripts/ci-local.sh" --release
}
