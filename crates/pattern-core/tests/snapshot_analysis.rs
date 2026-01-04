//! Snapshot tests for structure analysis results
//!
//! These tests use insta to capture and verify analysis results
//! remain consistent across changes.

use pattern_core::Pattern;

#[test]
fn test_analysis_atomic_pattern() {
    let pattern = Pattern::point("atom".to_string());
    let analysis = pattern.analyze_structure();

    insta::assert_snapshot!(
        "analysis_atomic_depth_distribution",
        format!("{:?}", analysis.depth_distribution)
    );
    insta::assert_snapshot!(
        "analysis_atomic_element_counts",
        format!("{:?}", analysis.element_counts)
    );
    insta::assert_snapshot!(
        "analysis_atomic_nesting_patterns",
        format!("{:?}", analysis.nesting_patterns)
    );
    insta::assert_snapshot!("analysis_atomic_summary", analysis.summary);
}

#[test]
fn test_analysis_nested_pattern() {
    let pattern = Pattern::pattern(
        "root".to_string(),
        vec![
            Pattern::point("child1".to_string()),
            Pattern::point("child2".to_string()),
        ],
    );

    let analysis = pattern.analyze_structure();

    insta::assert_snapshot!(
        "analysis_nested_depth_distribution",
        format!("{:?}", analysis.depth_distribution)
    );
    insta::assert_snapshot!(
        "analysis_nested_element_counts",
        format!("{:?}", analysis.element_counts)
    );
    insta::assert_snapshot!("analysis_nested_summary", analysis.summary);
}

#[test]
fn test_analysis_linear_pattern() {
    let pattern = Pattern::pattern(
        "level1".to_string(),
        vec![Pattern::pattern(
            "level2".to_string(),
            vec![Pattern::point("level3".to_string())],
        )],
    );

    let analysis = pattern.analyze_structure();

    insta::assert_snapshot!(
        "analysis_linear_nesting_patterns",
        format!("{:?}", analysis.nesting_patterns)
    );
    insta::assert_snapshot!("analysis_linear_summary", analysis.summary);
}
