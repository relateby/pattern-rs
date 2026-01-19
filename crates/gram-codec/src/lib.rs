//! # Gram Codec
//!
//! Bidirectional codec between Gram notation (human-readable text format) and Pattern data structures.
//!
//! This crate provides:
//! - **Parsing**: Transform Gram notation text into Pattern structures
//! - **Serialization**: Transform Pattern structures into valid Gram notation
//!
//! ## Features
//!
//! - Full support for all Gram syntax forms (nodes, relationships, subject patterns, annotations)
//! - Round-trip correctness (parse → serialize → parse produces equivalent pattern)
//! - Error recovery (reports all syntax errors, not just the first)
//! - Multi-platform support (native Rust, WebAssembly, Python)
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use gram_codec::{parse_gram_notation, serialize_pattern};
//!
//! // Parse gram notation into patterns
//! let gram_text = "(alice:Person {name: \"Alice\"})-[:KNOWS]->(bob:Person {name: \"Bob\"})";
//! let patterns = parse_gram_notation(gram_text)?;
//!
//! // Serialize patterns back to gram notation
//! for pattern in &patterns {
//!     let output = serialize_pattern(pattern)?;
//!     println!("{}", output);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Grammar Authority
//!
//! This codec uses [`tree-sitter-gram`](https://github.com/gram-data/tree-sitter-gram) as the
//! authoritative grammar specification. The parser implementation is pure Rust using nom parser
//! combinators, validated for 100% conformance with the tree-sitter-gram test corpus.

// Module declarations
pub mod ast;
mod error;
mod serializer;
mod value;

// TODO: Temporarily commented out during migration to nom parser
// Old tree-sitter parser (will be replaced)
// mod parser;
// pub(crate) mod transform;

// New nom-based parser module (under development)
mod parser;

// Optional platform-specific modules
#[cfg(feature = "wasm")]
mod wasm;

#[cfg(feature = "python")]
mod python;

// Public API exports
pub use ast::{AstPattern, AstSubject};
pub use error::{Location, SerializeError};
// Use the new nom-based ParseError from the parser module
pub use parser::ParseError;
pub use serializer::{serialize_pattern, serialize_patterns, to_gram, to_gram_with_header};
pub use value::Value;

// Re-export Pattern and Subject from pattern-core for convenience
pub use pattern_core::{Pattern, PropertyRecord as Record, Subject};

// --- New nom-based parser API ---

/// Parse gram notation text into a collection of Pattern structures.
///
/// This is the foundational parser for gram notation. It returns all top-level elements,
/// including any leading record (which appears as a bare pattern with properties but
/// no identity, labels, or elements).
///
/// # Arguments
///
/// * `input` - Gram notation text to parse
///
/// # Returns
///
/// * `Ok(Vec<Pattern<Subject>>)` - Successfully parsed patterns
/// * `Err(ParseError)` - Parse error with location information
pub fn parse_gram(input: &str) -> Result<Vec<Pattern<Subject>>, ParseError> {
    // Handle empty/whitespace-only input
    if input.trim().is_empty() {
        return Ok(vec![]);
    }

    // Parse using nom parser
    match parser::gram_patterns(input) {
        Ok((remaining, patterns)) => {
            // Check if all input was consumed
            if !remaining.trim().is_empty() {
                let offset = input.len() - remaining.len();
                let location = parser::Location::from_offset(input, offset);
                return Err(ParseError::UnexpectedInput {
                    location,
                    snippet: remaining.chars().take(20).collect(),
                });
            }
            Ok(patterns)
        }
        Err(e) => Err(parser::ParseError::from_nom_error(input, e)),
    }
}

/// Parse gram notation, separating an optional header record from the patterns.
///
/// If the first element is a bare record (identity and labels are empty, and it has no elements),
/// it is returned separately as the header.
///
/// # Arguments
///
/// * `input` - Gram notation text to parse
///
/// # Returns
///
/// * `Ok((Option<Record>, Vec<Pattern<Subject>>))` - Successfully parsed header and patterns
/// * `Err(ParseError)` - If parsing fails
pub fn parse_gram_with_header(
    input: &str,
) -> Result<(Option<Record>, Vec<Pattern<Subject>>), ParseError> {
    let mut patterns = parse_gram(input)?;

    if patterns.is_empty() {
        return Ok((None, vec![]));
    }

    // Check if the first pattern is a bare record
    let first = &patterns[0];
    if first.value.identity.0.is_empty()
        && first.value.labels.is_empty()
        && first.elements.is_empty()
        && !first.value.properties.is_empty()
    {
        let header_record = patterns.remove(0).value.properties;
        Ok((Some(header_record), patterns))
    } else {
        Ok((None, patterns))
    }
}

/// Parse gram notation to AST (Abstract Syntax Tree).
///
/// Returns a single AstPattern representing the file-level pattern.
/// This is the **recommended output format** for cross-language consumption.
///
/// # Why AST?
///
/// - **Language-agnostic**: Pure JSON, works everywhere.
/// - **Complete**: No information loss.
/// - **Simple**: Just patterns and subjects (no graph concepts).
///
/// # Arguments
///
/// * `input` - Gram notation text to parse
///
/// # Returns
///
/// * `Ok(AstPattern)` - The parsed pattern as AST
/// * `Err(ParseError)` - If parsing fails
pub fn parse_to_ast(input: &str) -> Result<AstPattern, ParseError> {
    let patterns = parse_gram(input)?;

    if patterns.is_empty() {
        return Ok(AstPattern::empty());
    }

    // Maintain "single file-level pattern" contract for AST
    // If there's exactly one pattern and it's not a bare record, return it.
    // Otherwise, wrap everything in a file-level pattern.
    let document_pattern = wrap_as_document(patterns);
    Ok(AstPattern::from_pattern(&document_pattern))
}

/// Internal helper to wrap multiple patterns into a single document-level pattern.
fn wrap_as_document(mut patterns: Vec<Pattern<Subject>>) -> Pattern<Subject> {
    if patterns.len() == 1 {
        let first = &patterns[0];
        // If it's a "real" pattern (has identity or labels or elements), return it.
        // Also return it if it has properties but no other fields (a bare record),
        // because as a single pattern it represents the whole document.
        if !first.value.identity.0.is_empty()
            || !first.value.labels.is_empty()
            || !first.elements.is_empty()
            || !first.value.properties.is_empty()
        {
            return patterns.remove(0);
        }
    }

    // Otherwise wrap everything (including the bare record if present)
    // Actually, if the first is a bare record, it becomes the document's properties
    let mut properties = Record::new();
    if !patterns.is_empty() {
        let first = &patterns[0];
        if first.value.identity.0.is_empty()
            && first.value.labels.is_empty()
            && first.elements.is_empty()
            && !first.value.properties.is_empty()
        {
            properties = patterns.remove(0).value.properties;
        }
    }

    let subject = Subject {
        identity: pattern_core::Symbol(String::new()),
        labels: std::collections::HashSet::new(),
        properties,
    };
    Pattern::pattern(subject, patterns)
}

/// Validate gram notation syntax without constructing patterns.
pub fn validate_gram(input: &str) -> Result<(), ParseError> {
    parse_gram(input).map(|_| ())
}

/// Parse a single Gram pattern from text.
pub fn parse_single_pattern(input: &str) -> Result<Pattern<Subject>, ParseError> {
    let patterns = parse_gram(input)?;

    match patterns.len() {
        0 => Err(ParseError::UnexpectedInput {
            location: parser::Location::start(),
            snippet: "Input contains no patterns".to_string(),
        }),
        1 => Ok(patterns.into_iter().next().unwrap()),
        n => Err(ParseError::UnexpectedInput {
            location: parser::Location::start(),
            snippet: format!("Input contains {} patterns, expected exactly 1", n),
        }),
    }
}

// Backward compatibility aliases
pub use parse_gram as parse_gram_notation;
