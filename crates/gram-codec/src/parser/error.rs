//! Error types for the nom-based parser

use super::types::Location;
use thiserror::Error;

/// Errors that can occur during parsing
#[derive(Debug, Error)]
pub enum ParseError {
    /// Syntax error with location and context
    #[error("Syntax error at {location}: expected {expected}, found '{found}'")]
    SyntaxError {
        location: Location,
        expected: String,
        found: String,
        context: Vec<String>,
    },

    /// Unexpected input after successful parse
    #[error("Unexpected input at {location}: {snippet}")]
    UnexpectedInput { location: Location, snippet: String },

    /// Invalid value (number, string, identifier)
    #[error("Invalid {kind} at {location}: {reason}")]
    InvalidValue {
        location: Location,
        kind: String,
        reason: String,
    },

    /// Unmatched delimiter (bracket, paren, brace)
    #[error("Unmatched {delimiter} at {location}")]
    UnmatchedDelimiter { location: Location, delimiter: char },

    /// Internal parser error (should not occur in production)
    #[error("Internal parser error: {message}")]
    Internal { message: String },
}

impl ParseError {
    /// Create a syntax error from nom's VerboseError
    pub fn from_nom_error(input: &str, err: nom::Err<nom::error::VerboseError<&str>>) -> Self {
        match err {
            nom::Err::Error(e) | nom::Err::Failure(e) => {
                let (error_input, kind) = e
                    .errors
                    .first()
                    .map(|(i, k)| (*i, k))
                    .unwrap_or((input, &nom::error::VerboseErrorKind::Context("unknown")));

                let offset = input.len() - error_input.len();
                let location = Location::from_offset(input, offset);

                let found = error_input.chars().take(20).collect::<String>();
                let expected = format!("{:?}", kind);

                ParseError::SyntaxError {
                    location,
                    expected,
                    found,
                    context: Vec::new(),
                }
            }
            nom::Err::Incomplete(_) => ParseError::Internal {
                message: "Unexpected incomplete parse (streaming not supported)".to_string(),
            },
        }
    }

    /// Get the location of this error
    pub fn location(&self) -> Option<Location> {
        match self {
            ParseError::SyntaxError { location, .. }
            | ParseError::UnexpectedInput { location, .. }
            | ParseError::InvalidValue { location, .. }
            | ParseError::UnmatchedDelimiter { location, .. } => Some(*location),
            ParseError::Internal { .. } => None,
        }
    }

    /// Add context to this error
    pub fn with_context(mut self, context: String) -> Self {
        if let ParseError::SyntaxError {
            context: ref mut ctx,
            ..
        } = self
        {
            ctx.push(context);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_location() {
        let err = ParseError::SyntaxError {
            location: Location::new(2, 5, 10),
            expected: "identifier".to_string(),
            found: "123".to_string(),
            context: vec![],
        };

        assert_eq!(err.location().unwrap().line, 2);
        assert_eq!(err.location().unwrap().column, 5);
    }

    #[test]
    fn test_error_with_context() {
        let err = ParseError::SyntaxError {
            location: Location::new(1, 1, 0),
            expected: "value".to_string(),
            found: "x".to_string(),
            context: vec![],
        };

        let err = err.with_context("in record".to_string());

        match err {
            ParseError::SyntaxError { context, .. } => {
                assert_eq!(context.len(), 1);
                assert_eq!(context[0], "in record");
            }
            _ => panic!("Expected SyntaxError"),
        }
    }
}
