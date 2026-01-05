//! Tests for Pattern::contains
//!
//! This test suite verifies the behavior of the `contains` method, which checks if a pattern
//! contains another pattern as a subpattern anywhere in its structure.
//!
//! Test Requirements from contracts/type-signatures.md:
//! - Returns true for self-containment
//! - Returns true when subpattern is direct element
//! - Returns true when subpattern is nested descendant
//! - Returns false when subpattern not found
//! - Transitive: a.contains(&b) && b.contains(&c) => a.contains(&c)
//! - Weaker than matches: p.matches(&q) => p.contains(&q)
//! - Works with atomic patterns
//! - Works with empty elements
//! - Works with deeply nested structures
//! - Handles multiple occurrences (returns true if any match)

use pattern_core::Pattern;

// ============================================================================
// T034: Test contains returning true for self-containment
// ============================================================================

#[test]
fn test_contains_self_atomic() {
    let pattern = Pattern::point("a");
    assert!(pattern.contains(&pattern));
}

#[test]
fn test_contains_self_nested() {
    let pattern = Pattern::pattern(
        "root",
        vec![Pattern::point("child1"), Pattern::point("child2")],
    );

    assert!(pattern.contains(&pattern));
}

#[test]
fn test_contains_self_deeply_nested() {
    let pattern = Pattern::pattern(
        "root",
        vec![Pattern::pattern(
            "branch",
            vec![Pattern::pattern("deep", vec![Pattern::point("leaf")])],
        )],
    );

    assert!(pattern.contains(&pattern));
}

// ============================================================================
// T035: Test contains returning true when subpattern is direct element
// ============================================================================

#[test]
fn test_contains_direct_element_atomic() {
    let pattern = Pattern::pattern(
        "root",
        vec![Pattern::point("child1"), Pattern::point("child2")],
    );

    let subpattern = Pattern::point("child1");
    assert!(pattern.contains(&subpattern));
}

#[test]
fn test_contains_direct_element_nested() {
    let pattern = Pattern::pattern(
        "root",
        vec![
            Pattern::pattern("branch", vec![Pattern::point("leaf")]),
            Pattern::point("sibling"),
        ],
    );

    let subpattern = Pattern::pattern("branch", vec![Pattern::point("leaf")]);
    assert!(pattern.contains(&subpattern));
}

#[test]
fn test_contains_multiple_direct_elements() {
    let pattern = Pattern::pattern(
        "root",
        vec![
            Pattern::point("a"),
            Pattern::point("b"),
            Pattern::point("c"),
        ],
    );

    assert!(pattern.contains(&Pattern::point("a")));
    assert!(pattern.contains(&Pattern::point("b")));
    assert!(pattern.contains(&Pattern::point("c")));
}

// ============================================================================
// T036: Test contains returning true when subpattern is nested descendant
// ============================================================================

#[test]
fn test_contains_nested_descendant() {
    let pattern = Pattern::pattern(
        "root",
        vec![
            Pattern::pattern("branch", vec![Pattern::point("leaf")]),
            Pattern::point("sibling"),
        ],
    );

    let subpattern = Pattern::point("leaf");
    assert!(pattern.contains(&subpattern));
}

#[test]
fn test_contains_deeply_nested_descendant() {
    let pattern = Pattern::pattern(
        "root",
        vec![Pattern::pattern(
            "level1",
            vec![Pattern::pattern(
                "level2",
                vec![Pattern::pattern("level3", vec![Pattern::point("deep")])],
            )],
        )],
    );

    let subpattern = Pattern::point("deep");
    assert!(pattern.contains(&subpattern));
}

#[test]
fn test_contains_intermediate_subpattern() {
    let pattern = Pattern::pattern(
        "root",
        vec![Pattern::pattern(
            "level1",
            vec![Pattern::pattern("level2", vec![Pattern::point("leaf")])],
        )],
    );

    // Contains the intermediate pattern
    let subpattern = Pattern::pattern("level2", vec![Pattern::point("leaf")]);
    assert!(pattern.contains(&subpattern));
}

// ============================================================================
// T037: Test contains returning false when subpattern not found
// ============================================================================

#[test]
fn test_contains_not_found_atomic() {
    let pattern = Pattern::pattern(
        "root",
        vec![Pattern::point("child1"), Pattern::point("child2")],
    );

    let subpattern = Pattern::point("nonexistent");
    assert!(!pattern.contains(&subpattern));
}

#[test]
fn test_contains_not_found_nested() {
    let pattern = Pattern::pattern(
        "root",
        vec![Pattern::pattern("branch", vec![Pattern::point("leaf")])],
    );

    let subpattern = Pattern::pattern("other", vec![Pattern::point("leaf")]);
    assert!(!pattern.contains(&subpattern));
}

#[test]
fn test_contains_partial_match_not_enough() {
    let pattern = Pattern::pattern(
        "root",
        vec![Pattern::pattern("branch", vec![Pattern::point("a")])],
    );

    // Subpattern has additional element not in pattern
    let subpattern = Pattern::pattern(
        "branch",
        vec![
            Pattern::point("a"),
            Pattern::point("b"), // Extra element!
        ],
    );
    assert!(!pattern.contains(&subpattern));
}

// ============================================================================
// T038: Test contains transitivity property
// ============================================================================

#[test]
fn test_contains_transitivity() {
    let a = Pattern::pattern("a", vec![Pattern::pattern("b", vec![Pattern::point("c")])]);
    let b = Pattern::pattern("b", vec![Pattern::point("c")]);
    let c = Pattern::point("c");

    // If a contains b and b contains c, then a contains c
    assert!(a.contains(&b));
    assert!(b.contains(&c));
    assert!(a.contains(&c)); // Transitive
}

#[test]
fn test_contains_transitivity_complex() {
    let level1 = Pattern::pattern(
        "1",
        vec![Pattern::pattern(
            "2",
            vec![Pattern::pattern("3", vec![Pattern::point("4")])],
        )],
    );
    let level2 = Pattern::pattern("2", vec![Pattern::pattern("3", vec![Pattern::point("4")])]);
    let level3 = Pattern::pattern("3", vec![Pattern::point("4")]);
    let level4 = Pattern::point("4");

    assert!(level1.contains(&level2));
    assert!(level2.contains(&level3));
    assert!(level3.contains(&level4));
    assert!(level1.contains(&level3)); // Transitivity 1-3
    assert!(level1.contains(&level4)); // Transitivity 1-4
}

// ============================================================================
// T039: Test contains being weaker than matches
// ============================================================================

#[test]
fn test_contains_weaker_than_matches_identical() {
    let p1 = Pattern::pattern("root", vec![Pattern::point("a"), Pattern::point("b")]);
    let p2 = Pattern::pattern("root", vec![Pattern::point("a"), Pattern::point("b")]);

    // If patterns match, they contain each other
    assert!(p1.matches(&p2));
    assert!(p1.contains(&p2));
    assert!(p2.contains(&p1));
}

#[test]
fn test_contains_but_not_matches() {
    let pattern = Pattern::pattern(
        "root",
        vec![Pattern::pattern("branch", vec![Pattern::point("leaf")])],
    );
    let subpattern = Pattern::point("leaf");

    // pattern contains subpattern, but they don't match
    assert!(pattern.contains(&subpattern));
    assert!(!pattern.matches(&subpattern));
    assert!(!subpattern.matches(&pattern));
}

#[test]
fn test_matches_implies_contains() {
    let p1 = Pattern::point("a");
    let p2 = Pattern::point("a");

    // matches implies contains
    if p1.matches(&p2) {
        assert!(p1.contains(&p2));
    }
}

// ============================================================================
// T040: Test contains with atomic patterns
// ============================================================================

#[test]
fn test_contains_atomic_self() {
    let pattern = Pattern::point("a");
    assert!(pattern.contains(&pattern));
}

#[test]
fn test_contains_atomic_in_nested() {
    let pattern = Pattern::pattern("root", vec![Pattern::point("a")]);
    let subpattern = Pattern::point("a");

    assert!(pattern.contains(&subpattern));
}

#[test]
fn test_contains_atomic_not_found() {
    let pattern = Pattern::point("a");
    let subpattern = Pattern::point("b");

    assert!(!pattern.contains(&subpattern));
}

// ============================================================================
// T041: Test contains with empty elements
// ============================================================================

#[test]
fn test_contains_empty_elements_self() {
    let pattern = Pattern::pattern("root", vec![]);
    assert!(pattern.contains(&pattern));
}

#[test]
fn test_contains_empty_elements_in_nested() {
    let pattern = Pattern::pattern(
        "root",
        vec![Pattern::pattern("empty", vec![]), Pattern::point("leaf")],
    );
    let subpattern = Pattern::pattern("empty", vec![]);

    assert!(pattern.contains(&subpattern));
}

#[test]
fn test_contains_empty_elements_not_in_atomic() {
    let pattern = Pattern::point("a");
    let subpattern = Pattern::pattern("a", vec![]);

    // Atomic pattern does not contain a pattern with empty elements
    // (if they're structurally different)
    // Actually, point("a") and pattern("a", vec![]) are structurally identical
    assert!(pattern.contains(&subpattern));
}

// ============================================================================
// T042: Test contains with deeply nested structures
// ============================================================================

#[test]
fn test_contains_deeply_nested() {
    let pattern = Pattern::pattern(
        "root",
        vec![Pattern::pattern(
            "level1",
            vec![Pattern::pattern(
                "level2",
                vec![Pattern::pattern(
                    "level3",
                    vec![Pattern::pattern("level4", vec![Pattern::point("deep")])],
                )],
            )],
        )],
    );

    let subpattern = Pattern::point("deep");
    assert!(pattern.contains(&subpattern));
}

#[test]
fn test_contains_100_level_nesting() {
    // Create a 100-level deep structure
    let mut pattern = Pattern::point("bottom".to_string());
    for i in (0..100).rev() {
        pattern = Pattern::pattern(format!("level{}", i), vec![pattern]);
    }

    let subpattern = Pattern::point("bottom".to_string());
    assert!(pattern.contains(&subpattern));
}

#[test]
fn test_contains_nested_at_various_depths() {
    let pattern = Pattern::pattern(
        "root",
        vec![
            Pattern::pattern("level1a", vec![Pattern::point("deep1")]),
            Pattern::pattern(
                "level1b",
                vec![Pattern::pattern("level2", vec![Pattern::point("deep2")])],
            ),
        ],
    );

    assert!(pattern.contains(&Pattern::point("deep1")));
    assert!(pattern.contains(&Pattern::point("deep2")));
    assert!(pattern.contains(&Pattern::pattern("level2", vec![Pattern::point("deep2"),])));
}

// ============================================================================
// T043: Test contains handling multiple occurrences
// ============================================================================

#[test]
fn test_contains_multiple_occurrences_same_value() {
    let pattern = Pattern::pattern(
        "root",
        vec![
            Pattern::point("a"),
            Pattern::point("a"), // Duplicate
            Pattern::point("a"), // Triplicate
        ],
    );

    let subpattern = Pattern::point("a");
    assert!(pattern.contains(&subpattern)); // Should find any of them
}

#[test]
fn test_contains_multiple_occurrences_same_structure() {
    let pattern = Pattern::pattern(
        "root",
        vec![
            Pattern::pattern("branch", vec![Pattern::point("leaf")]),
            Pattern::point("other"),
            Pattern::pattern("branch", vec![Pattern::point("leaf")]), // Duplicate structure
        ],
    );

    let subpattern = Pattern::pattern("branch", vec![Pattern::point("leaf")]);
    assert!(pattern.contains(&subpattern));
}

#[test]
fn test_contains_finds_first_occurrence() {
    let pattern = Pattern::pattern(
        "root",
        vec![
            Pattern::point("target"),
            Pattern::pattern(
                "branch",
                vec![
                    Pattern::point("target"), // Second occurrence
                ],
            ),
        ],
    );

    let subpattern = Pattern::point("target");
    // Should find it (either occurrence is fine)
    assert!(pattern.contains(&subpattern));
}
