//! Pure Rust nom-based parser for gram notation
//!
//! This module replaces the tree-sitter-based parser with a pure Rust implementation
//! using nom parser combinators, enabling zero C dependencies and seamless WASM builds.

// Module declarations
pub mod annotation;
pub mod combinators;
pub mod error;
pub mod node;
pub mod relationship;
pub mod subject;
pub mod types;
pub mod value;

// Re-exports
pub use error::ParseError;
pub use types::{Location, ParseResult};

use combinators::ws;
use nom::{
    branch::alt,
    character::complete::char,
    combinator::{cut, map, opt, success},
    multi::separated_list0,
    sequence::{delimited, pair, separated_pair},
};
use pattern_core::{Pattern, Subject};

/// Parse a pattern reference: just a bare identifier (e.g., `alice`)
fn pattern_reference(input: &str) -> ParseResult<'_, Pattern<Subject>> {
    map(value::unquoted_identifier, |id| {
        Pattern::point(Subject {
            identity: pattern_core::Symbol(id),
            labels: std::collections::HashSet::new(),
            properties: std::collections::HashMap::new(),
        })
    })(input)
}

/// Parse an element in a subject pattern: can be a full pattern or just a reference
fn subject_element(input: &str) -> ParseResult<'_, Pattern<Subject>> {
    alt((
        gram_pattern,      // Try full pattern first
        pattern_reference, // Fall back to bare identifier reference
    ))(input)
}

/// Parse a subject pattern: [subject | elements] or [subject] or []
/// This is defined here to avoid circular dependencies
pub fn subject_pattern(input: &str) -> ParseResult<'_, Pattern<Subject>> {
    delimited(
        char('['),
        delimited(
            ws,
            alt((
                // Form 1: [subject | elements]
                map(
                    separated_pair(
                        subject::subject,
                        delimited(ws, char('|'), ws),
                        separated_list0(
                            delimited(ws, char(','), ws),
                            subject_element, // Can be pattern or reference
                        ),
                    ),
                    |(subj, elements)| Pattern::pattern(subj, elements),
                ),
                // Form 2: [subject] - just subject, no elements
                map(subject::subject, Pattern::point),
                // Form 3: [] - empty subject, no elements
                map(success(()), |_| {
                    Pattern::point(Subject {
                        identity: pattern_core::Symbol(String::new()),
                        labels: std::collections::HashSet::new(),
                        properties: std::collections::HashMap::new(),
                    })
                }),
            )),
            ws,
        ),
        cut(char(']')),
    )(input)
}

/// Parse an annotated pattern: @key(value) pattern
fn annotated_pattern(input: &str) -> ParseResult<'_, Pattern<Subject>> {
    map(
        pair(delimited(ws, annotation::annotation, ws), gram_pattern),
        |(_ann, pattern)| {
            // For now, annotations are not stored in the Pattern structure
            // They would need to be added to the Pattern type in pattern-core
            // TODO: Handle annotation metadata properly
            pattern
        },
    )(input)
}

/// Parse any gram pattern (non-top-level)
/// Dispatch to the appropriate parser based on syntax
/// Note: Standalone records `{}` are only valid at top-level and handled by gram_patterns
pub fn gram_pattern(input: &str) -> ParseResult<'_, Pattern<Subject>> {
    delimited(
        ws,
        alt((
            annotated_pattern,          // @key(value) pattern
            subject_pattern,            // [subject | elements]
            relationship::path_pattern, // (a)-->(b)-->(c)
            node::node,                 // (subject)
        )),
        ws,
    )(input)
}

/// Parse multiple gram patterns (top-level)
///
/// Returns all top-level patterns found in the input.
/// If a leading record `{}` is present, it is returned as the first pattern
/// (a bare pattern with properties but no identity/labels/elements).
pub fn gram_patterns(input: &str) -> ParseResult<'_, Vec<Pattern<Subject>>> {
    use nom::multi::many0;

    map(
        delimited(
            ws,
            pair(
                // Optional leading record
                opt(subject::record),
                // All patterns
                many0(delimited(
                    ws,
                    alt((
                        annotated_pattern,
                        subject_pattern,
                        relationship::path_pattern,
                        node::node,
                    )),
                    ws,
                )),
            ),
            ws,
        ),
        |(properties_opt, mut elements): (
            Option<std::collections::HashMap<String, pattern_core::Value>>,
            Vec<Pattern<Subject>>,
        )| {
            if let Some(properties) = properties_opt {
                let header_subject = Subject {
                    identity: pattern_core::Symbol(String::new()),
                    labels: std::collections::HashSet::new(),
                    properties,
                };
                elements.insert(0, Pattern::point(header_subject));
            }
            elements
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gram_pattern_node() {
        let (remaining, pattern) = gram_pattern("(hello)").unwrap();
        assert_eq!(pattern.value().identity.0, "hello");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_gram_pattern_relationship() {
        let (remaining, pattern) = gram_pattern("(a)-->(b)").unwrap();
        assert_eq!(pattern.elements().len(), 2);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_gram_pattern_subject_pattern() {
        let (remaining, pattern) = gram_pattern("[team | (alice), (bob)]").unwrap();
        assert_eq!(pattern.value().identity.0, "team");
        assert_eq!(pattern.elements().len(), 2);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_gram_patterns_multiple() {
        let (remaining, patterns) = gram_patterns("(a) (b) (c)").unwrap();
        // Returns all 3 patterns directly
        assert_eq!(patterns.len(), 3);
        assert_eq!(patterns[0].value().identity.0, "a");
        assert_eq!(patterns[1].value().identity.0, "b");
        assert_eq!(patterns[2].value().identity.0, "c");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_gram_patterns_empty() {
        let (remaining, patterns) = gram_patterns("").unwrap();
        assert_eq!(patterns.len(), 0);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_gram_patterns_with_whitespace() {
        let (remaining, patterns) = gram_patterns("  (a)  \n  (b)  ").unwrap();
        assert_eq!(patterns.len(), 2);
        assert_eq!(patterns[0].value().identity.0, "a");
        assert_eq!(patterns[1].value().identity.0, "b");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_gram_patterns_with_leading_record() {
        let (remaining, patterns) = gram_patterns("{k:'v'} (a)").unwrap();
        assert_eq!(patterns.len(), 2);
        // First pattern is the bare record
        assert_eq!(patterns[0].value().identity.0, "");
        assert_eq!(patterns[0].value().properties.len(), 1);
        // Second pattern is the node
        assert_eq!(patterns[1].value().identity.0, "a");
        assert_eq!(remaining, "");
    }
}
