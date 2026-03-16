# Quickstart: Preparing and Publishing a Stable Release

**Feature**: 037-publishing-workflow  
**Audience**: Maintainers releasing stable versions

## Prerequisites

- You are on `main` and `origin/main` is up to date locally.
- Your working tree is clean.
- GitHub Actions publishing credentials or trusted-publisher configuration are already set for crates.io, npm, and PyPI.
- Local release tooling is installed: Rust, Node.js/npm, Python, `wasm-pack`, `maturin`, and `twine`.

## Prepare the release

1. Run:
   ```bash
   ./scripts/new-release.sh 0.2.0
   ```
2. Confirm the script:
   - updates release-managed versions
   - runs the full release validation suite
   - creates one release commit
   - creates annotated tag `v0.2.0`

## Trigger publishing

1. Push the release commit and tag:
   ```bash
   git push origin main --follow-tags
   ```
2. Watch the tag-triggered GitHub Actions publish workflow.
3. The workflow should:
   - re-run release validation
   - publish `relateby-pattern`
   - publish `relateby-gram`
   - publish `@relateby/pattern`
   - publish the combined Python distribution `relateby-pattern`

## Verify the release

1. Check crates.io and docs.rs for the Rust crates.
2. Check npm:
   ```bash
   npm view @relateby/pattern@0.2.0
   ```
3. Check PyPI:
   ```bash
   pip install relateby-pattern==0.2.0
   python -c "import relateby.pattern; import relateby.gram; print('OK')"
   ```

## Expected release policy

- Stable tags only: `v<major>.<minor>.<patch>`
- npm publishes only stable releases
- `relateby` is a Python namespace/import root, not a published distribution
- All remote publishing happens in GitHub Actions, not from the local release script

## Recovery notes

- If local validation fails, fix the issue and rerun `scripts/new-release.sh`.
- If remote validation fails, no artifact should publish; fix forward and create a new release tag.
- If one immutable registry publish succeeds and a later one fails, follow the documented recovery path and issue a new patch release if needed.
