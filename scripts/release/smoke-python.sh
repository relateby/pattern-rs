#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(CDPATH="" cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
WORK_DIR="$(mktemp -d)"
trap 'rm -rf "$WORK_DIR"' EXIT

usage() {
    cat <<'EOF'
Usage:
  scripts/release/smoke-python.sh --wheel <path-to-wheel>
  scripts/release/smoke-python.sh --spec <package-spec> [--index-url <url>]
EOF
}

WHEEL_PATH=""
PACKAGE_SPEC=""
INDEX_URL=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --wheel)
            WHEEL_PATH="$2"
            shift 2
            ;;
        --spec)
            PACKAGE_SPEC="$2"
            shift 2
            ;;
        --index-url)
            INDEX_URL="$2"
            shift 2
            ;;
        *)
            usage
            exit 1
            ;;
    esac
done

if [[ -z "$WHEEL_PATH" && -z "$PACKAGE_SPEC" ]]; then
    usage
    exit 1
fi

PYTHON_EXE="${PYTHON:-python}"
"$PYTHON_EXE" -m venv "$WORK_DIR/venv"
source "$WORK_DIR/venv/bin/activate"

python -m pip install --upgrade pip >/dev/null

if [[ -n "$WHEEL_PATH" ]]; then
    python -m pip install "$WHEEL_PATH" >/dev/null
else
    if [[ -n "$INDEX_URL" ]]; then
        python -m pip install --index-url "$INDEX_URL" "$PACKAGE_SPEC" >/dev/null
    else
        python -m pip install "$PACKAGE_SPEC" >/dev/null
    fi
fi

python "$REPO_ROOT/scripts/release/python-smoke.py"
