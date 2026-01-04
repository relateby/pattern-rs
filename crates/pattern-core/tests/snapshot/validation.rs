//! Snapshot tests for validation error messages
//!
//! These tests use insta to capture and verify validation error messages
//! remain consistent across changes.

use pattern_core::{Pattern, ValidationRules};

#[test]
fn test_validation_error_max_depth() {
    let pattern = Pattern::pattern("root".to_string(), vec![
        Pattern::pattern("child".to_string(), vec![
            Pattern::point("grandchild".to_string()),
        ]),
    ]);
    
    let rules = ValidationRules {
        max_depth: Some(1),
        ..Default::default()
    };
    
    let result = pattern.validate(&rules);
    assert!(result.is_err());
    
    if let Err(e) = result {
        insta::assert_snapshot!("validation_error_max_depth", e.message);
        insta::assert_snapshot!("validation_error_max_depth_rule", e.rule_violated);
        insta::assert_snapshot!("validation_error_max_depth_location", format!("{:?}", e.location));
    }
}

#[test]
fn test_validation_error_max_elements() {
    let pattern = Pattern::pattern("root".to_string(), vec![
        Pattern::point("child1".to_string()),
        Pattern::point("child2".to_string()),
        Pattern::point("child3".to_string()),
    ]);
    
    let rules = ValidationRules {
        max_elements: Some(2),
        ..Default::default()
    };
    
    let result = pattern.validate(&rules);
    assert!(result.is_err());
    
    if let Err(e) = result {
        insta::assert_snapshot!("validation_error_max_elements", e.message);
        insta::assert_snapshot!("validation_error_max_elements_rule", e.rule_violated);
    }
}

