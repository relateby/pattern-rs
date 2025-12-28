# Porting Guide: gram-hs to gram-rs

This guide provides a systematic approach for porting features from the gram-hs reference implementation to gram-rs.

## Reference Implementation Location

The gram-hs reference implementation is available locally at:
- **Path**: `../gram-hs` (relative to gram-rs repository root)
- **Feature Specifications**: `../gram-hs/specs/` - Contains incremental feature development
- **Source Code**: `../gram-hs/libs/` - Haskell library implementations
- **Tests**: `../gram-hs/libs/*/tests/` - Test suites for verification

## Porting Workflow

### 1. Identify Feature to Port

Check `../gram-hs/specs/` for available features. Features are numbered incrementally:
- `001-pattern-data-structure`
- `002-basic-pattern-type`
- `003-pattern-structure-review`
- etc.

### 2. Study the Reference Implementation

For each feature, review:

**Feature Specification** (`../gram-hs/specs/XXX-feature-name/`):
- `spec.md` - Feature requirements and user stories
- `plan.md` - Implementation plan
- `contracts/type-signatures.md` - API contracts
- `quickstart.md` - Usage examples
- `data-model.md` - Data structures

**Haskell Implementation** (`../gram-hs/libs/`):
- Source files in `libs/*/src/`
- Test files in `libs/*/tests/`
- Documentation in source files (Haddock comments)

### 3. Create Feature Specification

Use `/speckit.specify` to create a new feature specification:

```bash
/speckit.specify Port Feature XXX from gram-hs reference implementation. Reference ../gram-hs/specs/XXX-feature-name/ for requirements and ../gram-hs/libs/ for implementation details.
```

**Important**: In your spec, include:
- Reference to the gram-hs feature: `../gram-hs/specs/XXX-feature-name/`
- Link to Haskell source: `../gram-hs/libs/*/src/`
- Behavioral equivalence requirements

### 4. Port Type Signatures

Start with `../gram-hs/specs/XXX-feature-name/contracts/type-signatures.md`:

**Haskell → Rust Translation**:
- `data Pattern v` → `pub struct Pattern<V>`
- `type` aliases → `type` aliases (same)
- Typeclasses → Traits (see translation guide below)
- Functions → Functions (with Rust naming: `snake_case`)

### 5. Port Tests (TDD Approach)

Port test cases from `../gram-hs/libs/*/tests/`:

1. Create test file in `tests/equivalence/` or `tests/integration/`
2. Port test cases maintaining the same test data and expected outputs
   - **Tip**: Use `gram-hs generate --type suite` to generate test cases in the correct format
   - See [gram-hs CLI Testing Guide](docs/gram-hs-cli-testing-guide.md) for test suite generation
3. Run tests (they should fail initially)
4. Implement functionality to make tests pass

**Alternative: Generate Test Cases from gram-hs**:

You can also generate test cases directly from gram-hs using the CLI:
```bash
# Generate test suite with 100 test cases
gram-hs generate --type suite --count 100 --seed 42 --complexity standard \
    --format json --value-only > tests/common/test_cases.json
```

See [gram-hs CLI Testing Guide](docs/gram-hs-cli-testing-guide.md) for more details on test case generation and extraction.

### 6. Implement Functionality

Port the Haskell implementation from `../gram-hs/libs/*/src/`:

- Translate Haskell patterns to Rust idioms
- Use Rust's type system (enums, Result, ownership)
- Follow Rust naming conventions
- Maintain behavioral equivalence

### 7. Verify Equivalence

Before marking complete:

1. **Compare outputs**: Run both implementations on same inputs
   - Use `gram-hs` CLI tool with `--value-only` and `--canonical` flags for reliable comparison
   - See [gram-hs CLI Testing Guide](docs/gram-hs-cli-testing-guide.md) for detailed usage
2. **Check edge cases**: Ensure all edge cases from gram-hs tests pass
3. **Verify documentation**: Ensure Rust docs match Haskell semantics
4. **Test WASM compilation**: Verify `cargo build --target wasm32-unknown-unknown`

**Using gram-hs CLI for Equivalence Checking**:

The `gram-hs` CLI tool provides several flags that make equivalence checking easier:
- `--value-only`: Output only the pattern value without metadata (enables direct JSON comparison)
- `--deterministic`: Use fixed timestamp and hash for reproducible outputs
- `--canonical`: Sort JSON keys for byte-for-byte identical output

See [gram-hs CLI Testing Guide](docs/gram-hs-cli-testing-guide.md) for comprehensive examples and integration patterns.

### 8. Update Examples

Update `examples/wasm-js/` to demonstrate new functionality (if applicable).

## Haskell → Rust Translation Guide

### Data Types

```haskell
-- Haskell
data Pattern v = Pattern
  { value :: v
  , elements :: [Pattern v]
  }
```

```rust
// Rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern<V> {
    pub value: V,
    pub elements: Vec<Pattern<V>>,
}
```

### Typeclasses → Traits

| Haskell | Rust |
|---------|------|
| `Show` | `Debug` + `Display` |
| `Eq` | `PartialEq` + `Eq` |
| `Ord` | `PartialOrd` + `Ord` |
| `Functor` | Custom trait or `Iterator` |
| `Foldable` | `Iterator` methods |
| `Traversable` | `Iterator` + `collect` |
| `Semigroup` | Custom trait or use `Add` |
| `Monoid` | Custom trait or use `Default` |

### Error Handling

| Haskell | Rust |
|---------|------|
| `Maybe a` | `Option<T>` |
| `Either e a` | `Result<T, E>` |
| Exceptions | `Result<T, E>` (preferred) or `panic!` (avoid) |

### Lists and Collections

| Haskell | Rust |
|---------|------|
| `[a]` | `Vec<T>` |
| `Set a` | `HashSet<T>` or `BTreeSet<T>` |
| `Map k v` | `HashMap<K, V>` or `BTreeMap<K, V>` |

### Functions

```haskell
-- Haskell (camelCase)
getValue :: Pattern v -> v
getValue p = value p
```

```rust
// Rust (snake_case)
pub fn get_value<V>(p: &Pattern<V>) -> &V {
    &p.value
}
```

## Feature Porting Checklist

For each feature being ported:

- [ ] Reviewed `../gram-hs/specs/XXX-feature-name/spec.md`
- [ ] Reviewed `../gram-hs/specs/XXX-feature-name/contracts/type-signatures.md`
- [ ] Studied Haskell implementation in `../gram-hs/libs/`
- [ ] Created feature specification in `specs/XXX-feature-name/`
- [ ] Ported type signatures to Rust
- [ ] Ported test cases from gram-hs
- [ ] Implemented functionality
- [ ] Verified behavioral equivalence
- [ ] Tested WASM compilation
- [ ] Updated examples (if applicable)
- [ ] Documented any intentional deviations

## Available Features in gram-hs

Current features available for porting (check `../gram-hs/specs/` for the complete and latest list):

The gram-hs reference implementation contains multiple features organized incrementally. To see all available features:

```bash
ls -1 ../gram-hs/specs/
```

Each feature directory contains:
- `spec.md` - Feature specification
- `plan.md` - Implementation plan
- `contracts/` - API contracts and type signatures
- `quickstart.md` - Usage examples
- Other design artifacts

**Note**: Features are numbered sequentially. When creating a new feature in gram-rs, use the same feature number and name from gram-hs to maintain consistency (e.g., `002-basic-pattern-type`).

## Resources

- **gram-hs Repository**: https://github.com/gram-data/gram-hs
- **Local Reference**: `../gram-hs`
- **gram-hs CLI Tool**: `/Users/akollegger/.cabal/bin/gram-hs` (or `gram-hs` if in PATH)
  - **Testing Guide**: [gram-hs CLI Testing Guide](docs/gram-hs-cli-testing-guide.md) - Comprehensive guide for using gram-hs CLI for testing and equivalence checking
  - **Manpage**: `/Users/akollegger/.cabal/share/man/man1/gram-hs.1` or `man gram-hs`
- **Constitution**: `.specify/memory/constitution.md`
- **Rust Book**: https://doc.rust-lang.org/book/
- **Haskell → Rust Patterns**: See translation guide above
- **Testing Infrastructure**: [Testing Infrastructure Guide](docs/testing-infrastructure.md) - Overview of testing tools and utilities

