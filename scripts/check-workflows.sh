#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(CDPATH="" cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

if ! command -v actionlint >/dev/null 2>&1; then
    echo "[check-workflows] ERROR: actionlint is required but was not found on PATH" >&2
    exit 1
fi

cd "$REPO_ROOT"

echo "[check-workflows] Running actionlint"
actionlint -color

if command -v shellcheck >/dev/null 2>&1; then
    echo "[check-workflows] Running shellcheck on workflow helper scripts"
    shellcheck -x -e SC1091,SC2329 scripts/*.sh scripts/release/*.sh
else
    echo "[check-workflows] shellcheck not found; skipping shell script lint"
fi
