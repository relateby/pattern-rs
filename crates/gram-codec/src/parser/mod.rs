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
/// A gram file itself is a pattern:
/// - Optional leading `{}` becomes the file-level pattern's properties
/// - All patterns in the file become the file-level pattern's elements
/// - Always returns a single pattern (the file-level pattern)
pub fn gram_patterns(input: &str) -> ParseResult<'_, Vec<Pattern<Subject>>> {
    use nom::multi::many0;

    map(
        delimited(
            ws,
            pair(
                // Optional leading record = file properties
                opt(subject::record),
                // All patterns = file elements
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
        |(properties_opt, elements): (
            Option<std::collections::HashMap<String, pattern_core::Value>>,
            Vec<Pattern<Subject>>,
        )| {
            // Special cases:
            // 1. Empty file → return empty vec
            if properties_opt.is_none() && elements.is_empty() {
                return vec![];
            }
            // 2. Single pattern, no properties → return it directly
            if properties_opt.is_none() && elements.len() == 1 {
                vec![elements.into_iter().next().unwrap()]
            }
            // 3. Multiple patterns OR has properties → wrap in file-level pattern
            else {
                let subject = Subject {
                    identity: pattern_core::Symbol(String::new()),
                    labels: std::collections::HashSet::new(),
                    properties: properties_opt.unwrap_or_default(),
                };
                vec![Pattern::pattern(subject, elements)]
            }
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
        // Multiple patterns → wrapped in file-level pattern
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].elements().len(), 3); // File-level pattern with 3 elements
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
        // Multiple patterns → wrapped in file-level pattern
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].elements().len(), 2); // File-level pattern with 2 elements
        assert_eq!(remaining, "");
    }
}
