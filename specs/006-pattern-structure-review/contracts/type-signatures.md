# Type Signatures: Pattern Structure Validation

**Feature**: 006-pattern-structure-review  
**Date**: 2025-01-27

## Overview

This document defines the public API type signatures for pattern validation functions and structure analysis utilities. These serve as the contracts that define the interface users will interact with. All signatures must match the gram-hs reference implementation in `../gram-hs/libs/pattern/src/Pattern.hs` (to be verified during implementation).

## Core Module: pattern_core

### Validation Types

#### `ValidationRules`

Configurable constraints that define what constitutes valid pattern structure.

```rust
/// Configurable validation rules for pattern structure.
///
/// Rules can specify limits on nesting depth, element counts, or other structural properties.
/// Rules are optional (None means no limit).
///
/// # Examples
///
/// ```
/// use pattern_core::ValidationRules;
///
/// // No constraints (all patterns valid)
/// let rules = ValidationRules::default();
///
/// // Maximum depth constraint
/// let rules = ValidationRules {
///     max_depth: Some(10),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default)]
pub struct ValidationRules {
    /// Maximum nesting depth allowed (None = no limit)
    pub max_depth: Option<usize>,
    /// Maximum element count allowed (None = no limit)
    pub max_elements: Option<usize>,
    /// Required fields (reserved for future value-specific validation)
    pub required_fields: Vec<String>,
}
```

#### `ValidationError`

Error type for pattern validation failures.

```rust
/// Error type for pattern validation failures.
///
/// Provides detailed information about what rule was violated and where
/// in the pattern structure the violation occurred.
///
/// # Examples
///
/// ```
/// use pattern_core::ValidationError;
///
/// let error = ValidationError {
///     message: "Pattern depth exceeds maximum".to_string(),
///     rule_violated: "max_depth".to_string(),
///     location: vec!["elements".to_string(), "0".to_string()],
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    /// Human-readable error message
    pub message: String,
    /// Name of violated rule (e.g., "max_depth", "max_elements")
    pub rule_violated: String,
    /// Path to violating node in pattern structure
    pub location: Vec<String>,
}
```

### Structure Analysis Types

#### `StructureAnalysis`

Results from structure analysis utilities.

```rust
/// Results from structure analysis utilities.
///
/// Provides detailed information about pattern structural characteristics
/// including depth distribution, element counts, nesting patterns, and summaries.
///
/// # Examples
///
/// ```
/// use pattern_core::{Pattern, StructureAnalysis};
///
/// let pattern = Pattern::pattern("root".to_string(), vec![/* ... */]);
/// let analysis = pattern.analyze_structure();
///
/// println!("Depth distribution: {:?}", analysis.depth_distribution);
/// println!("Summary: {}", analysis.summary);
/// ```
#[derive(Debug, Clone)]
pub struct StructureAnalysis {
    /// Count of nodes at each depth level (index = depth, value = count)
    pub depth_distribution: Vec<usize>,
    /// Element counts at each level (index = level, value = count)
    pub element_counts: Vec<usize>,
    /// Identified structural patterns (e.g., "linear", "tree", "balanced")
    pub nesting_patterns: Vec<String>,
    /// Human-readable summary of structure
    pub summary: String,
}
```

### Pattern Validation Functions

#### `Pattern::validate`

Validates pattern structure against configurable rules and constraints.

```rust
impl<V> Pattern<V> {
    /// Validates pattern structure against configurable rules and constraints.
    ///
    /// Returns `Ok(())` if the pattern is valid according to the rules,
    /// or `Err(ValidationError)` if validation fails.
    ///
    /// # Arguments
    ///
    /// * `rules` - Validation rules to apply
    ///
    /// # Returns
    ///
    /// * `Ok(())` if pattern is valid
    /// * `Err(ValidationError)` if validation fails, containing detailed error information
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::{Pattern, ValidationRules};
    ///
    /// let pattern = Pattern::pattern("root".to_string(), vec![/* ... */]);
    ///
    /// let rules = ValidationRules {
    ///     max_depth: Some(10),
    ///     ..Default::default()
    /// };
    ///
    /// match pattern.validate(&rules) {
    ///     Ok(()) => println!("Pattern is valid"),
    ///     Err(e) => println!("Validation failed: {} at {:?}", e.message, e.location),
    /// }
    /// ```
    ///
    /// # Performance
    ///
    /// This operation is O(n) where n is the number of nodes in the pattern.
    /// Must handle at least 100 nesting levels without stack overflow.
    pub fn validate(&self, rules: &ValidationRules) -> Result<(), ValidationError>;
}
```

### Structure Analysis Utilities

#### `Pattern::analyze_structure`

Analyzes pattern structure and returns detailed information about structural characteristics.

```rust
impl<V> Pattern<V> {
    /// Analyzes pattern structure and returns detailed information about structural characteristics.
    ///
    /// Provides comprehensive structural analysis including depth distribution, element counts,
    /// nesting patterns, and a human-readable summary.
    ///
    /// # Returns
    ///
    /// `StructureAnalysis` containing:
    /// - Depth distribution: Count of nodes at each depth level
    /// - Element counts: Element count at each level
    /// - Nesting patterns: Identified structural patterns
    /// - Summary: Human-readable text summary
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern("root".to_string(), vec![/* ... */]);
    /// let analysis = pattern.analyze_structure();
    ///
    /// println!("Depth distribution: {:?}", analysis.depth_distribution);
    /// println!("Element counts: {:?}", analysis.element_counts);
    /// println!("Nesting patterns: {:?}", analysis.nesting_patterns);
    /// println!("Summary: {}", analysis.summary);
    /// ```
    ///
    /// # Performance
    ///
    /// This operation is O(n) where n is the number of nodes in the pattern.
    /// Must handle at least 100 nesting levels without stack overflow.
    /// Must handle at least 10,000 elements efficiently.
    pub fn analyze_structure(&self) -> StructureAnalysis;
}
```

## Type Constraints

All functions are generic over value type `V` with no trait bounds (beyond what `Pattern<V>` itself requires). The functions work with any value type that can be used with `Pattern<V>`.

## Error Handling

- **Validation functions**: Return `Result<(), ValidationError>` - `Ok(())` for valid patterns, `Err(ValidationError)` for invalid patterns
- **Analysis functions**: Return `StructureAnalysis` by value - always succeeds (analysis is always possible)

## Performance Characteristics

- **Validation**: O(n) where n is the number of nodes in the pattern. Must handle at least 100 nesting levels without stack overflow.
- **Analysis**: O(n) where n is the number of nodes in the pattern. Must handle at least 100 nesting levels without stack overflow. Must handle at least 10,000 elements efficiently.

## WASM Compatibility

All functions are compatible with WebAssembly targets. They use only standard library types and operations that compile to WASM.

## Behavioral Equivalence

All functions must maintain behavioral equivalence with the gram-hs reference implementation:
- Validation functions must produce identical validation results for identical inputs and rules
- Analysis functions must produce identical analysis results for identical inputs

Reference implementation: `../gram-hs/libs/pattern/src/Pattern.hs` (to be verified during implementation)

