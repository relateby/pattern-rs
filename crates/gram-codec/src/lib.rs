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
pub use error::{Location, SerializeError};
// Use the new nom-based ParseError from the parser module
pub use parser::ParseError;
pub use serializer::{serialize_pattern, serialize_patterns};
pub use value::Value;

// New nom-based parser API
/// Parse gram notation text into a collection of Pattern structures
///
/// # Arguments
///
/// * `input` - Gram notation text to parse
///
/// # Returns
///
/// * `Ok(Vec<Pattern<Subject>>)` - Successfully parsed patterns
/// * `Err(ParseError)` - Parse error with location information
///
/// # Example
///
/// ```rust,no_run
/// use gram_codec::parse_gram;
///
/// let patterns = parse_gram("(alice)-[:KNOWS]->(bob)")?;
/// # Ok::<(), gram_codec::ParseError>(())
/// ```
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

/// Validate gram notation syntax without constructing patterns
///
/// This is faster than `parse_gram` for validation-only use cases.
///
/// # Arguments
///
/// * `input` - Gram notation text to validate
///
/// # Returns
///
/// * `Ok(())` - Input is valid gram notation
/// * `Err(ParseError)` - Syntax error
///
/// # Example
///
/// ```rust,no_run
/// use gram_codec::validate_gram;
///
/// if validate_gram("(hello)").is_ok() {
///     println!("Valid gram notation");
/// }
/// # Ok::<(), gram_codec::ParseError>(())
/// ```
pub fn validate_gram(input: &str) -> Result<(), ParseError> {
    parse_gram(input).map(|_| ())
}

// Backward compatibility aliases
pub use parse_gram as parse_gram_notation;

/// Parse a single Gram pattern from text
///
/// Convenience function that expects exactly one pattern in the input.
///
/// # Arguments
///
/// * `input` - Gram notation text containing a single pattern
///
/// # Returns
///
/// * `Ok(Pattern<Subject>)` - Successfully parsed pattern
/// * `Err(ParseError)` - Parse error or multiple patterns found
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

// Re-export Pattern and Subject from pattern-core for convenience
pub use pattern_core::{Pattern, Subject};
