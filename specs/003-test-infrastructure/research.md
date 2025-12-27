# Research: Testing Infrastructure

**Feature**: 003-test-infrastructure  
**Date**: 2025-01-27  
**Purpose**: Research technical decisions for testing infrastructure implementation

## Research Questions

### 1. WASM Compatibility of Testing Libraries

**Question**: Are proptest, insta, and criterion compatible with WASM targets?

**Research Findings**:

- **proptest**: 
  - Supports WASM targets with some limitations
  - Requires `proptest` feature flag for WASM compatibility
  - Random number generation works on WASM
  - Shrinking (counterexample minimization) may have performance implications on WASM
  - **Decision**: Use proptest with WASM feature flag, test on `wasm32-unknown-unknown` target

- **insta**:
  - Fully compatible with WASM targets
  - Snapshot storage works identically on native and WASM
  - No special configuration needed
  - **Decision**: Use insta as-is, no WASM-specific configuration required

- **criterion**:
  - Benchmarking on WASM is complex due to timing limitations
  - Criterion relies on system time which is limited in WASM
  - For WASM targets, benchmarks may need to be disabled or use alternative timing
  - **Decision**: Use criterion for native targets, disable or use simplified benchmarks for WASM (conditional compilation)

**Rationale**: All three libraries can be used, but criterion requires conditional compilation for WASM targets. Property-based testing and snapshot testing work well on WASM.

**Alternatives Considered**:
- Custom benchmarking for WASM: Too complex for initial implementation
- Skip WASM benchmarks entirely: Acceptable, but limits performance tracking

### 2. Property-Based Testing Best Practices

**Question**: What are best practices for property-based testing in Rust, especially for recursive data structures like patterns?

**Research Findings**:

- **Test Strategy**:
  - Start with simple properties (equality symmetry, associativity)
  - Use custom generators for complex types (patterns will need custom generators)
  - Configure test case count (default 256, can be reduced for slower tests)
  - Use `proptest!` macro for simple properties, `TestRunner` for complex scenarios

- **Generator Design**:
  - Use `proptest::collection` for collections (Vec, etc.)
  - Recursive types need `prop_recursive` or manual size limiting
  - Pattern generators will need to respect pattern data model constraints (to be defined in feature 004)
  - Use `prop_oneof!` for enum-like types

- **Performance**:
  - Property tests can be slow; use `#[proptest]` attribute with `cases` parameter
  - Consider running fewer cases in CI vs local development
  - Use `proptest::prelude::*` for common imports

**Decision**: 
- Use proptest with custom generators for patterns (to be implemented when pattern types are defined)
- Configure test case count per property (100+ cases as per SC-001)
- Use `#[proptest]` attribute for integration with `cargo test`
- Create generator utilities in test-utils crate

**Rationale**: Property-based testing is well-established in Rust ecosystem. Custom generators are necessary for pattern types but follow standard patterns.

**Alternatives Considered**:
- QuickCheck (Haskell-style): Less idiomatic for Rust, proptest is more Rust-native
- Custom property testing: Too much work, proptest is mature and well-maintained

### 3. Equivalence Checking Architecture

**Question**: How should equivalence checking between gram-rs and gram-hs be implemented?

**Research Findings**:

- **Approach Options**:
  1. **Direct comparison**: Run gram-hs executable and compare outputs (requires gram-hs to be built)
  2. **JSON serialization**: Serialize both outputs to JSON and compare (works if gram-hs can export JSON)
  3. **Test data comparison**: Use extracted test cases from gram-hs and validate gram-rs produces same outputs
  4. **Hybrid**: Use test data for most cases, direct comparison for complex scenarios

- **Implementation Strategy**:
  - Start with test data comparison (uses extracted test cases from feature 002)
  - Add direct comparison capability if gram-hs is available locally
  - Use serde for serialization to enable JSON comparison
  - Provide clear error messages showing differences

- **Error Reporting**:
  - Use `diff`-style output for structural differences
  - Highlight specific fields that differ
  - Support approximate equality for floating-point values (if applicable)

**Decision**: 
- Implement equivalence checking using extracted test data primarily
- Support optional direct gram-hs comparison if available
- Use serde_json for serialization and comparison
- Provide detailed diff output for failures

**Rationale**: Test data comparison is most practical initially and builds on existing infrastructure. Direct comparison can be added later if needed.

**Alternatives Considered**:
- Always require gram-hs: Too restrictive, not all developers will have it
- Only test data: Sufficient for most cases, direct comparison is optional enhancement

### 4. Test Utility Organization

**Question**: Should test utilities be a separate crate or a module within pattern-core?

**Research Findings**:

- **Option 1: Separate test-utils crate**
  - Pros: Clear separation, can be versioned independently, easy to share
  - Cons: Additional crate to maintain, dependency management
  - Use case: When utilities are substantial and used by many crates

- **Option 2: Module in pattern-core**
  - Pros: Simpler structure, no extra crate, pattern-core is natural home
  - Cons: Couples test utilities to pattern-core, may need re-exporting
  - Use case: When utilities are small and pattern-core is the primary user

- **Option 3: Workspace-level test module**
  - Pros: Available to all crates without dependencies
  - Cons: Not idiomatic Rust, harder to version, no clear home
  - Use case: Not recommended

**Decision**: Start with Option 2 (module in pattern-core) for initial implementation. If utilities grow significantly or are needed by many crates, migrate to Option 1 (separate crate).

**Rationale**: Test utilities will initially be used primarily with pattern types defined in pattern-core. Starting simple allows us to validate the approach before adding complexity.

**Alternatives Considered**:
- Separate crate from start: Premature optimization, adds complexity
- Workspace-level module: Not idiomatic, harder to manage

### 5. Snapshot Testing Integration

**Question**: How should snapshot testing integrate with the workspace structure?

**Research Findings**:

- **Snapshot Storage**:
  - insta stores snapshots in `__snapshots__/` directory by default
  - Can be configured per crate or workspace-level
  - Snapshots should be version-controlled

- **Workspace Integration**:
  - Each crate can have its own snapshots directory
  - Or use workspace-level snapshots directory
  - insta supports both approaches

- **Best Practices**:
  - Store snapshots alongside test files
  - Use descriptive snapshot names
  - Review snapshot changes carefully (insta provides review workflow)
  - Update snapshots intentionally, not automatically

**Decision**: 
- Use crate-level snapshot directories (`tests/__snapshots__/` in each crate)
- Configure insta to use consistent naming
- Document snapshot review workflow in quickstart

**Rationale**: Crate-level snapshots provide better organization and isolation. Each crate manages its own snapshots.

**Alternatives Considered**:
- Workspace-level snapshots: Less isolation, harder to manage
- Single snapshot file: Too monolithic, harder to review

### 6. Benchmark Suite Organization

**Question**: How should benchmarks be organized in a Cargo workspace?

**Research Findings**:

- **Criterion Structure**:
  - Benchmarks go in `benches/` directory at workspace root
  - Each benchmark file is a separate binary
  - Can benchmark functions from any crate in workspace

- **Organization Patterns**:
  - One file per operation type (pattern_operations.rs, codec_operations.rs)
  - Or one file per crate (pattern_core_bench.rs, pattern_ops_bench.rs)
  - Group related benchmarks together

- **Workspace Integration**:
  - Benchmarks can depend on workspace crates
  - Use `cargo bench` to run all benchmarks
  - Can run specific benchmarks with `cargo bench --bench <name>`

**Decision**: 
- Organize benchmarks by operation domain (pattern_operations.rs, codec_operations.rs, etc.)
- Each benchmark file can test functions from multiple crates
- Use descriptive benchmark names following criterion conventions

**Rationale**: Domain-based organization makes it easier to find relevant benchmarks. Operation-based grouping is more intuitive than crate-based.

**Alternatives Considered**:
- Crate-based organization: Less intuitive, operations may span crates
- Single benchmark file: Too monolithic, harder to maintain

### 7. Test Extraction from gram-hs

**Question**: How should test extraction from gram-hs be implemented?

**Research Findings**:

- **Existing Infrastructure** (from feature 002):
  - JSON test case format defined in `contracts/test-sync-format.md`
  - Placeholder scripts in `scripts/sync-tests/`
  - Test cases stored in `tests/common/test_cases.json`

- **Extraction Approaches**:
  1. **Parse Haskell test files**: Complex, requires Haskell parsing
  2. **Use gram-hs test output**: If gram-hs can export test cases as JSON
  3. **Manual extraction**: Start with manual, automate later
  4. **Hybrid**: Use gram-hs test runner output if available, fallback to parsing

- **Implementation Strategy**:
  - Start with manual extraction and JSON format validation
  - Investigate gram-hs test export capabilities
  - Build extraction utilities incrementally
  - Support both automated and manual workflows

**Decision**: 
- Implement extraction utilities that can:
  1. Validate JSON test case format
  2. Parse gram-hs test output if available (future enhancement)
  3. Support manual test case addition
  4. Compare extracted test cases with gram-rs test results

**Rationale**: Start simple with manual extraction and format validation. Automation can be added incrementally as we understand gram-hs test structure better.

**Alternatives Considered**:
- Full automation from start: Too complex, requires deep gram-hs knowledge
- Only manual: Too limiting, but acceptable for initial implementation

## Summary of Decisions

1. **proptest**: Use with WASM feature flag, test on WASM target
2. **insta**: Use as-is, fully WASM compatible
3. **criterion**: Use for native targets, conditional compilation for WASM
4. **Property-based testing**: Use proptest with custom generators (to be implemented with pattern types)
5. **Equivalence checking**: Use test data comparison primarily, optional direct gram-hs comparison
6. **Test utilities**: Start as module in pattern-core, migrate to separate crate if needed
7. **Snapshot testing**: Crate-level snapshot directories
8. **Benchmarks**: Domain-based organization in workspace `benches/` directory
9. **Test extraction**: Incremental implementation starting with manual extraction and format validation

## Unresolved Questions

- Pattern generator implementation details (depends on pattern type definition in feature 004)
- Exact gram-hs test export format (to be investigated during implementation)
- Benchmark timing strategy for WASM (may need WASM-specific approach)

## Next Steps

1. Add test dependencies to workspace and crate Cargo.toml files
2. Create test utility module structure
3. Implement basic property-based test example
4. Implement equivalence checking utilities
5. Set up snapshot testing infrastructure
6. Create benchmark suite structure
7. Enhance test extraction utilities

