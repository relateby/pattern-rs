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

use crate::ast::{AstPattern, ParseWithHeaderResult};
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

    crate::to_gram(&patterns).map_err(|e| PyValueError::new_err(format!("Serialize error: {}", e)))
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

/// Parse gram notation and return each top-level pattern as a Python dict.
///
/// Returns a list of dicts suitable for reconstructing Pattern<Subject> objects
/// in the relateby.pattern package. Each dict has the structure:
///   `{'subject': {'identity': str, 'labels': [str], 'properties': dict}, 'elements': [...]}`
///
/// Args:
///     input (str): Gram notation string
///
/// Returns:
///     list[dict]: One dict per top-level pattern
///
/// Raises:
///     ValueError: If parsing fails
#[pyfunction]
fn parse_patterns_as_dicts(py: Python, input: &str) -> PyResult<PyObject> {
    use crate::ast::AstPattern;
    let patterns = crate::parse_gram(input)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Parse error: {}", e)))?;

    let dicts: Vec<String> = patterns
        .iter()
        .map(|p| {
            let ast = AstPattern::from_pattern(p);
            serde_json::to_string(&ast).map_err(|e| {
                pyo3::exceptions::PyValueError::new_err(format!("Serialization error: {}", e))
            })
        })
        .collect::<PyResult<Vec<_>>>()?;

    let json_array = format!("[{}]", dicts.join(","));
    let json_module = py.import("json")?;
    let loads = json_module.getattr("loads")?;
    loads.call1((json_array,)).map(|obj| obj.into())
}

/// Parse gram notation, returning a Python list of AstPattern dicts.
///
/// Each dict has structure `{"subject": {"identity": str, "labels": [...],
/// "properties": {...}}, "elements": [...]}`.
///
/// Args:
///     input (str): Gram notation string
///
/// Returns:
///     list[dict]: One dict per top-level pattern
///
/// Raises:
///     ValueError: If parsing fails
#[pyfunction(name = "parse")]
fn parse_py(py: Python, input: &str) -> PyResult<PyObject> {
    if input.trim().is_empty() {
        return Ok(pyo3::types::PyList::empty(py).into());
    }
    let patterns = crate::parse_gram(input)
        .map_err(|e| PyValueError::new_err(format!("Parse error: {}", e)))?;
    let asts: Vec<AstPattern> = patterns.iter().map(AstPattern::from_pattern).collect();
    pythonize::pythonize(py, &asts)
        .map(|b| b.unbind())
        .map_err(|e| PyValueError::new_err(format!("Serialization error: {}", e)))
}

/// Serialize a Python list of AstPattern dicts to gram notation.
///
/// Args:
///     patterns_obj: Python list of AstPattern dicts
///
/// Returns:
///     str: Gram notation string
///
/// Raises:
///     ValueError: If deserialization or serialization fails
#[pyfunction(name = "stringify")]
fn stringify_py(py: Python, patterns_obj: Bound<'_, PyAny>) -> PyResult<String> {
    let asts: Vec<AstPattern> = pythonize::depythonize(&patterns_obj)
        .map_err(|e| PyValueError::new_err(format!("Deserialization error: {}", e)))?;
    let patterns: Vec<crate::Pattern<crate::Subject>> = asts
        .iter()
        .map(|ast| ast.to_pattern())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| PyValueError::new_err(format!("Conversion error: {}", e)))?;
    let _ = py;
    crate::to_gram(&patterns).map_err(|e| PyValueError::new_err(format!("Stringify error: {}", e)))
}

/// Parse gram notation, returning `{"header": dict | None, "patterns": [...]}`
///
/// Args:
///     input (str): Gram notation string
///
/// Returns:
///     dict: `{"header": dict | None, "patterns": list[dict]}`
///
/// Raises:
///     ValueError: If parsing fails
#[pyfunction(name = "parse_with_header")]
fn parse_with_header_py(py: Python, input: &str) -> PyResult<PyObject> {
    let (header, patterns) = crate::parse_gram_with_header(input)
        .map_err(|e| PyValueError::new_err(format!("Parse error: {}", e)))?;
    let result = ParseWithHeaderResult::from_parts(header, patterns);
    pythonize::pythonize(py, &result)
        .map(|b| b.unbind())
        .map_err(|e| PyValueError::new_err(format!("Serialization error: {}", e)))
}

/// Serialize `{"header": dict | None, "patterns": [...]}` to gram notation.
///
/// Args:
///     result_obj: dict with "header" and "patterns" keys
///
/// Returns:
///     str: Gram notation string
///
/// Raises:
///     ValueError: If deserialization or serialization fails
#[pyfunction(name = "stringify_with_header")]
fn stringify_with_header_py(py: Python, result_obj: Bound<'_, PyAny>) -> PyResult<String> {
    let result: ParseWithHeaderResult = pythonize::depythonize(&result_obj)
        .map_err(|e| PyValueError::new_err(format!("Deserialization error: {}", e)))?;
    let header = result
        .header_to_record()
        .map_err(|e| PyValueError::new_err(format!("Conversion error: {}", e)))?
        .unwrap_or_default();
    let patterns: Vec<crate::Pattern<crate::Subject>> = result
        .patterns
        .iter()
        .map(|ast| ast.to_pattern())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| PyValueError::new_err(format!("Conversion error: {}", e)))?;
    let _ = py;
    crate::to_gram_with_header(header, &patterns)
        .map_err(|e| PyValueError::new_err(format!("Stringify error: {}", e)))
}

/// Validate gram notation and return a list of error strings.
///
/// Args:
///     input (str): Gram notation string
///
/// Returns:
///     list[str]: Empty list if valid, list of error strings if invalid
#[pyfunction(name = "gram_validate")]
fn gram_validate_py(input: &str) -> Vec<String> {
    match crate::validate_gram(input) {
        Ok(()) => vec![],
        Err(e) => vec![e.to_string()],
    }
}

/// Python module initialization
#[pymodule]
fn gram_codec(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // New pythonize-based FFI functions (canonical names)
    m.add_function(wrap_pyfunction!(parse_py, m)?)?;
    m.add_function(wrap_pyfunction!(stringify_py, m)?)?;
    m.add_function(wrap_pyfunction!(parse_with_header_py, m)?)?;
    m.add_function(wrap_pyfunction!(stringify_with_header_py, m)?)?;
    m.add_function(wrap_pyfunction!(gram_validate_py, m)?)?;
    // Legacy functions retained for compatibility
    m.add_function(wrap_pyfunction!(parse_gram, m)?)?;
    m.add_function(wrap_pyfunction!(parse_to_ast, m)?)?;
    m.add_function(wrap_pyfunction!(parse_patterns_as_dicts, m)?)?;
    m.add_function(wrap_pyfunction!(validate_gram, m)?)?;
    m.add_function(wrap_pyfunction!(round_trip, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_class::<ParseResult>()?;

    // Add module metadata
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
