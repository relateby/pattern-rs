//! Subject type definition
//!
//! This module provides the Subject type and related types (Symbol, Value, RangeValue, PropertyRecord)
//! for use as pattern values in `Pattern<Subject>`.

use std::fmt;

/// Symbol identifier that uniquely identifies the subject.
///
/// A `Symbol` is a wrapper around a `String` that represents an identifier.
/// In gram notation, symbols appear before labels and properties.
///
/// # Examples
///
/// ```rust
/// use pattern_core::Symbol;
///
/// let symbol = Symbol("n".to_string());
/// assert_eq!(symbol.0, "n");
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct Symbol(pub String);

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Symbol").field(&self.0).finish()
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Range value for numeric ranges (lower and upper bounds, both optional).
///
/// Used in `Value::VRange` to represent numeric ranges with optional bounds.
///
/// Note: This type only implements `PartialEq`, not `Eq`, because `f64` doesn't implement `Eq`
/// (due to NaN != NaN).
///
/// # Examples
///
/// ```rust
/// use pattern_core::RangeValue;
///
/// // Closed range: 1.0 to 10.0
/// let range1 = RangeValue {
///     lower: Some(1.0),
///     upper: Some(10.0),
/// };
///
/// // Lower bound only: 1.0 to infinity
/// let range2 = RangeValue {
///     lower: Some(1.0),
///     upper: None,
/// };
///
/// // Upper bound only: negative infinity to 10.0
/// let range3 = RangeValue {
///     lower: None,
///     upper: Some(10.0),
/// };
/// ```
#[derive(Clone, PartialEq)]
pub struct RangeValue {
    /// Lower bound of the range (inclusive), `None` means unbounded below
    pub lower: Option<f64>,
    /// Upper bound of the range (inclusive), `None` means unbounded above
    pub upper: Option<f64>,
}

impl fmt::Debug for RangeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RangeValue")
            .field("lower", &self.lower)
            .field("upper", &self.upper)
            .finish()
    }
}

impl fmt::Display for RangeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.lower, self.upper) {
            (Some(lower), Some(upper)) => write!(f, "{}..{}", lower, upper),
            (Some(lower), None) => write!(f, "{}..", lower),
            (None, Some(upper)) => write!(f, "..{}", upper),
            (None, None) => write!(f, ".."),
        }
    }
}

/// Property value types for Subject properties.
///
/// `Value` is an enum that represents rich value types that can be stored in Subject properties.
/// It supports standard types (integers, decimals, booleans, strings, symbols) and extended types
/// (tagged strings, arrays, maps, ranges, measurements).
///
/// Note: This type only implements `PartialEq`, not `Eq`, because it contains `RangeValue`
/// which uses `f64` (`f64` doesn't implement `Eq` due to NaN != NaN).
///
/// # Examples
///
/// ```rust
/// use pattern_core::Value;
///
/// // Standard types
/// let int_val = Value::VInteger(42);
/// let decimal_val = Value::VDecimal(3.14);
/// let bool_val = Value::VBoolean(true);
/// let string_val = Value::VString("hello".to_string());
/// let symbol_val = Value::VSymbol("sym".to_string());
///
/// // Extended types
/// let tagged = Value::VTaggedString {
///     tag: "type".to_string(),
///     content: "value".to_string(),
/// };
/// let array = Value::VArray(vec![
///     Value::VInteger(1),
///     Value::VInteger(2),
/// ]);
/// ```
#[derive(Clone, PartialEq)]
pub enum Value {
    /// Integer value (i64)
    VInteger(i64),
    /// Decimal value (f64)
    VDecimal(f64),
    /// Boolean value
    VBoolean(bool),
    /// String value
    VString(String),
    /// Symbol value (string identifier)
    VSymbol(String),
    /// Tagged string with a type tag and content
    VTaggedString {
        /// The type tag
        tag: String,
        /// The string content
        content: String,
    },
    /// Array of values
    VArray(Vec<Value>),
    /// Map from string keys to values
    VMap(std::collections::HashMap<String, Value>),
    /// Numeric range value
    VRange(RangeValue),
    /// Measurement with unit and numeric value (e.g., "5kg" -> unit="kg", value=5.0)
    VMeasurement {
        /// The unit string (e.g., "kg", "m", "s")
        unit: String,
        /// The numeric value
        value: f64,
    },
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::VInteger(i) => f.debug_tuple("VInteger").field(i).finish(),
            Value::VDecimal(d) => f.debug_tuple("VDecimal").field(d).finish(),
            Value::VBoolean(b) => f.debug_tuple("VBoolean").field(b).finish(),
            Value::VString(s) => f.debug_tuple("VString").field(s).finish(),
            Value::VSymbol(s) => f.debug_tuple("VSymbol").field(s).finish(),
            Value::VTaggedString { tag, content } => f
                .debug_struct("VTaggedString")
                .field("tag", tag)
                .field("content", content)
                .finish(),
            Value::VArray(arr) => f.debug_list().entries(arr).finish(),
            Value::VMap(map) => f.debug_map().entries(map.iter()).finish(),
            Value::VRange(r) => f.debug_tuple("VRange").field(r).finish(),
            Value::VMeasurement { unit, value } => f
                .debug_struct("VMeasurement")
                .field("unit", unit)
                .field("value", value)
                .finish(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::VInteger(i) => write!(f, "{}", i),
            Value::VDecimal(d) => write!(f, "{}", d),
            Value::VBoolean(b) => write!(f, "{}", b),
            Value::VString(s) => write!(f, "\"{}\"", s),
            Value::VSymbol(s) => write!(f, "{}", s),
            Value::VTaggedString { tag, content } => write!(f, "{}:{}", tag, content),
            Value::VArray(arr) => {
                write!(f, "[")?;
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Value::VMap(map) => {
                write!(f, "{{")?;
                let mut first = true;
                for (k, v) in map.iter() {
                    if !first {
                        write!(f, ", ")?;
                    }
                    first = false;
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
            Value::VRange(r) => write!(f, "{}", r),
            Value::VMeasurement { unit, value } => write!(f, "{}{}", value, unit),
        }
    }
}

/// Property record type alias.
///
/// A `PropertyRecord` is a map from string keys to `Value` types, storing
/// structured data about a Subject.
///
/// # Examples
///
/// ```rust
/// use pattern_core::{PropertyRecord, Value};
/// use std::collections::HashMap;
///
/// let mut props: PropertyRecord = HashMap::new();
/// props.insert("name".to_string(), Value::VString("Alice".to_string()));
/// props.insert("age".to_string(), Value::VInteger(30));
/// ```
pub type PropertyRecord = std::collections::HashMap<String, Value>;

/// Self-descriptive object with identity, labels, and properties.
///
/// `Subject` is designed to be the primary content type for patterns
/// (i.e., `Pattern<Subject>` will be the common use case).
///
/// A Subject contains:
/// - **Identity**: A required symbol identifier that uniquely identifies the subject
/// - **Labels**: A set of label strings that categorize or classify the subject
/// - **Properties**: A key-value map storing properties with rich value types
///
/// Note: This type only implements `PartialEq`, not `Eq`, because it contains `Value`
/// which uses `f64` (`f64` doesn't implement `Eq` due to NaN != NaN).
///
/// # Examples
///
/// ```rust
/// use pattern_core::{Subject, Symbol, Value};
/// use std::collections::{HashSet, HashMap};
///
/// let subject = Subject {
///     identity: Symbol("n".to_string()),
///     labels: {
///         let mut s = HashSet::new();
///         s.insert("Person".to_string());
///         s
///     },
///     properties: {
///         let mut m = HashMap::new();
///         m.insert("name".to_string(), Value::VString("Alice".to_string()));
///         m.insert("age".to_string(), Value::VInteger(30));
///         m
///     },
/// };
/// ```
///
/// # Usage with Pattern
///
/// ```rust
/// use pattern_core::{Pattern, Subject, Symbol};
/// use std::collections::HashSet;
///
/// let subject = Subject {
///     identity: Symbol("n".to_string()),
///     labels: HashSet::new(),
///     properties: std::collections::HashMap::new(),
/// };
///
/// let pattern: Pattern<Subject> = Pattern {
///     value: subject,
///     elements: vec![],
/// };
/// ```
#[derive(Clone, PartialEq)]
pub struct Subject {
    /// Symbol identifier that uniquely identifies the subject.
    ///
    /// The identity is always required. In gram notation, identities appear
    /// before labels and properties.
    pub identity: Symbol,

    /// Set of label strings that categorize or classify the subject.
    ///
    /// Labels provide classification information. The set can be empty (no labels)
    /// or contain one or more unique labels. In gram notation, labels are prefixed
    /// with `:` or `::` and appear after the identity and before properties.
    pub labels: std::collections::HashSet<String>,

    /// Key-value property map storing structured data about the subject.
    ///
    /// Properties store attributes and metadata. The property record can be empty
    /// (no properties) or contain any number of key-value pairs. In gram notation,
    /// properties appear in curly braces: `{name:"Alice", age:30}`.
    pub properties: PropertyRecord,
}

impl fmt::Debug for Subject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Subject")
            .field("identity", &self.identity)
            .field("labels", &self.labels)
            .field("properties", &self.properties)
            .finish()
    }
}

impl fmt::Display for Subject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.identity)?;

        if !self.labels.is_empty() {
            write!(f, ":")?;
            let mut labels_vec: Vec<_> = self.labels.iter().collect();
            labels_vec.sort();
            for (i, label) in labels_vec.iter().enumerate() {
                if i > 0 {
                    write!(f, "::")?;
                }
                write!(f, "{}", label)?;
            }
        }

        if !self.properties.is_empty() {
            write!(f, " {{")?;
            let mut props_vec: Vec<_> = self.properties.iter().collect();
            props_vec.sort_by_key(|(k, _)| *k);
            for (i, (key, value)) in props_vec.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}: {}", key, value)?;
            }
            write!(f, "}}")?;
        }

        Ok(())
    }
}
