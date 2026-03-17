# Release Process

This repository publishes four stable release artifacts from the same release tag:

- `relateby-pattern` on crates.io
- `relateby-gram` on crates.io
- `@relateby/pattern` on npm
- `relateby-pattern` on PyPI

Python imports remain `relateby.pattern` and `relateby.gram`. `relateby` is a namespace only, not a published PyPI project.

## Release flow

1. Run the local release-prep script from a clean, up-to-date `main` checkout:
   ```bash
   ./scripts/new-release.sh 0.2.0
   ```
2. The script:
   - verifies `main`, a clean worktree, and `origin/main` sync
   - updates release-managed versions
   - runs `./scripts/ci-local.sh --release`
   - creates a release commit
   - creates annotated tag `v0.2.0`
3. Push the prepared release:
   ```bash
   git push origin main --follow-tags
   ```
4. GitHub Actions validates the release again and publishes automatically.

## Release-managed versions

The release script treats these files as the authoritative version set:

- `Cargo.toml`
- `crates/gram-codec/Cargo.toml`
- `typescript/@relateby/pattern/package.json`
- `python/relateby/pyproject.toml`

## Local validation

Standard validation:

```bash
./scripts/ci-local.sh
```

Release-grade validation:

```bash
./scripts/ci-local.sh --release
```

Release mode checks:

- Rust fmt, clippy, build, tests, docs
- WASM workspace build
- `cargo publish --dry-run` for both crates
- `@relateby/pattern` build, runtime tests, public export inventory, public consumer typecheck, pack, and packed-artifact smoke install
- combined Python wheel build, public stub validation, metadata check, packaged-stub verification, and wheel smoke install

Maintainer notes:

- Native Rust validation excludes `pattern-wasm`; that crate is validated in the dedicated WASM build because it is a wasm-target package.
- `relateby-gram` depends on `relateby-pattern`, so `cargo publish --dry-run` for the gram crate only works once the matching pattern version exists on crates.io. Before that, `scripts/ci-local.sh --release` falls back to `cargo package --list` for `relateby-gram` after the `relateby-pattern` dry-run.

## Stable tags only

- Valid publish tags are `v<major>.<minor>.<patch>`
- npm publishing is stable-only
- non-stable tags must not publish

## Registry credentials

Do not commit credentials. Configure GitHub Actions secrets instead:

- `CARGO_REGISTRY_TOKEN`
- `NPM_TOKEN`
- `PYPI_API_TOKEN`

The publish workflow reads tokens from Actions secrets and performs all registry writes remotely.

## Publish order

The publish workflow validates first, then publishes:

1. `relateby-pattern` crate
2. `relateby-gram` crate
3. `@relateby/pattern`
4. `relateby-pattern` Python wheel

## Verification

After publish:

- Verify crates.io and docs.rs for `relateby-pattern` and `relateby-gram`
- Verify npm:
  ```bash
  npm view @relateby/pattern@0.2.0
  ```
- Verify PyPI:
  ```bash
  pip install relateby-pattern==0.2.0
  python -c "import relateby.pattern; import relateby.gram; print('OK')"
  ```
- Verify the published Python wheel includes `relateby/pattern/__init__.pyi`, `relateby/gram/__init__.pyi`, and `relateby/py.typed`

## Public Package Boundary

The release gate for this feature treats these as the supported developer surfaces:

- `@relateby/pattern`
- `relateby.pattern`
- `relateby.gram`

Release validation is expected to fail if docs, stubs, runtime exports, or smoke-install workflows require internal modules such as `wasm/`, `wasm-node/`, `pattern_core`, or `gram_codec`.

## Recovery

- If local release preparation fails, fix the issue and rerun `./scripts/new-release.sh <version>`.
- If remote validation fails, no registry publish should occur; fix forward and cut a new tag.
- If one immutable registry publish succeeds and a later publish step fails, do not attempt to republish the same version. Follow up with a new patch release.
