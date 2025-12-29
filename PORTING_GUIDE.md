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

**CRITICAL: The Haskell Implementation is the Source of Truth**

The Haskell implementation in `../gram-hs/libs/` is the authoritative source of truth. Design documents in `../gram-hs/specs/` are useful for context and understanding the feature's purpose, but they may contain:
- Outdated information that was corrected during implementation
- Design mistakes that were fixed in the actual code
- Progressive design changes where later work overrides earlier work

**Always prefer the Haskell implementation over design documents.**

**Primary Source (Authoritative)**:
- **Haskell Implementation** (`../gram-hs/libs/`):
  - Source files in `libs/*/src/` - **This is the source of truth for type signatures and behavior**
  - Test files in `libs/*/tests/` - **This is the source of truth for expected behavior**
  - Documentation in source files (Haddock comments) - **This is the source of truth for API documentation**

**Secondary Sources (Context Only)**:
- **Feature Specification** (`../gram-hs/specs/XXX-feature-name/`):
  - `spec.md` - Feature requirements and user stories (useful for understanding purpose)
  - `plan.md` - Implementation plan (may be outdated)
  - `contracts/type-signatures.md` - API contracts (may not reflect final implementation)
  - `quickstart.md` - Usage examples (may be outdated)
  - `data-model.md` - Data structures (may not match actual implementation)

**When in doubt, check the actual Haskell source code.**

### 3. Create Feature Specification

Use `/speckit.specify` to create a new feature specification:

```bash
/speckit.specify Port Feature XXX from gram-hs reference implementation. Reference ../gram-hs/libs/ for the authoritative implementation. Design documents in ../gram-hs/specs/XXX-feature-name/ are for context only.
```

**Important**: In your spec, include:
- **Primary reference**: Link to Haskell source code: `../gram-hs/libs/*/src/` - **This is the source of truth**
- **Secondary reference**: Link to gram-hs feature specs: `../gram-hs/specs/XXX-feature-name/` - **For context only, may be outdated**
- Behavioral equivalence requirements (verify against actual Haskell implementation, not design docs)

### 4. Port Type Signatures

**CRITICAL: Use the Haskell Implementation as the Source of Truth**

Start with the actual Haskell source code in `../gram-hs/libs/*/src/`. The design documents in `../gram-hs/specs/XXX-feature-name/contracts/type-signatures.md` may be outdated or incorrect.

**Step 1: Find the Haskell Implementation**

1. Identify which library module(s) implement the feature:
   ```bash
   # Find relevant source files
   find ../gram-hs/libs -name "*.hs" -type f | grep -i pattern
   find ../gram-hs/libs -name "*.hs" -type f | grep -i subject
   ```

2. Read the actual type definitions from the `.hs` source files:
   - Look for `data` declarations for types
   - Look for `type` declarations for type aliases
   - Look for `class` declarations for typeclasses
   - Look for function signatures to understand the API

**Step 2: Verify What's Actually Defined**

Before porting, verify what types actually exist in the Haskell implementation:
- Read the actual `.hs` source files, not just design documents
- Only port types that are explicitly defined in the source code
- Do NOT assume types exist just because they're mentioned in:
  - Design documents (`spec.md`, `contracts/type-signatures.md`)
  - Feature requirements (FR-XXX)
  - User stories
  - TODO checklists

**Step 3: Port the Types**

Port the types you found in the Haskell source:

**Haskell → Rust Translation**:
- `data Pattern v = Pattern { value :: v, elements :: [Pattern v] }` → `pub struct Pattern<V> { pub value: V, pub elements: Vec<Pattern<V>> }`
- `type` aliases → `type` aliases (same)
- Typeclasses → Traits (see translation guide below)
- Functions → Functions (with Rust naming: `snake_case`)

**Example**: To find the Pattern type definition:
```bash
# Look in the pattern library
cat ../gram-hs/libs/pattern/src/Pattern.hs | grep -A 5 "^data Pattern"
```

**Example**: To find the Subject type definition:
```bash
# Look in the subject library
cat ../gram-hs/libs/subject/src/Subject/Core.hs | grep -A 10 "^data Subject"
```

### 4.5. Common Pitfalls: Assuming Types Exist

**Warning**: Do not assume a type needs to be ported just because it's mentioned in:
- Design documents (`spec.md`, `contracts/type-signatures.md`)
- Feature requirements (FR-XXX)
- User stories
- Key Entities sections
- TODO checklists

**Always verify** by checking:
1. **The actual Haskell source code** in `../gram-hs/libs/*/src/` - **This is the source of truth**
2. The gram-hs test files in `../gram-hs/libs/*/tests/` - **This shows what actually exists**

**Example**: If a requirement says "provide Subject types" but you can't find `data Subject` in the Haskell source for that feature's library, then:
- Subject types are NOT defined in this feature
- Pattern<V> is generic and can work with any value type V
- Subject types, if they exist, are defined in other libraries and are just value types

**Rule of thumb**: Only port types that are explicitly defined in the Haskell source code (`*.hs` files) for the feature you're porting. Design documents are for context only.

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

- [ ] **Studied Haskell implementation** in `../gram-hs/libs/*/src/` - **Primary source of truth**
- [ ] Reviewed Haskell tests in `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [ ] Reviewed `../gram-hs/specs/XXX-feature-name/spec.md` - **For context only, may be outdated**
- [ ] Reviewed `../gram-hs/specs/XXX-feature-name/contracts/type-signatures.md` - **For context only, verify against actual code**
- [ ] Created feature specification in `specs/XXX-feature-name/`
- [ ] Ported type signatures to Rust (from actual Haskell source, not design docs)
- [ ] Ported test cases from gram-hs (from actual test files)
- [ ] Implemented functionality (matching actual Haskell implementation)
- [ ] Verified behavioral equivalence (against actual Haskell implementation)
- [ ] Tested WASM compilation
- [ ] Updated examples (if applicable)
- [ ] Documented any intentional deviations

## Available Features in gram-hs

Current features available for porting:

The gram-hs reference implementation contains multiple features organized incrementally. To see all available features:

```bash
# List feature specifications (for context)
ls -1 ../gram-hs/specs/

# List actual library implementations (source of truth)
ls -1 ../gram-hs/libs/
```

**Important**: The actual implementations are in `../gram-hs/libs/`. The specifications in `../gram-hs/specs/` are design documents that may be outdated. Always check the actual Haskell source code in `libs/*/src/` as the authoritative source.

Each feature directory in `specs/` contains (for context only, may be outdated):
- `spec.md` - Feature specification
- `plan.md` - Implementation plan
- `contracts/` - API contracts and type signatures
- `quickstart.md` - Usage examples
- Other design artifacts

**Note**: Features are numbered sequentially. When creating a new feature in gram-rs, use the same feature number and name from gram-hs to maintain consistency (e.g., `002-basic-pattern-type`). However, always verify the actual implementation in `../gram-hs/libs/` rather than relying solely on the design documents.

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

