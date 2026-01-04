//! Test to demonstrate the balanced tree bug

use pattern_core::Pattern;

#[test]
fn test_balanced_tree_with_leaf_nodes() {
    // Create a balanced binary tree:
    // Level 0: 2 elements (root has 2 children)
    // Level 1: 2 elements (each child has 2 children)
    // Level 2: 0 elements (leaf nodes have no children)
    //
    // This should be identified as "balanced" but currently isn't because
    // the balanced check rejects zero counts

    let leaf1 = Pattern::point("leaf1".to_string());
    let leaf2 = Pattern::point("leaf2".to_string());
    let leaf3 = Pattern::point("leaf3".to_string());
    let leaf4 = Pattern::point("leaf4".to_string());

    let node1 = Pattern::pattern("node1".to_string(), vec![leaf1, leaf2]);
    let node2 = Pattern::pattern("node2".to_string(), vec![leaf3, leaf4]);

    let root = Pattern::pattern("root".to_string(), vec![node1, node2]);

    let analysis = root.analyze_structure();

    println!("Element counts: {:?}", analysis.element_counts);
    println!("Nesting patterns: {:?}", analysis.nesting_patterns);

    // This should be marked as "balanced" because:
    // - Level 0: 2 elements
    // - Level 1: 2 elements (same as level 0)
    // - Level 2: 0 elements (leaf nodes - should be skipped in balanced check)

    // After fix, should contain "balanced" because:
    // - Level 0: 2 elements
    // - Level 1: 2 elements (same as level 0, ratio = 1.0, within 0.5-2.0 range)
    // - Level 2: 0 elements (leaf nodes - now skipped in balanced check)
    assert!(
        analysis.nesting_patterns.contains(&"balanced".to_string()),
        "Balanced tree with leaf nodes should be identified as balanced. Got: {:?}",
        analysis.nesting_patterns
    );
}
