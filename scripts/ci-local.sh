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

PYTHON_EXE=""
for candidate in python python3; do
    if command -v "$candidate" >/dev/null 2>&1; then
        if "$candidate" -c "import sys; raise SystemExit(0 if (3, 8) <= sys.version_info[:2] < (3, 14) else 1)" 2>/dev/null; then
            PYTHON_EXE="$candidate"
            break
        fi
    fi
done

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

npm_pack_smoke() {
    local smoke_dir="$REPO_ROOT/scripts/release/npm-smoke"
    local pack_dir="$REPO_ROOT/target/npm-packages"
    local tarball

    mkdir -p "$pack_dir"
    rm -f "$pack_dir"/*.tgz
    npm run pack:release --workspace=@relateby/pattern >/dev/null
    tarball="$(ls "$pack_dir"/*.tgz 2>/dev/null | head -n 1)"
    if [[ -z "$tarball" ]]; then
        echo "No npm tarball found in $pack_dir" >&2
        return 1
    fi

    rm -rf "$smoke_dir/node_modules" "$smoke_dir/package-lock.json"
    (
        cd "$smoke_dir" &&
        npm install --silent "$tarball" &&
        npm run smoke
    )
}

python_release_build() {
    local pyproject_dir="$REPO_ROOT/python/relateby"
    if [[ -z "$PYTHON_EXE" ]]; then
        echo "Need Python 3.8-3.13 to build the combined wheel" >&2
        return 1
    fi
    rm -rf "$pyproject_dir/dist"
    (
        cd "$pyproject_dir" &&
        "$PYTHON_EXE" -m pip wheel . -w dist --no-deps
    )
}

python_release_smoke() {
    local wheel
    wheel="$(ls "$REPO_ROOT/python/relateby/dist/"*.whl 2>/dev/null | head -n 1)"
    if [[ -z "$wheel" ]]; then
        echo "No Python wheel found in python/relateby/dist" >&2
        return 1
    fi
    bash "$REPO_ROOT/scripts/release/smoke-python.sh" --wheel "$wheel"
}

crate_version_exists_on_crates_io() {
    local crate_name=$1
    local crate_version=$2
    python - "$crate_name" "$crate_version" <<'PY'
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
    cargo publish -p relateby-pattern --dry-run
    if crate_version_exists_on_crates_io "relateby-pattern" "$version"; then
        # When validating an already-published version, crates.io resolution for the
        # gram crate can point at the published pattern crate instead of the local tree.
        echo "relateby-pattern $version already exists on crates.io; using cargo package --no-verify fallback for relateby-gram" >&2
        cargo package -p relateby-gram --allow-dirty --no-verify
    else
        cargo publish -p relateby-gram --dry-run
    fi
}

run_check "Format check" cargo fmt --all -- --check || true
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
        run_check "WASM build" cargo build --target wasm32-unknown-unknown --workspace || true
    else
        run_optional_check "WASM build" cargo build --target wasm32-unknown-unknown --workspace
    fi
else
    echo -e "${YELLOW}⚠${NC} wasm32-unknown-unknown target not installed"
    [[ $RELEASE_MODE -eq 1 ]] && FAILED=1
fi
echo ""

if command -v npm >/dev/null 2>&1 && command -v wasm-pack >/dev/null 2>&1; then
    run_check "npm install" npm ci || true
    echo ""
    run_check "Pattern package build" npm run build --workspace=@relateby/pattern || true
    echo ""
    run_check "Pattern package tests" npm run test --workspace=@relateby/pattern || true
    echo ""
    if [[ $RELEASE_MODE -eq 1 ]]; then
        run_check "npm packed artifact smoke test" npm_pack_smoke || true
        echo ""
    fi
else
    echo -e "${YELLOW}⚠${NC} npm or wasm-pack unavailable"
    [[ $RELEASE_MODE -eq 1 ]] && FAILED=1
fi

if [[ -n "$PYTHON_EXE" ]]; then
    if [[ $RELEASE_MODE -eq 1 ]]; then
        run_check "Combined Python wheel build" python_release_build || true
        echo ""
        if "$PYTHON_EXE" -m twine --version >/dev/null 2>&1; then
            run_check "Combined Python metadata check" "$PYTHON_EXE" -m twine check "$REPO_ROOT"/python/relateby/dist/* || true
            echo ""
        else
            echo -e "${YELLOW}⚠${NC} twine unavailable"
            FAILED=1
        fi
        run_check "Combined Python smoke test" python_release_smoke || true
        echo ""
    else
        run_optional_check "Combined Python wheel build" python_release_build
        echo ""
    fi
else
    echo -e "${YELLOW}⚠${NC} Python 3.8-3.13 unavailable"
    [[ $RELEASE_MODE -eq 1 ]] && FAILED=1
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

