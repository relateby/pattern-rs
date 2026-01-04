//! Unit tests for pattern structure analysis functions

use pattern_core::Pattern;

#[test]
fn test_analyze_structure_with_atomic_pattern() {
    let pattern = Pattern::point("atom".to_string());
    let analysis = pattern.analyze_structure();

    assert_eq!(analysis.depth_distribution, vec![1]); // 1 node at depth 0
    assert_eq!(analysis.element_counts, Vec::<usize>::new()); // No elements (trailing zeros trimmed)
    assert!(analysis.nesting_patterns.contains(&"atomic".to_string()));
    assert!(!analysis.summary.is_empty());
}

#[test]
fn test_analyze_structure_depth_distribution() {
    let pattern = Pattern::pattern(
        "root".to_string(),
        vec![
            Pattern::point("child1".to_string()),
            Pattern::point("child2".to_string()),
        ],
    );

    let analysis = pattern.analyze_structure();

    // Should have nodes at depth 0 and depth 1
    assert_eq!(analysis.depth_distribution.len(), 2);
    assert_eq!(analysis.depth_distribution[0], 1); // 1 node at depth 0 (root)
    assert_eq!(analysis.depth_distribution[1], 2); // 2 nodes at depth 1 (children)
}

#[test]
fn test_analyze_structure_element_counts() {
    let pattern = Pattern::pattern(
        "root".to_string(),
        vec![
            Pattern::point("child1".to_string()),
            Pattern::point("child2".to_string()),
            Pattern::point("child3".to_string()),
        ],
    );

    let analysis = pattern.analyze_structure();

    // Should record element count at root level
    assert_eq!(analysis.element_counts[0], 3); // 3 elements at root
}

#[test]
fn test_analyze_structure_nesting_patterns_identification() {
    // Linear pattern (one element per level)
    let linear = Pattern::pattern(
        "level1".to_string(),
        vec![Pattern::pattern(
            "level2".to_string(),
            vec![Pattern::point("level3".to_string())],
        )],
    );

    let analysis = linear.analyze_structure();
    assert!(analysis.nesting_patterns.contains(&"linear".to_string()));

    // Tree pattern (multiple elements)
    let tree = Pattern::pattern(
        "root".to_string(),
        vec![
            Pattern::point("child1".to_string()),
            Pattern::point("child2".to_string()),
        ],
    );

    let analysis = tree.analyze_structure();
    assert!(analysis.nesting_patterns.contains(&"tree".to_string()));
}

#[test]
fn test_analyze_structure_summary_generation() {
    let pattern = Pattern::pattern(
        "root".to_string(),
        vec![
            Pattern::point("child1".to_string()),
            Pattern::point("child2".to_string()),
        ],
    );

    let analysis = pattern.analyze_structure();

    // Summary should be non-empty and descriptive
    assert!(!analysis.summary.is_empty());
    assert!(analysis.summary.contains("node"));
    assert!(analysis.summary.contains("level") || analysis.summary.contains("structure"));
}

#[test]
fn test_analysis_with_10000_plus_elements() {
    // Test that analysis handles large element counts efficiently
    let elements: Vec<Pattern<String>> = (0..10000)
        .map(|i| Pattern::point(format!("element{}", i)))
        .collect();

    let pattern = Pattern::pattern("root".to_string(), elements);

    // Should not panic or timeout
    let analysis = pattern.analyze_structure();
    assert_eq!(analysis.element_counts[0], 10000);
    assert!(!analysis.summary.is_empty());
}

#[test]
fn test_analysis_with_100_plus_nesting_levels() {
    // Test that analysis handles deep nesting without stack overflow
    fn create_deep_pattern(depth: usize) -> Pattern<String> {
        if depth == 0 {
            Pattern::point("leaf".to_string())
        } else {
            Pattern::pattern(
                format!("level{}", depth).to_string(),
                vec![create_deep_pattern(depth - 1)],
            )
        }
    }

    let deep = create_deep_pattern(100);

    // Should not panic or stack overflow
    let analysis = deep.analyze_structure();
    assert_eq!(analysis.depth_distribution.len(), 101); // 0-100 depths
    assert!(!analysis.summary.is_empty());
}
