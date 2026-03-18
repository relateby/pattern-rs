#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(CDPATH="" cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
WORK_DIR="$(mktemp -d)"
trap 'rm -rf "$WORK_DIR"' EXIT
UV_EXE="$(command -v uv || true)"

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

if [[ -z "$UV_EXE" ]]; then
    echo "uv is required for Python smoke tests" >&2
    exit 1
fi

PYTHON_EXE="${PYTHON:-}"
if [[ -z "$PYTHON_EXE" ]]; then
    for candidate in python3.13 python3.12 python3.11 python3.10 python3.9 python3.8 python3 python; do
        if command -v "$candidate" >/dev/null 2>&1; then
            if "$candidate" -c "import sys; raise SystemExit(0 if (3, 8) <= sys.version_info[:2] < (3, 14) else 1)" 2>/dev/null; then
                PYTHON_EXE="$candidate"
                break
            fi
        fi
    done
fi

if [[ -z "$PYTHON_EXE" ]]; then
    echo "Need Python 3.8-3.13 for Python smoke tests" >&2
    exit 1
fi

"$UV_EXE" venv --python "$PYTHON_EXE" "$WORK_DIR/.venv" >/dev/null
VENV_PY="$WORK_DIR/.venv/bin/python"

if [[ -n "$WHEEL_PATH" ]]; then
    "$UV_EXE" pip install --python "$VENV_PY" "$WHEEL_PATH" >/dev/null
else
    if [[ -n "$INDEX_URL" ]]; then
        "$UV_EXE" pip install --python "$VENV_PY" --index-url "$INDEX_URL" "$PACKAGE_SPEC" >/dev/null
    else
        "$UV_EXE" pip install --python "$VENV_PY" "$PACKAGE_SPEC" >/dev/null
    fi
fi

"$VENV_PY" "$REPO_ROOT/scripts/release/python-smoke.py"
