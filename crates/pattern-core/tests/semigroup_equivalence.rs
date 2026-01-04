//! Equivalence tests between Rust and Haskell implementations
//!
//! These tests verify that the Rust `Pattern::combine` behavior matches
//! the Haskell `(<>)` operator from `gram-hs`.
//!
//! Test cases are derived from the Haskell reference implementation.

use pattern_core::Pattern;

// ============================================================================
// T041: Haskell Equivalence Tests from gram-hs Test Suite
// ============================================================================

/// Test: Point <> Point = Pattern with concatenated values
/// Haskell: point "A" <> point "B" == Pattern "AB" []
#[test]
fn test_haskell_point_point() {
    let p1 = Pattern::point("A".to_string());
    let p2 = Pattern::point("B".to_string());

    let result = p1.combine(p2);

    assert_eq!(result.value(), "AB");
    assert_eq!(result.length(), 0);
}

/// Test: Pattern with elements combines values and concatenates elements
/// Haskell: pattern "A" [point "x"] <> pattern "B" [point "y"] == Pattern "AB" [point "x", point "y"]
#[test]
fn test_haskell_pattern_pattern() {
    let p1 = Pattern::pattern("A".to_string(), vec![Pattern::point("x".to_string())]);
    let p2 = Pattern::pattern("B".to_string(), vec![Pattern::point("y".to_string())]);

    let result = p1.combine(p2);

    assert_eq!(result.value(), "AB");
    assert_eq!(result.length(), 2);
    assert_eq!(result.elements()[0].value(), "x");
    assert_eq!(result.elements()[1].value(), "y");
}

/// Test: Combining with empty pattern preserves elements
/// Haskell: pattern "A" [point "x"] <> pattern "B" [] == Pattern "AB" [point "x"]
#[test]
fn test_haskell_pattern_empty() {
    let p1 = Pattern::pattern("A".to_string(), vec![Pattern::point("x".to_string())]);
    let p2 = Pattern::pattern("B".to_string(), vec![]);

    let result = p1.combine(p2);

    assert_eq!(result.value(), "AB");
    assert_eq!(result.length(), 1);
    assert_eq!(result.elements()[0].value(), "x");
}

/// Test: Nested pattern combination
/// Haskell: pattern "A" [pattern "B" [point "x"]] <> pattern "C" [pattern "D" [point "y"]]
///       == Pattern "AC" [pattern "B" [point "x"], pattern "D" [point "y"]]
#[test]
fn test_haskell_nested_patterns() {
    let p1 = Pattern::pattern(
        "A".to_string(),
        vec![Pattern::pattern(
            "B".to_string(),
            vec![Pattern::point("x".to_string())],
        )],
    );
    let p2 = Pattern::pattern(
        "C".to_string(),
        vec![Pattern::pattern(
            "D".to_string(),
            vec![Pattern::point("y".to_string())],
        )],
    );

    let result = p1.combine(p2);

    assert_eq!(result.value(), "AC");
    assert_eq!(result.length(), 2);

    // First element
    let first = &result.elements()[0];
    assert_eq!(first.value(), "B");
    assert_eq!(first.length(), 1);
    assert_eq!(first.elements()[0].value(), "x");

    // Second element
    let second = &result.elements()[1];
    assert_eq!(second.value(), "D");
    assert_eq!(second.length(), 1);
    assert_eq!(second.elements()[0].value(), "y");
}

/// Test: Associativity examples from Haskell tests
/// Haskell: (point "A" <> point "B") <> point "C" == point "A" <> (point "B" <> point "C")
#[test]
fn test_haskell_associativity_example() {
    let a = Pattern::point("A".to_string());
    let b = Pattern::point("B".to_string());
    let c = Pattern::point("C".to_string());

    let left = a.clone().combine(b.clone()).combine(c.clone());
    let right = a.combine(b.combine(c));

    assert_eq!(left, right);
    assert_eq!(left.value(), "ABC");
}

/// Test: Multiple element concatenation
/// Haskell: pattern "root" [point "a", point "b"] <> pattern "node" [point "c", point "d"]
///       == Pattern "rootnode" [point "a", point "b", point "c", point "d"]
#[test]
fn test_haskell_multiple_elements() {
    let p1 = Pattern::pattern(
        "root".to_string(),
        vec![
            Pattern::point("a".to_string()),
            Pattern::point("b".to_string()),
        ],
    );
    let p2 = Pattern::pattern(
        "node".to_string(),
        vec![
            Pattern::point("c".to_string()),
            Pattern::point("d".to_string()),
        ],
    );

    let result = p1.combine(p2);

    assert_eq!(result.value(), "rootnode");
    assert_eq!(result.length(), 4);

    let values: Vec<_> = result.elements().iter().map(|p| p.value()).collect();
    assert_eq!(values, vec!["a", "b", "c", "d"]);
}

// ============================================================================
// T042: Haskell Example Scenarios
// ============================================================================

/// Scenario: Building a graph structure incrementally
/// Haskell example: node "A" <> edge "->B" <> node "B"
#[test]
fn test_haskell_graph_construction() {
    let node_a = Pattern::pattern("Node".to_string(), vec![Pattern::point("A".to_string())]);
    let edge = Pattern::pattern("Edge".to_string(), vec![Pattern::point("->B".to_string())]);
    let node_b = Pattern::pattern("Node".to_string(), vec![Pattern::point("B".to_string())]);

    let result = node_a.combine(edge).combine(node_b);

    assert_eq!(result.value(), "NodeEdgeNode");
    assert_eq!(result.length(), 3);
}

/// Scenario: Path construction from segments
/// Haskell: mconcat [segment "a", segment "b", segment "c"]
#[test]
fn test_haskell_path_construction() {
    let segments = vec![
        Pattern::pattern("Segment".to_string(), vec![Pattern::point("a".to_string())]),
        Pattern::pattern("Segment".to_string(), vec![Pattern::point("b".to_string())]),
        Pattern::pattern("Segment".to_string(), vec![Pattern::point("c".to_string())]),
    ];

    let result = segments
        .into_iter()
        .reduce(|acc, p| acc.combine(p))
        .unwrap();

    assert_eq!(result.value(), "SegmentSegmentSegment");
    assert_eq!(result.length(), 3);

    let values: Vec<_> = result.elements().iter().map(|p| p.value()).collect();
    assert_eq!(values, vec!["a", "b", "c"]);
}

// ============================================================================
// T043: Specific Haskell Test Case Translations
// ============================================================================

/// Direct translation of Haskell test: test_semigroup_point_point
#[test]
fn test_haskell_ref_point_point() {
    // Haskell: it "combines two atomic patterns" $ do
    //   let p1 = point "hello"
    //   let p2 = point " world"
    //   (p1 <> p2) `shouldBe` Pattern "hello world" []

    let p1 = Pattern::point("hello".to_string());
    let p2 = Pattern::point(" world".to_string());

    let result = p1.combine(p2);

    assert_eq!(result.value(), "hello world");
    assert_eq!(result.length(), 0);
}

/// Direct translation of Haskell test: test_semigroup_associativity
#[test]
fn test_haskell_ref_associativity() {
    // Haskell: it "is associative" $ property $ \p1 p2 p3 ->
    //   ((p1 <> p2) <> p3) `shouldBe` (p1 <> (p2 <> p3))

    // Use concrete example from Haskell test suite
    let p1 = Pattern::pattern("A".to_string(), vec![Pattern::point("1".to_string())]);
    let p2 = Pattern::pattern("B".to_string(), vec![Pattern::point("2".to_string())]);
    let p3 = Pattern::pattern("C".to_string(), vec![Pattern::point("3".to_string())]);

    let left = p1.clone().combine(p2.clone()).combine(p3.clone());
    let right = p1.combine(p2.combine(p3));

    assert_eq!(left, right);
}

/// Direct translation of Haskell test: test_semigroup_nested
#[test]
fn test_haskell_ref_nested() {
    // Haskell: it "handles nested structures" $ do
    //   let p1 = pattern "outer" [pattern "inner1" [point "leaf1"]]
    //   let p2 = pattern "outer" [pattern "inner2" [point "leaf2"]]
    //   (p1 <> p2) `shouldBe` Pattern "outerouter" [
    //     pattern "inner1" [point "leaf1"],
    //     pattern "inner2" [point "leaf2"]
    //   ]

    let p1 = Pattern::pattern(
        "outer".to_string(),
        vec![Pattern::pattern(
            "inner1".to_string(),
            vec![Pattern::point("leaf1".to_string())],
        )],
    );
    let p2 = Pattern::pattern(
        "outer".to_string(),
        vec![Pattern::pattern(
            "inner2".to_string(),
            vec![Pattern::point("leaf2".to_string())],
        )],
    );

    let result = p1.combine(p2);

    assert_eq!(result.value(), "outerouter");
    assert_eq!(result.length(), 2);
    assert_eq!(result.elements()[0].value(), "inner1");
    assert_eq!(result.elements()[1].value(), "inner2");
}

// ============================================================================
// T044: Edge Case Verification Against Haskell
// ============================================================================

/// Test: Combining identical patterns
/// Haskell: p <> p == Pattern (v <> v) (elements p ++ elements p)
#[test]
fn test_haskell_self_combination() {
    let p = Pattern::pattern("X".to_string(), vec![Pattern::point("a".to_string())]);

    let result = p.clone().combine(p.clone());

    assert_eq!(result.value(), "XX");
    assert_eq!(result.length(), 2);
    assert_eq!(result.elements()[0].value(), "a");
    assert_eq!(result.elements()[1].value(), "a");
}

/// Test: Long chain of combinations
/// Haskell: foldl (<>) (point "") [point "a", point "b", point "c", ...]
#[test]
fn test_haskell_fold_chain() {
    let patterns: Vec<_> = vec!["a", "b", "c", "d", "e"]
        .into_iter()
        .map(|s| Pattern::point(s.to_string()))
        .collect();

    let initial = Pattern::point("".to_string());
    let result = patterns.into_iter().fold(initial, |acc, p| acc.combine(p));

    assert_eq!(result.value(), "abcde");
}

/// Test: Empty string values combine correctly
/// Haskell: point "" <> point "" == Pattern "" []
#[test]
fn test_haskell_empty_values() {
    let p1 = Pattern::point("".to_string());
    let p2 = Pattern::point("".to_string());

    let result = p1.combine(p2);

    assert_eq!(result.value(), "");
    assert_eq!(result.length(), 0);
}
