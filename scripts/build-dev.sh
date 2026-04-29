#!/bin/bash
# Development build script: rebuild native artifacts after Rust changes.
#
# Run this after changing any Rust code that crosses an FFI boundary:
#   - crates/gram-codec/src/python.rs  → updates Python bindings
#   - crates/pattern-core/src/python.rs → updates Python bindings
#   - adapters/wasm/pattern-wasm/src/  → updates WASM bindings
#
# What this does:
#   1. Rebuilds WASM binaries (Node.js + bundler targets) for TypeScript tests
#   2. Builds the combined Python wheel and extracts .so files into the
#      source tree so that pytest can find them via pythonpath = ["."]
#
# Usage:
#   ./scripts/build-dev.sh           # rebuild everything
#   ./scripts/build-dev.sh --wasm    # WASM only
#   ./scripts/build-dev.sh --python  # Python only

set -euo pipefail

SCRIPT_DIR="$(CDPATH="" cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

WASM_PKG_DIR="$REPO_ROOT/typescript/packages/pattern"
PYTHON_PKG_DIR="$REPO_ROOT/python/packages/relateby"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

step() { echo -e "\n${YELLOW}▶ $*${NC}"; }
ok()   { echo -e "${GREEN}✓ $*${NC}"; }
fail() { echo -e "${RED}✗ $*${NC}" >&2; exit 1; }

BUILD_WASM=1
BUILD_PYTHON=1

for arg in "$@"; do
    case "$arg" in
        --wasm)   BUILD_PYTHON=0 ;;
        --python) BUILD_WASM=0 ;;
        *) echo "Unknown flag: $arg" >&2; exit 1 ;;
    esac
done

# --- WASM ---

if [[ $BUILD_WASM -eq 1 ]]; then
    if ! command -v wasm-pack >/dev/null 2>&1; then
        echo -e "${YELLOW}⚠ wasm-pack not found — skipping WASM build${NC}"
        echo "  Install: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    else
        step "Building WASM (Node.js target)"
        (cd "$WASM_PKG_DIR" && npm run build:wasm:node)
        ok "wasm-node/ updated"

        step "Building WASM (bundler target)"
        (cd "$WASM_PKG_DIR" && npm run build:wasm)
        ok "wasm/ updated"
    fi
fi

# --- Python ---

if [[ $BUILD_PYTHON -eq 1 ]]; then
    UV_EXE="$(command -v uv || true)"
    if [[ -z "$UV_EXE" ]]; then
        fail "uv not found — install from https://docs.astral.sh/uv/"
    fi

    PYTHON_EXE=""
    for candidate in python3.13 python3.12 python3.11 python3.10 python3.9 python3.8 python3 python; do
        if command -v "$candidate" >/dev/null 2>&1; then
            if "$candidate" -c "import sys; raise SystemExit(0 if (3,8) <= sys.version_info[:2] < (3,14) else 1)" 2>/dev/null; then
                PYTHON_EXE="$candidate"
                break
            fi
        fi
    done
    if [[ -z "$PYTHON_EXE" ]]; then
        fail "No suitable Python (3.8–3.13) found"
    fi

    step "Building combined Python wheel (Python: $PYTHON_EXE)"
    (
        cd "$PYTHON_PKG_DIR"
        CARGO_TARGET_DIR="$REPO_ROOT/target" \
            "$UV_EXE" build --wheel --python "$PYTHON_EXE" --out-dir dist
    )
    ok "Wheel built in python/packages/relateby/dist/"

    step "Extracting .so files into source tree for pytest"
    shopt -s nullglob
    wheels=("$PYTHON_PKG_DIR/dist/"*.whl)
    shopt -u nullglob
    WHEEL="${wheels[0]:-}"
    if [[ -z "$WHEEL" ]]; then
        fail "No wheel found in $PYTHON_PKG_DIR/dist/"
    fi
    (
        cd "$PYTHON_PKG_DIR"
        unzip -o "$WHEEL" "relateby/_native/*.so" -d .
    )
    ok ".so files placed in python/packages/relateby/relateby/_native/"

    echo ""
    echo "Python dev environment is ready. Run tests with:"
    echo "  source python/packages/relateby/.venv/bin/activate"
    echo "  cd python/packages/relateby && pytest"
fi

echo ""
echo -e "${GREEN}Done.${NC}"
