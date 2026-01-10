//! Python bindings for Gram Codec using PyO3
//!
//! This module provides Python-friendly bindings for parsing and serializing
//! gram notation, enabling use in Python data science and analysis workflows.
//!
//! # Usage in Python
//!
//! ```python
//! import gram_codec
//!
//! # Parse gram notation
//! result = gram_codec.parse_gram("(alice)-[:KNOWS]->(bob)")
//! print(f"Parsed {result['pattern_count']} patterns")
//! print(f"Identifiers: {result['identifiers']}")
//!
//! # Validate gram notation
//! is_valid = gram_codec.validate_gram("(hello)-->(world)")
//! print(f"Valid: {is_valid}")
//!
//! # Round-trip test
//! serialized = gram_codec.round_trip("(a)-->(b)")
//! print(f"Serialized: {serialized}")
//! ```

use crate::ast::AstPattern;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

/// Result of parsing gram notation
#[pyclass]
#[derive(Clone)]
pub struct ParseResult {
    /// Number of top-level patterns parsed
    #[pyo3(get)]
    pub pattern_count: usize,
    /// Identifiers of root patterns (for debugging)
    #[pyo3(get)]
    pub identifiers: Vec<String>,
}

#[pymethods]
impl ParseResult {
    fn __repr__(&self) -> String {
        format!(
            "ParseResult(pattern_count={}, identifiers={:?})",
            self.pattern_count, self.identifiers
        )
    }

    fn __str__(&self) -> String {
        format!(
            "Parsed {} pattern(s) with identifiers: {:?}",
            self.pattern_count, self.identifiers
        )
    }

    /// Convert to dictionary for easy Python access
    fn to_dict(&self) -> HashMap<String, PyObject> {
        Python::with_gil(|py| {
            let mut dict = HashMap::new();
            dict.insert(
                "pattern_count".to_string(),
                self.pattern_count.to_object(py),
            );
            dict.insert("identifiers".to_string(), self.identifiers.to_object(py));
            dict
        })
    }
}

/// Parse gram notation and return information about the parsed patterns
///
/// Args:
///     input (str): Gram notation string to parse
///
/// Returns:
///     ParseResult: Object containing pattern_count and identifiers
///
/// Raises:
///     ValueError: If the gram notation is invalid
///
/// Example:
///     >>> import gram_codec
///     >>> result = gram_codec.parse_gram("(alice)-[:KNOWS]->(bob)")
///     >>> print(result.pattern_count)
///     1
///     >>> print(result.identifiers)
///     []
#[pyfunction]
fn parse_gram(input: &str) -> PyResult<ParseResult> {
    // Use the main parse function
    let patterns = crate::parse_gram(input)
        .map_err(|e| PyValueError::new_err(format!("Parse error: {}", e)))?;

    // Extract identifiers from patterns
    let identifiers: Vec<String> = patterns
        .iter()
        .filter_map(|p| {
            let id = &p.value().identity.0;
            if !id.is_empty() {
                Some(id.clone())
            } else {
                None
            }
        })
        .collect();

    Ok(ParseResult {
        pattern_count: patterns.len(),
        identifiers,
    })
}

/// Validate gram notation without parsing
///
/// Args:
///     input (str): Gram notation string to validate
///
/// Returns:
///     bool: True if valid, False otherwise
///
/// Example:
///     >>> import gram_codec
///     >>> gram_codec.validate_gram("(hello)")
///     True
///     >>> gram_codec.validate_gram("(unclosed")
///     False
#[pyfunction]
fn validate_gram(input: &str) -> bool {
    crate::parse_gram(input).is_ok()
}

/// Parse gram notation, serialize it back, and return the serialized form
///
/// This is useful for normalizing gram notation or testing round-trip correctness.
///
/// Args:
///     input (str): Gram notation string
///
/// Returns:
///     str: Serialized gram notation
///
/// Raises:
///     ValueError: If parsing or serialization fails
///
/// Example:
///     >>> import gram_codec
///     >>> gram_codec.round_trip("(alice)-->(bob)")
///     '(alice)-->(bob)'
#[pyfunction]
fn round_trip(input: &str) -> PyResult<String> {
    let patterns = crate::parse_gram(input)
        .map_err(|e| PyValueError::new_err(format!("Parse error: {}", e)))?;

    let serialized: Result<Vec<String>, _> =
        patterns.iter().map(crate::serialize_pattern).collect();

    let serialized =
        serialized.map_err(|e| PyValueError::new_err(format!("Serialize error: {}", e)))?;

    Ok(serialized.join("\n"))
}

/// Serialize patterns to gram notation
///
/// Args:
///     patterns (list): List of pattern objects
///
/// Returns:
///     str: Serialized gram notation
///
/// Raises:
///     ValueError: If serialization fails
///
/// Note: This function is currently a placeholder. For now, use round_trip()
///     for parse -> serialize workflows.
#[pyfunction]
fn serialize_patterns(_patterns: Bound<'_, PyAny>) -> PyResult<String> {
    // For now, return an error indicating this is not yet implemented
    // In the future, this would take Python pattern objects and serialize them
    Err(PyValueError::new_err(
        "Direct pattern serialization not yet implemented. Use round_trip() instead.",
    ))
}

/// Parse gram notation to AST (Python dict)
///
/// Returns a single pattern as a Python dictionary.
/// This is the recommended way to parse gram in Python.
///
/// Args:
///     input (str): Gram notation text
///
/// Returns:
///     dict: Dictionary with structure:
///         {
///           'subject': {
///             'identity': str,
///             'labels': list[str],
///             'properties': dict
///           },
///           'elements': list[dict]
///         }
///
/// Raises:
///     ValueError: If parsing fails
///
/// Example:
///     >>> import gram_codec
///     >>> ast = gram_codec.parse_to_ast("(alice:Person {name: 'Alice'})")
///     >>> print(ast['subject']['identity'])
///     alice
///     >>> print(ast['subject']['labels'])
///     ['Person']
#[pyfunction]
fn parse_to_ast(py: Python, input: &str) -> PyResult<PyObject> {
    let ast = crate::parse_to_ast(input)
        .map_err(|e| PyValueError::new_err(format!("Parse error: {}", e)))?;

    // Convert AST to Python dict manually
    // Serialize to JSON first, then parse as Python
    let json_str = serde_json::to_string(&ast)
        .map_err(|e| PyValueError::new_err(format!("Serialization error: {}", e)))?;

    // Use Python's json module to parse the JSON string
    let json_module = py.import("json")?;
    let loads = json_module.getattr("loads")?;
    loads.call1((json_str,)).map(|obj| obj.into())
}

/// Get the version of gram-codec
///
/// Returns:
///     str: Version string (e.g., "0.1.0")
///
/// Example:
///     >>> import gram_codec
///     >>> gram_codec.version()
///     '0.1.0'
#[pyfunction]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Python module initialization
#[pymodule]
fn gram_codec(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_gram, m)?)?;
    m.add_function(wrap_pyfunction!(parse_to_ast, m)?)?;
    m.add_function(wrap_pyfunction!(validate_gram, m)?)?;
    m.add_function(wrap_pyfunction!(round_trip, m)?)?;
    m.add_function(wrap_pyfunction!(serialize_patterns, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_class::<ParseResult>()?;

    // Add module metadata
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
