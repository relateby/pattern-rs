# Development Scripts

This directory contains utility scripts for development and CI/CD.

## ci-local.sh

Run all CI checks locally before pushing to catch issues early.

### Usage

```bash
./scripts/ci-local.sh
```

### What it checks

1. **Format check**: Verifies code formatting with `cargo fmt`
2. **Clippy lint**: Runs `cargo clippy` with strict warnings (`-D warnings`)
3. **Native build**: Builds all workspace crates for native target
4. **WASM build**: Builds pattern-core, pattern-ops, and gram-codec for WASM (if target is installed)
5. **Tests**: Runs all workspace tests

### Exit codes

- `0`: All checks passed
- `1`: One or more checks failed

### Tips

- Run this before every commit to avoid CI failures
- If WASM target is not installed, that check is skipped (with a warning)
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

