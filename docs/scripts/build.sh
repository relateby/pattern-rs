#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

echo "=== pattern-rs documentation build ==="
echo ""

# Step 1: Rust API reference
echo "Step 1: Generating Rust API reference (cargo doc)..."
cargo doc --workspace --no-deps 2>&1
rm -rf "${REPO_ROOT}/docs/public/reference/rust"
mkdir -p "${REPO_ROOT}/docs/public/reference/rust"
cp -r "${REPO_ROOT}/target/doc/." "${REPO_ROOT}/docs/public/reference/rust/"
echo "  → docs/public/reference/rust/ generated"

# Step 2: Python API reference
echo "Step 2: Generating Python API reference (pdoc)..."
VENV_PATH="${REPO_ROOT}/python/packages/relateby/.venv"
if [[ ! -d "${VENV_PATH}" ]]; then
  echo "ERROR: Python virtualenv not found at ${VENV_PATH}"
  echo "       Run: cd python/packages/relateby && uv venv --python 3.13 .venv && uv pip install '.[dev]'"
  exit 1
fi
source "${VENV_PATH}/bin/activate"
(cd "${REPO_ROOT}/python/packages/relateby" && \
  pdoc relateby --output-dir "${REPO_ROOT}/docs/public/reference/python")
echo "  → docs/public/reference/python/ generated"

# Step 3: TypeScript API reference
echo "Step 3: Generating TypeScript API reference (TypeDoc)..."
(cd "${REPO_ROOT}" && npx --prefix docs typedoc --options typedoc.json)
echo "  → docs/public/reference/ts/ generated"

# Ensure output directory exists for LLM files
mkdir -p "${REPO_ROOT}/docs/public"

# Step 4: Generate llms.txt
echo "Step 4: Generating llms.txt..."
LLMS_TXT="${REPO_ROOT}/docs/public/llms.txt"

generate_llms_txt() {
  local out="$1"

  echo "# pattern-rs" > "${out}"
  echo "" >> "${out}"
  echo "> pattern-rs provides the Pattern<V> data structure — a value paired with an" >> "${out}"
  echo "> ordered list of elements, each itself a Pattern<V>. This is the decorated" >> "${out}"
  echo "> sequence model, implemented in Rust with bindings for Python and TypeScript." >> "${out}"
  echo "> The library includes Gram notation, a human-readable serialisation format" >> "${out}"
  echo "> for patterns, bidirectionally parsed and serialised by the gram-codec crate." >> "${out}"
  echo "" >> "${out}"
  echo "## Explanations" >> "${out}"
  echo "" >> "${out}"

  for f in $(ls "${REPO_ROOT}/docs/explanations/"*.md | sort); do
    slug=$(basename "${f}" .md)
    [[ "${slug}" == "index" ]] && continue
    title=$(grep -m1 '^# ' "${f}" | sed 's/^# //')
    echo "- [${title}](/explanations/${slug})" >> "${out}"
  done

  echo "" >> "${out}"
  echo "## Guides" >> "${out}"
  echo "" >> "${out}"

  for f in $(ls "${REPO_ROOT}/docs/guides/"*.md | sort); do
    slug=$(basename "${f}" .md)
    [[ "${slug}" == "index" ]] && continue
    title=$(grep -m1 '^# ' "${f}" | sed 's/^# //')
    echo "- [${title}](/guides/${slug})" >> "${out}"
  done

  echo "" >> "${out}"
  echo "## API Reference" >> "${out}"
  echo "" >> "${out}"
  echo "- [Rust API](/reference/rust/)" >> "${out}"
  echo "- [Python API](/reference/python/)" >> "${out}"
  echo "- [TypeScript API](/reference/ts/)" >> "${out}"
}

generate_llms_txt "${LLMS_TXT}"
echo "  → docs/public/llms.txt generated"

# Step 5: Generate llms-full.txt
echo "Step 5: Generating llms-full.txt..."
LLMS_FULL="${REPO_ROOT}/docs/public/llms-full.txt"
: > "${LLMS_FULL}"

strip_frontmatter() {
  local file="$1"
  # Strip YAML frontmatter (lines between first --- pair at top of file)
  awk 'BEGIN{fm=0; done=0} /^---$/ && !done { if(fm==0){fm=1; next} else {fm=0; done=1; next} } !fm{print}' "${file}"
}

first=1
for section_dir in explanations guides; do
  for f in $(ls "${REPO_ROOT}/docs/${section_dir}/"*.md | sort); do
    slug=$(basename "${f}" .md)
    [[ "${slug}" == "index" ]] && continue
    if [[ "${first}" -eq 0 ]]; then
      printf '\n---\n\n' >> "${LLMS_FULL}"
    fi
    strip_frontmatter "${f}" >> "${LLMS_FULL}"
    first=0
  done
done

echo "  → docs/public/llms-full.txt generated"

# Step 6: Build VitePress site
echo "Step 6: Building VitePress site..."
(cd "${REPO_ROOT}" && npx --prefix docs vitepress build docs)
echo "  → docs/.vitepress/dist/ generated"

echo ""
echo "=== Build complete ==="
exit 0
