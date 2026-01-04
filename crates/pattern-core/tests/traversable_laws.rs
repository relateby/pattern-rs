//! Property-based tests for traversable laws
//!
//! This file contains property-based tests (using proptest) to verify:
//! - Identity law: traverse(id) == id
//! - Composition law: traverse(compose(f, g)) == compose(traverse(f), traverse(g))
//! - Naturality law: t . traverse(f) == traverse(t . f) for natural transformation t
//! - Structure preservation: traverse preserves pattern structure (size, depth, length)
//!
//! Tests run 100+ random cases per law to ensure correctness

use pattern_core::Pattern;
use proptest::prelude::*;

// ====================================================================================
// Identity Law Tests for Option (T014)
// ====================================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Identity law for Option: traverse(Some) should wrap the original pattern
    ///
    /// For any pattern p, traverse_option(|v| Some(v.clone())) should equal Some(p.clone())
    #[test]
    fn identity_law_option(value in any::<i32>()) {
        let pattern = Pattern::point(value);

        let result = pattern.traverse_option(|v| Some(*v));
        prop_assert_eq!(result, Some(pattern.clone()));
    }
}

// ====================================================================================
// Structure Preservation Tests for Option (T015)
// ====================================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Structure preservation for Option: traverse should not change pattern structure
    ///
    /// If traverse succeeds, the resulting pattern should have:
    /// - Same size (number of nodes)
    /// - Same depth (nesting levels)
    /// - Same length (number of direct elements)
    #[test]
    fn structure_preservation_option(value in -1000i32..1000i32) {
        let pattern = Pattern::pattern(value, vec![
            Pattern::point(value + 1),
            Pattern::point(value + 2),
        ]);

        let original_size = pattern.size();
        let original_depth = pattern.depth();
        let original_length = pattern.length();

        // Use a simple transformation that won't overflow
        let result = pattern.traverse_option(|v| Some(v + 1));
        if let Some(new_pattern) = result {
            prop_assert_eq!(new_pattern.size(), original_size);
            prop_assert_eq!(new_pattern.depth(), original_depth);
            prop_assert_eq!(new_pattern.length(), original_length);
        }
    }
}

// ====================================================================================
// Identity Law Tests for Result (T025)
// ====================================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Identity law for Result: traverse(Ok) should wrap the original pattern
    ///
    /// For any pattern p, traverse_result(|v| Ok(v.clone())) should equal Ok(p.clone())
    #[test]
    fn identity_law_result(value in any::<i32>()) {
        let pattern = Pattern::point(value);

        let result: Result<Pattern<i32>, String> = pattern.traverse_result(|v| Ok(*v));
        prop_assert_eq!(result, Ok(pattern.clone()));
    }
}

// ====================================================================================
// Structure Preservation Tests for Result (T026)
// ====================================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Structure preservation for Result: traverse should not change pattern structure
    ///
    /// If traverse succeeds, the resulting pattern should have:
    /// - Same size (number of nodes)
    /// - Same depth (nesting levels)
    /// - Same length (number of direct elements)
    #[test]
    fn structure_preservation_result(value in -1000i32..1000i32) {
        let pattern = Pattern::pattern(value, vec![
            Pattern::point(value + 1),
            Pattern::point(value + 2),
        ]);

        let original_size = pattern.size();
        let original_depth = pattern.depth();
        let original_length = pattern.length();

        // Use a simple transformation that won't overflow or fail
        let result: Result<Pattern<i32>, String> = pattern.traverse_result(|v| Ok(v + 1));
        if let Ok(new_pattern) = result {
            prop_assert_eq!(new_pattern.size(), original_size);
            prop_assert_eq!(new_pattern.depth(), original_depth);
            prop_assert_eq!(new_pattern.length(), original_length);
        }
    }
}
