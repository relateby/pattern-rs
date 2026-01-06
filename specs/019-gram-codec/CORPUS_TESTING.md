# Tree-Sitter-Gram Corpus Testing

**Feature**: 019-gram-codec  
**Created**: 2026-01-06  
**Status**: Documentation

## Overview

The gram-codec uses the official tree-sitter-gram test corpus to validate parser correctness. The corpus is included as a **git submodule** to ensure:

- ✅ **CI/CD compatibility**: Automated tests work in GitHub Actions and other CI systems
- ✅ **Version pinning**: Tests against specific, known-good corpus version
- ✅ **Single source of truth**: No duplication or manual syncing required
- ✅ **Easy updates**: Pull latest corpus changes with standard git commands

## Git Submodule Setup

### Initial Setup (Maintainers Only)

This has already been done for the repository:

```bash
# Add tree-sitter-gram as a submodule
git submodule add https://github.com/gram-data/tree-sitter-gram.git external/tree-sitter-gram
git commit -m "Add tree-sitter-gram as submodule for corpus tests"
```

### Developer Setup

When you clone `gram-rs`, initialize the submodule:

#### Option 1: Clone with Submodules (Recommended)

```bash
git clone --recurse-submodules https://github.com/gram-data/gram-rs.git
cd gram-rs
cargo test --package gram-codec
```

#### Option 2: Initialize After Clone

```bash
git clone https://github.com/gram-data/gram-rs.git
cd gram-rs

# Initialize and fetch submodules
git submodule update --init --recursive

# Now run tests
cargo test --package gram-codec
```

### Checking Submodule Status

```bash
# Check if submodule is initialized
git submodule status

# Should show:
#  <commit-hash> external/tree-sitter-gram (v0.2.0)
# (leading space means initialized)

# If you see a minus sign (-), it's not initialized:
# -<commit-hash> external/tree-sitter-gram
# Run: git submodule update --init --recursive
```

### Updating to Latest Corpus

To update the submodule to the latest tree-sitter-gram version:

```bash
# Navigate to submodule directory
cd external/tree-sitter-gram

# Pull latest changes
git checkout main
git pull origin main

# Return to repo root and commit the update
cd ../..
git add external/tree-sitter-gram
git commit -m "Update tree-sitter-gram submodule to latest"
```

## CI/CD Configuration

### GitHub Actions

Update `.github/workflows/*.yml` to initialize submodules:

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
        with:
          submodules: true  # or 'recursive' for nested submodules
      
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run tests
        run: cargo test --workspace
```

### Other CI Systems

**GitLab CI**:
```yaml
variables:
  GIT_SUBMODULE_STRATEGY: recursive
```

**CircleCI**:
```yaml
jobs:
  test:
    steps:
      - checkout
      - run: git submodule update --init --recursive
```

**Travis CI**:
```yaml
git:
  submodules: true
```

## Corpus Test Structure

### Corpus File Format

Tree-sitter corpus tests use a special format with `===` separators:

```
===================================
Test Name
===================================
input_gram_notation
---
(expected_parse_tree)
```

### Example Corpus File

From `external/tree-sitter-gram/test/corpus/identifiers.txt`:

```
===================================
simple identifier
===================================
(hello)
---
(gram_pattern
  (node_pattern
    (subject
      (identifier (symbol)))))
```

### Corpus Test Paths

- **Corpus directory**: `external/tree-sitter-gram/test/corpus/`
- **Corpus files**: `*.txt` (27 files covering all grammar features)
- **Test categories**:
  - `annotation.txt` - Annotation patterns
  - `array_properties.txt` - Array value types
  - `comments.txt` - Comment handling
  - `double_arrows.txt` - Bidirectional relationships
  - `identifiers.txt` - Identifier formats
  - `relationships.txt` - Relationship patterns
  - ... and more

## Corpus Test Implementation

### Phase 5 Tasks (Deferred)

Corpus integration is planned for Phase 5:

- **T059**: Create `crates/gram-codec/tests/corpus_tests.rs`
- **T060**: Implement corpus file parser (reads `===` format)
- **T061**: Implement `load_corpus_tests()` function
- **T062**: Implement `run_corpus_tests()` function
- **T063**: Test all 27 corpus files

### Conditional Testing Approach

Tests gracefully handle missing submodule:

```rust
#[cfg(test)]
mod corpus_tests {
    const CORPUS_PATH: &str = "../../external/tree-sitter-gram/test/corpus";
    
    fn corpus_available() -> bool {
        std::path::Path::new(CORPUS_PATH).exists()
    }
    
    #[test]
    fn test_corpus_annotation() {
        if !corpus_available() {
            eprintln!("⚠️  Skipping corpus tests (tree-sitter-gram submodule not initialized)");
            eprintln!("    Run: git submodule update --init --recursive");
            return;
        }
        
        // ... actual corpus test
    }
}
```

This allows:
- ✅ Basic development without corpus (unit tests still work)
- ✅ Full validation when corpus is available
- ✅ Clear instructions when corpus is missing

## Troubleshooting

### Problem: Tests Fail with "Corpus not found"

**Solution**: Initialize the submodule

```bash
git submodule update --init --recursive
```

### Problem: Submodule Detached HEAD

**Solution**: Checkout main branch in submodule

```bash
cd external/tree-sitter-gram
git checkout main
cd ../..
```

### Problem: CI Fails with Submodule Error

**Solution**: Ensure `submodules: true` in checkout action

```yaml
- uses: actions/checkout@v3
  with:
    submodules: true
```

### Problem: Submodule Out of Date

**Solution**: Update submodule to latest

```bash
cd external/tree-sitter-gram
git pull origin main
cd ../..
git add external/tree-sitter-gram
git commit -m "Update tree-sitter-gram submodule"
```

## Benefits of Submodule Approach

### vs. Peer Directory (`../tree-sitter-gram/`)

| Aspect | Submodule | Peer Directory |
|--------|-----------|----------------|
| CI/CD | ✅ Works automatically | ❌ Requires manual setup |
| Version control | ✅ Pinned to specific commit | ❌ Can drift |
| Setup complexity | ✅ One command | ❌ Manual clone |
| Team consistency | ✅ Everyone uses same version | ❌ May differ |
| Documentation | ✅ Standard git practice | ❌ Custom instructions |

### vs. Vendoring (Copy Files)

| Aspect | Submodule | Vendoring |
|--------|-----------|-----------|
| Updates | ✅ `git pull` in submodule | ❌ Manual copy |
| Size | ✅ Referenced, not duplicated | ❌ Increases repo size |
| Maintenance | ✅ Automatic | ❌ Manual sync |
| Authority | ✅ Always current | ❌ Can become stale |

### vs. Conditional Testing Only

| Aspect | Submodule | Conditional |
|--------|-----------|-------------|
| CI coverage | ✅ 100% corpus coverage | ❌ Corpus tests skipped |
| Developer experience | ✅ All tests available | ⚠️ Some tests skipped |
| Confidence | ✅ High (full corpus) | ⚠️ Medium (partial) |

## Summary

The git submodule approach provides:

1. **Reliable CI/CD**: Tests run automatically with full corpus coverage
2. **Version pinning**: Tests against known-good corpus version
3. **Easy setup**: Single command for developers (`git submodule update --init`)
4. **Standard practice**: Well-understood git workflow
5. **Low maintenance**: Pull updates with standard git commands
6. **Authoritative source**: Always references official tree-sitter-gram

This ensures all contributors and CI systems test against the same authoritative grammar specification.

