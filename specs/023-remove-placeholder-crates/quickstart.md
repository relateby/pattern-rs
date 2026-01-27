# Quickstart: Remove Unused Placeholder Crates

**Feature**: 023-remove-placeholder-crates  
**Date**: 2026-01-27

## Overview

This guide provides step-by-step instructions for removing the three unused placeholder crates from the workspace: `pattern-store`, `pattern-ops`, and `pattern-wasm`.

## Prerequisites

- Rust toolchain installed (1.70.0+)
- Cargo workspace access
- Git repository access

## Steps

### 1. Verify Current State

Before removal, verify the workspace builds successfully:

```bash
# Build the workspace
cargo build --workspace

# Run all tests
cargo test --workspace

# Verify placeholder crates exist
ls -d crates/pattern-store crates/pattern-ops crates/pattern-wasm
```

### 2. Remove Crate Directories

Remove the three placeholder crate directories:

```bash
# Remove pattern-store
rm -rf crates/pattern-store

# Remove pattern-ops
rm -rf crates/pattern-ops

# Remove pattern-wasm
rm -rf crates/pattern-wasm
```

Or using git (recommended for tracking):

```bash
git rm -r crates/pattern-store crates/pattern-ops crates/pattern-wasm
```

### 3. Verify Workspace Configuration

The workspace uses wildcard matching (`members = ["crates/*", "benches"]`), so no manual Cargo.toml updates are needed. Verify the configuration:

```bash
# Check workspace members
cargo tree --workspace --depth 0
```

Expected output should show only active crates (gram-codec, pattern-core) and benches.

### 4. Verify Build Success

After removal, verify everything still works:

```bash
# Build workspace
cargo build --workspace

# Run tests
cargo test --workspace

# Check for any references to removed crates
grep -r "pattern-store\|pattern-ops\|pattern-wasm" . --exclude-dir=target --exclude-dir=.git
```

The grep command should only find references in documentation files (which will be updated separately).

### 5. Update Documentation

Search and update documentation references:

```bash
# Find all references
grep -r "pattern-store\|pattern-ops\|pattern-wasm" . \
  --exclude-dir=target \
  --exclude-dir=.git \
  --exclude-dir=specs/023-remove-placeholder-crates
```

Update found references to:
- Remove mentions of these crates
- Clarify that they were placeholders that have been removed
- Update architecture diagrams if present

### 6. Final Verification

Complete verification checklist:

- [ ] Workspace builds successfully: `cargo build --workspace`
- [ ] All tests pass: `cargo test --workspace`
- [ ] No code references remain: `grep -r "pattern-store\|pattern-ops\|pattern-wasm" crates/`
- [ ] Documentation updated: Review and update any docs mentioning these crates
- [ ] Git status clean: `git status` shows only expected changes

## Expected Results

After completion:

- **Workspace members**: Only `gram-codec`, `pattern-core`, and `benches` remain
- **Build time**: Should be slightly faster (fewer crates to compile)
- **Codebase clarity**: No confusion about placeholder functionality
- **Git history**: Preserved (can view removed files via `git log`)

## Troubleshooting

### Build fails after removal

**Symptom**: `cargo build` fails with dependency errors

**Solution**: 
1. Check if any Cargo.toml files still reference removed crates
2. Run `cargo clean` and rebuild
3. Verify workspace member configuration

### Tests fail after removal

**Symptom**: Tests fail with import errors

**Solution**:
1. Search for any test files importing removed crates
2. Verify no test code depends on placeholder crates
3. Run `cargo test --workspace` to see specific errors

### Documentation references remain

**Symptom**: Grep still finds references

**Solution**:
1. Review each reference context
2. Update or remove as appropriate
3. Some references in historical specs may be left as-is (documenting past decisions)

## Next Steps

After successful removal:

1. Commit changes: `git commit -m "Remove unused placeholder crates (pattern-store, pattern-ops, pattern-wasm)"`
2. Verify CI passes: Push to remote and check CI pipeline
3. Update project documentation: Ensure README and architecture docs reflect current state
