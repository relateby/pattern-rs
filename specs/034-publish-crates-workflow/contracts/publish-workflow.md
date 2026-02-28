# Publish Workflow Contract

**Feature**: 034-publish-crates-workflow  
**Artifact**: GitHub Actions workflow triggered by version tags

## Trigger

- **Event**: `push` to tags matching `v*`.
- **Inputs**: None (tag ref and commit SHA are provided by GitHub).

## Secrets (required)

| Secret | Purpose |
|--------|--------|
| `CARGO_REGISTRY_TOKEN` | crates.io API token for `cargo publish` |

## Steps (contract)

1. **Checkout** – Repository at the tag ref, with submodules if needed.
2. **Setup Rust** – Stable toolchain (and optionally MSRV) for build/test/publish.
3. **Cache** – Cargo registry, git, and target directory.
4. **Build** – `cargo build --workspace` (or only publishable packages).
5. **Test** – `cargo test --workspace`.
6. **Lint** – `cargo clippy --workspace -- -D warnings`.
7. **Format** – `cargo fmt --all -- --check` (optional but recommended).
8. **Publish relateby-pattern** – `cargo publish -p relateby-pattern --token ${{ secrets.CARGO_REGISTRY_TOKEN }}`.
9. **Optional delay** – Short wait for crates.io to index relateby-pattern (e.g. 30s).
10. **Publish relateby-gram** – `cargo publish -p relateby-gram --token ${{ secrets.CARGO_REGISTRY_TOKEN }}`.

## Success

- All steps complete; both crates appear on crates.io at the version derived from the tag.
- docs.rs will build docs for the new versions automatically.

## Failure

- Any step fails → workflow stops; no publish (or no further publish) after that step.
- Duplicate version → crates.io returns an error; workflow fails.
- Partial publish (e.g. relateby-pattern ok, relateby-gram fails) → recovery documented in release instructions.

## Out of scope

- Building or uploading docs manually (docs.rs builds from published crates).
- Publishing to any registry other than crates.io.
- Version bumping or changelog generation (handled outside this workflow).
