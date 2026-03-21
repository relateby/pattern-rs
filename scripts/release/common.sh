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

release_branch_for_version() {
    printf 'release/v%s\n' "$1"
}

release_branch_exists() {
    local branch repo_root
    branch="$1"
    repo_root="${2:-$(release_repo_root)}"
    if git -C "$repo_root" show-ref --verify --quiet "refs/heads/$branch"; then
        return 0
    fi
    git -C "$repo_root" ls-remote --exit-code --heads origin "$branch" >/dev/null 2>&1
}

release_tag_exists() {
    local tag repo_root
    tag="$1"
    repo_root="${2:-$(release_repo_root)}"
    if git -C "$repo_root" show-ref --verify --quiet "refs/tags/$tag"; then
        return 0
    fi
    git -C "$repo_root" ls-remote --exit-code --tags origin "$tag" >/dev/null 2>&1
}

release_version_published() {
    local version
    version="$1"
    python3 - "$version" <<'PY'
from __future__ import annotations

import json
import subprocess
import sys
import urllib.error
import urllib.request

version = sys.argv[1]

def crates_io_has(crate: str) -> bool:
    with urllib.request.urlopen(f"https://crates.io/api/v1/crates/{crate}") as response:
        payload = json.load(response)
    return version in {item["num"] for item in payload.get("versions", [])}

def npm_has(package: str) -> bool:
    result = subprocess.run(
        ["npm", "view", f"{package}@{version}", "version"],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    return result.returncode == 0

def pypi_has(package: str) -> bool:
    with urllib.request.urlopen(f"https://pypi.org/pypi/{package}/json") as response:
        payload = json.load(response)
    return version in payload.get("releases", {})

checks = [
    ("crates.io", "relateby-pattern", lambda: crates_io_has("relateby-pattern")),
    ("crates.io", "relateby-gram", lambda: crates_io_has("relateby-gram")),
    ("npm", "@relateby/pattern", lambda: npm_has("@relateby/pattern")),
    ("npm", "@relateby/graph", lambda: npm_has("@relateby/graph")),
    ("npm", "@relateby/gram", lambda: npm_has("@relateby/gram")),
    ("PyPI", "relateby-pattern", lambda: pypi_has("relateby-pattern")),
]

try:
    for registry, subject, check in checks:
        if check():
            print(f"{registry}:{subject}:{version}")
            raise SystemExit(0)
except Exception as exc:  # noqa: BLE001
    print(f"release version lookup failed: {exc}", file=sys.stderr)
    raise SystemExit(2)

raise SystemExit(1)
PY
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
$repo_root/crates/pato/Cargo.toml
$ts_pattern_dir/package.json
$ts_graph_dir/package.json
$ts_gram_dir/package.json
$repo_root/package-lock.json
$repo_root/examples/typescript/graph/package-lock.json
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

def update_package_json(path: Path, *, depends_on_pattern: bool = False) -> None:
    pkg = json.loads(path.read_text(encoding="utf-8"))
    pkg["version"] = version
    if depends_on_pattern:
        pkg.setdefault("dependencies", {})["@relateby/pattern"] = version
    path.write_text(json.dumps(pkg, indent=2) + "\n", encoding="utf-8")

def update_lockfile(path: Path, workspace_paths, pattern_dependency_owner=None):
    lock = json.loads(path.read_text(encoding="utf-8"))
    packages = lock.get("packages", {})
    for workspace_path in workspace_paths:
        package = packages.get(workspace_path)
        if package is None:
            raise SystemExit(f"workspace entry not found in {path}: {workspace_path}")
        package["version"] = version
        if workspace_path.endswith("/gram"):
            package.setdefault("dependencies", {})["@relateby/pattern"] = version
    if pattern_dependency_owner is not None:
        dependency = lock.get("dependencies", {}).get(pattern_dependency_owner)
        if dependency is None:
            raise SystemExit(f"dependency entry not found in {path}: {pattern_dependency_owner}")
        dependency.setdefault("requires", {})["@relateby/pattern"] = version
    path.write_text(json.dumps(lock, indent=2) + "\n", encoding="utf-8")

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
replace(
    r'(pattern_core = \{ package = "relateby-pattern", path = "\.\./pattern-core", version = ")[^"]+(" \})',
    rf'\g<1>{version}\2',
    repo / "crates" / "pato" / "Cargo.toml",
)
replace(
    r'(gram_codec = \{ package = "relateby-gram", path = "\.\./gram-codec", version = ")[^"]+(")',
    rf'\g<1>{version}\2',
    repo / "crates" / "pato" / "Cargo.toml",
)

update_package_json(repo / "typescript" / "packages" / "pattern" / "package.json")
update_package_json(repo / "typescript" / "packages" / "graph" / "package.json")
update_package_json(repo / "typescript" / "packages" / "gram" / "package.json", depends_on_pattern=True)

update_lockfile(
    repo / "package-lock.json",
    [
        "typescript/packages/pattern",
        "typescript/packages/graph",
        "typescript/packages/gram",
    ],
    pattern_dependency_owner="@relateby/gram",
)

update_lockfile(
    repo / "examples" / "typescript" / "graph" / "package-lock.json",
    [
        "../../../typescript/packages/pattern",
        "../../../typescript/packages/graph",
    ],
)

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

proto = (repo / "crates" / "pato" / "Cargo.toml").read_text(encoding="utf-8")
pato_pattern_dep = re.search(
    r'^pattern_core = \{[^}]*version = "([^"]+)"[^}]*\}$',
    proto,
    re.M,
)
if not pato_pattern_dep or pato_pattern_dep.group(1) != version:
    errors.append("pato dependency on relateby-pattern version mismatch")

pato_gram_dep = re.search(
    r'^gram_codec = \{[^}]*version = "([^"]+)"[^}]*\}$',
    proto,
    re.M,
)
if not pato_gram_dep or pato_gram_dep.group(1) != version:
    errors.append("pato dependency on relateby-gram version mismatch")

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

def verify_lockfile(path: Path, workspace_paths, pattern_dependency_owner=None):
    lock = json.loads(path.read_text(encoding="utf-8"))
    packages = lock.get("packages", {})
    for workspace_path in workspace_paths:
        package = packages.get(workspace_path)
        if package is None or package.get("version") != version:
            errors.append(f"{path}: workspace entry version mismatch for {workspace_path}")
        if workspace_path.endswith("/gram"):
            if package is None or package.get("dependencies", {}).get("@relateby/pattern") != version:
                errors.append(f"{path}: @relateby/gram dependency on @relateby/pattern version mismatch")
    if pattern_dependency_owner is not None:
        dependency = lock.get("dependencies", {}).get(pattern_dependency_owner)
        if dependency is None or dependency.get("requires", {}).get("@relateby/pattern") != version:
            errors.append(f"{path}: top-level dependency on @relateby/pattern version mismatch")

verify_lockfile(
    repo / "package-lock.json",
    [
        "typescript/packages/pattern",
        "typescript/packages/graph",
        "typescript/packages/gram",
    ],
    pattern_dependency_owner="@relateby/gram",
)

verify_lockfile(
    repo / "examples" / "typescript" / "graph" / "package-lock.json",
    [
        "../../../typescript/packages/pattern",
        "../../../typescript/packages/graph",
    ],
)

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
