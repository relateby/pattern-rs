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

# 4. WASM build (optional - only builds implemented crates)
echo -n "Checking WASM target... "
if rustup target list --installed | grep -q wasm32-unknown-unknown; then
    echo -e "${GREEN}✓${NC}"
    # Only build crates that are actually implemented
    # WASM is optional for now, so don't fail the script if it fails
    echo -n "Running WASM build... "
    if cargo build --target wasm32-unknown-unknown -p pattern-core > /tmp/ci-check.log 2>&1; then
        echo -e "${GREEN}✓${NC}"
    else
        echo -e "${YELLOW}⚠${NC} (failed, but non-blocking)"
        echo "  WASM build failed - this is optional for now"
    fi
else
    echo -e "${YELLOW}⚠${NC} (not installed, skipping)"
    echo "  Install with: rustup target add wasm32-unknown-unknown"
fi
echo ""

# 5. Tests
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

