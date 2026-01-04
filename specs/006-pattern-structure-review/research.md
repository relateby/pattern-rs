# Research: Pattern Structure Validation

**Feature**: 006-pattern-structure-review  
**Date**: 2025-01-27

## Research Tasks

### 1. Pattern Validation Functions from gram-hs

**Task**: Understand what validation functions exist in the gram-hs reference implementation

**Findings**:
- **Decision**: Based on spec requirements and existing placeholder code, implement validation function:
  - `validate(&self, rules: &ValidationRules) -> Result<(), ValidationError>` - Validates pattern structure against rules
- **Rationale**: 
  - Spec requires validation functions that check structural constraints (FR-001, FR-003)
  - Existing placeholder `assert_pattern_structure_valid` in `test_utils/helpers.rs` suggests this pattern
  - Idiomatic Rust uses `Result` for validation (Ok for valid, Err for invalid)
- **Source**: To be verified from `../gram-hs/libs/pattern/src/Pattern.hs` (authoritative source of truth)
- **Note**: Design documents in `../gram-hs/specs/003-pattern-structure-review/` may provide context but the actual Haskell source code is authoritative

**Rust Translation**:
```rust
impl<V> Pattern<V> {
    /// Validate pattern structure against configurable rules
    /// 
    /// Returns `Ok(())` if pattern is valid, `Err(ValidationError)` if invalid
    pub fn validate(&self, rules: &ValidationRules) -> Result<(), ValidationError> {
        // Implementation to check max_depth, max_elements, etc.
    }
}
```

**Action Required**: 
- Study `../gram-hs/libs/pattern/src/Pattern.hs` to verify function signature and behavior
- Review test files in `../gram-hs/libs/pattern/tests/` to understand expected behavior
- Ensure implementation matches gram-hs behavior exactly

### 2. Structure Analysis Utilities from gram-hs

**Task**: Understand what structure analysis utilities exist in the gram-hs reference implementation

**Findings**:
- **Decision**: Based on spec requirements, implement structure analysis functions:
  - `analyze_structure(&self) -> StructureAnalysis` - Provides detailed structural information
  - Structure analysis should include: depth distribution, element counts, nesting patterns, structural summaries
- **Rationale**: 
  - Spec requires structure analysis utilities (FR-002, FR-005)
  - Analysis should provide detailed information beyond basic inspection (length, size, depth from feature 005)
  - Return structured data type for comprehensive analysis results
- **Source**: To be verified from `../gram-hs/libs/pattern/src/Pattern.hs` (authoritative source of truth)
- **Note**: Structure analysis complements basic inspection utilities (length, size, depth) from feature 005

**Rust Translation**:
```rust
/// Structure analysis results
pub struct StructureAnalysis {
    pub depth_distribution: Vec<usize>,  // Count of nodes at each depth level
    pub element_counts: Vec<usize>,      // Element counts at each level
    pub nesting_patterns: Vec<String>,   // Identified structural patterns
    pub summary: String,                 // Human-readable summary
}

impl<V> Pattern<V> {
    /// Analyze pattern structure and return detailed information
    pub fn analyze_structure(&self) -> StructureAnalysis {
        // Implementation to analyze depth distribution, element counts, etc.
    }
}
```

**Action Required**:
- Study `../gram-hs/libs/pattern/src/Pattern.hs` to verify analysis function signatures and return types
- Review test files in `../gram-hs/libs/pattern/tests/` to understand expected analysis output
- Ensure analysis results match gram-hs behavior exactly

### 3. Validation Rules and Constraints from gram-hs

**Task**: Understand what validation rules and constraints are supported in gram-hs

**Findings**:
- **Decision**: Use existing `ValidationRules` struct from `test_utils/helpers.rs` with extensions:
  - `max_depth: Option<usize>` - Maximum nesting depth allowed
  - `max_elements: Option<usize>` - Maximum element count allowed
  - `required_fields: Vec<String>` - Required fields (for future value-specific validation)
  - Additional rules may be added based on gram-hs implementation
- **Rationale**: 
  - Spec requires configurable validation rules (FR-003, FR-012)
  - Existing placeholder structure provides good foundation
  - `Option<usize>` allows optional constraints (None means no limit)
- **Source**: To be verified from `../gram-hs/libs/pattern/src/Pattern.hs` (authoritative source of truth)
- **Note**: The existing `ValidationRules` placeholder in `test_utils/helpers.rs` should be moved to main pattern module or kept in test_utils based on gram-hs structure

**Rust Translation**:
```rust
/// Configurable validation rules for pattern structure
#[derive(Debug, Clone, Default)]
pub struct ValidationRules {
    pub max_depth: Option<usize>,        // Maximum nesting depth (None = no limit)
    pub max_elements: Option<usize>,      // Maximum element count (None = no limit)
    pub required_fields: Vec<String>,     // Required fields (for future use)
}

impl Default for ValidationRules {
    fn default() -> Self {
        Self {
            max_depth: None,
            max_elements: None,
            required_fields: Vec::new(),
        }
    }
}
```

**Action Required**:
- Study `../gram-hs/libs/pattern/src/Pattern.hs` to verify rule structure matches gram-hs
- Check if additional rule types are needed (e.g., min_depth, element type constraints)
- Ensure rule application logic matches gram-hs behavior

### 4. Error Information Structure from gram-hs

**Task**: Understand how validation errors are structured and reported in gram-hs

**Findings**:
- **Decision**: Use existing `ValidationError` struct from `test_utils/helpers.rs` with location path:
  - `message: String` - Human-readable error message
  - `rule_violated: String` - Name/identifier of the violated rule
  - `location: Vec<String>` - Path to the violating node in pattern structure
- **Rationale**: 
  - Spec requires detailed error information (FR-004)
  - Location path helps developers identify where validation failed
  - Path can be represented as vector of strings (e.g., ["elements", "0", "elements", "1"])
- **Source**: To be verified from `../gram-hs/libs/pattern/src/Pattern.hs` (authoritative source of truth)
- **Note**: The existing `ValidationError` placeholder in `test_utils/helpers.rs` should be moved to main pattern module or kept in test_utils based on gram-hs structure

**Rust Translation**:
```rust
/// Error type for pattern validation failures
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub message: String,              // Human-readable error message
    pub rule_violated: String,        // Name of violated rule (e.g., "max_depth")
    pub location: Vec<String>,        // Path to violating node (e.g., ["elements", "0"])
}

impl ValidationError {
    /// Create a validation error with location path
    pub fn new(rule: &str, message: &str, location: Vec<String>) -> Self {
        Self {
            message: message.to_string(),
            rule_violated: rule.to_string(),
            location,
        }
    }
}
```

**Action Required**:
- Study `../gram-hs/libs/pattern/src/Pattern.hs` to verify error structure matches gram-hs
- Review test files to see how errors are constructed and what location format is used
- Ensure error messages match gram-hs format and style

### 5. Rust Implementation Strategy for Validation

**Task**: Determine idiomatic Rust patterns for implementing validation functions

**Findings**:
- **Decision**: Use `Result<(), ValidationError>` return type for validation functions
- **Rationale**: 
  - Rust's `Result` type is the idiomatic way to handle errors
  - Validation functions should return `Ok(())` for valid patterns, `Err(ValidationError)` for invalid patterns
  - This matches Rust error handling conventions and is compatible with `?` operator
- **Alternatives considered**:
  - Boolean return type - rejected (doesn't provide detailed error information)
  - Panic on invalid - rejected (not idiomatic, doesn't allow caller to handle errors)
  - Custom error trait - rejected (unnecessary complexity for this use case)

### 6. Rust Implementation Strategy for Structure Analysis

**Task**: Determine idiomatic Rust patterns for implementing structure analysis utilities

**Findings**:
- **Decision**: Use struct types for analysis results, return by value
- **Rationale**:
  - Structure analysis results may contain multiple pieces of information (depth distribution, element counts, etc.)
  - Struct types allow returning multiple values in a structured way
  - Returning by value is efficient for small structs and follows Rust ownership patterns
- **Alternatives considered**:
  - Return tuples - rejected (less readable, harder to extend)
  - Return references - rejected (lifetimes would be complex, analysis creates new data)
  - Mutable struct parameter - rejected (not idiomatic, analysis should be pure)

### 7. Performance Considerations for Deep Patterns

**Task**: Determine how to handle deep nesting without stack overflow

**Findings**:
- **Decision**: Use iterative algorithms or tail-recursive patterns where possible, with explicit stack management for very deep patterns
- **Rationale**:
  - Rust doesn't guarantee tail call optimization
  - Patterns can have 100+ nesting levels (per spec requirement)
  - Need to avoid stack overflow while maintaining performance
- **Alternatives considered**:
  - Recursive algorithms - acceptable for reasonable depths, but need fallback for very deep patterns
  - Iterative algorithms - preferred for very deep patterns, may be slightly slower for shallow patterns
  - Hybrid approach - use recursion for normal cases, iteration for very deep patterns

**Action Required**:
- Implement depth checking to switch to iterative algorithm if depth exceeds threshold
- Test with patterns of various depths to ensure no stack overflow
- Benchmark performance impact of iterative vs recursive approaches

## Resolved Clarifications

All NEEDS CLARIFICATION items from Technical Context have been addressed with implementation guidance based on spec requirements and existing code structure. Final verification required against actual gram-hs implementation:

- ✅ Validation function pattern: `validate(&self, rules: &ValidationRules) -> Result<(), ValidationError>` based on spec requirements and existing placeholder
- ✅ Structure analysis pattern: `analyze_structure(&self) -> StructureAnalysis` based on spec requirements for detailed structural information
- ✅ Validation rules structure: Use existing `ValidationRules` with `max_depth`, `max_elements`, `required_fields` based on spec requirements
- ✅ Error structure: Use existing `ValidationError` with `message`, `rule_violated`, `location` based on spec requirements

**Note**: All decisions are based on spec requirements and existing placeholder code. Final verification required by studying the actual gram-hs implementation in `../gram-hs/libs/pattern/src/Pattern.hs` to ensure exact behavioral equivalence.

### 8. Integration with Existing Pattern Type

**Task**: Determine how validation and analysis functions integrate with existing Pattern type

**Findings**:
- **Decision**: Add validation and analysis as methods or associated functions on `Pattern<V>`
- **Rationale**:
  - Pattern type is already defined in feature 004
  - Methods provide convenient API: `pattern.validate(&rules)?`
  - Associated functions allow generic implementations: `Pattern::validate(&pattern, &rules)?`
- **Alternatives considered**:
  - Standalone functions - acceptable but less ergonomic
  - Trait-based approach - rejected (unnecessary complexity for this use case)
  - Separate validation module - rejected (validation is core pattern functionality)

**Note**: The existing `Pattern<V>` type from feature 004 provides `length()`, `size()`, `depth()`, and `is_atomic()` methods. Validation and analysis functions should complement these, not duplicate them.

