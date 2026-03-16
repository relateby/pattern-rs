#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(CDPATH="" cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

if ! command -v actionlint >/dev/null 2>&1; then
    echo "[check-workflows] ERROR: actionlint is required but was not found on PATH" >&2
    exit 1
fi

cd "$REPO_ROOT"

validate_action_refs() {
    local -a action_uses=()
    local action_use action_path owner repo repo_spec ref cache_key
    local status=0
    local validated_refs=""

    while IFS= read -r action_use; do
        action_uses+=("$action_use")
    done < <(
        ruby <<'RUBY'
require "set"
require "yaml"

uses = Set.new

extract_uses = lambda do |value|
  case value
  when Hash
    value.each do |key, child|
      uses << child.to_s if key.to_s == "uses"
      extract_uses.call(child)
    end
  when Array
    value.each { |child| extract_uses.call(child) }
  end
end

Dir[".github/workflows/*.{yml,yaml}"].sort.each do |path|
  data = YAML.load_file(path)
  extract_uses.call(data)
end

uses.to_a.sort.each { |value| puts value }
RUBY
    )

    for action_use in "${action_uses[@]}"; do
        [[ -z "$action_use" ]] && continue
        [[ "$action_use" == ./* ]] && continue
        [[ "$action_use" == docker://* ]] && continue

        if [[ "$action_use" != *@* ]]; then
            echo "[check-workflows] ERROR: action ref is missing @version: $action_use" >&2
            status=1
            continue
        fi

        action_path="${action_use%@*}"
        ref="${action_use##*@}"
        IFS=/ read -r owner repo _ <<<"$action_path"
        if [[ -z "$owner" || -z "$repo" ]]; then
            echo "[check-workflows] ERROR: could not determine repository for action ref: $action_use" >&2
            status=1
            continue
        fi

        repo_spec="$owner/$repo"
        cache_key="$repo_spec@$ref"
        if [[ "$validated_refs" == *$'\n'"$cache_key"$'\n'* ]]; then
            continue
        fi
        validated_refs+=$'\n'"$cache_key"$'\n'

        if [[ "$ref" =~ ^[0-9a-fA-F]{40}$ ]]; then
            continue
        fi

        if ! git ls-remote --exit-code --refs "https://github.com/${repo_spec}.git" "refs/tags/${ref}" "refs/heads/${ref}" >/dev/null 2>&1; then
            echo "[check-workflows] ERROR: could not resolve action ref ${repo_spec}@${ref}" >&2
            status=1
        fi
    done

    return "$status"
}

echo "[check-workflows] Running actionlint"
actionlint -color

echo "[check-workflows] Validating external action refs"
validate_action_refs

if command -v shellcheck >/dev/null 2>&1; then
    echo "[check-workflows] Running shellcheck on workflow helper scripts"
    shellcheck -x -e SC1091,SC2329 scripts/*.sh scripts/release/*.sh
else
    echo "[check-workflows] shellcheck not found; skipping shell script lint"
fi
