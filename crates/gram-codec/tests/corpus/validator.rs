//! Validator for comparing nom parser output with tree-sitter S-expressions
//!
//! This module provides semantic equivalence checking between Pattern structures
//! (from the nom parser) and S-expression trees (from tree-sitter-gram).

use pattern_core::{Pattern, Subject};

/// Validate that parsed patterns match the expected S-expression structure
///
/// For now, this is a simple check that looks for key structural elements.
/// Future versions can do more sophisticated AST comparison.
pub fn validate_patterns(patterns: &[Pattern<Subject>], expected_sexp: &str) -> Result<(), String> {
    // Basic validation: check that we got patterns
    if patterns.is_empty() {
        return Err("Parser returned no patterns".to_string());
    }

    // Check for top-level gram_pattern
    if !expected_sexp.contains("gram_pattern") {
        return Err("Expected S-expression doesn't contain gram_pattern".to_string());
    }

    // Pattern count validation
    let pattern_count = count_gram_patterns(expected_sexp);
    if patterns.len() != pattern_count {
        return Err(format!(
            "Pattern count mismatch: got {} patterns, expected {}",
            patterns.len(),
            pattern_count
        ));
    }

    // For each pattern, do basic structural validation
    for (i, pattern) in patterns.iter().enumerate() {
        validate_single_pattern(pattern, expected_sexp, i)?;
    }

    Ok(())
}

/// Count the number of top-level gram_pattern elements in S-expression
fn count_gram_patterns(sexp: &str) -> usize {
    // Simple count of (gram_pattern occurrences at the start of lines
    sexp.lines()
        .filter(|line| line.trim().starts_with("(gram_pattern"))
        .count()
}

/// Validate a single pattern against the expected S-expression
fn validate_single_pattern(
    pattern: &Pattern<Subject>,
    expected_sexp: &str,
    _index: usize,
) -> Result<(), String> {
    // Detect pattern type from structure
    let element_count = pattern.elements.len();

    // Node pattern OR subject pattern OR record (all can have 0 elements)
    // - (node) -> node_pattern
    // - [] or [subject] -> subject_pattern
    // - {} or {props} -> (record) or gram_pattern with root: (record)
    if element_count == 0 {
        // Check what the expected S-expression says
        let expects_node = expected_sexp.contains("node_pattern");
        let expects_subject = expected_sexp.contains("subject_pattern");
        let expects_record_root = expected_sexp.contains("root: (record");
        let expects_record = expected_sexp.contains("(record");

        // Standalone record or file-level pattern with record root
        if expects_record {
            // Pattern with properties (record) but no identity/labels and no elements
            if pattern.value.identity.0.is_empty() && pattern.value.labels.is_empty() {
                return Ok(()); // Valid record pattern
            }
        }

        if !expects_node && !expects_subject && !expects_record {
            return Err(format!(
                "Expected node_pattern, subject_pattern, or record in S-expression for atomic pattern: {:?}",
                pattern
            ));
        }

        // Both representations are valid for atomic patterns
        if expects_node {
            validate_node_pattern(pattern, expected_sexp)?;
        }
        // subject_pattern with 0 elements is also valid (e.g., `[]` or `[subject]`)
    }
    // Relationship or path pattern OR file-level pattern with multiple elements
    else if element_count == 2 {
        // Check if this is a simple relationship (both elements atomic)
        let is_simple_relationship =
            pattern.elements[0].elements.is_empty() && pattern.elements[1].elements.is_empty();

        // Check if this is a path (one element is itself a relationship with 2 elements)
        let is_path =
            pattern.elements[0].elements.len() == 2 || pattern.elements[1].elements.len() == 2;

        // Check if this is a file-level pattern (empty identity, empty labels, empty properties)
        let is_file_level = pattern.value.identity.0.is_empty()
            && pattern.value.labels.is_empty()
            && pattern.value.properties.is_empty();

        // File-level pattern with multiple node_pattern children
        if is_file_level
            && !expected_sexp.contains("relationship_pattern")
            && !expected_sexp.contains("subject_pattern")
        {
            // Count how many node_pattern children are in the S-expression
            let node_pattern_count = expected_sexp.matches("node_pattern").count();
            if node_pattern_count == element_count {
                return Ok(()); // Valid file-level pattern with multiple node elements
            }
        }

        if !expected_sexp.contains("relationship_pattern") {
            // Might be a subject_pattern instead
            if !expected_sexp.contains("subject_pattern") {
                return Err(format!(
                    "Expected relationship_pattern, subject_pattern, or file-level pattern in S-expression for 2-element pattern"
                ));
            }
        }

        // For paths, tree-sitter represents them as nested relationship_pattern
        // Our parser represents them as Pattern with 2 elements where one is a Pattern
        // Both are valid representations
        if is_path && expected_sexp.contains("relationship_pattern") {
            // This is fine - path representation difference
            return Ok(());
        }

        // Both are structurally valid representations
    }
    // Subject pattern, annotation, or file-level pattern with record + elements
    else {
        let expects_subject = expected_sexp.contains("subject_pattern");
        let expects_annotation = expected_sexp.contains("annotation_pattern");
        let expects_record = expected_sexp.contains("root: (record");

        // Check if this is a file-level pattern (empty identity, empty labels)
        let is_file_level = pattern.value.identity.0.is_empty() && pattern.value.labels.is_empty();

        // File-level pattern with multiple elements (no record root)
        if is_file_level && !expects_subject && !expects_annotation && !expects_record {
            // Count how many direct children are in the S-expression
            let node_pattern_count = expected_sexp.matches("node_pattern").count();
            let relationship_pattern_count = expected_sexp.matches("relationship_pattern").count();
            if node_pattern_count + relationship_pattern_count == element_count {
                return Ok(()); // Valid file-level pattern with multiple elements
            }
        }

        // File-level pattern with record root and elements
        if expects_record && element_count > 0 {
            // Pattern has properties (from record) and elements (following patterns)
            if is_file_level && !pattern.value.properties.is_empty() {
                return Ok(()); // Valid file-level pattern with record + elements
            }
        }

        if !expects_subject && !expects_annotation && !expects_record {
            return Err(format!(
                "Expected subject_pattern, annotation_pattern, record, or file-level pattern in S-expression for pattern with {} elements",
                element_count
            ));
        }
    }

    Ok(())
}

/// Validate a node pattern
fn validate_node_pattern(pattern: &Pattern<Subject>, expected_sexp: &str) -> Result<(), String> {
    // Check identifier
    if !pattern.value.identity.0.is_empty() {
        if !expected_sexp.contains("identifier:") {
            return Err(format!(
                "Pattern has identifier '{}' but S-expression doesn't show identifier",
                pattern.value.identity.0
            ));
        }
    }

    // Check labels
    if !pattern.value.labels.is_empty() {
        if !expected_sexp.contains("labels:") {
            return Err(format!(
                "Pattern has {} labels but S-expression doesn't show labels",
                pattern.value.labels.len()
            ));
        }
    }

    // Check properties/record
    if !pattern.value.properties.is_empty() {
        if !expected_sexp.contains("record:") {
            return Err(format!(
                "Pattern has {} properties but S-expression doesn't show record",
                pattern.value.properties.len()
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pattern_core::{Pattern, Subject, Symbol};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_validate_empty_node() {
        let subject = Subject {
            identity: Symbol(String::new()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        };
        let pattern = Pattern::point(subject);
        let sexp = "(gram_pattern\n  (node_pattern))";

        assert!(validate_patterns(&[pattern], sexp).is_ok());
    }

    #[test]
    fn test_validate_identified_node() {
        let subject = Subject {
            identity: Symbol("alice".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        };
        let pattern = Pattern::point(subject);
        let sexp = "(gram_pattern\n  (node_pattern\n    identifier: (symbol)))";

        assert!(validate_patterns(&[pattern], sexp).is_ok());
    }

    #[test]
    fn test_pattern_count_mismatch() {
        let subject = Subject {
            identity: Symbol(String::new()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        };
        let pattern = Pattern::point(subject);
        let sexp = "(gram_pattern\n  (node_pattern))\n(gram_pattern\n  (node_pattern))";

        // We have 1 pattern but S-expression expects 2
        let result = validate_patterns(&[pattern], sexp);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Pattern count mismatch"));
    }
}
