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

release_typescript_pattern_dir() {
    local repo_root
    repo_root="${1:-$(release_repo_root)}"
    printf '%s\n' "$repo_root/typescript/packages/pattern"
}

release_typescript_graph_dir() {
    local repo_root
    repo_root="${1:-$(release_repo_root)}"
    printf '%s\n' "$repo_root/typescript/packages/graph"
}

release_typescript_gram_dir() {
    local repo_root
    repo_root="${1:-$(release_repo_root)}"
    printf '%s\n' "$repo_root/typescript/packages/gram"
}

release_python_package_dir() {
    local repo_root
    repo_root="${1:-$(release_repo_root)}"
    printf '%s\n' "$repo_root/python/packages/relateby"
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
    local ts_pattern_dir
    local ts_graph_dir
    local ts_gram_dir
    local python_pkg_dir
    repo_root="${1:-$(release_repo_root)}"
    ts_pattern_dir="$(release_typescript_pattern_dir "$repo_root")"
    ts_graph_dir="$(release_typescript_graph_dir "$repo_root")"
    ts_gram_dir="$(release_typescript_gram_dir "$repo_root")"
    python_pkg_dir="$(release_python_package_dir "$repo_root")"
    cat <<EOF
$repo_root/Cargo.toml
$repo_root/crates/gram-codec/Cargo.toml
$ts_pattern_dir/package.json
$ts_graph_dir/package.json
$ts_gram_dir/package.json
$python_pkg_dir/pyproject.toml
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

package_paths = {
    "pattern": repo / "typescript" / "packages" / "pattern" / "package.json",
    "graph": repo / "typescript" / "packages" / "graph" / "package.json",
    "gram": repo / "typescript" / "packages" / "gram" / "package.json",
}

for name, pkg_path in package_paths.items():
    pkg = json.loads(pkg_path.read_text(encoding="utf-8"))
    pkg["version"] = version
    if name == "gram":
        pkg.setdefault("dependencies", {})["@relateby/pattern"] = version
    pkg_path.write_text(json.dumps(pkg, indent=2) + "\n", encoding="utf-8")

pyproject_path = repo / "python" / "packages" / "relateby" / "pyproject.toml"
replace(
    r'(^version = ")[^"]+(")',
    rf'\g<1>{version}\2',
    pyproject_path,
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
gram_dep = re.search(
    r'^pattern_core = \{[^}]*version = "([^"]+)"[^}]*\}$',
    gram,
    re.M,
)
if not gram_dep or gram_dep.group(1) != version:
    errors.append("gram-codec dependency version mismatch")

package_paths = {
    "pattern": repo / "typescript" / "packages" / "pattern" / "package.json",
    "graph": repo / "typescript" / "packages" / "graph" / "package.json",
    "gram": repo / "typescript" / "packages" / "gram" / "package.json",
}

for name, pkg_path in package_paths.items():
    pkg = json.loads(pkg_path.read_text(encoding="utf-8"))
    if pkg.get("version") != version:
        errors.append(f"@relateby/{name} package version mismatch")
    if name == "gram" and pkg.get("dependencies", {}).get("@relateby/pattern") != version:
        errors.append("@relateby/gram dependency on @relateby/pattern version mismatch")

pyproject_path = repo / "python" / "packages" / "relateby" / "pyproject.toml"
pyproject = pyproject_path.read_text(encoding="utf-8")
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
