# Data Model: Pattern Structure Validation

**Feature**: 006-pattern-structure-review  
**Date**: 2025-01-27

## Overview

This document defines the validation functions and structure analysis utilities for the Pattern type. These functions operate on the existing `Pattern<V>` type defined in feature 004, adding validation and analysis capabilities for pattern structure. All functions must match the gram-hs reference implementation in `../pattern-hs/libs/pattern/src/Pattern.hs` (to be verified during implementation).

## Core Entities

### ValidationRules

Configurable constraints that define what constitutes valid pattern structure. Validation rules can specify limits on nesting depth, element counts, or other structural properties. Rules can be customized for different use cases and applied during validation.

**Structure**:
```rust
#[derive(Debug, Clone, Default)]
pub struct ValidationRules {
    pub max_depth: Option<usize>,        // Maximum nesting depth (None = no limit)
    pub max_elements: Option<usize>,      // Maximum element count (None = no limit)
    pub required_fields: Vec<String>,     // Required fields (for future value-specific validation)
}
```

**Characteristics**:
- **Optional constraints**: `Option<usize>` allows optional limits (None means no limit)
- **Configurable**: Rules can be customized for different use cases
- **Extensible**: Additional rule types can be added as needed
- **Default**: `Default` implementation provides rules with no constraints

**Usage**:
```rust
// No constraints (all patterns valid)
let rules = ValidationRules::default();

// Maximum depth constraint
let rules = ValidationRules {
    max_depth: Some(10),
    ..Default::default()
};

// Multiple constraints
let rules = ValidationRules {
    max_depth: Some(100),
    max_elements: Some(10000),
    ..Default::default()
};
```

### ValidationError

Error type for pattern validation failures. Provides detailed information about what rule was violated and where in the pattern structure the violation occurred.

**Structure**:
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub message: String,              // Human-readable error message
    pub rule_violated: String,        // Name of violated rule (e.g., "max_depth")
    pub location: Vec<String>,        // Path to violating node (e.g., ["elements", "0"])
}
```

**Characteristics**:
- **Detailed error information**: Includes message, rule name, and location
- **Location path**: Vector of strings representing path to violating node
- **Cloneable**: Can be cloned for error propagation
- **Equatable**: Can be compared for testing

**Location Path Format**:
- Empty vector `[]` represents the root pattern
- `["elements", "0"]` represents the first element of the root pattern
- `["elements", "0", "elements", "1"]` represents the second element of the first element
- Path components alternate between "elements" and indices

**Usage**:
```rust
let error = ValidationError {
    message: "Pattern depth exceeds maximum".to_string(),
    rule_violated: "max_depth".to_string(),
    location: vec!["elements".to_string(), "0".to_string()],
};
```

### StructureAnalysis

Results from structure analysis utilities. Provides detailed information about pattern structural characteristics including depth distribution, element counts, nesting patterns, and structural summaries.

**Structure**:
```rust
#[derive(Debug, Clone)]
pub struct StructureAnalysis {
    pub depth_distribution: Vec<usize>,  // Count of nodes at each depth level
    pub element_counts: Vec<usize>,      // Element counts at each level
    pub nesting_patterns: Vec<String>,    // Identified structural patterns
    pub summary: String,                 // Human-readable summary
}
```

**Characteristics**:
- **Depth distribution**: Vector where index is depth level and value is node count at that depth
- **Element counts**: Vector where index is depth level and value is element count at that depth
- **Nesting patterns**: Identified structural patterns (e.g., "linear", "tree", "balanced")
- **Summary**: Human-readable text summary of structure

**Usage**:
```rust
let analysis = pattern.analyze_structure();
println!("Depth distribution: {:?}", analysis.depth_distribution);
println!("Summary: {}", analysis.summary);
```

## Core Functions and Methods

### Pattern Validation Functions

Validation functions check whether patterns conform to expected structural rules and constraints.

#### `Pattern::validate(&self, rules: &ValidationRules) -> Result<(), ValidationError>`

Validates pattern structure against configurable rules and constraints. Returns `Ok(())` if the pattern is valid, or `Err(ValidationError)` if validation fails.

**Signature**:
```rust
impl<V> Pattern<V> {
    pub fn validate(&self, rules: &ValidationRules) -> Result<(), ValidationError>
}
```

**Parameters**:
- `self: &Pattern<V>` - The pattern to validate
- `rules: &ValidationRules` - Validation rules to apply

**Returns**: `Result<(), ValidationError>` - `Ok(())` if valid, `Err(ValidationError)` if invalid

**Characteristics**:
- Generic over value type `V`
- Validates entire recursive structure
- Returns detailed error information on failure
- O(n) operation where n is the number of nodes in the pattern
- Must handle at least 100 nesting levels without stack overflow

**Validation Rules Applied**:
- `max_depth`: Checks if pattern depth exceeds maximum (if specified)
- `max_elements`: Checks if any level has more elements than maximum (if specified)
- `required_fields`: Reserved for future value-specific validation

**Usage**:
```rust
let pattern = Pattern::pattern("root".to_string(), vec![/* ... */]);

let rules = ValidationRules {
    max_depth: Some(10),
    ..Default::default()
};

match pattern.validate(&rules) {
    Ok(()) => println!("Pattern is valid"),
    Err(e) => println!("Validation failed: {} at {:?}", e.message, e.location),
}
```

### Structure Analysis Utilities

Structure analysis utilities provide detailed information about pattern structural characteristics beyond basic inspection.

#### `Pattern::analyze_structure(&self) -> StructureAnalysis`

Analyzes pattern structure and returns detailed information about structural characteristics including depth distribution, element counts, nesting patterns, and structural summaries.

**Signature**:
```rust
impl<V> Pattern<V> {
    pub fn analyze_structure(&self) -> StructureAnalysis
}
```

**Parameters**:
- `self: &Pattern<V>` - The pattern to analyze

**Returns**: `StructureAnalysis` - Detailed structural analysis results

**Characteristics**:
- Generic over value type `V`
- Analyzes entire recursive structure
- Provides comprehensive structural information
- O(n) operation where n is the number of nodes in the pattern
- Must handle at least 100 nesting levels without stack overflow
- Must handle at least 10,000 elements efficiently

**Analysis Components**:
- **Depth distribution**: Count of nodes at each depth level (e.g., `[1, 3, 5]` means 1 node at depth 0, 3 at depth 1, 5 at depth 2)
- **Element counts**: Element count at each level (e.g., `[3, 5]` means 3 elements at root, 5 at first level)
- **Nesting patterns**: Identified structural patterns (e.g., "linear", "tree", "balanced", "irregular")
- **Summary**: Human-readable text summary (e.g., "Pattern with 3 levels, 10 nodes, tree-like structure")

**Usage**:
```rust
let pattern = Pattern::pattern("root".to_string(), vec![/* ... */]);
let analysis = pattern.analyze_structure();

println!("Depth distribution: {:?}", analysis.depth_distribution);
println!("Element counts: {:?}", analysis.element_counts);
println!("Nesting patterns: {:?}", analysis.nesting_patterns);
println!("Summary: {}", analysis.summary);
```

## Relationships

### Validation → Pattern Structure

Validation functions operate on pattern structure:
- `pattern.validate(&rules)` validates the entire recursive structure
- Validation checks structural properties (depth, element counts) not value properties
- Validation is independent of value type `V`

### Analysis → Pattern Structure

Structure analysis operates on pattern structure:
- `pattern.analyze_structure()` analyzes the entire recursive structure
- Analysis provides insights into structural characteristics
- Analysis complements basic inspection (length, size, depth from feature 005)

### Validation Rules → Validation

Validation rules configure validation behavior:
- `ValidationRules` specifies constraints to check
- Rules are applied during validation
- Rules can be customized for different use cases

### Validation Error → Validation

Validation errors report validation failures:
- `ValidationError` provides detailed failure information
- Error includes rule violated and location in pattern structure
- Errors help developers identify and fix structural issues

## Validation Rules

### Pattern Structure Validation

1. **Max Depth Constraint**: If `max_depth` is specified, pattern depth must not exceed the limit
2. **Max Elements Constraint**: If `max_elements` is specified, no level should have more elements than the limit
3. **Required Fields**: Reserved for future value-specific validation (not used for structural validation)

### Validation Process

1. **Depth Check**: Recursively calculate depth and check against `max_depth` if specified
2. **Element Count Check**: Recursively check element counts at each level against `max_elements` if specified
3. **Error Reporting**: If validation fails, construct `ValidationError` with rule name, message, and location path

## State Transitions

N/A - Validation and analysis functions operate on immutable pattern instances. No state transitions involved.

## Constraints

- All functions must work generically with any value type `V` that Pattern supports
- Validation functions must handle at least 100 nesting levels without stack overflow
- Analysis functions must handle at least 10,000 elements efficiently
- Validation and analysis must maintain behavioral equivalence with gram-hs reference implementation
- Functions must compile for `wasm32-unknown-unknown` target
- Error messages must be clear and actionable

## Edge Cases

- **Empty patterns**: Atomic patterns (no elements) have depth 0 and should validate successfully
- **Very deep patterns**: Must handle 100+ nesting levels without stack overflow (use iterative algorithms if needed)
- **Very large patterns**: Must handle 10,000+ elements efficiently
- **No constraints**: `ValidationRules::default()` should accept all patterns
- **Conflicting rules**: Rules are independent (no conflict detection needed)
- **Invalid rules**: Invalid rule values (e.g., negative max_depth) should be handled gracefully
