# Research: Publish Crates, Docs, Examples, and Tag-Based Workflow

**Feature**: 034-publish-crates-workflow  
**Date**: 2025-02-28

## 1. Registry and Publish Tooling

**Decision**: Use crates.io as the public registry and `cargo publish` as the publish mechanism. Use `cargo publish --dry-run` for local validation before real publish.

**Rationale**: pattern-rs is a Rust workspace; crates.io is the standard Rust registry. No alternative registry was requested. Dry-run is the standard way to verify metadata and packaging without uploading.

**Alternatives considered**: Private registry (not in scope); manual upload (error-prone; spec requires automation).

---

## 2. Tag Format and Workflow Trigger

**Decision**: Use semantic version tags (e.g. `v0.1.0`) on the default branch to trigger the publish workflow. Workflow triggers on `push: tags: ['v*']` so only tags matching `v*` run publish.

**Rationale**: Matches common Rust/crates.io practice; version tag maps to release version; filtering on `v*` avoids running on non-version tags.

**Alternatives considered**: `v*.*.*` (more strict; not required for initial release); tag-per-crate (rejected; spec expects one release flow for the workspace).

---

## 3. Publish Order and Dependency Versioning

**Decision**: Publish relateby-pattern first, then relateby-gram. In gram-codec’s Cargo.toml, use a version dependency for the pattern crate when publishing (e.g. `relateby-pattern = { path = "../pattern-core", version = "0.1.0" }`) so that after publish the registry dependency resolves. Package names in Cargo.toml are `relateby-pattern` and `relateby-gram`; directory names remain `pattern-core` and `gram-codec`.

**Rationale**: gram-codec depends on pattern-core; crates.io requires version bounds; path is for local dev, version for published crate. The relateby- prefix groups crates under a single namespace on crates.io. Alternatives: publish script that temporarily rewrites Cargo.toml (fragile); workspace version inheritance already in use.

---

## 4. Documentation Hosting

**Decision**: Rely on docs.rs to build and host API documentation for each published crate. Set `documentation` in Cargo.toml to the docs.rs URL for each crate (e.g. `https://docs.rs/relateby-pattern`, `https://docs.rs/relateby-gram`). No separate doc-publish step in the workflow.

**Rationale**: docs.rs builds docs from published crates automatically; no extra CI job or hosting. Spec requires “API documentation for each published version is available at the documented URL.”

**Alternatives considered**: Custom doc job uploading to GitHub Pages (extra complexity); building docs in CI and uploading elsewhere (not needed if docs.rs is used).

---

## 5. Examples Inclusion

**Decision**: Include examples in the published crate when they live under the crate directory (e.g. `crates/pattern-core/examples/` or `[[example]]` with paths under the crate). For examples that currently live under `examples/` at workspace root, either move them into each crate’s `examples/` or document them as “clone and run” in the crate README/quickstart; fix any `[[example]]` paths that point outside the package so they are included in the tarball.

**Rationale**: crates.io packages only include the crate directory; paths like `../../examples/` are outside the package and are dropped. Spec requires “examples available to users” and “run documented example commands”; inclusion in the package is the standard approach.

**Alternatives considered**: Keeping examples only at workspace root and linking from README (acceptable if documented; less convenient than `cargo run --example` from the crate).

---

## 6. Secrets and Credentials

**Decision**: Use a single crates.io token stored as a GitHub Actions secret (e.g. `CARGO_REGISTRY_TOKEN`). Workflow uses `cargo publish --token ${{ secrets.CARGO_REGISTRY_TOKEN }}`. Document that the token is created at crates.io and added under repository Settings → Secrets.

**Rationale**: crates.io supports token-based publish; GitHub Secrets is the standard for CI. One token can publish all crates the workflow is allowed to publish.

**Alternatives considered**: Per-crate tokens (unnecessary); keyless or OIDC (crates.io does not require it for this setup).

---

## 7. Workflow Failure and Partial Publish

**Decision**: Run build, test, and lint before any publish. On failure, exit without publishing. Document that if relateby-pattern publishes and relateby-gram fails, the maintainer can re-run the workflow (gram-codec step only) or publish relateby-gram manually with the same version; avoid re-publishing relateby-pattern (crates.io rejects same version twice).

**Rationale**: Spec requires “workflow must not publish when validation fails” and “handle partial publish” with clear recovery. Fail-fast plus documented recovery satisfies both.

---

## 8. Doc Warnings and Build Warnings

**Decision**: Treat doc build as a validation step: run `cargo doc --no-deps` (or equivalent) for each publishable crate in CI; allow warnings initially but document the choice. Prefer fixing doc warnings so that `cargo doc` and docs.rs build cleanly; if not, document “publish with doc warnings” in release instructions.

**Rationale**: Spec edge case: “documentation build has warnings or non-fatal errors” — document the chosen behavior. Fixing warnings is preferred for maintainability.

---

## 9. Duplicate Tag / Idempotency

**Decision**: If the same version tag is pushed again, the workflow runs again. crates.io rejects a second publish of the same version with a clear error. Workflow will fail at the publish step; document that re-publishing the same version is not supported and that a new release requires a new version and tag.

**Rationale**: Spec asks for “duplicate appropriately (idempotent skip or clear error)”. crates.io behavior gives “clear error”; no need for custom idempotent skip logic.
