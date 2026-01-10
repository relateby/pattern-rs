//! Value enum for heterogeneous property types in Gram notation

use std::fmt;

/// Represents all possible value types in Gram notation property records.
/// Supports all value types defined in the tree-sitter-gram grammar.
#[derive(Debug, Clone)]
pub enum Value {
    /// Unquoted or quoted string
    /// Example: `"Alice"`, `hello`
    String(String),

    /// Integer value with full i64 range
    /// Example: `42`, `-10`, `0`
    Integer(i64),

    /// Decimal/floating-point value
    /// Example: `3.14`, `-2.5`, `0.0`
    Decimal(f64),

    /// Boolean value
    /// Example: `true`, `false`
    Boolean(bool),

    /// Array of values (may be heterogeneous)
    /// Example: `["rust", 42, true]`
    Array(Vec<Value>),

    /// Numeric range with inclusive bounds
    /// Example: `1..10`, `0..100`
    Range { lower: i64, upper: i64 },

    /// Tagged string with format identifier
    /// Example: `"""markdown # Heading"""`
    TaggedString { tag: String, content: String },
}

impl Value {
    // TODO: tree-sitter parsing methods removed during nom parser migration
    // Value parsing is now handled by parser::value module

    /* Commented out during migration to nom parser
    pub fn from_tree_sitter_node_OLD(
        node: &TREE_SITTER_NODE,
        source: &str,
    ) -> Result<Self, ParseError> {
        match node.kind() {
            "symbol" => {
                let text = node
                    .utf8_text(source.as_bytes())
                    .map_err(|e| Self::node_parse_error(node, format!("UTF-8 error: {}", e)))?;
                Ok(Value::String(text.to_string()))
            }
            "string_literal" => {
                let content = extract_string_content(node, source)?;
                Ok(Value::String(content))
            }
            "integer" => {
                let text = node
                    .utf8_text(source.as_bytes())
                    .map_err(|e| Self::node_parse_error(node, format!("UTF-8 error: {}", e)))?;
                let value = text
                    .parse::<i64>()
                    .map_err(|e| Self::node_parse_error(node, format!("Invalid integer: {}", e)))?;
                Ok(Value::Integer(value))
            }
            "decimal" => {
                let text = node
                    .utf8_text(source.as_bytes())
                    .map_err(|e| Self::node_parse_error(node, format!("UTF-8 error: {}", e)))?;
                let value = text
                    .parse::<f64>()
                    .map_err(|e| Self::node_parse_error(node, format!("Invalid decimal: {}", e)))?;
                Ok(Value::Decimal(value))
            }
            "boolean_literal" => {
                let text = node
                    .utf8_text(source.as_bytes())
                    .map_err(|e| Self::node_parse_error(node, format!("UTF-8 error: {}", e)))?;
                let value = text == "true";
                Ok(Value::Boolean(value))
            }
            "array" => {
                let mut values = Vec::new();
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.is_named() && child.kind() != "," {
                        values.push(Value::from_tree_sitter_node(&child, source)?);
                    }
                }
                Ok(Value::Array(values))
            }
            "range" => {
                let lower = extract_range_bound(node, "lower", source)?;
                let upper = extract_range_bound(node, "upper", source)?;
                Ok(Value::Range { lower, upper })
            }
            "tagged_string" => {
                let (tag, content) = extract_tagged_string(node, source)?;
                Ok(Value::TaggedString { tag, content })
            }
            _ => panic!("tree-sitter parsing no longer supported"),
        }
    }
    */
    // End of commented tree-sitter code

    /// Serialize value to gram notation
    pub fn to_gram_notation(&self) -> String {
        match self {
            // Strings are always quoted in gram notation property values
            Value::String(s) => format!("\"{}\"", escape_string(s)),
            Value::Integer(i) => i.to_string(),
            Value::Decimal(f) => format_decimal(*f),
            Value::Boolean(b) => b.to_string(),
            Value::Array(values) => {
                let items: Vec<String> = values.iter().map(|v| v.to_gram_notation()).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Range { lower, upper } => format!("{}..{}", lower, upper),
            Value::TaggedString { tag, content } => {
                if tag.is_empty() {
                    format!("\"\"\"{}\"\"\"", content)
                } else {
                    format!("\"\"\"{}{}\"\"\"", tag, content)
                }
            }
        }
    }

    /// Get the type name of this value (for error messages)
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::String(_) => "string",
            Value::Integer(_) => "integer",
            Value::Decimal(_) => "decimal",
            Value::Boolean(_) => "boolean",
            Value::Array(_) => "array",
            Value::Range { .. } => "range",
            Value::TaggedString { .. } => "tagged string",
        }
    }

    // TODO: node_parse_error removed during migration
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_gram_notation())
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Decimal(a), Value::Decimal(b)) => {
                // Use epsilon comparison for floats
                (a - b).abs() < f64::EPSILON
            }
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => a == b,
            (
                Value::Range {
                    lower: l1,
                    upper: u1,
                },
                Value::Range {
                    lower: l2,
                    upper: u2,
                },
            ) => l1 == l2 && u1 == u2,
            (
                Value::TaggedString {
                    tag: t1,
                    content: c1,
                },
                Value::TaggedString {
                    tag: t2,
                    content: c2,
                },
            ) => t1 == t2 && c1 == c2,
            _ => false,
        }
    }
}

// Helper Functions

/// Escape special characters in strings
pub(crate) fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\t', "\\t")
        .replace('\r', "\\r")
}

/// Format decimal to avoid unnecessary trailing zeros while distinguishing from integers
pub(crate) fn format_decimal(f: f64) -> String {
    if f.fract() == 0.0 && f.is_finite() {
        format!("{:.1}", f) // Always include .0 to distinguish from integer
    } else {
        f.to_string()
    }
}

// TODO: tree-sitter helper functions removed during migration to nom parser
// Value parsing is now handled by parser::value module

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_value_serialization() {
        // Strings are always quoted in gram notation property values
        assert_eq!(
            Value::String("hello".to_string()).to_gram_notation(),
            "\"hello\""
        );
        assert_eq!(
            Value::String("Hello World".to_string()).to_gram_notation(),
            "\"Hello World\""
        );
        assert_eq!(Value::String("".to_string()).to_gram_notation(), "\"\"");
    }

    #[test]
    fn test_integer_value_serialization() {
        assert_eq!(Value::Integer(42).to_gram_notation(), "42");
        assert_eq!(Value::Integer(-10).to_gram_notation(), "-10");
        assert_eq!(Value::Integer(0).to_gram_notation(), "0");
    }

    #[test]
    fn test_decimal_value_serialization() {
        assert_eq!(Value::Decimal(3.14).to_gram_notation(), "3.14");
        assert_eq!(Value::Decimal(0.0).to_gram_notation(), "0.0");
        assert_eq!(Value::Decimal(-2.5).to_gram_notation(), "-2.5");
    }

    #[test]
    fn test_boolean_value_serialization() {
        assert_eq!(Value::Boolean(true).to_gram_notation(), "true");
        assert_eq!(Value::Boolean(false).to_gram_notation(), "false");
    }

    #[test]
    fn test_array_value_serialization() {
        let v = Value::Array(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3),
        ]);
        assert_eq!(v.to_gram_notation(), "[1, 2, 3]");

        // Heterogeneous array
        let v = Value::Array(vec![
            Value::String("rust".to_string()),
            Value::Integer(42),
            Value::Boolean(true),
        ]);
        // Strings are always quoted
        assert_eq!(v.to_gram_notation(), "[\"rust\", 42, true]");

        // Empty array
        assert_eq!(Value::Array(vec![]).to_gram_notation(), "[]");
    }

    #[test]
    fn test_range_value_serialization() {
        let v = Value::Range {
            lower: 1,
            upper: 10,
        };
        assert_eq!(v.to_gram_notation(), "1..10");

        let v = Value::Range {
            lower: -5,
            upper: 5,
        };
        assert_eq!(v.to_gram_notation(), "-5..5");
    }

    #[test]
    fn test_tagged_string_serialization() {
        let v = Value::TaggedString {
            tag: "markdown".to_string(),
            content: "# Heading".to_string(),
        };
        assert_eq!(v.to_gram_notation(), "\"\"\"markdown# Heading\"\"\"");

        let v = Value::TaggedString {
            tag: String::new(),
            content: "Plain text".to_string(),
        };
        assert_eq!(v.to_gram_notation(), "\"\"\"Plain text\"\"\"");
    }

    #[test]
    fn test_value_type_names() {
        assert_eq!(Value::String("".to_string()).type_name(), "string");
        assert_eq!(Value::Integer(0).type_name(), "integer");
        assert_eq!(Value::Decimal(0.0).type_name(), "decimal");
        assert_eq!(Value::Boolean(false).type_name(), "boolean");
        assert_eq!(Value::Array(vec![]).type_name(), "array");
        assert_eq!(Value::Range { lower: 0, upper: 0 }.type_name(), "range");
        assert_eq!(
            Value::TaggedString {
                tag: String::new(),
                content: String::new()
            }
            .type_name(),
            "tagged string"
        );
    }

    #[test]
    fn test_value_equality() {
        assert_eq!(Value::Integer(42), Value::Integer(42));
        assert_ne!(Value::Integer(42), Value::Integer(43));
        assert_ne!(Value::Integer(42), Value::String("42".to_string()));

        // Decimal epsilon comparison
        assert_eq!(Value::Decimal(1.0), Value::Decimal(1.0));

        // Arrays
        assert_eq!(
            Value::Array(vec![Value::Integer(1), Value::Integer(2)]),
            Value::Array(vec![Value::Integer(1), Value::Integer(2)])
        );
    }

    #[test]
    fn test_escape_string() {
        assert_eq!(escape_string("hello"), "hello");
        assert_eq!(escape_string("hello\"world"), "hello\\\"world");
        assert_eq!(escape_string("line1\nline2"), "line1\\nline2");
        assert_eq!(escape_string("tab\there"), "tab\\there");
        assert_eq!(escape_string("back\\slash"), "back\\\\slash");
    }

    #[test]
    fn test_format_decimal() {
        assert_eq!(format_decimal(3.14), "3.14");
        assert_eq!(format_decimal(0.0), "0.0");
        assert_eq!(format_decimal(1.0), "1.0");
        assert_eq!(format_decimal(-2.5), "-2.5");
    }
}
