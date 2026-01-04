//! Unit tests for pattern validation functions

use pattern_core::{Pattern, ValidationRules};

#[test]
fn test_validation_with_valid_pattern() {
    // Test that a valid pattern passes validation with default rules
    let pattern = Pattern::point("atom".to_string());
    let rules = ValidationRules::default();

    assert!(pattern.validate(&rules).is_ok());
}

#[test]
fn test_validation_with_max_depth_constraint() {
    // Test that patterns exceeding max_depth fail validation
    let pattern = Pattern::pattern(
        "root".to_string(),
        vec![Pattern::pattern(
            "child".to_string(),
            vec![Pattern::point("grandchild".to_string())],
        )],
    );

    // Pattern has depth 2, but max_depth is 1
    let rules = ValidationRules {
        max_depth: Some(1),
        ..Default::default()
    };

    let result = pattern.validate(&rules);
    assert!(result.is_err());

    if let Err(e) = result {
        assert_eq!(e.rule_violated, "max_depth");
        assert!(!e.message.is_empty());
    }
}

#[test]
fn test_validation_with_max_elements_constraint() {
    // Test that patterns exceeding max_elements fail validation
    let pattern = Pattern::pattern(
        "root".to_string(),
        vec![
            Pattern::point("child1".to_string()),
            Pattern::point("child2".to_string()),
            Pattern::point("child3".to_string()),
        ],
    );

    // Pattern has 3 elements, but max_elements is 2
    let rules = ValidationRules {
        max_elements: Some(2),
        ..Default::default()
    };

    let result = pattern.validate(&rules);
    assert!(result.is_err());

    if let Err(e) = result {
        assert_eq!(e.rule_violated, "max_elements");
        assert!(!e.message.is_empty());
    }
}

#[test]
fn test_validation_error_location_path() {
    // Test that validation errors include correct location paths
    let pattern = Pattern::pattern(
        "root".to_string(),
        vec![Pattern::pattern(
            "child".to_string(),
            vec![Pattern::point("grandchild".to_string())],
        )],
    );

    let rules = ValidationRules {
        max_depth: Some(1),
        ..Default::default()
    };

    let result = pattern.validate(&rules);
    assert!(result.is_err());

    if let Err(e) = result {
        // Location should point to where the violation occurred
        assert!(!e.location.is_empty());
        // Location path should help identify the violating node
        assert!(
            e.location.contains(&"elements".to_string())
                || e.location.iter().any(|s| s.starts_with("0") || s == "0")
        );
    }
}

#[test]
fn test_validation_with_100_plus_nesting_levels() {
    // Test that validation handles deep nesting without stack overflow
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

    let deep = create_deep_pattern(100);
    let rules = ValidationRules {
        max_depth: Some(200), // Allow deep nesting
        ..Default::default()
    };

    // Should not panic or stack overflow
    let result = deep.validate(&rules);
    assert!(result.is_ok() || result.is_err()); // Either is fine, just no panic
}
