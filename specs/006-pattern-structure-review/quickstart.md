# Quickstart: Pattern Structure Validation

**Feature**: 006-pattern-structure-review  
**Date**: 2025-01-27

This guide provides quick examples for using pattern validation functions and structure analysis utilities. All functions must match the gram-hs reference implementation (to be verified during implementation).

## Basic Usage

### Validating Patterns

#### Basic Validation

Validate a pattern with default rules (no constraints):

```rust
use pattern_core::{Pattern, ValidationRules};

let pattern = Pattern::pattern("root".to_string(), vec![
    Pattern::point("child".to_string()),
]);

// No constraints - all patterns valid
let rules = ValidationRules::default();
match pattern.validate(&rules) {
    Ok(()) => println!("Pattern is valid"),
    Err(e) => println!("Validation failed: {}", e.message),
}
```

#### Validating with Depth Constraint

Validate a pattern with maximum depth constraint:

```rust
use pattern_core::{Pattern, ValidationRules};

let pattern = Pattern::pattern("root".to_string(), vec![
    Pattern::pattern("child".to_string(), vec![
        Pattern::point("grandchild".to_string()),
    ]),
]);

let rules = ValidationRules {
    max_depth: Some(2),  // Maximum depth of 2
    ..Default::default()
};

match pattern.validate(&rules) {
    Ok(()) => println!("Pattern depth is within limit"),
    Err(e) => {
        println!("Validation failed: {}", e.message);
        println!("Rule violated: {}", e.rule_violated);
        println!("Location: {:?}", e.location);
    },
}
```

#### Validating with Element Count Constraint

Validate a pattern with maximum element count constraint:

```rust
use pattern_core::{Pattern, ValidationRules};

let pattern = Pattern::pattern("root".to_string(), vec![
    Pattern::point("child1".to_string()),
    Pattern::point("child2".to_string()),
    Pattern::point("child3".to_string()),
]);

let rules = ValidationRules {
    max_elements: Some(2),  // Maximum 2 elements per level
    ..Default::default()
};

match pattern.validate(&rules) {
    Ok(()) => println!("Pattern element counts are within limit"),
    Err(e) => {
        println!("Validation failed: {}", e.message);
        println!("Violation at: {:?}", e.location);
    },
}
```

#### Validating with Multiple Constraints

Validate a pattern with multiple constraints:

```rust
use pattern_core::{Pattern, ValidationRules};

let pattern = Pattern::pattern("root".to_string(), vec![
    Pattern::point("child".to_string()),
]);

let rules = ValidationRules {
    max_depth: Some(10),
    max_elements: Some(100),
    ..Default::default()
};

match pattern.validate(&rules) {
    Ok(()) => println!("Pattern meets all constraints"),
    Err(e) => println!("Validation failed: {}", e.message),
}
```

### Analyzing Pattern Structure

#### Basic Structure Analysis

Analyze a pattern's structure:

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern("root".to_string(), vec![
    Pattern::pattern("child1".to_string(), vec![
        Pattern::point("grandchild1".to_string()),
    ]),
    Pattern::pattern("child2".to_string(), vec![
        Pattern::point("grandchild2".to_string()),
    ]),
]);

let analysis = pattern.analyze_structure();

println!("Depth distribution: {:?}", analysis.depth_distribution);
println!("Element counts: {:?}", analysis.element_counts);
println!("Nesting patterns: {:?}", analysis.nesting_patterns);
println!("Summary: {}", analysis.summary);
```

#### Understanding Analysis Results

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern("root".to_string(), vec![
    Pattern::point("child1".to_string()),
    Pattern::point("child2".to_string()),
    Pattern::point("child3".to_string()),
]);

let analysis = pattern.analyze_structure();

// Depth distribution: count of nodes at each depth level
// [1, 3] means: 1 node at depth 0 (root), 3 nodes at depth 1 (children)
println!("Nodes at each depth: {:?}", analysis.depth_distribution);

// Element counts: element count at each level
// [3] means: 3 elements at root level
println!("Elements at each level: {:?}", analysis.element_counts);

// Nesting patterns: identified structural patterns
// e.g., ["linear", "balanced"] for different parts of the structure
println!("Structural patterns: {:?}", analysis.nesting_patterns);

// Summary: human-readable description
// e.g., "Pattern with 2 levels, 4 nodes, tree-like structure"
println!("Summary: {}", analysis.summary);
```

## Advanced Patterns

### Validating Deeply Nested Patterns

```rust
use pattern_core::{Pattern, ValidationRules};

fn create_deep_pattern(depth: usize) -> Pattern<String> {
    if depth == 0 {
        Pattern::point("leaf".to_string())
    } else {
        Pattern::pattern(
            format!("level{}", depth).to_string(),
            vec![create_deep_pattern(depth - 1)],
        )
    }
}

let deep = create_deep_pattern(5);
let rules = ValidationRules {
    max_depth: Some(10),
    ..Default::default()
};

match deep.validate(&rules) {
    Ok(()) => println!("Deep pattern is valid"),
    Err(e) => println!("Too deep: {}", e.message),
}
```

### Analyzing Complex Structures

```rust
use pattern_core::Pattern;

// Create a complex nested structure
let complex = Pattern::pattern("root".to_string(), vec![
    Pattern::pattern("branch1".to_string(), vec![
        Pattern::point("leaf1".to_string()),
        Pattern::point("leaf2".to_string()),
    ]),
    Pattern::pattern("branch2".to_string(), vec![
        Pattern::point("leaf3".to_string()),
    ]),
    Pattern::point("leaf4".to_string()),
]);

let analysis = complex.analyze_structure();

// Understand the structure
println!("Total nodes: {}", analysis.depth_distribution.iter().sum::<usize>());
println!("Maximum depth: {}", analysis.depth_distribution.len() - 1);
println!("Structure type: {:?}", analysis.nesting_patterns);
```

### Combining Validation and Analysis

```rust
use pattern_core::{Pattern, ValidationRules};

let pattern = Pattern::pattern("root".to_string(), vec![/* ... */]);

// First, analyze the structure
let analysis = pattern.analyze_structure();
println!("Structure summary: {}", analysis.summary);

// Then, validate based on analysis
let max_depth = analysis.depth_distribution.len();
let rules = ValidationRules {
    max_depth: Some(max_depth + 5),  // Allow 5 more levels
    ..Default::default()
};

match pattern.validate(&rules) {
    Ok(()) => println!("Pattern structure is acceptable"),
    Err(e) => println!("Structure validation failed: {}", e.message),
}
```

## Common Patterns

### Validating Patterns from External Sources

```rust
use pattern_core::{Pattern, ValidationRules};

fn validate_external_pattern(pattern: &Pattern<String>) -> Result<(), String> {
    // Strict validation rules for external data
    let rules = ValidationRules {
        max_depth: Some(50),
        max_elements: Some(1000),
        ..Default::default()
    };

    pattern.validate(&rules)
        .map_err(|e| format!("Invalid pattern: {} at {:?}", e.message, e.location))
}

let external_pattern = Pattern::pattern("data".to_string(), vec![/* ... */]);
match validate_external_pattern(&external_pattern) {
    Ok(()) => println!("External pattern is valid"),
    Err(msg) => println!("Rejected: {}", msg),
}
```

### Analyzing Patterns for Debugging

```rust
use pattern_core::Pattern;

fn debug_pattern_structure(pattern: &Pattern<String>) {
    let analysis = pattern.analyze_structure();
    
    println!("=== Pattern Structure Debug ===");
    println!("Summary: {}", analysis.summary);
    println!("Depth levels: {}", analysis.depth_distribution.len());
    println!("Total nodes: {}", analysis.depth_distribution.iter().sum::<usize>());
    println!("Depth distribution: {:?}", analysis.depth_distribution);
    println!("Element counts: {:?}", analysis.element_counts);
    println!("Patterns identified: {:?}", analysis.nesting_patterns);
}

let pattern = Pattern::pattern("root".to_string(), vec![/* ... */]);
debug_pattern_structure(&pattern);
```

### Custom Validation Logic

```rust
use pattern_core::{Pattern, ValidationRules, ValidationError};

fn validate_with_custom_rules(pattern: &Pattern<String>) -> Result<(), ValidationError> {
    // First, check basic constraints
    let basic_rules = ValidationRules {
        max_depth: Some(100),
        max_elements: Some(10000),
        ..Default::default()
    };
    
    pattern.validate(&basic_rules)?;
    
    // Then, add custom validation logic
    let analysis = pattern.analyze_structure();
    if analysis.depth_distribution.len() > 50 {
        return Err(ValidationError {
            message: "Pattern exceeds custom depth limit".to_string(),
            rule_violated: "custom_depth_limit".to_string(),
            location: vec![],
        });
    }
    
    Ok(())
}
```

## Testing Examples

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use pattern_core::{Pattern, ValidationRules};

    #[test]
    fn test_validation_success() {
        let pattern = Pattern::point("atom".to_string());
        let rules = ValidationRules::default();
        assert!(pattern.validate(&rules).is_ok());
    }

    #[test]
    fn test_validation_depth_failure() {
        let pattern = Pattern::pattern("root".to_string(), vec![
            Pattern::pattern("child".to_string(), vec![
                Pattern::point("grandchild".to_string()),
            ]),
        ]);
        
        let rules = ValidationRules {
            max_depth: Some(1),  // Pattern has depth 2
            ..Default::default()
        };
        
        let result = pattern.validate(&rules);
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.rule_violated, "max_depth");
        }
    }

    #[test]
    fn test_analysis_basic() {
        let pattern = Pattern::point("atom".to_string());
        let analysis = pattern.analyze_structure();
        
        assert_eq!(analysis.depth_distribution, vec![1]);  // 1 node at depth 0
        assert_eq!(analysis.element_counts, vec![]);  // No elements
    }

    #[test]
    fn test_analysis_nested() {
        let pattern = Pattern::pattern("root".to_string(), vec![
            Pattern::point("child1".to_string()),
            Pattern::point("child2".to_string()),
        ]);
        
        let analysis = pattern.analyze_structure();
        
        assert_eq!(analysis.depth_distribution, vec![1, 2]);  // 1 at depth 0, 2 at depth 1
        assert_eq!(analysis.element_counts, vec![2]);  // 2 elements at root
    }
}
```

## Performance Considerations

- **Validation**: O(n) where n is the number of nodes. Must handle at least 100 nesting levels without stack overflow.
- **Analysis**: O(n) where n is the number of nodes. Must handle at least 100 nesting levels without stack overflow. Must handle at least 10,000 elements efficiently.
- Both operations traverse the entire pattern structure recursively.

## WASM Usage

All functions compile for WebAssembly:

```bash
cargo build --package pattern-core --target wasm32-unknown-unknown
```

The functions can be used in WASM modules, though JavaScript bindings are deferred to later features.

## Behavioral Equivalence with gram-hs

All functions must maintain behavioral equivalence with the gram-hs reference implementation:
- Validation functions must produce identical validation results for identical inputs and rules
- Analysis functions must produce identical analysis results for identical inputs

Reference: `../gram-hs/libs/pattern/src/Pattern.hs` (to be verified during implementation)

## Next Steps

- See `specs/004-pattern-data-structure/` for Pattern type definition
- See `specs/005-basic-pattern-type/` for construction, access, and basic inspection
- See `PORTING_GUIDE.md` for porting workflow
- See `docs/gram-rs-project-plan.md` for overall architecture

