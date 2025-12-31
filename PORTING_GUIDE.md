# Porting Guide: gram-hs to gram-rs

This guide provides a systematic approach for porting features from the gram-hs reference implementation to gram-rs.

## Reference Implementation Location

The gram-hs reference implementation is available locally at:
- **Path**: `../gram-hs` (relative to gram-rs repository root)
- **Source Code (Authoritative)**: `../gram-hs/libs/` - Haskell library implementations - **This is the source of truth**
- **Documentation (Up-to-date)**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
- **Tests (Authoritative)**: `../gram-hs/libs/*/tests/` - Test suites for verification - **Shows expected behavior**
- **Historical Notes (Context Only)**: `../gram-hs/specs/` - Historical notes that guided incremental development (may be outdated, use for context only)

## Porting Workflow

### 1. Identify Feature to Port

Check `../gram-hs/specs/` for available features. Features are numbered incrementally:
- `001-pattern-data-structure`
- `002-basic-pattern-type`
- `003-pattern-structure-review`
- etc.

### 2. Study the Reference Implementation

**CRITICAL: The Haskell Implementation is the Source of Truth**

The Haskell implementation in `../gram-hs/libs/` is the authoritative source of truth. We are porting the actual Haskell implementation to idiomatic Rust.

Historical notes in `../gram-hs/specs/` guided incremental development and may be useful for understanding the feature's purpose and approach, but they are NOT authoritative. They may contain:
- Outdated information that was corrected during implementation
- Design mistakes that were fixed in the actual code
- Progressive design changes where later work overrides earlier work
- Incomplete or speculative design decisions

**Always prefer the Haskell implementation over design documents. When in doubt, check the actual source code.**

**Primary Source (Authoritative)**:
- **Haskell Implementation** (`../gram-hs/libs/`):
  - Source files in `libs/*/src/` - **This is the source of truth for type signatures and behavior**
  - Test files in `libs/*/tests/` - **This is the source of truth for expected behavior**
  - Documentation in source files (Haddock comments) - **This is the source of truth for API documentation**

**Documentation (Up-to-date)**:
- **Implementation Documentation** (`../gram-hs/docs/`):
  - `docs/reference/` - Architecture and feature documentation (up-to-date information about the implementation)
  - `docs/users/` - User guides and examples (up-to-date usage information)
  - `docs/design/` - Design documentation (up-to-date design information)

**Historical Notes (Context Only)**:
- **Feature Specification** (`../gram-hs/specs/XXX-feature-name/`):
  - These are historical notes that guided incremental development, NOT authoritative sources
  - `spec.md` - Feature requirements and user stories (useful for understanding purpose and approach)
  - `plan.md` - Implementation plan (may be outdated)
  - `contracts/type-signatures.md` - API contracts (may not reflect final implementation - verify against actual code)
  - `quickstart.md` - Usage examples (may be outdated)
  - `data-model.md` - Data structures (may not match actual implementation - verify against actual code)

**When in doubt, check the actual Haskell source code.**

### 3. Create Feature Specification

Use `/speckit.specify` to create a new feature specification:

```bash
/speckit.specify Port Feature XXX from gram-hs reference implementation. Reference ../gram-hs/libs/ for the authoritative implementation. Design documents in ../gram-hs/specs/XXX-feature-name/ are for context only.
```

**Important**: In your spec, include:
- **Primary reference**: Link to Haskell source code: `../gram-hs/libs/*/src/` - **This is the source of truth**
- **Documentation reference**: Link to gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- **Historical reference**: Link to gram-hs feature specs: `../gram-hs/specs/XXX-feature-name/` - **Historical notes for context only, may be outdated**
- Behavioral equivalence requirements (verify against actual Haskell implementation, not historical notes)
- Note that we are porting the Haskell implementation to idiomatic Rust

### 4. Port Type Signatures

**CRITICAL: Use the Haskell Implementation as the Source of Truth**

Start with the actual Haskell source code in `../gram-hs/libs/*/src/`. We are porting the Haskell implementation to idiomatic Rust. Also review the up-to-date documentation in `../gram-hs/docs/`. The historical notes in `../gram-hs/specs/XXX-feature-name/contracts/type-signatures.md` may be outdated or incorrect - always verify against the actual source code.

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
- Historical notes (`spec.md`, `contracts/type-signatures.md`) - These guided incremental development but may be outdated
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

**Rule of thumb**: Only port types that are explicitly defined in the Haskell source code (`*.hs` files) for the feature you're porting. Historical notes in `../gram-hs/specs/` guided incremental development but are NOT authoritative - always verify against the actual source code and up-to-date documentation in `../gram-hs/docs/`.

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

Port the Haskell implementation from `../gram-hs/libs/*/src/` to idiomatic Rust:

- Translate Haskell patterns to Rust idioms
- Use Rust's type system (enums, Result, ownership)
- Follow Rust naming conventions
- Maintain behavioral equivalence with the actual Haskell implementation
- Do not rely on design documents - use the actual source code as the reference

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

### Balancing Rust Conventions with Cross-Language Equivalence

When porting from gram-hs to gram-rs, there's a tension between:
- **Rust conventions**: Idiomatic Rust patterns (e.g., `Type::new()` for constructors)
- **Cross-language equivalence**: Matching the gram-hs API exactly (e.g., `Pattern::point()` and `Pattern::pattern()`)

**Principle: Logical Equivalence Over Syntactic Equivalence**

The most important goal is **logical equivalence** - ensuring that the Rust implementation behaves identically to the Haskell implementation. Syntactic differences (like naming conventions) are acceptable and often necessary.

**Guidelines:**

1. **Primary API**: Match gram-hs function names when they're part of the public API contract
   - Example: `Pattern::point()` matches `point :: v -> Pattern v` from gram-hs
   - Example: `Pattern::pattern()` matches `pattern :: v -> [Pattern v] -> Pattern v` from gram-hs
   - This enables direct comparison and verification against the reference implementation

2. **Breaking Changes**: When gram-hs introduces breaking API changes, adopt them to maintain equivalence
   - Example: gram-hs changed from `pattern :: v -> Pattern v` to `point :: v -> Pattern v` for atomic patterns
   - Example: gram-hs changed from `patternWith :: v -> [Pattern v] -> Pattern v` to `pattern :: v -> [Pattern v] -> Pattern v` for the primary constructor
   - Always update Rust implementation to match the current gram-hs API

3. **Documentation**: Clearly document the API and its relationship to gram-hs
   - Note which functions match gram-hs exactly
   - Explain the semantic meaning (e.g., `point` is the special case for atomic patterns, `pattern` is the primary constructor)
   - Document any intentional deviations

4. **Linting**: Use `#[allow]` attributes when necessary to document intentional deviations
   - Example: `#[allow(clippy::self_named_constructors)]` for `Pattern::pattern()` (matches gram-hs API)
   - Include a comment explaining why the deviation is intentional

5. **Testing**: Test the API to ensure behavioral equivalence
   - Verify that functions produce identical results to gram-hs
   - Document equivalence in tests

**Example Pattern (Current API):**

```rust
impl<V> Pattern<V> {
    /// Creates an atomic pattern. Equivalent to gram-hs `point :: v -> Pattern v`.
    pub fn point(value: V) -> Self {
        Pattern { value, elements: vec![] }
    }

    /// Creates a pattern with a value and elements. This is the primary constructor.
    /// Equivalent to gram-hs `pattern :: v -> [Pattern v] -> Pattern v`.
    #[allow(clippy::self_named_constructors)]
    pub fn pattern(value: V, elements: Vec<Pattern<V>>) -> Self {
        Pattern { value, elements }
    }
}
```

This approach:
- ✅ Maintains behavioral equivalence with gram-hs (primary goal)
- ✅ Matches gram-hs API exactly for direct comparison
- ✅ Enables direct API comparison for verification
- ✅ Documents intentional linting deviations

**API Evolution:**

The gram-hs API has evolved over time. When porting, always use the **current** gram-hs API as the source of truth:
- Check `../gram-hs/libs/pattern/src/Pattern/Core.hs` for the actual function signatures
- Update Rust implementation to match current gram-hs API, even if it means breaking changes
- Document breaking changes in migration notes if needed

## Feature Porting Checklist

For each feature being ported:

- [ ] **Studied Haskell implementation** in `../gram-hs/libs/*/src/` - **Primary source of truth**
- [ ] Reviewed gram-hs documentation in `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [ ] Reviewed Haskell tests in `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [ ] Reviewed `../gram-hs/specs/XXX-feature-name/spec.md` - **Historical notes for context only, may be outdated**
- [ ] Reviewed `../gram-hs/specs/XXX-feature-name/contracts/type-signatures.md` - **Historical notes for context only, verify against actual code**
- [ ] Created feature specification in `specs/XXX-feature-name/`
- [ ] Ported type signatures to Rust (from actual Haskell source, not design docs)
- [ ] Ported test cases from gram-hs (from actual test files)
- [ ] Implemented functionality in idiomatic Rust (matching actual Haskell implementation)
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

**Important**: The actual implementations are in `../gram-hs/libs/`. Up-to-date documentation is in `../gram-hs/docs/`. The specifications in `../gram-hs/specs/` are historical notes that guided incremental development but may be outdated. Always check the actual Haskell source code in `libs/*/src/` as the authoritative source, and refer to `../gram-hs/docs/` for up-to-date documentation. We are porting the Haskell implementation to idiomatic Rust.

Each feature directory in `specs/` contains (historical notes for context only, may be outdated):
- `spec.md` - Feature specification (historical notes)
- `plan.md` - Implementation plan (may be outdated)
- `contracts/` - API contracts and type signatures (verify against actual code)
- `quickstart.md` - Usage examples (may be outdated)
- Other design artifacts

**Note**: Features are numbered sequentially. When creating a new feature in gram-rs, use the same feature number and name from gram-hs to maintain consistency (e.g., `002-basic-pattern-type`). However, always verify the actual implementation in `../gram-hs/libs/` and refer to up-to-date documentation in `../gram-hs/docs/` rather than relying on the historical notes. The historical notes guided incremental development but are not authoritative sources.

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

