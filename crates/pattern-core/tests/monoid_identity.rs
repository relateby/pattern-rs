//! Property-based tests for Pattern Default trait monoid identity laws
//!
//! These tests use proptest to verify that the default pattern satisfies
//! monoid identity laws for thousands of randomly generated patterns.
//!
//! # Monoid Identity Laws
//!
//! - **Left Identity**: `Pattern::default().combine(x) == x` for all patterns x
//! - **Right Identity**: `x.combine(Pattern::default()) == x` for all patterns x
//!
//! # Test Organization
//!
//! - **String Pattern Tests**: Test with String value types
//! - **Vec Pattern Tests**: Test with Vec<i32> value types
//! - **Nested Pattern Tests**: Test with deeply nested structures
//! - **Edge Case Tests**: Test specific edge cases

use pattern_core::{Combinable, Pattern};
use proptest::prelude::*;

// ============================================================================
// Pattern Generators for Property Testing
// ============================================================================

/// Strategy to generate arbitrary patterns with String values
///
/// Generates patterns with various structures:
/// - Atomic patterns (no elements)
/// - Patterns with 1-5 elements
/// - Nested patterns up to depth 3
fn arb_string_pattern() -> impl Strategy<Value = Pattern<String>> {
    let leaf = any::<String>().prop_map(Pattern::point);
    leaf.prop_recursive(
        3,  // max depth
        10, // max nodes
        5,  // max elements per node
        |inner| {
            (any::<String>(), prop::collection::vec(inner, 0..5))
                .prop_map(|(value, elements)| Pattern::pattern(value, elements))
        },
    )
}

/// Strategy to generate arbitrary patterns with Vec<i32> values
fn arb_vec_pattern() -> impl Strategy<Value = Pattern<Vec<i32>>> {
    let leaf = prop::collection::vec(any::<i32>(), 0..10).prop_map(Pattern::point);
    leaf.prop_recursive(
        3,  // max depth
        10, // max nodes
        5,  // max elements per node
        |inner| {
            (
                prop::collection::vec(any::<i32>(), 0..10),
                prop::collection::vec(inner, 0..5),
            )
                .prop_map(|(value, elements)| Pattern::pattern(value, elements))
        },
    )
}

/// Strategy to generate deeply nested string patterns (up to depth 10)
fn arb_deep_string_pattern() -> impl Strategy<Value = Pattern<String>> {
    let leaf = any::<String>().prop_map(Pattern::point);
    leaf.prop_recursive(
        10, // max depth (deep)
        20, // max nodes
        3,  // max elements per node
        |inner| {
            (any::<String>(), prop::collection::vec(inner, 0..3))
                .prop_map(|(value, elements)| Pattern::pattern(value, elements))
        },
    )
}

/// Strategy to generate wide string patterns (many elements, shallow depth)
fn arb_wide_string_pattern() -> impl Strategy<Value = Pattern<String>> {
    let leaf = any::<String>().prop_map(Pattern::point);
    leaf.prop_recursive(
        2,  // max depth (shallow)
        50, // max nodes (many)
        20, // max elements per node (wide)
        |inner| {
            (any::<String>(), prop::collection::vec(inner, 0..20))
                .prop_map(|(value, elements)| Pattern::pattern(value, elements))
        },
    )
}

// ============================================================================
// T024: Left Identity Law with String Patterns
// ============================================================================

proptest! {
    #[test]
    fn prop_left_identity_string(p in arb_string_pattern()) {
        let empty = Pattern::<String>::default();

        // Left identity: empty.combine(p) == p
        let result = empty.combine(p.clone());
        prop_assert_eq!(result, p);
    }
}

// ============================================================================
// T025: Right Identity Law with String Patterns
// ============================================================================

proptest! {
    #[test]
    fn prop_right_identity_string(p in arb_string_pattern()) {
        let empty = Pattern::<String>::default();

        // Right identity: p.combine(empty) == p
        let result = p.clone().combine(empty);
        prop_assert_eq!(result, p);
    }
}

// ============================================================================
// T026: Left Identity Law with Vec Patterns
// ============================================================================

proptest! {
    #[test]
    fn prop_left_identity_vec(p in arb_vec_pattern()) {
        let empty = Pattern::<Vec<i32>>::default();

        // Left identity: empty.combine(p) == p
        let result = empty.combine(p.clone());
        prop_assert_eq!(result, p);
    }
}

// ============================================================================
// T027: Right Identity Law with Vec Patterns
// ============================================================================

proptest! {
    #[test]
    fn prop_right_identity_vec(p in arb_vec_pattern()) {
        let empty = Pattern::<Vec<i32>>::default();

        // Right identity: p.combine(empty) == p
        let result = p.clone().combine(empty);
        prop_assert_eq!(result, p);
    }
}

// ============================================================================
// T028: Left Identity with Deeply Nested Patterns
// ============================================================================

proptest! {
    #[test]
    fn prop_left_identity_deep(p in arb_deep_string_pattern()) {
        let empty = Pattern::<String>::default();

        // Left identity holds even for deeply nested patterns
        let result = empty.combine(p.clone());
        prop_assert_eq!(result, p);
    }
}

// ============================================================================
// T029: Right Identity with Deeply Nested Patterns
// ============================================================================

proptest! {
    #[test]
    fn prop_right_identity_deep(p in arb_deep_string_pattern()) {
        let empty = Pattern::<String>::default();

        // Right identity holds even for deeply nested patterns
        let result = p.clone().combine(empty);
        prop_assert_eq!(result, p);
    }
}

// ============================================================================
// Additional Property Tests for Edge Cases
// ============================================================================

proptest! {
    #[test]
    fn prop_left_identity_wide(p in arb_wide_string_pattern()) {
        let empty = Pattern::<String>::default();

        // Left identity holds for wide patterns (many elements)
        let result = empty.combine(p.clone());
        prop_assert_eq!(result, p);
    }
}

proptest! {
    #[test]
    fn prop_right_identity_wide(p in arb_wide_string_pattern()) {
        let empty = Pattern::<String>::default();

        // Right identity holds for wide patterns (many elements)
        let result = p.clone().combine(empty);
        prop_assert_eq!(result, p);
    }
}

// ============================================================================
// T030: Identity Laws with Empty Elements (Atomic Patterns)
// ============================================================================

proptest! {
    #[test]
    fn prop_identity_atomic_string(value in any::<String>()) {
        let empty = Pattern::<String>::default();
        let p = Pattern::point(value);

        // Both identity laws hold for atomic patterns
        prop_assert_eq!(empty.clone().combine(p.clone()), p.clone());
        prop_assert_eq!(p.clone().combine(empty), p);
    }
}

proptest! {
    #[test]
    fn prop_identity_atomic_vec(value in prop::collection::vec(any::<i32>(), 0..100)) {
        let empty = Pattern::<Vec<i32>>::default();
        let p = Pattern::point(value);

        // Both identity laws hold for atomic patterns with vectors
        prop_assert_eq!(empty.clone().combine(p.clone()), p.clone());
        prop_assert_eq!(p.clone().combine(empty), p);
    }
}

// ============================================================================
// T031: Combining Default with Itself
// ============================================================================

#[test]
fn test_default_combine_default_string() {
    let empty1 = Pattern::<String>::default();
    let empty2 = Pattern::<String>::default();

    // Combining two defaults yields a default
    let result = empty1.combine(empty2.clone());
    assert_eq!(result, empty2);
    assert_eq!(result.value(), "");
    assert_eq!(result.length(), 0);
}

#[test]
fn test_default_combine_default_vec() {
    let empty1 = Pattern::<Vec<i32>>::default();
    let empty2 = Pattern::<Vec<i32>>::default();

    // Combining two defaults yields a default
    let result = empty1.combine(empty2.clone());
    assert_eq!(result, empty2);
    assert_eq!(result.length(), 0);
}

#[test]
fn test_default_combine_default_unit() {
    let empty1 = Pattern::<()>::default();
    let empty2 = Pattern::<()>::default();

    // Combining two defaults yields a default (trivial for unit)
    let result = empty1.combine(empty2.clone());
    assert_eq!(result, empty2);
}

// ============================================================================
// Identity Preservation Tests
// ============================================================================

proptest! {
    #[test]
    fn prop_identity_preserves_value(value in any::<String>()) {
        let empty = Pattern::<String>::default();
        let p = Pattern::point(value.clone());

        // Identity operations preserve the original value
        let left_result = empty.clone().combine(p.clone());
        let right_result = p.clone().combine(empty);

        prop_assert_eq!(left_result.value(), &value);
        prop_assert_eq!(right_result.value(), &value);
    }
}

proptest! {
    #[test]
    fn prop_identity_preserves_structure(
        value in any::<String>(),
        elements in prop::collection::vec(any::<String>().prop_map(Pattern::point), 0..10)
    ) {
        let empty = Pattern::<String>::default();
        let p = Pattern::pattern(value.clone(), elements.clone());

        // Identity operations preserve structure (value and element count)
        let left_result = empty.clone().combine(p.clone());
        let right_result = p.clone().combine(empty);

        prop_assert_eq!(left_result.value(), &value);
        prop_assert_eq!(left_result.length(), elements.len());
        prop_assert_eq!(right_result.value(), &value);
        prop_assert_eq!(right_result.length(), elements.len());
    }
}

proptest! {
    #[test]
    fn prop_identity_preserves_depth(p in arb_deep_string_pattern()) {
        let empty = Pattern::<String>::default();
        let original_depth = p.depth();

        // Identity operations preserve pattern depth
        let left_result = empty.clone().combine(p.clone());
        let right_result = p.clone().combine(empty);

        prop_assert_eq!(left_result.depth(), original_depth);
        prop_assert_eq!(right_result.depth(), original_depth);
    }
}

// ============================================================================
// Comprehensive Identity Verification
// ============================================================================

proptest! {
    /// Comprehensive test verifying both identity laws simultaneously
    #[test]
    fn prop_both_identity_laws_string(p in arb_string_pattern()) {
        let empty = Pattern::<String>::default();

        // Both laws must hold
        let left_identity = empty.clone().combine(p.clone());
        let right_identity = p.clone().combine(empty);

        prop_assert_eq!(left_identity, p.clone(), "Left identity failed");
        prop_assert_eq!(right_identity, p, "Right identity failed");
    }
}

proptest! {
    /// Comprehensive test verifying both identity laws for Vec patterns
    #[test]
    fn prop_both_identity_laws_vec(p in arb_vec_pattern()) {
        let empty = Pattern::<Vec<i32>>::default();

        // Both laws must hold
        let left_identity = empty.clone().combine(p.clone());
        let right_identity = p.clone().combine(empty);

        prop_assert_eq!(left_identity, p.clone(), "Left identity failed");
        prop_assert_eq!(right_identity, p, "Right identity failed");
    }
}
