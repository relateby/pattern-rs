//! Property-based tests for Comonad laws.
//!
//! This module verifies that Pattern's Comonad operations satisfy the three
//! fundamental Comonad laws:
//!
//! 1. **Left Identity** (extract-extend): `extract(extend(f, p)) == f(p)`
//! 2. **Right Identity** (extend-extract): `extend(extract, p) == p`
//! 3. **Associativity**: `extend(f, extend(g, p)) == extend(f ∘ extend(g), p)`
//!
//! These laws ensure that Comonad operations behave predictably and compose correctly.

use pattern_core::Pattern;
use proptest::prelude::*;

// ============================================================================
// Arbitrary Pattern Generator
// ============================================================================

/// Generates arbitrary patterns for property-based testing.
///
/// Limits depth to avoid stack overflow and ensures reasonable sizes for testing.
fn arbitrary_pattern_i32() -> impl Strategy<Value = Pattern<i32>> {
    let leaf = any::<i32>().prop_map(Pattern::point);
    leaf.prop_recursive(
        3,  // max depth
        10, // max total nodes
        5,  // max children per node
        |inner| {
            (any::<i32>(), prop::collection::vec(inner, 0..5)).prop_map(|(v, elements)| {
                if elements.is_empty() {
                    Pattern::point(v)
                } else {
                    Pattern::pattern(v, elements)
                }
            })
        },
    )
}

/// Generates arbitrary patterns with string values for property-based testing.
fn arbitrary_pattern_string() -> impl Strategy<Value = Pattern<String>> {
    let leaf = "[a-z]{1,3}".prop_map(Pattern::point);
    leaf.prop_recursive(
        3,  // max depth
        10, // max total nodes
        5,  // max children per node
        |inner| {
            ("[a-z]{1,3}", prop::collection::vec(inner, 0..5)).prop_map(|(v, elements)| {
                if elements.is_empty() {
                    Pattern::point(v)
                } else {
                    Pattern::pattern(v, elements)
                }
            })
        },
    )
}

// ============================================================================
// Comonad Law 1: Left Identity (Extract-Extend)
// ============================================================================

proptest! {
    /// **Law 1: Left Identity (Extract-Extend)**
    ///
    /// `extract(extend(f, p)) == f(p)`
    ///
    /// This law states that extracting the root value after extending with function `f`
    /// gives the same result as applying `f` directly to the pattern.
    ///
    /// This is true by definition of `extend`: the root value of `extend(f, p)` is `f(p)`.
    #[test]
    fn comonad_law_left_identity_depth(p in arbitrary_pattern_i32()) {
        // Function: compute depth of subpattern
        let f = |subp: &Pattern<i32>| subp.depth();

        // Left side: extract after extend
        let left = *p.extend(&f).extract();

        // Right side: apply function directly
        let right = f(&p);

        prop_assert_eq!(left, right, "Left identity failed for depth function");
    }

    /// Test left identity with size function.
    #[test]
    fn comonad_law_left_identity_size(p in arbitrary_pattern_i32()) {
        let f = |subp: &Pattern<i32>| subp.size();
        let left = *p.extend(&f).extract();
        let right = f(&p);
        prop_assert_eq!(left, right, "Left identity failed for size function");
    }

    /// Test left identity with value extraction (identity function on values).
    #[test]
    fn comonad_law_left_identity_value(p in arbitrary_pattern_i32()) {
        let f = |subp: &Pattern<i32>| *subp.extract();
        let left = *p.extend(&f).extract();
        let right = f(&p);
        prop_assert_eq!(left, right, "Left identity failed for value extraction");
    }
}

// ============================================================================
// Comonad Law 2: Right Identity (Extend-Extract)
// ============================================================================

proptest! {
    /// **Law 2: Right Identity (Extend-Extract)**
    ///
    /// `extend(extract, p) == p`
    ///
    /// This law states that extending with `extract` returns the pattern unchanged.
    ///
    /// At each position, we're computing the decoration using `extract`, which returns
    /// the existing decoration, so the result should be identical to the input.
    #[test]
    fn comonad_law_right_identity(p in arbitrary_pattern_i32()) {
        // Extend with extract (should return p unchanged)
        let result = p.extend(&|subp: &Pattern<i32>| *subp.extract());

        prop_assert_eq!(result, p, "Right identity failed: extend(extract) should equal identity");
    }

    /// Test right identity with string patterns.
    #[test]
    fn comonad_law_right_identity_string(p in arbitrary_pattern_string()) {
        let result = p.extend(&|subp: &Pattern<String>| subp.extract().clone());
        prop_assert_eq!(result, p, "Right identity failed for string patterns");
    }
}

// ============================================================================
// Comonad Law 3: Associativity
// ============================================================================

proptest! {
    /// **Law 3: Associativity**
    ///
    /// `extend(f, extend(g, p)) == extend(f ∘ extend(g), p)`
    ///
    /// This law states that extending twice in sequence is the same as extending once
    /// with the composed function.
    ///
    /// The order of applying context-aware transformations shouldn't change the result.
    #[test]
    fn comonad_law_associativity_depth_size(p in arbitrary_pattern_i32()) {
        // Two functions to compose
        let g = |subp: &Pattern<i32>| subp.size();
        let f = |subp: &Pattern<usize>| subp.depth();

        // Left side: extend twice in sequence
        let left = p.extend(&g).extend(&f);

        // Right side: extend once with composed function
        let right = p.extend(&|subp: &Pattern<i32>| {
            let temp = subp.extend(&g);
            f(&temp)
        });

        prop_assert_eq!(left, right, "Associativity failed for depth ∘ size");
    }

    /// Test associativity with size and value extraction.
    #[test]
    fn comonad_law_associativity_size_value(p in arbitrary_pattern_i32()) {
        let g = |subp: &Pattern<i32>| *subp.extract();
        let f = |subp: &Pattern<i32>| subp.size();

        let left = p.extend(&g).extend(&f);
        let right = p.extend(&|subp: &Pattern<i32>| {
            let temp = subp.extend(&g);
            f(&temp)
        });

        prop_assert_eq!(left, right, "Associativity failed for size ∘ extract");
    }

    /// Test associativity with depth twice (depth of depths).
    #[test]
    fn comonad_law_associativity_depth_depth(p in arbitrary_pattern_i32()) {
        let g = |subp: &Pattern<i32>| subp.depth();
        let f = |subp: &Pattern<usize>| subp.depth();

        let left = p.extend(&g).extend(&f);
        let right = p.extend(&|subp: &Pattern<i32>| {
            let temp = subp.extend(&g);
            f(&temp)
        });

        prop_assert_eq!(left, right, "Associativity failed for depth ∘ depth");
    }
}

// ============================================================================
// Structure Preservation Property
// ============================================================================

proptest! {
    /// **Property: Structure Preservation**
    ///
    /// `extend` must preserve the pattern structure:
    /// - Same number of nodes
    /// - Same tree shape (nesting structure)
    /// - Only values change
    #[test]
    fn extend_preserves_structure(p in arbitrary_pattern_i32()) {
        let f = |subp: &Pattern<i32>| subp.depth();
        let result = p.extend(&f);

        // Same number of nodes
        prop_assert_eq!(result.size(), p.size(), "extend changed node count");

        // Same number of elements at root
        prop_assert_eq!(result.elements().len(), p.elements().len(), "extend changed element count");

        // Same maximum depth
        prop_assert_eq!(result.depth(), p.depth(), "extend changed depth");
    }

    /// Test that depth_at preserves structure.
    #[test]
    fn depth_at_preserves_structure(p in arbitrary_pattern_i32()) {
        let result = p.depth_at();
        prop_assert_eq!(result.size(), p.size());
        prop_assert_eq!(result.depth(), p.depth());
    }

    /// Test that size_at preserves structure.
    #[test]
    fn size_at_preserves_structure(p in arbitrary_pattern_i32()) {
        let result = p.size_at();
        prop_assert_eq!(result.size(), p.size());
        prop_assert_eq!(result.depth(), p.depth());
    }
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn atomic_pattern_laws() {
    // Test laws on atomic patterns
    let p = Pattern::point(42);

    // Left identity
    let f = |subp: &Pattern<i32>| subp.depth();
    assert_eq!(*p.extend(&f).extract(), f(&p));

    // Right identity
    let result = p.extend(&|subp: &Pattern<i32>| *subp.extract());
    assert_eq!(result, p);
}

#[test]
fn deeply_nested_pattern() {
    // Build a deeply nested pattern manually (depth 4)
    let p = Pattern::pattern(
        1,
        vec![Pattern::pattern(
            2,
            vec![Pattern::pattern(
                3,
                vec![Pattern::pattern(4, vec![Pattern::point(5)])],
            )],
        )],
    );

    // Verify depth is correct (depth 4: levels 1->2->3->4, with 5 being atomic/depth 0)
    assert_eq!(p.depth(), 4);

    // Test laws
    let f = |subp: &Pattern<i32>| subp.size();
    assert_eq!(*p.extend(&f).extract(), f(&p));

    let result = p.extend(&|subp: &Pattern<i32>| *subp.extract());
    assert_eq!(result, p);
}

#[test]
fn pattern_with_many_children() {
    // Pattern with many children at root
    let p = Pattern::pattern(0, (1..=10).map(Pattern::point).collect());

    assert_eq!(p.elements().len(), 10);
    assert_eq!(p.size(), 11);

    // Test laws
    let f = |subp: &Pattern<i32>| subp.elements().len();
    assert_eq!(*p.extend(&f).extract(), f(&p));
}
