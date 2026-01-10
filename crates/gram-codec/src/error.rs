//! Error types for gram codec operations

use std::fmt;

/// Location in source code (1-indexed line and column)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location {
    /// Line number (1-indexed)
    pub line: usize,

    /// Column number (1-indexed)
    pub column: usize,
}

impl Location {
    /// Create a new location
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    // TODO: tree-sitter methods removed during nom parser migration
    // Location tracking is now handled by parser::Location

    /// Location at start of file
    pub fn start() -> Self {
        Self { line: 1, column: 1 }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Default for Location {
    fn default() -> Self {
        Self::start()
    }
}

/// Error during gram notation parsing
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct ParseError {
    /// Location where error occurred
    pub location: Location,

    /// Primary error message
    pub message: String,

    /// Additional errors (for error recovery)
    /// When error recovery is enabled, this contains all errors found
    pub errors: Vec<ParseError>,
}

impl ParseError {
    /// Create a new parse error at a specific location
    pub fn new(location: Location, message: String) -> Self {
        Self {
            location,
            message,
            errors: Vec::new(),
        }
    }

    // TODO: tree-sitter methods removed during nom parser migration
    // Error construction is now handled by parser::ParseError

    /// Add additional error (for error recovery)
    #[allow(dead_code)]
    pub fn with_error(mut self, error: ParseError) -> Self {
        self.errors.push(error);
        self
    }

    /// Create error for invalid pattern structure
    #[allow(dead_code)]
    pub fn invalid_structure(location: Location, details: String) -> Self {
        Self::new(location, format!("Invalid pattern structure: {}", details))
    }

    /// Total number of errors (including nested)
    #[allow(dead_code)]
    pub fn error_count(&self) -> usize {
        1 + self.errors.len()
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Parse error at {}:{}: {}",
            self.location.line, self.location.column, self.message
        )?;

        if !self.errors.is_empty() {
            write!(f, "\nAdditional errors:")?;
            for error in &self.errors {
                write!(f, "\n  - {}", error)?;
            }
        }

        Ok(())
    }
}

impl std::error::Error for ParseError {}

/// Error during pattern serialization to gram notation
#[derive(Debug, Clone, PartialEq)]
pub enum SerializeError {
    /// Pattern structure cannot be represented in gram notation
    InvalidStructure { reason: String },

    /// Invalid value that cannot be serialized
    InvalidValue { value_type: String, reason: String },

    /// Invalid identifier (contains disallowed characters)
    InvalidIdentifier { identifier: String, reason: String },

    /// Serialized output failed validation
    ValidationFailed { gram: String, reason: String },

    /// I/O error during serialization (e.g., writing to file)
    IoError { message: String },
}

impl SerializeError {
    /// Create error for invalid pattern structure
    pub fn invalid_structure(reason: impl Into<String>) -> Self {
        Self::InvalidStructure {
            reason: reason.into(),
        }
    }

    /// Create error for invalid value
    pub fn invalid_value(value_type: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidValue {
            value_type: value_type.into(),
            reason: reason.into(),
        }
    }

    /// Create error for invalid identifier
    pub fn invalid_identifier(identifier: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidIdentifier {
            identifier: identifier.into(),
            reason: reason.into(),
        }
    }

    /// Create error for validation failure
    pub fn validation_failed(gram: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::ValidationFailed {
            gram: gram.into(),
            reason: reason.into(),
        }
    }
}

impl fmt::Display for SerializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidStructure { reason } => {
                write!(f, "Cannot serialize pattern structure: {}", reason)
            }
            Self::InvalidValue { value_type, reason } => {
                write!(f, "Cannot serialize {} value: {}", value_type, reason)
            }
            Self::InvalidIdentifier { identifier, reason } => {
                write!(f, "Invalid identifier '{}': {}", identifier, reason)
            }
            Self::ValidationFailed { gram, reason } => {
                write!(
                    f,
                    "Serialized gram notation failed validation: {}\n{}",
                    reason, gram
                )
            }
            Self::IoError { message } => {
                write!(f, "I/O error: {}", message)
            }
        }
    }
}

impl std::error::Error for SerializeError {}

impl From<std::io::Error> for SerializeError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError {
            message: err.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_creation() {
        let loc = Location::new(10, 25);
        assert_eq!(loc.line, 10);
        assert_eq!(loc.column, 25);
    }

    #[test]
    fn test_location_display() {
        let loc = Location::new(1, 5);
        assert_eq!(loc.to_string(), "1:5");
    }

    #[test]
    fn test_location_start() {
        let loc = Location::start();
        assert_eq!(loc.line, 1);
        assert_eq!(loc.column, 1);
    }

    #[test]
    fn test_parse_error_creation() {
        let error = ParseError::new(Location::new(1, 5), "Unexpected token".to_string());

        assert_eq!(error.location.line, 1);
        assert_eq!(error.location.column, 5);
        assert_eq!(error.message, "Unexpected token");
        assert_eq!(error.errors.len(), 0);
    }

    #[test]
    fn test_parse_error_display() {
        let error = ParseError::new(Location::new(1, 5), "Unexpected token".to_string());

        assert_eq!(error.to_string(), "Parse error at 1:5: Unexpected token");
    }

    #[test]
    fn test_parse_error_with_recovery() {
        let mut primary = ParseError::new(Location::start(), "Multiple errors".to_string());
        primary = primary.with_error(ParseError::new(Location::new(1, 1), "Error 1".to_string()));
        primary = primary.with_error(ParseError::new(Location::new(2, 1), "Error 2".to_string()));

        assert_eq!(primary.error_count(), 3); // 1 primary + 2 additional

        let display = primary.to_string();
        assert!(display.contains("Multiple errors"));
        assert!(display.contains("Additional errors"));
        assert!(display.contains("Error 1"));
        assert!(display.contains("Error 2"));
    }

    #[test]
    fn test_serialize_error_invalid_structure() {
        let error = SerializeError::invalid_structure("Test reason");
        let display = error.to_string();
        assert!(display.contains("Test reason"));
        assert!(display.contains("Cannot serialize pattern structure"));
    }

    #[test]
    fn test_serialize_error_invalid_value() {
        let error = SerializeError::invalid_value("CustomType", "Not supported");
        let display = error.to_string();
        assert!(display.contains("CustomType"));
        assert!(display.contains("Not supported"));
    }

    #[test]
    fn test_serialize_error_invalid_identifier() {
        let error = SerializeError::invalid_identifier("hello world", "Contains whitespace");
        let display = error.to_string();
        assert!(display.contains("hello world"));
        assert!(display.contains("Contains whitespace"));
    }

    #[test]
    fn test_serialize_error_validation_failed() {
        let error = SerializeError::validation_failed("(unclosed", "gram-lint failed");
        let display = error.to_string();
        assert!(display.contains("(unclosed"));
        assert!(display.contains("gram-lint failed"));
    }

    #[test]
    fn test_serialize_error_io_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let error: SerializeError = io_err.into();

        match error {
            SerializeError::IoError { message } => {
                assert!(message.contains("File not found"));
            }
            _ => panic!("Expected IoError variant"),
        }
    }
}
