//! Arrow style variant tests - documenting visual forms and normalized kinds
//!
//! The tree-sitter-gram grammar accepts multiple visual arrow styles but normalizes
//! them to 4 semantic arrow kinds. This test suite documents and validates all variants.

use gram_codec::parse_gram_notation;

// ============================================================================
// Right Arrow Variants (normalized to `right_arrow`)
// ============================================================================

#[test]
fn test_single_stroke_right_arrow() {
    // Visual: -->
    // Normalized: right_arrow
    let result = parse_gram_notation("(a)-->(b)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].elements.len(), 2);
    // First element is 'a', second is 'b' (left to right order)
    assert_eq!(patterns[0].elements[0].value.identity.0, "a");
    assert_eq!(patterns[0].elements[1].value.identity.0, "b");
}

#[test]
fn test_double_stroke_right_arrow() {
    // Visual: ==>
    // Normalized: right_arrow
    let result = parse_gram_notation("(a)==>(b)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].elements.len(), 2);
    assert_eq!(patterns[0].elements[0].value.identity.0, "a");
    assert_eq!(patterns[0].elements[1].value.identity.0, "b");
}

#[test]
fn test_squiggle_right_arrow() {
    // Visual: ~~>
    // Normalized: right_arrow
    let result = parse_gram_notation("(a)~~>(b)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].elements.len(), 2);
    assert_eq!(patterns[0].elements[0].value.identity.0, "a");
    assert_eq!(patterns[0].elements[1].value.identity.0, "b");
}

// ============================================================================
// Left Arrow Variants (normalized to `left_arrow`)
// ============================================================================

#[test]
fn test_single_stroke_left_arrow() {
    // Visual: <--
    // Normalized: left_arrow
    // Note: Element order is REVERSED (right to left)
    let result = parse_gram_notation("(a)<--(b)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].elements.len(), 2);
    // Left arrow reverses: (a)<--(b) becomes b-->a
    assert_eq!(patterns[0].elements[0].value.identity.0, "b");
    assert_eq!(patterns[0].elements[1].value.identity.0, "a");
}

#[test]
fn test_double_stroke_left_arrow() {
    // Visual: <==
    // Normalized: left_arrow
    let result = parse_gram_notation("(a)<==(b)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].elements.len(), 2);
    // Element order reversed
    assert_eq!(patterns[0].elements[0].value.identity.0, "b");
    assert_eq!(patterns[0].elements[1].value.identity.0, "a");
}

#[test]
fn test_squiggle_left_arrow() {
    // Visual: <~~
    // Normalized: left_arrow
    let result = parse_gram_notation("(a)<~~(b)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].elements.len(), 2);
    // Element order reversed
    assert_eq!(patterns[0].elements[0].value.identity.0, "b");
    assert_eq!(patterns[0].elements[1].value.identity.0, "a");
}

// ============================================================================
// Bidirectional Arrow Variants (normalized to `bidirectional_arrow`)
// ============================================================================

#[test]
fn test_single_stroke_bidirectional_arrow() {
    // Visual: <-->
    // Normalized: bidirectional_arrow
    let result = parse_gram_notation("(a)<-->(b)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].elements.len(), 2);
    // Bidirectional preserves order
    assert_eq!(patterns[0].elements[0].value.identity.0, "a");
    assert_eq!(patterns[0].elements[1].value.identity.0, "b");
}

#[test]
fn test_double_stroke_bidirectional_arrow() {
    // Visual: <==>
    // Normalized: bidirectional_arrow
    let result = parse_gram_notation("(a)<==>(b)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].elements.len(), 2);
    assert_eq!(patterns[0].elements[0].value.identity.0, "a");
    assert_eq!(patterns[0].elements[1].value.identity.0, "b");
}

// ============================================================================
// Undirected Arrow Variants (normalized to `undirected_arrow`)
// ============================================================================

#[test]
fn test_squiggle_undirected_arrow() {
    // Visual: ~~
    // Normalized: undirected_arrow
    let result = parse_gram_notation("(a)~~(b)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].elements.len(), 2);
    // Undirected preserves order as written
    assert_eq!(patterns[0].elements[0].value.identity.0, "a");
    assert_eq!(patterns[0].elements[1].value.identity.0, "b");
}

#[test]
fn test_double_stroke_undirected_arrow() {
    // Visual: ==
    // Normalized: undirected_arrow
    let result = parse_gram_notation("(a)==(b)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].elements.len(), 2);
    assert_eq!(patterns[0].elements[0].value.identity.0, "a");
    assert_eq!(patterns[0].elements[1].value.identity.0, "b");
}

// ============================================================================
// Arrow Variants with Labels and Properties
// ============================================================================

#[test]
fn test_double_stroke_right_with_label() {
    let result = parse_gram_notation("(a)=[:KNOWS]=>(b)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert!(patterns[0].value.labels.contains("KNOWS"));
}

#[test]
fn test_double_stroke_left_with_label() {
    let result = parse_gram_notation("(a)<=[:KNOWS]=(b)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert!(patterns[0].value.labels.contains("KNOWS"));
    // Elements reversed
    assert_eq!(patterns[0].elements[0].value.identity.0, "b");
    assert_eq!(patterns[0].elements[1].value.identity.0, "a");
}

#[test]
fn test_squiggle_right_with_label() {
    // Note: squiggle syntax with label is ~[:LABEL]~> not ~~[:LABEL]~>
    let result = parse_gram_notation("(a)~[:WAVY]~>(b)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert!(patterns[0].value.labels.contains("WAVY"));
}

// ============================================================================
// Mixed Arrow Variants in Paths
// ============================================================================

#[test]
fn test_mixed_arrow_styles_in_path() {
    // Mixing single, double, and squiggle arrows in a path
    let result = parse_gram_notation("(a)-->(b)==>(c)~~>(d)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let patterns = result.unwrap();
    assert_eq!(patterns.len(), 1);
    // All variants parse as right_arrow, creating nested structure
}

#[test]
fn test_alternating_directions() {
    // Mix of left and right arrows
    let result = parse_gram_notation("(a)<--(b)-->(c)<==(d)==>(e)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

#[test]
fn test_undirected_chain() {
    // Chain of undirected arrows (both variants)
    let result = parse_gram_notation("(a)~~(b)==(c)~~(d)");
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// ============================================================================
// Round-Trip Tests for Arrow Variants
// ============================================================================

#[test]
fn test_round_trip_preserves_semantics_double_stroke() {
    use gram_codec::serialize_pattern;

    // Parse double-stroke right arrow
    let original = "(a)==>(b)";
    let parsed = parse_gram_notation(original).unwrap();

    // Serialize (will output canonical single-stroke form)
    let serialized = serialize_pattern(&parsed[0]).unwrap();

    // Re-parse serialized form
    let reparsed = parse_gram_notation(&serialized).unwrap();

    // Semantic structure should be identical
    assert_eq!(parsed[0].elements.len(), reparsed[0].elements.len());
    assert_eq!(
        parsed[0].elements[0].value.identity.0,
        reparsed[0].elements[0].value.identity.0
    );
    assert_eq!(
        parsed[0].elements[1].value.identity.0,
        reparsed[0].elements[1].value.identity.0
    );
}

#[test]
fn test_round_trip_preserves_semantics_squiggle() {
    use gram_codec::serialize_pattern;

    // Parse squiggle right arrow
    let original = "(a)~~>(b)";
    let parsed = parse_gram_notation(original).unwrap();

    // Serialize
    let serialized = serialize_pattern(&parsed[0]).unwrap();

    // Re-parse
    let reparsed = parse_gram_notation(&serialized).unwrap();

    // Semantic structure preserved
    assert_eq!(parsed[0].elements.len(), 2);
    assert_eq!(reparsed[0].elements.len(), 2);
}

// ============================================================================
// Documentation Test: Arrow Kind Summary
// ============================================================================

#[test]
fn test_arrow_kind_documentation() {
    // This test documents the complete arrow kind mapping
    //
    // RIGHT_ARROW (directed left-to-right, elements: [left, right])
    // - Visual forms: -->, ==>, ~~>
    // - Element order: preserved as written
    // - Example: (a)-->(b) means "a points to b"
    assert!(parse_gram_notation("(a)-->(b)").is_ok());
    assert!(parse_gram_notation("(a)==>(b)").is_ok());
    assert!(parse_gram_notation("(a)~~>(b)").is_ok());

    // LEFT_ARROW (directed right-to-left, elements: [right, left] - REVERSED)
    // - Visual forms: <--, <==, <~~
    // - Element order: REVERSED from visual order
    // - Example: (a)<--(b) means "b points to a", stored as [b, a]
    let left_result = parse_gram_notation("(a)<--(b)").unwrap();
    assert_eq!(left_result[0].elements[0].value.identity.0, "b"); // Reversed!

    // BIDIRECTIONAL_ARROW (bidirectional, elements: [left, right])
    // - Visual forms: <-->, <==>
    // - Element order: preserved as written
    // - Example: (a)<-->(b) means "a and b point to each other"
    assert!(parse_gram_notation("(a)<-->(b)").is_ok());
    assert!(parse_gram_notation("(a)<==>(b)").is_ok());

    // UNDIRECTED_ARROW (no direction, elements: [first, second])
    // - Visual forms: ~~, ==
    // - Element order: preserved as written
    // - Example: (a)~~(b) means "a and b are connected without direction"
    assert!(parse_gram_notation("(a)~~(b)").is_ok());
    assert!(parse_gram_notation("(a)==(b)").is_ok());
}
