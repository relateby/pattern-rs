# Research: Core Pattern Data Structure

**Feature**: 004-pattern-data-structure  
**Date**: 2025-01-27

## Research Tasks

### 1. Pattern Type Structure from gram-hs

**Task**: Understand the Pattern type structure from gram-hs reference implementation

**Findings**:
- **Decision**: Pattern type is `data Pattern v = Pattern { value :: v, elements :: [Pattern v] }`
- **Rationale**: This is the canonical structure from the actual gram-hs implementation in `../gram-hs/libs/pattern/src/Pattern.hs`
- **Alternatives considered**: None - this is the reference implementation structure
- **Source**: `../gram-hs/libs/pattern/src/Pattern.hs` (authoritative source of truth)
- **Note**: Design documents in `../gram-hs/specs/001-pattern-data-structure/contracts/type-signatures.md` were reviewed for context but the actual Haskell source code is authoritative

**Rust Translation**:
```rust
pub struct Pattern<V> {
    pub value: V,
    pub elements: Vec<Pattern<V>>,
}
```

**Note**: The actual gram-hs implementation in `../gram-hs/libs/pattern/src/Pattern.hs` defines the `Pattern v` type. The Subject type is defined in the gram-hs `libs/subject/src/Subject/Core.hs` module (verified in actual source code, not design documents) and can be ported as part of this feature. Subject is a single type (not Node, Edge, etc.) with identity, labels, and properties.

### 2. Rust Trait Implementation Strategy

**Task**: Determine how to implement required traits (Debug, Display, Clone, PartialEq, Eq)

**Findings**:
- **Decision**: Use derive macros where possible, custom implementations for Debug/Display
- **Rationale**: 
  - `Clone`, `PartialEq`, `Eq` can use `#[derive(...)]` for recursive structures
  - `Debug` and `Display` need custom implementations for readable output
  - Recursive structures require careful formatting to avoid infinite output
- **Alternatives considered**: 
  - Fully custom implementations - rejected (unnecessary for Clone/Eq)
  - Using existing formatting crates - deferred (keep simple for initial implementation)
- **Implementation Notes**:
  - Debug should show structure clearly with indentation/truncation for deep nesting
  - Display should be human-readable, may differ from gram-hs format (per spec assumptions)

### 3. WASM Compilation Compatibility

**Task**: Ensure pattern types compile for WASM target

**Findings**:
- **Decision**: Use only standard library and workspace dependencies (serde, serde_json)
- **Rationale**: 
  - Standard library types (Vec, structs) compile to WASM
  - serde is WASM-compatible
  - No platform-specific code needed for basic data structures
- **Alternatives considered**: 
  - Platform-specific optimizations - deferred to later features
  - Custom allocators - not needed for basic types
- **Verification**: Compile with `cargo build --target wasm32-unknown-unknown`

### 4. Behavioral Equivalence Testing Strategy

**Task**: Determine how to verify behavioral equivalence with gram-hs

**Findings**:
- **Decision**: Use existing test utilities in `crates/pattern-core/src/test_utils/` for equivalence checking
- **Rationale**: 
  - Test infrastructure already exists (feature 003)
  - Can extract test cases from gram-hs using test synchronization infrastructure
  - Equivalence checking utilities are placeholder but structure is ready
- **Alternatives considered**: 
  - Manual comparison - rejected (too error-prone)
  - External testing tools - rejected (use existing infrastructure)
- **Implementation Notes**:
  - Port test cases from `../gram-hs/libs/*/tests/`
  - Use `gram-hs` CLI tool for test case generation if needed (per `docs/gram-hs-cli-testing-guide.md`)

### 5. S-Expression vs Tree Terminology

**Task**: Clarify pattern structure terminology (s-expression-like, not trees)

**Findings**:
- **Decision**: Patterns are s-expression-like recursive nested structures, not trees
- **Rationale**: 
  - Clarification from spec: patterns may appear tree-like and accept tree-like operations, but are fundamentally s-expression-like
  - Value provides "information about the elements" - intimate pairing
  - Elements are themselves patterns (recursive)
- **Alternatives considered**: None - this is a conceptual clarification from spec
- **Implementation Impact**: 
  - Documentation and naming should reflect s-expression nature
  - May accept tree-like operations in future features, but structure is s-expression-like

## Resolved Clarifications

All NEEDS CLARIFICATION items from Technical Context have been resolved:
- ✅ Pattern structure: `Pattern { value: V, elements: Vec<Pattern<V>> }`
- ✅ Trait implementation: derive for Clone/Eq, custom for Debug/Display
- ✅ WASM compatibility: Standard library only, no platform-specific code
- ✅ Testing strategy: Use existing test infrastructure and port from gram-hs

## Open Questions (Deferred to Implementation)

1. **Debug/Display Format**: Exact formatting style (readable but may differ from gram-hs per spec)
2. **Edge Case Handling**: Deep nesting limits, circular reference detection (if needed)

## References

- **Primary Source (Authoritative)**: gram-hs Implementation: `../gram-hs/libs/`
  - Pattern: `../gram-hs/libs/pattern/src/Pattern.hs`
  - Subject: `../gram-hs/libs/subject/src/Subject/Core.hs`
  - Tests: `../gram-hs/libs/*/tests/`
- **Secondary Source (Context Only)**: gram-hs Design Documents: `../gram-hs/specs/001-pattern-data-structure/`
  - Type Signatures: `../gram-hs/specs/001-pattern-data-structure/contracts/type-signatures.md` (may be outdated)
  - Feature Spec: `../gram-hs/specs/001-pattern-data-structure/spec.md` (for context)
- Porting Guide: `PORTING_GUIDE.md`
- Test Infrastructure: `specs/003-test-infrastructure/`
- gram-hs CLI Testing Guide: `docs/gram-hs-cli-testing-guide.md`
