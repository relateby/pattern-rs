#!/bin/bash
# Local CI / release validation script

set -euo pipefail

SCRIPT_DIR="$(CDPATH="" cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
RELEASE_MODE=0

if [[ "${1:-}" == "--release" ]]; then
    RELEASE_MODE=1
fi

# shellcheck source=./release/common.sh
# shellcheck disable=SC1091
source "$REPO_ROOT/scripts/release/common.sh"

echo "🔨 Running local CI checks..."
if [[ $RELEASE_MODE -eq 1 ]]; then
    echo "Mode: release"
else
    echo "Mode: standard"
fi
echo ""

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'
FAILED=0

UV_EXE="$(command -v uv || true)"
PYTHON_EXE=""
for candidate in "${PYTHON:-}" python3.13 python3.12 python3.11 python3.10 python3.9 python3.8 python3 python; do
    [[ -n "$candidate" ]] || continue
    if command -v "$candidate" >/dev/null 2>&1; then
        if "$candidate" -c "import sys; raise SystemExit(0 if (3, 8) <= sys.version_info[:2] < (3, 14) else 1)" 2>/dev/null; then
            PYTHON_EXE="$candidate"
            break
        fi
    fi
done
PYTHON_PACKAGE_DIR="$(release_python_package_dir "$REPO_ROOT")"
PYTHON_VENV="$PYTHON_PACKAGE_DIR/.venv"

run_check() {
    local name=$1
    shift
    echo -n "Running $name... "
    if "$@" > /tmp/ci-check.log 2>&1; then
        echo -e "${GREEN}✓${NC}"
        return 0
    else
        echo -e "${RED}✗${NC}"
        tail -20 /tmp/ci-check.log
        FAILED=1
        return 1
    fi
}

run_optional_check() {
    local name=$1
    shift
    echo -n "Running $name... "
    if "$@" > /tmp/ci-check.log 2>&1; then
        echo -e "${GREEN}✓${NC}"
        return 0
    fi
    echo -e "${YELLOW}⚠${NC} (non-blocking)"
    tail -20 /tmp/ci-check.log
    return 0
}

npm_workspace_node_major() {
    node -p "Number(process.versions.node.split('.')[0])" 2>/dev/null || echo ""
}

expect_node_20_for_npm_workspaces() {
    local major
    major="$(npm_workspace_node_major)"
    if [[ -z "$major" ]]; then
        echo "node is required for npm workspace checks" >&2
        return 1
    fi
    if [[ "$major" -ne 20 ]]; then
        echo "npm workspaces in this repo are validated on Node.js 20.x (see .nvmrc). Current: $(node --version 2>/dev/null || echo unknown)" >&2
        return 1
    fi
    return 0
}

npm_pack_smoke() {
    local smoke_dir="$REPO_ROOT/scripts/release/npm-smoke"
    local pack_dir="$REPO_ROOT/target/npm-packages"
    local -a tarballs=()
    local tmp_dir
    local status

    mkdir -p "$pack_dir"
    rm -f "$pack_dir"/*.tgz
    npm run pack:release --workspace=@relateby/pattern >/dev/null
    npm run pack:release --workspace=@relateby/graph >/dev/null
    npm run pack:release --workspace=@relateby/gram >/dev/null
    shopt -s nullglob
    tarballs=("$pack_dir"/*.tgz)
    shopt -u nullglob
    if [[ ${#tarballs[@]} -lt 3 ]]; then
        echo "No npm tarball found in $pack_dir" >&2
        return 1
    fi

    tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/relateby-npm-smoke.XXXXXX")"
    cp "$smoke_dir/package.json" "$smoke_dir/smoke.mjs" "$tmp_dir/"
    (
        cd "$tmp_dir" &&
        npm install --silent --no-save --package-lock=false "${tarballs[@]}" &&
        npm run smoke
    )
    status=$?
    rm -rf "$tmp_dir"
    return "$status"
}

python_release_build() {
    local pyproject_dir
    pyproject_dir="$(release_python_package_dir "$REPO_ROOT")"
    if ! ensure_python_venv; then
        return 1
    fi
    rm -rf "$pyproject_dir/dist"
    (
        cd "$pyproject_dir" &&
        CARGO_TARGET_DIR="$REPO_ROOT/target" "$UV_EXE" build --wheel --python "$PYTHON_EXE" --out-dir dist
    )
}

python_release_smoke() {
    local -a wheels=()
    local wheel
    local pyproject_dir
    pyproject_dir="$(release_python_package_dir "$REPO_ROOT")"
    shopt -s nullglob
    wheels=("$pyproject_dir/dist/"*.whl)
    shopt -u nullglob
    wheel="${wheels[0]:-}"
    if [[ -z "$wheel" ]]; then
        echo "No Python wheel found in $pyproject_dir/dist" >&2
        return 1
    fi
    bash "$REPO_ROOT/scripts/release/smoke-python.sh" --wheel "$wheel"
}

python_public_api_tests() {
    local pyproject_dir
    pyproject_dir="$(release_python_package_dir "$REPO_ROOT")"
    if ! ensure_python_venv; then
        return 1
    fi
    (
        cd "$pyproject_dir" &&
        "$PYTHON_VENV/bin/python" -m pytest tests/test_public_api.py
    )
}

ensure_python_venv() {
    if [[ -z "$PYTHON_EXE" ]]; then
        echo "Need Python 3.8-3.13 to run Python validation" >&2
        return 1
    fi
    if [[ -z "$UV_EXE" ]]; then
        echo "Need uv to run Python validation" >&2
        return 1
    fi
    mkdir -p "$PYTHON_PACKAGE_DIR"
    if [[ ! -x "$PYTHON_VENV/bin/python" ]]; then
        "$UV_EXE" venv --python "$PYTHON_EXE" "$PYTHON_VENV" >/dev/null
    fi
    "$UV_EXE" pip install --python "$PYTHON_VENV/bin/python" --quiet pytest twine maturin "tomli>=2.0; python_version<'3.11'" >/dev/null
}

crate_version_exists_on_crates_io() {
    local crate_name=$1
    local crate_version=$2
    if [[ -z "$PYTHON_EXE" ]]; then
        echo "Need Python 3.8-3.13 to query crates.io" >&2
        return 1
    fi
    "$PYTHON_EXE" - "$crate_name" "$crate_version" <<'PY'
import json
import sys
import urllib.error
import urllib.request

crate_name = sys.argv[1]
crate_version = sys.argv[2]
try:
    with urllib.request.urlopen(f"https://crates.io/api/v1/crates/{crate_name}") as response:
        payload = json.load(response)
except urllib.error.URLError:
    raise SystemExit(1)

versions = {item["num"] for item in payload.get("versions", [])}
raise SystemExit(0 if crate_version in versions else 1)
PY
}

cargo_publish_dry_run_all() {
    local version
    version="$(read_release_version "$REPO_ROOT")"
    cargo publish -p relateby-pattern --dry-run --allow-dirty
    if crate_version_exists_on_crates_io "relateby-pattern" "$version"; then
        cargo publish -p relateby-gram --dry-run --allow-dirty
    else
        # Before relateby-pattern is published, cargo publish/package for relateby-gram
        # resolves the dependency against crates.io and fails. Fall back to listing the
        # packaged contents after the normal workspace build/test validation above.
        echo "relateby-pattern $version is not yet on crates.io; using cargo package --list fallback for relateby-gram" >&2
        cargo package -p relateby-gram --allow-dirty --list >/dev/null
    fi
}

run_check "Format check" cargo fmt --all -- --check || true
echo ""
if command -v actionlint >/dev/null 2>&1; then
    run_check "Workflow lint" bash "$REPO_ROOT/scripts/check-workflows.sh" || true
else
    echo -e "${YELLOW}⚠${NC} actionlint unavailable"
fi
echo ""
run_check "Clippy lint" cargo clippy --workspace --exclude pattern-wasm -- -D warnings || true
echo ""
# pattern-wasm is validated separately in the dedicated wasm build.
run_check "Native build" cargo build --workspace --exclude pattern-wasm || true
echo ""
run_check "Tests" cargo test --workspace --exclude pattern-wasm || true
echo ""
run_check "Docs build" cargo doc --no-deps -p relateby-pattern -p relateby-gram || true
echo ""

if rustup target list --installed 2>/dev/null | grep -q wasm32-unknown-unknown; then
    if [[ $RELEASE_MODE -eq 1 ]]; then
        # Only validate crates that are meant to compile for wasm.
        run_check "WASM build" cargo build --target wasm32-unknown-unknown -p relateby-pattern -p relateby-gram -p pattern-wasm || true
    else
        run_optional_check "WASM build" cargo build --target wasm32-unknown-unknown -p relateby-pattern -p relateby-gram -p pattern-wasm
    fi
else
    echo -e "${YELLOW}⚠${NC} wasm32-unknown-unknown target not installed"
    [[ $RELEASE_MODE -eq 1 ]] && FAILED=1
fi
echo ""

if command -v npm >/dev/null 2>&1 && command -v wasm-pack >/dev/null 2>&1; then
    RUN_NPM_WORKSPACE_CHECKS=1
    if [[ $RELEASE_MODE -eq 1 ]]; then
        if ! run_check "Node.js 20.x (npm workspaces)" expect_node_20_for_npm_workspaces; then
            RUN_NPM_WORKSPACE_CHECKS=0
        fi
        echo ""
    else
        if ! expect_node_20_for_npm_workspaces >/dev/null 2>&1; then
            echo -e "${YELLOW}⚠${NC} npm workspace checks expect Node 20.x (see .nvmrc); current $(node --version 2>/dev/null || echo node missing)"
        fi
        echo ""
    fi
    if [[ $RUN_NPM_WORKSPACE_CHECKS -eq 1 ]]; then
        run_check "npm install" npm ci || true
        echo ""
        run_check "Pattern package build" npm run build --workspace=@relateby/pattern || true
        echo ""
        run_check "Pattern package tests" npm run test --workspace=@relateby/pattern || true
        echo ""
        run_check "Graph package build" npm run build --workspace=@relateby/graph || true
        echo ""
        run_check "Graph package tests" npm run test --workspace=@relateby/graph || true
        echo ""
        run_check "Gram package build" npm run build --workspace=@relateby/gram || true
        echo ""
        run_check "Gram package tests" npm run test --workspace=@relateby/gram || true
        echo ""
        run_check "Pattern package public export test" npm run test:public-api --workspace=@relateby/pattern || true
        echo ""
        run_check "Pattern package public typecheck" npm run test:public-api:types --workspace=@relateby/pattern || true
        echo ""
    elif [[ $RELEASE_MODE -eq 1 ]]; then
        echo -e "${YELLOW}⚠${NC} Skipping npm workspace checks because Node.js 20.x is required in --release mode"
        echo ""
    fi
    if [[ $RELEASE_MODE -eq 1 && $RUN_NPM_WORKSPACE_CHECKS -eq 1 ]]; then
        run_check "npm packed artifact smoke test" npm_pack_smoke || true
        echo ""
    fi
else
    echo -e "${YELLOW}⚠${NC} npm or wasm-pack unavailable"
    [[ $RELEASE_MODE -eq 1 ]] && FAILED=1
fi

if [[ -n "$PYTHON_EXE" && -n "$UV_EXE" ]]; then
    if [[ $RELEASE_MODE -eq 1 ]]; then
        run_check "Combined Python wheel build" python_release_build || true
        echo ""
        if ensure_python_venv && "$PYTHON_VENV/bin/python" -m twine --version >/dev/null 2>&1; then
            pyproject_dir="$(release_python_package_dir "$REPO_ROOT")"
            run_check "Combined Python metadata check" "$PYTHON_VENV/bin/python" -m twine check "$pyproject_dir"/dist/* || true
            echo ""
        else
            echo -e "${RED}✗${NC} twine unavailable in $PYTHON_VENV"
            FAILED=1
        fi
        run_check "Combined Python smoke test" python_release_smoke || true
        echo ""
    else
        run_optional_check "Combined Python wheel build" python_release_build
        echo ""
    fi
else
    echo -e "${YELLOW}⚠${NC} Python 3.8-3.13 or uv unavailable"
    [[ $RELEASE_MODE -eq 1 ]] && FAILED=1
fi

if [[ -n "$PYTHON_EXE" ]]; then
    run_check "Combined Python public API tests" python_public_api_tests || true
    echo ""
fi

if [[ $RELEASE_MODE -eq 1 ]]; then
    run_check "Cargo publish dry-run" cargo_publish_dry_run_all || true
    echo ""
fi

echo "=========================================="
if [[ $FAILED -eq 0 ]]; then
    echo -e "${GREEN}All checks passed!${NC}"
    exit 0
fi

echo -e "${RED}Some checks failed. See output above.${NC}"
exit 1
