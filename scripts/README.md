# Development Scripts

This directory contains utility scripts for development and CI/CD.

## ci-local.sh

Run all CI checks locally before pushing to catch issues early.

### Usage

```bash
./scripts/ci-local.sh
```

Release-grade validation:

```bash
./scripts/ci-local.sh --release
```

### What it checks

1. **Format check**: Verifies code formatting with `cargo fmt`
2. **Clippy lint**: Runs `cargo clippy` with strict warnings (`-D warnings`)
3. **Native build / tests / docs**: Validates the Rust workspace
4. **WASM build**: Builds the workspace for `wasm32-unknown-unknown`
5. **npm package validation**: Builds, tests, packs, and smoke-installs `@relateby/pattern`
6. **Python package validation**: Builds, checks, and smoke-installs the combined `relateby-pattern` wheel
7. **Cargo dry-runs**: Included in `--release` mode

### Exit codes

- `0`: All checks passed
- `1`: One or more checks failed

### Tips

- Run this before every commit to avoid CI failures
- Use `--release` before cutting a tag
- All output is logged to `/tmp/ci-check.log` for debugging

### Alternatives

- **`act` with Docker**: For full workflow simulation using Docker containers
- **`act` without Docker**: Use `act -P ubuntu-latest=-self-hosted` to run workflows directly on your host (no Docker required)
- See [.github/workflows/README.md](../.github/workflows/README.md) for more details on using `act`

### Example output

```
🔨 Running local CI checks...

Running Format check... ✓

Running Clippy lint... ✓

Running Native build... ✓

Checking WASM target... ✓
Running WASM build... ✓

Running Tests... ✓

==========================================
All checks passed!
```

## new-release.sh

Prepare a stable release from `main`:

```bash
./scripts/new-release.sh 0.2.0
./scripts/new-release.sh --push 0.2.0
```

This script updates the release-managed versions, runs release validation, creates the release commit, and creates annotated tag `v0.2.0`.

