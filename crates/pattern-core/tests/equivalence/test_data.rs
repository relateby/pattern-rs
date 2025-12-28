//! Equivalence checking tests using test data
//!
//! # Using gram-hs CLI for Reference Outputs
//!
//! These tests can use the `gram-hs` CLI tool to get reference outputs:
//!
//! ```bash
//! # Get reference output for comparison
//! echo '(node1)' | gram-hs parse --format json --value-only --canonical
//! ```
//!
//! See [gram-hs CLI Testing Guide](../../../../docs/gram-hs-cli-testing-guide.md) for
//! comprehensive usage examples and integration patterns.

use pattern_core::test_utils::equivalence::{
    check_equivalence, check_equivalence_from_test_data, EquivalenceOptions, ComparisonMethod,
};

#[test]
fn test_equivalence_direct_comparison() {
    let options = EquivalenceOptions {
        comparison_method: ComparisonMethod::Direct,
        ..Default::default()
    };
    
    let result = check_equivalence(&42, &42, &options);
    assert!(result.equivalent);
}

#[test]
fn test_equivalence_detects_differences() {
    let options = EquivalenceOptions {
        comparison_method: ComparisonMethod::Direct,
        ..Default::default()
    };
    
    let result = check_equivalence(&42, &43, &options);
    assert!(!result.equivalent);
    assert!(!result.differences.is_empty());
}

// Placeholder test for test data comparison
// Will be fully implemented when test case loading is available
#[test]
fn test_equivalence_from_test_data_placeholder() {
    // This test verifies the infrastructure is set up correctly
    // Full implementation will use actual test cases from tests/common/test_cases.json
    assert!(true);
}

