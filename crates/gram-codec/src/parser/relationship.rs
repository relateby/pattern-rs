//! Relationship and path pattern parsers for gram notation

use super::combinators::ws;
use super::node::node;
use super::types::{ArrowType, ParseResult};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, value as nom_value},
    multi::many1,
    sequence::{delimited, pair, tuple},
};
use pattern_core::{Pattern, Subject};

/// Parse an arrow: all gram notation arrow types
/// Order matters: longer patterns must come first to avoid partial matches
pub fn arrow(input: &str) -> ParseResult<'_, ArrowType> {
    delimited(
        ws,
        alt((
            // Squiggle arrows (check 4-char patterns first)
            nom_value(ArrowType::SquiggleBidirectional, tag("<~~>")),
            // Double arrows (check 4-char patterns)
            nom_value(ArrowType::DoubleBidirectional, tag("<==>")),
            // Single arrows (check 4-char patterns)
            nom_value(ArrowType::Bidirectional, tag("<-->")),
            // Squiggle arrows (3-char patterns)
            nom_value(ArrowType::SquiggleRight, tag("~~>")),
            nom_value(ArrowType::SquiggleLeft, tag("<~~")),
            // Double arrows (3-char patterns)
            nom_value(ArrowType::DoubleRight, tag("==>")),
            nom_value(ArrowType::DoubleLeft, tag("<==")),
            // Single arrows (3-char patterns)
            nom_value(ArrowType::Right, tag("-->")),
            nom_value(ArrowType::Left, tag("<--")),
            // Undirected (2-char patterns)
            nom_value(ArrowType::Squiggle, tag("~~")),
            nom_value(ArrowType::DoubleUndirected, tag("==")),
            nom_value(ArrowType::Undirected, tag("--")),
        )),
        ws,
    )(input)
}

/// Parse a relationship: (a)-->(b) or (a)-[subject]->(b)
/// Relationships have 2 elements (both atomic nodes)
#[allow(dead_code)]
pub fn relationship(input: &str) -> ParseResult<'_, Pattern<Subject>> {
    alt((relationship_with_edge_subject, relationship_simple))(input)
}

/// Parse a simple relationship without edge subject: (a)-->(b)
#[allow(dead_code)]
fn relationship_simple(input: &str) -> ParseResult<'_, Pattern<Subject>> {
    map(
        pair(node, pair(arrow, node)),
        |(left, (arrow_type, right))| {
            // Handle arrow directionality
            let (first, second) = if arrow_type.is_backward() {
                // Left arrows: reverse element order
                (right, left)
            } else {
                // Right, bidirectional, undirected: keep order
                (left, right)
            };

            // Create empty subject for relationship
            let empty_subject = Subject {
                identity: pattern_core::Symbol(String::new()),
                labels: std::collections::HashSet::new(),
                properties: std::collections::HashMap::new(),
            };

            Pattern::pattern(empty_subject, vec![first, second])
        },
    )(input)
}

/// Parse a relationship with edge subject: (a)-[r:LABEL]->(b)
#[allow(dead_code)]
fn relationship_with_edge_subject(input: &str) -> ParseResult<'_, Pattern<Subject>> {
    use super::subject::subject;

    map(
        tuple((
            node,
            ws,
            // Arrow left part: -, <-, ~, <~, =, <=, etc.
            arrow_left_part,
            // Edge subject in brackets
            delimited(char('['), delimited(ws, subject, ws), char(']')),
            // Arrow right part: ->, -, ~>, ~, =>, =, etc.
            arrow_right_part,
            ws,
            node,
        )),
        |(left, _, arrow_left, edge_subject, _arrow_right, _, right)| {
            // Determine directionality from arrow parts
            let is_backward = arrow_left.starts_with('<');

            let (first, second) = if is_backward {
                (right, left)
            } else {
                (left, right)
            };

            Pattern::pattern(edge_subject, vec![first, second])
        },
    )(input)
}

/// Parse left part of arrow: -, <-, ~, <~, =, <=
fn arrow_left_part(input: &str) -> ParseResult<'_, &str> {
    alt((
        tag("<~~"),
        tag("<=="),
        tag("<--"),
        tag("~~"),
        tag("=="),
        tag("--"),
        tag("<~"),
        tag("<="),
        tag("<-"),
        tag("~"),
        tag("="),
        tag("-"),
    ))(input)
}

/// Parse right part of arrow: ->, -, ~>, ~, =>, =
fn arrow_right_part(input: &str) -> ParseResult<'_, &str> {
    alt((
        tag("~~>"),
        tag("==>"),
        tag("-->"),
        tag("~~"),
        tag("=="),
        tag("--"),
        tag("~>"),
        tag("=>"),
        tag("->"),
        tag("~"),
        tag("="),
        tag("-"),
    ))(input)
}

/// Parse an arrow segment (with or without edge subject)
/// Returns: (ArrowType, Option<Subject>, Pattern<Subject>)
fn arrow_segment(input: &str) -> ParseResult<'_, (ArrowType, Option<Subject>, Pattern<Subject>)> {
    alt((arrow_segment_with_edge, arrow_segment_simple))(input)
}

/// Parse simple arrow segment: --> (node)
fn arrow_segment_simple(
    input: &str,
) -> ParseResult<'_, (ArrowType, Option<Subject>, Pattern<Subject>)> {
    map(pair(arrow, node), |(arrow_type, node_pattern)| {
        (arrow_type, None, node_pattern)
    })(input)
}

/// Parse arrow segment with edge subject: -[subject]-> (node)
fn arrow_segment_with_edge(
    input: &str,
) -> ParseResult<'_, (ArrowType, Option<Subject>, Pattern<Subject>)> {
    use super::subject::subject;

    map(
        tuple((
            ws,
            arrow_left_part,
            delimited(char('['), delimited(ws, subject, ws), char(']')),
            arrow_right_part,
            ws,
            node,
        )),
        |(_, arrow_left, edge_subject, arrow_right, _, next_node)| {
            // Determine arrow type from parts
            let arrow_type = determine_arrow_type(arrow_left, arrow_right);
            (arrow_type, Some(edge_subject), next_node)
        },
    )(input)
}

/// Determine arrow type from left and right parts
fn determine_arrow_type(left: &str, right: &str) -> ArrowType {
    // Combine to see what arrow it represents
    match (left, right) {
        // Bidirectional
        ("<-", "->") | ("<--", "-->") => ArrowType::Bidirectional,
        ("<~", "~>") | ("<~~", "~~>") => ArrowType::SquiggleBidirectional,
        ("<=", "=>") | ("<==", "==>") => ArrowType::DoubleBidirectional,
        // Right arrows
        ("-", "->") | ("--", "-->") => ArrowType::Right,
        ("~", "~>") | ("~~", "~~>") => ArrowType::SquiggleRight,
        ("=", "=>") | ("==", "==>") => ArrowType::DoubleRight,
        // Left arrows
        ("<-", "-") | ("<--", "--") => ArrowType::Left,
        ("<~", "~") | ("<~~", "~~") => ArrowType::SquiggleLeft,
        ("<=", "=") | ("<==", "==") => ArrowType::DoubleLeft,
        // Undirected
        ("-", "-") | ("--", "--") => ArrowType::Undirected,
        ("~", "~") | ("~~", "~~") => ArrowType::Squiggle,
        ("=", "=") | ("==", "==") => ArrowType::DoubleUndirected,
        // Default to undirected if unclear
        _ => ArrowType::Undirected,
    }
}

/// Parse a path pattern: (a)-->(b)-->(c) or (a)-[:LABEL]->(b)
/// Paths are flattened into nested structures from left to right
pub fn path_pattern(input: &str) -> ParseResult<'_, Pattern<Subject>> {
    map(pair(node, many1(arrow_segment)), |(first, segments)| {
        flatten_path_with_edges(first, segments)
    })(input)
}

/// Flatten path segments with optional edge subjects into nested pattern structure
fn flatten_path_with_edges(
    first: Pattern<Subject>,
    segments: Vec<(ArrowType, Option<Subject>, Pattern<Subject>)>,
) -> Pattern<Subject> {
    let mut current = first;

    for (arrow_type, edge_subject_opt, next_node) in segments {
        // Create a relationship pattern for each segment
        let (left, right) = if arrow_type.is_backward() {
            // Left arrows: reverse element order
            (next_node, current)
        } else {
            // Right, bidirectional, undirected: keep order
            (current, next_node)
        };

        // Use provided edge subject or create empty one
        let edge_subject = edge_subject_opt.unwrap_or_else(|| Subject {
            identity: pattern_core::Symbol(String::new()),
            labels: std::collections::HashSet::new(),
            properties: std::collections::HashMap::new(),
        });

        current = Pattern::pattern(edge_subject, vec![left, right]);
    }

    current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrow_right() {
        let (remaining, arr) = arrow("-->").unwrap();
        assert_eq!(arr, ArrowType::Right);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_arrow_left() {
        let (remaining, arr) = arrow("<--").unwrap();
        assert_eq!(arr, ArrowType::Left);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_arrow_bidirectional() {
        let (remaining, arr) = arrow("<-->").unwrap();
        assert_eq!(arr, ArrowType::Bidirectional);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_arrow_squiggle() {
        let (remaining, arr) = arrow("~~").unwrap();
        assert_eq!(arr, ArrowType::Squiggle);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_arrow_squiggle_right() {
        let (remaining, arr) = arrow("~~>").unwrap();
        assert_eq!(arr, ArrowType::SquiggleRight);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_arrow_undirected() {
        let (remaining, arr) = arrow("--").unwrap();
        assert_eq!(arr, ArrowType::Undirected);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_arrow_double_right() {
        let (remaining, arr) = arrow("==>").unwrap();
        assert_eq!(arr, ArrowType::DoubleRight);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_arrow_double_left() {
        let (remaining, arr) = arrow("<==").unwrap();
        assert_eq!(arr, ArrowType::DoubleLeft);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_arrow_double_bidirectional() {
        let (remaining, arr) = arrow("<==>").unwrap();
        assert_eq!(arr, ArrowType::DoubleBidirectional);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_arrow_squiggle_left() {
        let (remaining, arr) = arrow("<~~").unwrap();
        assert_eq!(arr, ArrowType::SquiggleLeft);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_arrow_squiggle_bidirectional() {
        let (remaining, arr) = arrow("<~~>").unwrap();
        assert_eq!(arr, ArrowType::SquiggleBidirectional);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_relationship_simple() {
        let (remaining, pattern) = relationship("(a)-->(b)").unwrap();
        assert_eq!(pattern.elements().len(), 2);
        assert_eq!(pattern.elements()[0].value().identity.0, "a");
        assert_eq!(pattern.elements()[1].value().identity.0, "b");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_relationship_left_arrow() {
        let (remaining, pattern) = relationship("(a)<--(b)").unwrap();
        // Left arrow reverses elements
        assert_eq!(pattern.elements().len(), 2);
        assert_eq!(pattern.elements()[0].value().identity.0, "b");
        assert_eq!(pattern.elements()[1].value().identity.0, "a");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_relationship_with_labels() {
        let (remaining, pattern) = relationship("(alice:Person)-->(bob:Person)").unwrap();
        assert_eq!(pattern.elements().len(), 2);
        assert!(pattern.elements()[0].value().labels.contains("Person"));
        assert!(pattern.elements()[1].value().labels.contains("Person"));
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_path_three_nodes() {
        let (remaining, pattern) = path_pattern("(a)-->(b)-->(c)").unwrap();
        assert_eq!(pattern.elements().len(), 2);

        // Nested structure: outer pattern has 2 elements
        // Left element is itself a relationship (a)-->(b)
        let left_rel = &pattern.elements()[0];
        assert_eq!(left_rel.elements().len(), 2);
        assert_eq!(left_rel.elements()[0].value().identity.0, "a");
        assert_eq!(left_rel.elements()[1].value().identity.0, "b");

        // Right element is node (c)
        assert_eq!(pattern.elements()[1].value().identity.0, "c");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_path_with_mixed_arrows() {
        let (remaining, pattern) = path_pattern("(a)-->(b)<--(c)").unwrap();
        assert_eq!(pattern.elements().len(), 2);
        assert_eq!(remaining, "");
    }
}
