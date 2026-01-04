//! Test to verify trailing zeros are trimmed from element_counts

use pattern_core::Pattern;

#[test]
fn test_element_counts_no_trailing_zeros() {
    // Atomic pattern should have empty element_counts
    let atomic = Pattern::point("atom".to_string());
    let analysis = atomic.analyze_structure();
    assert_eq!(
        analysis.element_counts,
        Vec::<usize>::new(),
        "Atomic pattern should have empty element_counts"
    );

    // 2-level tree: root with 3 children (all atomic)
    // Should have element_counts = [3], not [3, 0]
    let tree = Pattern::pattern(
        "root".to_string(),
        vec![
            Pattern::point("child1".to_string()),
            Pattern::point("child2".to_string()),
            Pattern::point("child3".to_string()),
        ],
    );
    let analysis = tree.analyze_structure();
    assert_eq!(
        analysis.element_counts,
        vec![3],
        "2-level tree should have [3], not [3, 0]"
    );

    // 3-level tree: root -> node -> leaf
    // Should have element_counts = [1, 1], not [1, 1, 0]
    // Root has 1 element (node), node has 1 element (leaf), leaf has 0 (trimmed)
    let nested = Pattern::pattern(
        "root".to_string(),
        vec![Pattern::pattern(
            "node".to_string(),
            vec![Pattern::point("leaf".to_string())],
        )],
    );
    let analysis = nested.analyze_structure();
    assert_eq!(
        analysis.element_counts,
        vec![1, 1],
        "3-level linear tree should have [1, 1], not [1, 1, 0]"
    );
}

#[test]
fn test_balanced_pattern_2_levels() {
    // 2-level pattern with consistent counts should be identified as balanced
    // Root has 2 children, each child has 2 children
    // element_counts should be [2, 2] (after trimming trailing zeros)
    // This should be identified as "balanced"

    let child1 = Pattern::pattern(
        "child1".to_string(),
        vec![
            Pattern::point("leaf1".to_string()),
            Pattern::point("leaf2".to_string()),
        ],
    );
    let child2 = Pattern::pattern(
        "child2".to_string(),
        vec![
            Pattern::point("leaf3".to_string()),
            Pattern::point("leaf4".to_string()),
        ],
    );

    let root = Pattern::pattern("root".to_string(), vec![child1, child2]);

    let analysis = root.analyze_structure();

    println!("Element counts: {:?}", analysis.element_counts);
    println!("Nesting patterns: {:?}", analysis.nesting_patterns);

    // Should be identified as balanced (2 levels with same count: 2)
    // Currently fails because element_counts.len() > 2 check blocks 2-level patterns
    assert!(
        analysis.nesting_patterns.contains(&"balanced".to_string()),
        "2-level pattern with consistent counts should be identified as balanced. Got: {:?}",
        analysis.nesting_patterns
    );
}
