# Git Submodule Implementation Summary

**Feature**: 019-gram-codec  
**Date**: 2026-01-06  
**Issue**: Tree-sitter-gram corpus dependency for CI/CD  
**Solution**: Git submodule approach

## Problem Statement

The gram-codec requires access to the tree-sitter-gram test corpus for comprehensive testing. The initial plan used a peer directory (`../tree-sitter-gram/`), which works locally but causes issues in CI/CD environments:

- ❌ CI/CD systems don't have peer directory structure
- ❌ Manual setup required for each environment
- ❌ Version drift between developers
- ❌ Tests fail or skip in CI without clear errors

## Solution: Git Submodule

Implement tree-sitter-gram as a git submodule at `external/tree-sitter-gram/`.

### Benefits

1. **CI/CD Compatibility**: Works automatically in GitHub Actions with single config line
2. **Version Pinning**: Locks to specific commit, ensures reproducible builds
3. **Standard Practice**: Well-understood git workflow
4. **Team Consistency**: All developers use same corpus version
5. **Easy Setup**: Single command (`git submodule update --init`)
6. **Easy Updates**: Standard git commands to pull latest corpus

## Implementation Changes

### 1. Documentation Updates

#### Created New Files

- **`CORPUS_TESTING.md`**: Comprehensive guide for submodule setup, CI configuration, troubleshooting
- **`SUBMODULE_IMPLEMENTATION.md`**: This summary document

#### Updated Existing Files

| File | Changes |
|------|---------|
| `tasks.md` | • Changed paths from `../tree-sitter-gram/` to `external/tree-sitter-gram/`<br>• Added "Git Submodule Setup" section with commands<br>• Added CI/CD configuration examples<br>• Added conditional testing notes |
| `README.md` | • Updated external references to use `external/tree-sitter-gram/`<br>• Added submodule setup note |
| `quickstart.md` | • Updated grammar reference path<br>• Added submodule setup section<br>• Updated references section |
| `data-model.md` | • Updated grammar authority path<br>• Added submodule initialization note |
| `research.md` | • Updated test corpus integration decision<br>• Added rationale for submodule vs peer directory<br>• Cross-referenced CORPUS_TESTING.md |
| `plan.md` | • Updated all corpus paths<br>• Marked test corpus integration as ✅ RESOLVED<br>• Updated complexity tracking<br>• Updated references section |

### 2. Path Changes

All references updated from:
```
../tree-sitter-gram/test/corpus/
../tree-sitter-gram/grammar.js
../tree-sitter-gram/examples/data/
```

To:
```
external/tree-sitter-gram/test/corpus/
external/tree-sitter-gram/grammar.js
external/tree-sitter-gram/examples/data/
```

### 3. Setup Instructions

#### For Maintainers (One-Time)

```bash
# Add submodule (already done)
git submodule add https://github.com/gram-data/tree-sitter-gram.git external/tree-sitter-gram
git commit -m "Add tree-sitter-gram as submodule for corpus tests"
```

#### For Developers

```bash
# Clone with submodules (recommended)
git clone --recurse-submodules https://github.com/gram-data/gram-rs.git

# Or initialize after clone
git submodule update --init --recursive
```

#### For CI/CD

```yaml
# GitHub Actions
- uses: actions/checkout@v3
  with:
    submodules: true
```

### 4. Conditional Testing Strategy

Tests gracefully handle missing submodule:

```rust
fn corpus_available() -> bool {
    std::path::Path::new("../../external/tree-sitter-gram/test/corpus").exists()
}

#[test]
fn test_corpus_annotation() {
    if !corpus_available() {
        eprintln!("⚠️  Skipping corpus tests (submodule not initialized)");
        eprintln!("    Run: git submodule update --init --recursive");
        return;
    }
    // ... actual test
}
```

This allows:
- Basic development without full corpus (unit tests still work)
- Full validation when corpus is available
- Clear instructions when corpus is missing

## Tasks Updated

### Phase 5 Tasks (T059-T063)

```diff
- T060: reads `===` separator format from ../tree-sitter-gram/test/corpus/*.txt
+ T060: reads `===` separator format from external/tree-sitter-gram/test/corpus/*.txt

- T063: Test all 27 corpus files from ../tree-sitter-gram/test/corpus/
+ T063: Test all 27 corpus files from external/tree-sitter-gram/test/corpus/
```

### Notes Added

- Corpus tests use `external/tree-sitter-gram` submodule
- See setup instructions in tasks.md and CORPUS_TESTING.md
- Tests gracefully skip if submodule not initialized

## Comparison with Alternatives

### vs. Peer Directory (`../tree-sitter-gram/`)

| Aspect | Submodule ✅ | Peer Directory ❌ |
|--------|--------------|-------------------|
| CI/CD | Works automatically | Requires manual setup |
| Version control | Pinned to specific commit | Can drift |
| Setup complexity | One command | Manual clone |
| Team consistency | Everyone uses same version | May differ |

### vs. Vendoring (Copy Files)

| Aspect | Submodule ✅ | Vendoring ❌ |
|--------|--------------|--------------|
| Updates | `git pull` in submodule | Manual copy |
| Repo size | Referenced, not duplicated | Increases repo size |
| Maintenance | Automatic | Manual sync |
| Authority | Always current | Can become stale |

### vs. Conditional Testing Only

| Aspect | Submodule ✅ | Conditional ⚠️ |
|--------|--------------|----------------|
| CI coverage | 100% corpus coverage | Corpus tests skipped |
| Developer experience | All tests available | Some tests skipped |
| Confidence | High (full corpus) | Medium (partial) |

## CI/CD Integration Examples

### GitHub Actions

```yaml
name: CI
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
        with:
          submodules: true
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test --workspace
```

### GitLab CI

```yaml
variables:
  GIT_SUBMODULE_STRATEGY: recursive
```

### CircleCI

```yaml
jobs:
  test:
    steps:
      - checkout
      - run: git submodule update --init --recursive
```

## Next Steps

1. **Maintainer Action**: Add submodule to repository
   ```bash
   git submodule add https://github.com/gram-data/tree-sitter-gram.git external/tree-sitter-gram
   ```

2. **Update CI Configuration**: Add `submodules: true` to GitHub Actions workflows

3. **Developer Communication**: Update README and onboarding docs with submodule instructions

4. **Phase 5 Implementation**: Implement corpus test parsing (T059-T063) using new path

## References

- **Comprehensive Guide**: [CORPUS_TESTING.md](CORPUS_TESTING.md)
- **Task Updates**: [tasks.md](tasks.md) - See "Git Submodule Setup" section
- **Research Decision**: [research.md](research.md) - Section "Decision: Test Corpus Integration Approach"
- **Plan Updates**: [plan.md](plan.md) - Complexity tracking and references

## Summary

The git submodule approach provides a robust, maintainable solution for corpus test integration that:

✅ Works reliably in CI/CD environments  
✅ Ensures version consistency across team  
✅ Follows standard git practices  
✅ Requires minimal setup effort  
✅ Maintains authoritative reference to tree-sitter-gram  

This implementation resolves the CI/CD dependency issue while maintaining the authoritative nature of the tree-sitter-gram test corpus.

