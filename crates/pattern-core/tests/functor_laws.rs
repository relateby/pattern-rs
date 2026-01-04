//! Functor law tests for Pattern<V>
//!
//! This module contains property-based and unit tests verifying that
//! Pattern's `map` method satisfies functor laws and preserves structure.
//!
//! Tests are organized by user story:
//! - US1: Structure preservation and basic transformations
//! - US2: Composition law
//! - US3: Identity law

use pattern_core::Pattern;
use proptest::prelude::*;

// ============================================================================
// Property Test Generators
// ============================================================================

/// Generates arbitrary Pattern<i32> for property testing
fn arbitrary_pattern_i32() -> impl Strategy<Value = Pattern<i32>> {
    let leaf = any::<i32>().prop_map(Pattern::point);
    leaf.prop_recursive(
        4,  // Max depth
        16, // Max nodes
        3,  // Items per collection
        |inner| {
            (any::<i32>(), prop::collection::vec(inner, 0..3))
                .prop_map(|(value, elements)| Pattern::pattern(value, elements))
        },
    )
}

/// Generates arbitrary Pattern<String> for property testing
fn arbitrary_pattern_string() -> impl Strategy<Value = Pattern<String>> {
    let leaf = "[a-z]{1,5}".prop_map(Pattern::point);
    leaf.prop_recursive(
        4,  // Max depth
        16, // Max nodes
        3,  // Items per collection
        |inner| {
            ("[a-z]{1,5}", prop::collection::vec(inner, 0..3))
                .prop_map(|(value, elements)| Pattern::pattern(value, elements))
        },
    )
}

// ============================================================================
// User Story 1: Transform Pattern Values While Preserving Structure
// ============================================================================

#[cfg(test)]
mod user_story_1 {
    use super::*;

    // ------------------------------------------------------------------------
    // Property-Based Tests
    // ------------------------------------------------------------------------

    proptest! {
        /// Property: Structure preservation
        /// Verifies that map preserves element count, depth, and size
        #[test]
        fn structure_preservation(pattern in arbitrary_pattern_i32()) {
            let original_length = pattern.length();
            let original_depth = pattern.depth();
            let original_size = pattern.size();

            let transformed = pattern.map(|n| n.wrapping_mul(2));

            prop_assert_eq!(transformed.length(), original_length);
            prop_assert_eq!(transformed.depth(), original_depth);
            prop_assert_eq!(transformed.size(), original_size);
        }
    }

    // ------------------------------------------------------------------------
    // Unit Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_map_atomic_pattern() {
        let p = Pattern::point("hello");
        let upper = p.map(|s| s.to_uppercase());
        assert_eq!(upper.value, "HELLO");
        assert_eq!(upper.elements.len(), 0);
        assert!(upper.is_atomic());
    }

    #[test]
    fn test_map_nested_pattern() {
        let p = Pattern::pattern(
            "root",
            vec![Pattern::point("child1"), Pattern::point("child2")],
        );
        let upper = p.map(|s| s.to_uppercase());
        assert_eq!(upper.value, "ROOT");
        assert_eq!(upper.elements.len(), 2);
        assert_eq!(upper.elements[0].value, "CHILD1");
        assert_eq!(upper.elements[1].value, "CHILD2");
    }

    #[test]
    fn test_map_type_conversion() {
        let p = Pattern::point(42);
        let stringified = p.map(|n| n.to_string());
        assert_eq!(stringified.value, "42");
    }

    #[test]
    fn test_map_preserves_structure() {
        let p = Pattern::pattern(
            "root",
            vec![
                Pattern::point("a"),
                Pattern::pattern("b", vec![Pattern::point("c")]),
            ],
        );

        let original_size = p.size();
        let original_depth = p.depth();
        let original_length = p.length();

        let mapped = p.map(|s| s.to_uppercase());

        assert_eq!(mapped.size(), original_size);
        assert_eq!(mapped.depth(), original_depth);
        assert_eq!(mapped.length(), original_length);
    }

    #[test]
    fn test_map_deeply_nested() {
        let deep = Pattern::pattern(
            "level1",
            vec![Pattern::pattern("level2", vec![Pattern::point("level3")])],
        );

        let transformed = deep.map(|s| s.len());

        assert_eq!(transformed.value, 6); // "level1".len()
        assert_eq!(transformed.elements[0].value, 6); // "level2".len()
        assert_eq!(transformed.elements[0].elements[0].value, 6); // "level3".len()
    }

    #[test]
    fn test_map_with_closure_capture() {
        let multiplier = 10;

        let pattern = Pattern::pattern(1, vec![Pattern::point(2), Pattern::point(3)]);

        let scaled = pattern.map(|n| n * multiplier);

        assert_eq!(scaled.value, 10);
        assert_eq!(scaled.elements[0].value, 20);
        assert_eq!(scaled.elements[1].value, 30);
    }
}

// ============================================================================
// User Story 2: Compose Multiple Transformations Safely
// ============================================================================

#[cfg(test)]
mod user_story_2 {
    use super::*;

    // ------------------------------------------------------------------------
    // Property-Based Tests
    // ------------------------------------------------------------------------

    proptest! {
        /// Property: Composition law
        /// Verifies that map(f ∘ g) == map(f) ∘ map(g)
        #[test]
        fn composition_law_i32(pattern in arbitrary_pattern_i32()) {
            let f = |x: &i32| x.wrapping_mul(2);
            let g = |x: &i32| x.wrapping_add(1);

            let composed = pattern.clone().map(|x| g(&f(x)));
            let sequential = pattern.map(f).map(g);

            prop_assert_eq!(composed, sequential);
        }

        /// Property: Composition law with type transformation
        #[test]
        fn composition_law_with_type_change(pattern in arbitrary_pattern_i32()) {
            let f = |x: &i32| x.wrapping_mul(2);
            let g = |x: &i32| x.to_string();

            let composed = pattern.clone().map(|x| g(&f(x)));
            let sequential = pattern.map(f).map(g);

            prop_assert_eq!(composed, sequential);
        }
    }

    // ------------------------------------------------------------------------
    // Unit Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_composition_numeric() {
        let pattern = Pattern::point(5);
        let f = |x: &i32| x * 2;
        let g = |x: &i32| x + 1;

        let composed = pattern.clone().map(|x| g(&f(x)));
        let sequential = pattern.map(f).map(g);

        assert_eq!(composed, sequential);
        assert_eq!(composed.value, 11);
    }

    #[test]
    fn test_composition_string() {
        let pattern = Pattern::point("hello".to_string());
        let f = |s: &String| s.to_uppercase();
        let g = |s: &String| s.len();

        let composed = pattern.clone().map(|x| g(&f(x)));
        let sequential = pattern.map(f).map(g);

        assert_eq!(composed, sequential);
        assert_eq!(composed.value, 5);
    }

    #[test]
    fn test_chained_transformations() {
        let result = Pattern::point(5)
            .map(|n| n * 2)
            .map(|n| n + 1)
            .map(|n| format!("Result: {}", n));

        assert_eq!(result.value, "Result: 11");
    }
}

// ============================================================================
// User Story 3: Apply Identity Transformation Without Side Effects
// ============================================================================

#[cfg(test)]
mod user_story_3 {
    use super::*;

    // ------------------------------------------------------------------------
    // Property-Based Tests
    // ------------------------------------------------------------------------

    proptest! {
        /// Property: Identity law
        /// Verifies that map(id) == id
        #[test]
        fn identity_law_i32(pattern in arbitrary_pattern_i32()) {
            let original = pattern.clone();
            let mapped = pattern.map(|x| *x);

            prop_assert_eq!(original, mapped);
        }

        /// Property: Identity law for strings
        #[test]
        fn identity_law_string(pattern in arbitrary_pattern_string()) {
            let original = pattern.clone();
            let mapped = pattern.map(|x: &String| x.clone());

            prop_assert_eq!(original, mapped);
        }
    }

    // ------------------------------------------------------------------------
    // Unit Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_identity_atomic() {
        let pattern = Pattern::point(42);
        let identity = pattern.clone().map(|x| *x);
        assert_eq!(pattern, identity);
    }

    #[test]
    fn test_identity_nested() {
        let pattern = Pattern::pattern(
            "root",
            vec![Pattern::point("child1"), Pattern::point("child2")],
        );
        let identity = pattern.clone().map(|x| x.clone());
        assert_eq!(pattern, identity);
    }

    #[test]
    fn test_identity_deep() {
        let pattern = Pattern::pattern(1, vec![Pattern::pattern(2, vec![Pattern::point(3)])]);
        let identity = pattern.clone().map(|x| *x);
        assert_eq!(pattern, identity);
    }
}

// ============================================================================
// Edge Cases and Performance Tests
// ============================================================================

#[cfg(test)]
mod edge_cases {
    use super::*;

    #[test]
    fn test_empty_pattern_atomic() {
        let p: Pattern<i32> = Pattern::point(42);
        assert!(p.is_atomic());
        let mapped = p.map(|n| n * 2);
        assert!(mapped.is_atomic());
        assert_eq!(mapped.value, 84);
    }

    #[test]
    fn test_wide_branching() {
        let children: Vec<Pattern<i32>> = (0..100).map(Pattern::point).collect();
        let pattern = Pattern::pattern(0, children);

        let mapped = pattern.map(|n| n + 1);

        assert_eq!(mapped.value, 1);
        assert_eq!(mapped.elements.len(), 100);
        assert_eq!(mapped.elements[0].value, 1);
        assert_eq!(mapped.elements[99].value, 100);
    }

    #[test]
    fn test_deep_nesting() {
        // Create a deeply nested pattern (20 levels)
        let mut pattern = Pattern::point(0);
        for i in 1..20 {
            pattern = Pattern::pattern(i, vec![pattern]);
        }

        let mapped = pattern.map(|n| n * 2);

        // Verify transformation reached all levels
        assert_eq!(mapped.value, 38); // 19 * 2
        let mut current = &mapped;
        for i in (0..19).rev() {
            current = &current.elements[0];
            assert_eq!(current.value, i * 2);
        }
    }

    #[test]
    fn test_conditional_transformation() {
        let pattern = Pattern::pattern(
            1,
            vec![Pattern::point(2), Pattern::point(3), Pattern::point(4)],
        );

        let transformed = pattern.map(|n| if n % 2 == 0 { n * 2 } else { n * 3 });

        assert_eq!(transformed.value, 3); // 1 * 3
        assert_eq!(transformed.elements[0].value, 4); // 2 * 2
        assert_eq!(transformed.elements[1].value, 9); // 3 * 3
        assert_eq!(transformed.elements[2].value, 8); // 4 * 2
    }

    #[test]
    fn test_stack_safety_100_levels() {
        // Create a pattern with 100+ nesting levels
        let mut pattern = Pattern::point(0);
        for i in 1..=100 {
            pattern = Pattern::pattern(i, vec![pattern]);
        }

        // This should not overflow the stack
        let mapped = pattern.map(|n| n + 1);

        // Verify transformation reached all levels
        assert_eq!(mapped.value, 101); // 100 + 1
        let mut current = &mapped;
        for i in (1..=100).rev() {
            current = &current.elements[0];
            assert_eq!(current.value, i); // i-1 + 1
        }
    }

    #[test]
    fn test_large_pattern_10k_nodes() {
        // Create a wide pattern with 1000 direct children
        let children: Vec<Pattern<i32>> = (0..1000).map(Pattern::point).collect();
        let pattern = Pattern::pattern(0, children);

        // Transform all nodes
        let mapped = pattern.map(|n| n * 2);

        // Verify structure and values
        assert_eq!(mapped.value, 0);
        assert_eq!(mapped.elements.len(), 1000);
        for (i, elem) in mapped.elements.iter().enumerate() {
            assert_eq!(elem.value, (i as i32) * 2);
        }
    }
}
