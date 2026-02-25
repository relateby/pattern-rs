#!/bin/bash
# Local CI script - Run all CI checks locally
# This reproduces what GitHub Actions does

set -e  # Exit on error

echo "🔨 Running local CI checks..."
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Track failures
FAILED=0

# Function to run a check
run_check() {
    local name=$1
    shift
    echo -n "Running $name... "
    if "$@" > /tmp/ci-check.log 2>&1; then
        echo -e "${GREEN}✓${NC}"
        return 0
    else
        echo -e "${RED}✗${NC}"
        echo "Error output:"
        tail -20 /tmp/ci-check.log
        FAILED=1
        return 1
    fi
}

# 1. Format check
run_check "Format check" cargo fmt --all -- --check
echo ""

# 2. Lint check
run_check "Clippy lint" cargo clippy --workspace -- -D warnings
echo ""

# 3. Native build
run_check "Native build" cargo build --workspace
echo ""

# 4. WASM build (optional - matches GitHub Actions: cargo build --workspace)
echo -n "Checking WASM target... "
# Prefer rustup toolchain for WASM builds (Homebrew Rust lacks wasm32 target)
RUSTUP_BIN="$HOME/.rustup/toolchains/stable-$(rustup show active-toolchain 2>/dev/null | awk '{print $1}' | sed 's/stable-//')/bin"
if [ -d "$RUSTUP_BIN" ] && "$RUSTUP_BIN/rustup" target list --installed 2>/dev/null | grep -q wasm32-unknown-unknown 2>/dev/null || \
   rustup target list --installed 2>/dev/null | grep -q wasm32-unknown-unknown; then
    echo -e "${GREEN}✓${NC}"
    # Build the full workspace to match CI - catches type errors in all WASM crates
    # WASM is optional for now, so don't fail the script if it fails
    echo -n "Running WASM build... "
    if PATH="${RUSTUP_BIN}:$PATH" cargo build --target wasm32-unknown-unknown --workspace > /tmp/ci-check.log 2>&1; then
        echo -e "${GREEN}✓${NC}"
    else
        echo -e "${YELLOW}⚠${NC} (failed, but non-blocking)"
        echo "  WASM build failed - this is optional for now"
        tail -20 /tmp/ci-check.log
    fi
else
    echo -e "${YELLOW}⚠${NC} (not installed, skipping)"
    echo "  Install with: rustup target add wasm32-unknown-unknown"
fi
echo ""

# 4b. TypeScript package builds (optional - requires npm and wasm-pack)
echo -n "Checking TypeScript build setup... "
if command -v npm >/dev/null 2>&1 && command -v wasm-pack >/dev/null 2>&1; then
    echo -e "${GREEN}✓${NC}"
    REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

    echo -n "Building @relateby/graph... "
    if (cd "$REPO_ROOT/typescript/@relateby/graph" && npm install --silent && npm run build) > /tmp/ci-check.log 2>&1; then
        echo -e "${GREEN}✓${NC}"
    else
        echo -e "${YELLOW}⚠${NC} (failed, but non-blocking)"
        tail -10 /tmp/ci-check.log
    fi

    echo -n "Building @relateby/pattern WASM (bundler)... "
    if (cd "$REPO_ROOT/typescript/@relateby/pattern" && npm run build:wasm) > /tmp/ci-check.log 2>&1; then
        echo -e "${GREEN}✓${NC}"
    else
        echo -e "${YELLOW}⚠${NC} (failed, but non-blocking)"
        tail -10 /tmp/ci-check.log
    fi

    echo -n "Building @relateby/pattern WASM (nodejs)... "
    if (cd "$REPO_ROOT/typescript/@relateby/pattern" && npm run build:wasm:node) > /tmp/ci-check.log 2>&1; then
        echo -e "${GREEN}✓${NC}"
    else
        echo -e "${YELLOW}⚠${NC} (failed, but non-blocking)"
        tail -10 /tmp/ci-check.log
    fi

    echo -n "Building @relateby/pattern TypeScript... "
    # npm's file: protocol with scoped @-prefixed packages can create a broken
    # relative symlink (known npm limitation). If the symlink is broken, replace
    # it with an absolute symlink so the build can resolve @relateby/graph.
    GRAPH_DIR="$REPO_ROOT/typescript/@relateby/graph"
    PATTERN_LINK="$REPO_ROOT/typescript/@relateby/pattern/node_modules/@relateby/graph"
    (cd "$REPO_ROOT/typescript/@relateby/pattern" && npm install --silent) > /tmp/ci-check.log 2>&1
    if [ -L "$PATTERN_LINK" ] && [ ! -e "$PATTERN_LINK" ]; then
        rm "$PATTERN_LINK" && ln -s "$GRAPH_DIR" "$PATTERN_LINK"
    fi
    if (cd "$REPO_ROOT/typescript/@relateby/pattern" && npm run build:ts) > /tmp/ci-check.log 2>&1; then
        echo -e "${GREEN}✓${NC}"
    else
        echo -e "${YELLOW}⚠${NC} (failed, but non-blocking)"
        tail -10 /tmp/ci-check.log
    fi
else
    echo -e "${YELLOW}⚠${NC} (not available, skipping)"
    if ! command -v npm >/dev/null 2>&1; then
        echo "  Install Node.js/npm"
    fi
    if ! command -v wasm-pack >/dev/null 2>&1; then
        echo "  Install wasm-pack: cargo install wasm-pack"
    fi
fi
echo ""

# 5. Python build (optional - requires maturin and Python)
echo -n "Checking Python build setup... "
if command -v maturin >/dev/null 2>&1 && command -v python3 >/dev/null 2>&1; then
    echo -e "${GREEN}✓${NC}"
    # Python build is optional for now, so don't fail the script if it fails
    echo -n "Running Python build... "
    cd crates/pattern-core
    if maturin build --release --features python > /tmp/ci-check.log 2>&1; then
        echo -e "${GREEN}✓${NC}"
    else
        echo -e "${YELLOW}⚠${NC} (failed, but non-blocking)"
        echo "  Python build failed - this is optional for now"
        tail -10 /tmp/ci-check.log
    fi
    cd ../..
else
    echo -e "${YELLOW}⚠${NC} (not available, skipping)"
    if ! command -v maturin >/dev/null 2>&1; then
        echo "  Install maturin with: pip install maturin"
        echo "  Or with uv: cd crates/pattern-core && uv pip install -e '.[dev]'"
    fi
    if ! command -v python3 >/dev/null 2>&1; then
        echo "  Install Python 3.8+ (python3)"
    fi
fi
echo ""

# 6. Tests
run_check "Tests" cargo test --workspace
echo ""

# Summary
echo "=========================================="
if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}All checks passed!${NC}"
    exit 0
else
    echo -e "${RED}Some checks failed. See output above.${NC}"
    exit 1
fi

