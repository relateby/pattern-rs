//! Python bindings for Pattern-Core using PyO3
//!
//! This module provides Python-friendly bindings for pattern-core, enabling
//! Python developers to programmatically construct and operate on Pattern and Subject instances.
//!
//! # Usage in Python
//!
//! ```python
//! import pattern_core
//!
//! # Create an atomic pattern
//! atomic = pattern_core.Pattern.point("hello")
//!
//! # Create a pattern with Subject value
//! subject = pattern_core.Subject(
//!     identity="alice",
//!     labels={"Person"},
//!     properties={"name": pattern_core.Value.string("Alice")}
//! )
//! pattern = pattern_core.PatternSubject.point(subject)
//! ```

#[cfg(feature = "python")]
use pyo3::exceptions::{PyRecursionError, PyRuntimeError, PyTypeError, PyValueError};
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::{PyDict, PyList, PySet};
#[cfg(feature = "python")]
use std::collections::{HashMap, HashSet};

#[cfg(feature = "python")]
use crate::pattern::{Pattern, StructureAnalysis, ValidationError, ValidationRules};
#[cfg(feature = "python")]
use crate::subject::{RangeValue, Subject, Symbol, Value};

// ============================================================================
// Error Conversion Helpers
// ============================================================================

/// Convert Rust ValidationError to Python ValueError
#[cfg(feature = "python")]
pub(crate) fn validation_error_to_python(err: &ValidationError) -> PyErr {
    PyValueError::new_err(format!("Validation error: {:?}", err))
}

/// Convert type conversion errors to Python TypeError
#[cfg(feature = "python")]
pub(crate) fn type_error_to_python(msg: &str) -> PyErr {
    PyTypeError::new_err(msg.to_string())
}

/// Convert unexpected errors to Python RuntimeError
#[cfg(feature = "python")]
pub(crate) fn runtime_error_to_python(msg: &str) -> PyErr {
    PyRuntimeError::new_err(msg.to_string())
}

/// Convert stack overflow to Python RecursionError
#[cfg(feature = "python")]
#[allow(dead_code)]
pub(crate) fn recursion_error_to_python(msg: &str) -> PyErr {
    PyRecursionError::new_err(msg.to_string())
}

// ============================================================================
// Type Conversion Helpers
// ============================================================================

/// Convert Python value to Rust Value enum
#[cfg(feature = "python")]
#[allow(clippy::only_used_in_recursion)]
fn python_to_value(py: Python, obj: &Bound<'_, PyAny>) -> PyResult<Value> {
    // Try to extract as different Python types
    if let Ok(s) = obj.extract::<String>() {
        return Ok(Value::VString(s));
    }
    if let Ok(i) = obj.extract::<i64>() {
        return Ok(Value::VInteger(i));
    }
    if let Ok(f) = obj.extract::<f64>() {
        return Ok(Value::VDecimal(f));
    }
    if let Ok(b) = obj.extract::<bool>() {
        return Ok(Value::VBoolean(b));
    }
    if let Ok(list) = obj.downcast::<PyList>() {
        let mut vec = Vec::new();
        for item in list.iter() {
            vec.push(python_to_value(py, &item)?);
        }
        return Ok(Value::VArray(vec));
    }
    if let Ok(dict) = obj.downcast::<PyDict>() {
        let mut map = HashMap::new();
        for (key, value) in dict.iter() {
            let key_str: String = key.extract()?;
            let value_obj = python_to_value(py, &value)?;
            map.insert(key_str, value_obj);
        }
        return Ok(Value::VMap(map));
    }

    // If it's already a PyValue object, extract the inner value
    if let Ok(value_obj) = obj.extract::<PyValue>() {
        return Ok(value_obj.value.clone());
    }

    Err(type_error_to_python(&format!(
        "Cannot convert Python object to Value: {:?}",
        obj.get_type().name()?
    )))
}

/// Convert Rust Value enum to Python object
#[cfg(feature = "python")]
#[allow(deprecated)]
fn value_to_python(py: Python, value: &Value) -> PyResult<PyObject> {
    match value {
        Value::VString(s) => Ok(s.to_object(py)),
        Value::VInteger(i) => Ok(i.to_object(py)),
        Value::VDecimal(f) => Ok(f.to_object(py)),
        Value::VBoolean(b) => Ok(b.to_object(py)),
        Value::VSymbol(s) => Ok(s.to_object(py)),
        Value::VTaggedString { tag, content } => {
            let dict = PyDict::new_bound(py);
            dict.set_item("tag", tag)?;
            dict.set_item("content", content)?;
            Ok(dict.to_object(py))
        }
        Value::VArray(vec) => {
            let list = PyList::empty_bound(py);
            for item in vec {
                list.append(value_to_python(py, item)?.into_bound(py))?;
            }
            Ok(list.to_object(py))
        }
        Value::VMap(map) => {
            let dict = PyDict::new_bound(py);
            for (key, value) in map {
                dict.set_item(key, value_to_python(py, value)?.into_bound(py))?;
            }
            Ok(dict.to_object(py))
        }
        Value::VRange(range) => {
            let dict = PyDict::new_bound(py);
            dict.set_item("lower", range.lower.to_object(py))?;
            dict.set_item("upper", range.upper.to_object(py))?;
            Ok(dict.to_object(py))
        }
        Value::VMeasurement { unit, value } => {
            let dict = PyDict::new_bound(py);
            dict.set_item("unit", unit)?;
            dict.set_item("value", *value)?;
            Ok(dict.to_object(py))
        }
    }
}

/// Convert Python set to Rust HashSet<String>
#[cfg(feature = "python")]
fn python_set_to_hashset(py: Python, py_set: &Bound<'_, PySet>) -> PyResult<HashSet<String>> {
    let mut set = HashSet::new();
    for item in py_set.iter() {
        let s: String = item.extract()?;
        set.insert(s);
    }
    Ok(set)
}

/// Convert Rust HashSet<String> to Python set
#[cfg(feature = "python")]
fn hashset_to_python_set(py: Python, set: &HashSet<String>) -> PyResult<PyObject> {
    let py_set = PySet::empty_bound(py)?;
    for item in set {
        py_set.add(item)?;
    }
    Ok(py_set.to_object(py))
}

/// Convert Python dict to Rust HashMap<String, Value>
#[cfg(feature = "python")]
fn python_dict_to_value_map(
    py: Python,
    py_dict: &Bound<'_, PyDict>,
) -> PyResult<HashMap<String, Value>> {
    let mut map = HashMap::new();
    for (key, value) in py_dict.iter() {
        let key_str: String = key.extract()?;
        let value_obj = python_to_value(py, &value)?;
        map.insert(key_str, value_obj);
    }
    Ok(map)
}

/// Convert Rust HashMap<String, Value> to Python dict
#[cfg(feature = "python")]
fn value_map_to_python_dict(py: Python, map: &HashMap<String, Value>) -> PyResult<PyObject> {
    let dict = PyDict::new_bound(py);
    for (key, value) in map {
        dict.set_item(key, value_to_python(py, value)?.into_bound(py))?;
    }
    Ok(dict.to_object(py))
}

// ============================================================================
// Value Python Class
// ============================================================================

/// Python wrapper for Value enum.
///
/// Value represents property value types that can be stored in Subject properties.
/// Supports standard types (string, int, decimal, boolean, symbol) and extended types
/// (tagged string, array, map, range, measurement).
#[cfg(feature = "python")]
#[pyclass(name = "Value")]
#[derive(Clone)]
pub struct PyValue {
    value: Value,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyValue {
    /// Create a string value.
    ///
    /// Args:
    ///     s (str): String value
    ///
    /// Returns:
    ///     Value: String value instance
    #[staticmethod]
    fn string(s: String) -> Self {
        Self {
            value: Value::VString(s),
        }
    }

    /// Create an integer value.
    ///
    /// Args:
    ///     i (int): Integer value
    ///
    /// Returns:
    ///     Value: Integer value instance
    #[staticmethod]
    fn int(i: i64) -> Self {
        Self {
            value: Value::VInteger(i),
        }
    }

    /// Create a decimal value.
    ///
    /// Args:
    ///     f (float): Decimal/float value
    ///
    /// Returns:
    ///     Value: Decimal value instance
    #[staticmethod]
    fn decimal(f: f64) -> Self {
        Self {
            value: Value::VDecimal(f),
        }
    }

    /// Create a boolean value.
    ///
    /// Args:
    ///     b (bool): Boolean value
    ///
    /// Returns:
    ///     Value: Boolean value instance
    #[staticmethod]
    fn boolean(b: bool) -> Self {
        Self {
            value: Value::VBoolean(b),
        }
    }

    /// Create a symbol value.
    ///
    /// Args:
    ///     s (str): Symbol identifier string
    ///
    /// Returns:
    ///     Value: Symbol value instance
    #[staticmethod]
    fn symbol(s: String) -> Self {
        Self {
            value: Value::VSymbol(s),
        }
    }

    /// Create an array value.
    ///
    /// Args:
    ///     items (list): List of Value instances
    ///
    /// Returns:
    ///     Value: Array value instance
    #[staticmethod]
    fn array(py: Python, items: &Bound<'_, PyList>) -> PyResult<Self> {
        let mut vec = Vec::new();
        for item in items.iter() {
            vec.push(python_to_value(py, &item)?);
        }
        Ok(Self {
            value: Value::VArray(vec),
        })
    }

    /// Create a map value.
    ///
    /// Args:
    ///     items (dict): Dictionary mapping string keys to Value instances
    ///
    /// Returns:
    ///     Value: Map value instance
    #[staticmethod]
    fn map(py: Python, items: &Bound<'_, PyDict>) -> PyResult<Self> {
        Ok(Self {
            value: Value::VMap(python_dict_to_value_map(py, items)?),
        })
    }

    /// Create a range value.
    ///
    /// Args:
    ///     lower (float, optional): Lower bound (inclusive)
    ///     upper (float, optional): Upper bound (inclusive)
    ///
    /// Returns:
    ///     Value: Range value instance
    #[staticmethod]
    #[pyo3(signature = (lower=None, upper=None))]
    fn range(lower: Option<f64>, upper: Option<f64>) -> Self {
        Self {
            value: Value::VRange(RangeValue { lower, upper }),
        }
    }

    /// Create a measurement value.
    ///
    /// Args:
    ///     value (float): Numeric value
    ///     unit (str): Unit string (e.g., "kg", "m", "s")
    ///
    /// Returns:
    ///     Value: Measurement value instance
    #[staticmethod]
    fn measurement(value: f64, unit: String) -> Self {
        Self {
            value: Value::VMeasurement { unit, value },
        }
    }

    /// Extract string value
    fn as_string(&self) -> PyResult<String> {
        match &self.value {
            Value::VString(s) => Ok(s.clone()),
            Value::VSymbol(s) => Ok(s.clone()),
            _ => Err(type_error_to_python("Value is not a string or symbol")),
        }
    }

    /// Extract integer value
    fn as_int(&self) -> PyResult<i64> {
        match &self.value {
            Value::VInteger(i) => Ok(*i),
            _ => Err(type_error_to_python("Value is not an integer")),
        }
    }

    /// Extract decimal value
    fn as_decimal(&self) -> PyResult<f64> {
        match &self.value {
            Value::VDecimal(f) => Ok(*f),
            _ => Err(type_error_to_python("Value is not a decimal")),
        }
    }

    /// Extract boolean value
    fn as_boolean(&self) -> PyResult<bool> {
        match &self.value {
            Value::VBoolean(b) => Ok(*b),
            _ => Err(type_error_to_python("Value is not a boolean")),
        }
    }

    /// Extract array value
    fn as_array(&self, py: Python) -> PyResult<PyObject> {
        match &self.value {
            Value::VArray(vec) => {
                let list = PyList::empty_bound(py);
                for item in vec {
                    list.append(value_to_python(py, item)?.into_bound(py))?;
                }
                Ok(list.to_object(py))
            }
            _ => Err(type_error_to_python("Value is not an array")),
        }
    }

    /// Extract map value
    fn as_map(&self, py: Python) -> PyResult<PyObject> {
        match &self.value {
            Value::VMap(map) => value_map_to_python_dict(py, map),
            _ => Err(type_error_to_python("Value is not a map")),
        }
    }

    fn __repr__(&self) -> String {
        format!("Value({:?})", self.value)
    }

    fn __str__(&self) -> String {
        format!("{}", self.value)
    }
}

// ============================================================================
// Subject Python Class
// ============================================================================

/// Python wrapper for Subject.
///
/// Subject is a self-descriptive value type with identity, labels, and properties.
/// Designed to be used as the value type in Pattern<Subject>.
#[cfg(feature = "python")]
#[pyclass(name = "Subject")]
#[derive(Clone)]
pub struct PySubject {
    subject: Subject,
}

#[cfg(feature = "python")]
#[pymethods]
impl PySubject {
    /// Create a Subject with identity, labels, and properties.
    ///
    /// Args:
    ///     identity (str): Symbol identifier that uniquely identifies the subject
    ///     labels (set[str], optional): Set of label strings
    ///     properties (dict[str, Value], optional): Map of property names to Value instances
    ///
    /// Returns:
    ///     Subject: New Subject instance
    #[new]
    #[pyo3(signature = (identity, labels = None, properties = None))]
    fn new(
        py: Python,
        identity: String,
        labels: Option<&Bound<'_, PySet>>,
        properties: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        let labels_set = if let Some(labels) = labels {
            python_set_to_hashset(py, labels)?
        } else {
            HashSet::new()
        };

        let properties_map = if let Some(properties) = properties {
            python_dict_to_value_map(py, properties)?
        } else {
            HashMap::new()
        };

        Ok(Self {
            subject: Subject {
                identity: Symbol(identity),
                labels: labels_set,
                properties: properties_map,
            },
        })
    }

    /// Get the identity
    #[getter]
    fn identity(&self) -> String {
        self.subject.identity.0.clone()
    }

    /// Get the labels
    fn get_labels(&self, py: Python) -> PyResult<PyObject> {
        hashset_to_python_set(py, &self.subject.labels)
    }

    /// Get the properties
    fn get_properties(&self, py: Python) -> PyResult<PyObject> {
        value_map_to_python_dict(py, &self.subject.properties)
    }

    /// Add a label
    fn add_label(&mut self, label: String) {
        self.subject.labels.insert(label);
    }

    /// Remove a label
    fn remove_label(&mut self, label: String) {
        self.subject.labels.remove(&label);
    }

    /// Check if label exists
    fn has_label(&self, label: String) -> bool {
        self.subject.labels.contains(&label)
    }

    /// Get a property value
    fn get_property(&self, _py: Python, name: String) -> PyResult<Option<PyValue>> {
        if let Some(value) = self.subject.properties.get(&name) {
            Ok(Some(PyValue {
                value: value.clone(),
            }))
        } else {
            Ok(None)
        }
    }

    /// Set a property value
    fn set_property(&mut self, py: Python, name: String, value: &Bound<'_, PyAny>) -> PyResult<()> {
        let rust_value = python_to_value(py, value)?;
        self.subject.properties.insert(name, rust_value);
        Ok(())
    }

    /// Remove a property
    fn remove_property(&mut self, name: String) {
        self.subject.properties.remove(&name);
    }

    fn __repr__(&self) -> String {
        format!("Subject(identity={:?})", self.subject.identity.0)
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Helper to recursively collect values from PyPattern
fn collect_pattern_values(py: Python, pattern: &PyPattern, result: &mut Vec<PyObject>) {
    result.push(pattern.value.clone_ref(py));
    for elem in &pattern.elements {
        collect_pattern_values(py, elem, result);
    }
}

/// Helper to recursively filter PyPattern instances
fn filter_pattern_recursive(
    pattern: &PyPattern,
    predicate: &Bound<'_, PyAny>,
    result: &mut Vec<PyPattern>,
) {
    let py_pattern = pattern.clone();
    match predicate.call1((py_pattern.clone(),)) {
        Ok(pred_result) => {
            if pred_result.extract::<bool>().unwrap_or(false) {
                result.push(py_pattern);
            }
        }
        Err(_) => {}
    }
    for elem in &pattern.elements {
        filter_pattern_recursive(elem, predicate, result);
    }
}

/// Helper to recursively find first matching PyPattern
fn find_first_pattern_recursive(
    pattern: &PyPattern,
    predicate: &Bound<'_, PyAny>,
) -> Option<PyPattern> {
    let py_pattern = pattern.clone();
    let matches = match predicate.call1((py_pattern.clone(),)) {
        Ok(pred_result) => pred_result.extract::<bool>().unwrap_or(false),
        Err(_) => false,
    };
    if matches {
        return Some(py_pattern);
    }
    for elem in &pattern.elements {
        if let Some(found) = find_first_pattern_recursive(elem, predicate) {
            return Some(found);
        }
    }
    None
}

/// Helper to recursively compute indices_at
fn indices_at_pattern_recursive(
    py: Python,
    pattern: &PyPattern,
    path: &mut Vec<usize>,
) -> PyPattern {
    // Convert path to Python list
    let path_list = PyList::new(py, path.iter().copied()).unwrap();
    let mut new_elements = Vec::new();
    for (i, elem) in pattern.elements.iter().enumerate() {
        path.push(i);
        new_elements.push(indices_at_pattern_recursive(py, elem, path));
        path.pop();
    }
    PyPattern {
        value: path_list.into(),
        elements: new_elements,
    }
}

/// Helper to convert PyPattern to Rust Pattern<String>
fn to_rust_pattern(pattern: &PyPattern) -> Pattern<String> {
    Python::with_gil(|py| {
        let value_str = pattern.value.bind(py).str().unwrap().to_string();
        Pattern {
            value: value_str,
            elements: pattern
                .elements
                .iter()
                .map(|e| to_rust_pattern(e))
                .collect(),
        }
    })
}

// ============================================================================
// Pattern Python Class
// ============================================================================

/// Python wrapper for Pattern<V> (generic pattern).
///
/// Pattern is a recursive, nested structure (s-expression-like) that can hold any value type.
///
/// Pattern<V> is fully generic - it can hold primitives, complex objects, or even other Patterns,
/// enabling nested structures like Pattern<Pattern<T>>.
#[cfg(feature = "python")]
#[pyclass(name = "Pattern")]
pub struct PyPattern {
    value: Py<PyAny>, // Generic Python object (can be any type including Pattern)
    elements: Vec<PyPattern>,
}

// Manual Clone implementation because Py<PyAny> requires Python reference
#[cfg(feature = "python")]
impl Clone for PyPattern {
    fn clone(&self) -> Self {
        Python::with_gil(|py| Self {
            value: self.value.clone_ref(py),
            elements: self.elements.clone(),
        })
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl PyPattern {
    /// Create an atomic pattern (no elements).
    ///
    /// Accepts any Python value including primitives, objects, and other Patterns.
    /// This enables nesting: Pattern.point(Pattern.point(x)) creates Pattern<Pattern<V>>.
    ///
    /// Args:
    ///     value: The value component (any Python type)
    ///
    /// Returns:
    ///     Pattern: Atomic pattern instance
    #[staticmethod]
    fn point(py: Python, value: &Bound<'_, PyAny>) -> PyResult<Self> {
        // Store the Python object directly - no type restrictions
        Ok(Self {
            value: value.clone().unbind(),
            elements: vec![],
        })
    }

    /// Create a pattern with value decoration and elements.
    ///
    /// The value decorates or describes the pattern represented by the elements.
    /// Accepts any Python value including other Patterns.
    ///
    /// Args:
    ///     value: The value decoration (any Python type)
    ///     elements: List of Pattern instances that form the pattern
    ///
    /// Returns:
    ///     Pattern: Pattern instance with value decoration and elements
    #[staticmethod]
    fn pattern(py: Python, value: &Bound<'_, PyAny>, elements: Vec<PyPattern>) -> PyResult<Self> {
        // Store the Python object directly - no type restrictions
        Ok(Self {
            value: value.clone().unbind(),
            elements,
        })
    }

    /// Alias for point(). Lift a value into a Pattern.
    ///
    /// This follows the functional programming convention where
    /// `of` is used to lift a value into a functor/applicative.
    /// Works uniformly on all values including other Patterns.
    ///
    /// Args:
    ///     value: The value component (any Python type)
    ///
    /// Returns:
    ///     Pattern: Atomic pattern instance
    ///
    /// Example:
    ///     >>> # Lift a primitive
    ///     >>> p1 = Pattern.of(42)
    ///     >>> # Lift a Pattern (nesting)
    ///     >>> p2 = Pattern.of(p1)  # Pattern<Pattern<int>>
    #[staticmethod]
    fn of(py: Python, value: &Bound<'_, PyAny>) -> PyResult<Self> {
        Self::point(py, value)
    }

    /// Convert a list of values into a list of patterns.
    ///
    /// Applies Pattern.of() (which is Pattern.point()) to every value in the list,
    /// uniformly lifting each value into a Pattern. Works on any type including Patterns.
    ///
    /// Args:
    ///     values: List of values to convert (any type)
    ///
    /// Returns:
    ///     List[Pattern]: List of pattern instances
    ///
    /// Example:
    ///     >>> # From primitives
    ///     >>> patterns = Pattern.from_values([1, 2, 3])
    ///     >>> len(patterns)
    ///     3
    ///     >>> # From nested patterns
    ///     >>> p1 = Pattern.point("a")
    ///     >>> patterns = Pattern.from_values([p1])  # Creates Pattern<Pattern<str>>
    ///     >>> len(patterns)
    ///     1
    #[staticmethod]
    fn from_values(py: Python, values: &Bound<'_, PyList>) -> PyResult<Vec<Self>> {
        let mut patterns = Vec::new();
        for item in values.iter() {
            // Apply Pattern.point() uniformly to all values
            patterns.push(PyPattern::point(py, &item)?);
        }
        Ok(patterns)
    }

    /// Get the value (any Python type)
    #[getter]
    fn value(&self, py: Python) -> PyObject {
        self.value.clone_ref(py)
    }

    /// Get the elements
    #[getter]
    fn elements(&self) -> Vec<PyPattern> {
        self.elements.clone()
    }

    /// Check if pattern is atomic
    fn is_atomic(&self) -> bool {
        self.elements.is_empty()
    }

    /// Return the number of direct elements in this pattern.
    ///
    /// Returns:
    ///     int: Number of elements
    fn length(&self) -> usize {
        self.elements.len()
    }

    /// Return the total number of nodes in the pattern.
    ///
    /// Returns:
    ///     int: Total number of nodes (including all nested patterns)
    fn size(&self) -> usize {
        1 + self.elements.iter().map(|e| e.size()).sum::<usize>()
    }

    /// Return the maximum nesting depth.
    ///
    /// Returns:
    ///     int: Maximum nesting depth (0 for atomic patterns)
    fn depth(&self) -> usize {
        if self.elements.is_empty() {
            0
        } else {
            1 + self.elements.iter().map(|e| e.depth()).max().unwrap_or(0)
        }
    }

    /// Extract all values as a flat list (pre-order traversal).
    ///
    /// Returns:
    ///     list[Any]: All values in pre-order (root first, then elements)
    fn values(&self, py: Python) -> Vec<PyObject> {
        let mut result = Vec::new();
        collect_pattern_values(py, self, &mut result);
        result
    }

    /// Check if any value satisfies the predicate.
    fn any_value(&self, py: Python, predicate: &Bound<'_, PyAny>) -> PyResult<bool> {
        let values = self.values(py);
        let result = values.iter().any(|v| {
            let bound_value = v.bind(py);
            match predicate.call1((bound_value,)) {
                Ok(result) => result.extract::<bool>().unwrap_or(false),
                Err(_) => false,
            }
        });
        Ok(result)
    }

    /// Check if all values satisfy the predicate.
    fn all_values(&self, py: Python, predicate: &Bound<'_, PyAny>) -> PyResult<bool> {
        let values = self.values(py);
        let result = values.iter().all(|v| {
            let bound_value = v.bind(py);
            match predicate.call1((bound_value,)) {
                Ok(result) => result.extract::<bool>().unwrap_or(false),
                Err(_) => false,
            }
        });
        Ok(result)
    }

    /// Filter subpatterns that satisfy the predicate.
    fn filter(&self, py: Python, predicate: &Bound<'_, PyAny>) -> PyResult<Vec<PyPattern>> {
        let mut result = Vec::new();
        filter_pattern_recursive(self, predicate, &mut result);
        Ok(result)
    }

    /// Find the first subpattern that satisfies the predicate.
    fn find_first(&self, py: Python, predicate: &Bound<'_, PyAny>) -> PyResult<Option<PyPattern>> {
        Ok(find_first_pattern_recursive(self, predicate))
    }

    /// Check if patterns have identical structure.
    fn matches(&self, py: Python, other: &PyPattern) -> bool {
        // Compare values using Python equality
        let values_equal = Python::with_gil(|py| {
            self.value
                .bind(py)
                .eq(other.value.bind(py))
                .unwrap_or(false)
        });

        if !values_equal || self.elements.len() != other.elements.len() {
            return false;
        }
        self.elements
            .iter()
            .zip(other.elements.iter())
            .all(|(a, b)| a.matches(py, b))
    }

    /// Check if pattern contains other as subpattern.
    fn contains(&self, py: Python, other: &PyPattern) -> bool {
        if self.matches(py, other) {
            return true;
        }
        self.elements.iter().any(|e| e.contains(py, other))
    }

    /// Transform values while preserving structure.
    fn map(&self, py: Python, func: &Bound<'_, PyAny>) -> PyResult<PyPattern> {
        // Apply function to the value (works on any type)
        let bound_value = self.value.bind(py);
        let new_value = match func.call1((bound_value,)) {
            Ok(result) => result.unbind(),
            Err(_) => self.value.clone_ref(py),
        };

        let new_elements: Vec<PyPattern> = self
            .elements
            .iter()
            .map(|e| e.map(py, func))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(PyPattern {
            value: new_value,
            elements: new_elements,
        })
    }

    /// Fold over all values.
    fn fold(
        &self,
        py: Python,
        init: &Bound<'_, PyAny>,
        func: &Bound<'_, PyAny>,
    ) -> PyResult<PyObject> {
        let mut acc = init.to_object(py);
        for value in self.values(py) {
            acc = func.call1((acc.bind(py), value.bind(py)))?.to_object(py);
        }
        Ok(acc)
    }

    /// Paramorphism: structure-aware fold with access to pattern structure.
    ///
    /// At each node, applies func to (current_pattern, element_results) where
    /// element_results is a list of results from recursively processing elements.
    /// Evaluation is bottom-up (elements before parents) and left-to-right.
    ///
    /// Atomic patterns receive an empty list for element_results.
    ///
    /// Args:
    ///     func: Callable taking (Pattern, List[R]) -> R
    ///
    /// Returns:
    ///     Result of applying func across the entire pattern structure
    ///
    /// Example:
    ///     >>> # Depth-weighted sum
    ///     >>> pattern = Pattern.pattern(1, [
    ///     ...     Pattern.point(2),
    ///     ...     Pattern.pattern(3, [Pattern.point(4)])
    ///     ... ])
    ///     >>> result = pattern.para(lambda p, rs: p.value + sum(rs))
    fn para(&self, py: Python, func: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        // Recursively process elements first (bottom-up)
        let element_results = self
            .elements
            .iter()
            .map(|elem| elem.para(py, func))
            .collect::<PyResult<Vec<PyObject>>>()?;

        // Convert element results to Python list
        let py_element_results = PyList::new(py, &element_results)?;

        // Create a PyPattern view of the current node for the callback
        let pattern_view = PyPattern {
            value: self.value.clone_ref(py),
            elements: self.elements.clone(),
        };

        // Call the Python function with (pattern, element_results)
        func.call1((pattern_view, py_element_results))
            .map(|result| result.to_object(py))
    }

    /// Combine two patterns associatively.
    fn combine(&self, py: Python, other: PyPattern) -> PyResult<PyPattern> {
        use crate::Combinable;

        // Try to combine values intelligently
        let combined_value = Python::with_gil(|py| {
            let left_val = self.value.bind(py);
            let right_val = other.value.bind(py);

            // First, check if both values are Subjects
            if let (Ok(left_subj), Ok(right_subj)) = (
                left_val.extract::<PySubject>(),
                right_val.extract::<PySubject>(),
            ) {
                // Use Subject's Combinable implementation (merge strategy)
                let combined_subject = left_subj.subject.combine(right_subj.subject);
                return PySubject {
                    subject: combined_subject,
                }
                .into_py(py);
            }

            // Otherwise, try to add/concatenate the values
            match left_val.call_method1("__add__", (right_val,)) {
                Ok(result) => result.unbind(),
                Err(_) => self.value.clone_ref(py),
            }
        });

        let mut combined_elements = self.elements.clone();
        combined_elements.extend(other.elements);
        Ok(PyPattern {
            value: combined_value,
            elements: combined_elements,
        })
    }

    /// Create patterns by combining three lists pointwise.
    ///
    /// Takes three lists and combines them element-wise to create relationship patterns.
    /// Each resulting pattern has value from the values list and elements [left, right].
    ///
    /// Args:
    ///     left (List[Pattern]): First list of patterns (e.g., source nodes)
    ///     right (List[Pattern]): Second list of patterns (e.g., target nodes)
    ///     values (List[Any]): List of values for the new patterns
    ///
    /// Returns:
    ///     List[Pattern]: List of patterns with structure [value, [left, right]]
    ///
    /// Example:
    ///     >>> sources = [Pattern.point("Alice"), Pattern.point("Bob")]
    ///     >>> targets = [Pattern.point("Company"), Pattern.point("Project")]
    ///     >>> rel_types = ["WORKS_FOR", "MANAGES"]
    ///     >>> relationships = Pattern.zip3(sources, targets, rel_types)
    #[staticmethod]
    fn zip3(
        py: Python,
        left: Vec<PyPattern>,
        right: Vec<PyPattern>,
        values: &Bound<'_, PyList>,
    ) -> PyResult<Vec<PyPattern>> {
        let mut results = Vec::new();

        for ((l, r), val) in left.into_iter().zip(right).zip(values.iter()) {
            results.push(PyPattern {
                value: val.unbind(),
                elements: vec![l, r],
            });
        }

        Ok(results)
    }

    /// Create patterns by applying a function to pairs from two lists.
    ///
    /// Takes two lists of patterns and applies a function to each pair to compute
    /// the value for the resulting pattern.
    ///
    /// Args:
    ///     left (List[Pattern]): First list of patterns (e.g., source nodes)
    ///     right (List[Pattern]): Second list of patterns (e.g., target nodes)
    ///     value_fn (Callable[[Pattern, Pattern], Any]): Function to compute value
    ///
    /// Returns:
    ///     List[Pattern]: List of patterns with computed values
    ///
    /// Example:
    ///     >>> people = [Pattern.point("Alice"), Pattern.point("Bob")]
    ///     >>> companies = [Pattern.point("TechCorp"), Pattern.point("StartupInc")]
    ///     >>> relationships = Pattern.zip_with(people, companies,
    ///     ...     lambda p, c: f"{p.value}_WORKS_AT_{c.value}")
    #[staticmethod]
    fn zip_with(
        py: Python,
        left: Vec<PyPattern>,
        right: Vec<PyPattern>,
        value_fn: &Bound<'_, PyAny>,
    ) -> PyResult<Vec<PyPattern>> {
        let mut results = Vec::new();

        for (l, r) in left.iter().zip(right.iter()) {
            // Call Python function with pattern references
            let value_obj = value_fn.call1((l.clone(), r.clone()))?;

            results.push(PyPattern {
                value: value_obj.unbind(),
                elements: vec![l.clone(), r.clone()],
            });
        }

        Ok(results)
    }

    /// Extract value at current position (comonad).
    fn extract(&self, py: Python) -> PyObject {
        self.value.clone_ref(py)
    }

    /// Apply function to all contexts (comonad).
    fn extend(&self, py: Python, func: &Bound<'_, PyAny>) -> PyResult<PyPattern> {
        // Call function with this pattern to get new value
        let py_pattern = PyPattern {
            value: self.value.clone_ref(py),
            elements: self.elements.clone(),
        };
        let new_value = match func.call1((py_pattern,)) {
            Ok(result) => result.unbind(),
            Err(_) => self.value.clone_ref(py),
        };

        let new_elements: Vec<PyPattern> = self
            .elements
            .iter()
            .map(|e| e.extend(py, func))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(PyPattern {
            value: new_value,
            elements: new_elements,
        })
    }

    /// Decorate each position with depth.
    fn depth_at(&self, py: Python) -> PyResult<PyPattern> {
        let depth = self.depth();
        let new_elements: Vec<PyPattern> = self
            .elements
            .iter()
            .map(|e| e.depth_at(py))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(PyPattern {
            value: depth.to_object(py),
            elements: new_elements,
        })
    }

    /// Decorate each position with subtree size.
    fn size_at(&self, py: Python) -> PyResult<PyPattern> {
        let size = self.size();
        let new_elements: Vec<PyPattern> = self
            .elements
            .iter()
            .map(|e| e.size_at(py))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(PyPattern {
            value: size.to_object(py),
            elements: new_elements,
        })
    }

    /// Decorate each position with path from root.
    fn indices_at(&self, py: Python) -> PyResult<PyPattern> {
        Ok(indices_at_pattern_recursive(py, self, &mut Vec::new()))
    }

    /// Validate pattern structure.
    fn validate(&self, rules: &PyValidationRules) -> PyResult<()> {
        // Convert PyPattern to Rust Pattern<String> for validation
        let rust_pattern = to_rust_pattern(self);
        rust_pattern
            .validate(&rules.rules)
            .map_err(|e| validation_error_to_python(&e))?;
        Ok(())
    }

    /// Analyze pattern structure.
    fn analyze_structure(&self) -> PyStructureAnalysis {
        let rust_pattern = to_rust_pattern(self);
        let analysis = rust_pattern.analyze_structure();
        PyStructureAnalysis { analysis }
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        // Get string representation of the value
        let value_repr = self.value.bind(py).repr()?.to_string();
        Ok(format!(
            "Pattern(value={}, elements={})",
            value_repr,
            self.elements.len()
        ))
    }
}

// ============================================================================
// PatternSubject Python Class
// ============================================================================

// ============================================================================
// ValidationRules Python Class
// ============================================================================

/// Python wrapper for ValidationRules
#[cfg(feature = "python")]
#[pyclass(name = "ValidationRules")]
#[derive(Clone)]
pub struct PyValidationRules {
    rules: ValidationRules,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyValidationRules {
    /// Create validation rules.
    ///
    /// Args:
    ///     max_depth (int, optional): Maximum nesting depth allowed
    ///     max_elements (int, optional): Maximum element count allowed
    ///     max_size (int, optional): Maximum total nodes allowed
    ///
    /// Returns:
    ///     ValidationRules: New validation rules instance
    #[new]
    #[pyo3(signature = (max_depth = None, max_elements = None))]
    fn new(max_depth: Option<usize>, max_elements: Option<usize>) -> Self {
        Self {
            rules: ValidationRules {
                max_depth,
                max_elements,
                required_fields: vec![],
            },
        }
    }

    #[getter]
    fn max_depth(&self) -> Option<usize> {
        self.rules.max_depth
    }

    #[getter]
    fn max_elements(&self) -> Option<usize> {
        self.rules.max_elements
    }
}

// ============================================================================
// ValidationError Python Exception Class
// ============================================================================

/// Python exception for validation errors
#[cfg(feature = "python")]
#[pyclass(name = "ValidationError", extends = PyValueError)]
pub struct PyValidationError {
    message: String,
    rule: String,
    location: Vec<String>,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyValidationError {
    #[getter]
    fn message(&self) -> String {
        self.message.clone()
    }

    #[getter]
    fn rule(&self) -> String {
        self.rule.clone()
    }

    #[getter]
    fn location(&self) -> Vec<String> {
        self.location.clone()
    }
}

// ============================================================================
// StructureAnalysis Python Class
// ============================================================================

/// Python wrapper for StructureAnalysis
#[cfg(feature = "python")]
#[pyclass(name = "StructureAnalysis")]
#[derive(Clone)]
pub struct PyStructureAnalysis {
    analysis: StructureAnalysis,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyStructureAnalysis {
    #[getter]
    fn summary(&self) -> String {
        self.analysis.summary.clone()
    }

    #[getter]
    fn depth_distribution(&self) -> Vec<usize> {
        self.analysis.depth_distribution.clone()
    }

    #[getter]
    fn element_counts(&self) -> Vec<usize> {
        self.analysis.element_counts.clone()
    }

    #[getter]
    fn nesting_patterns(&self) -> Vec<String> {
        self.analysis.nesting_patterns.clone()
    }
}

// ============================================================================
// Python Module Initialization
// ============================================================================

/// Initialize the Python module
#[cfg(feature = "python")]
#[pymodule]
fn pattern_core(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyValue>()?;
    m.add_class::<PySubject>()?;
    m.add_class::<PyPattern>()?;
    m.add_class::<PyValidationRules>()?;
    m.add_class::<PyStructureAnalysis>()?;
    m.add_class::<PyValidationError>()?;
    Ok(())
}
