//! Serializer for Pattern structures to Gram notation

use crate::{SerializeError, Value};
use pattern_core::{Pattern, Subject};
use std::collections::HashMap;

/// Serialize a Pattern structure to Gram notation
pub fn to_gram_pattern(pattern: &Pattern<Subject>) -> Result<String, SerializeError> {
    let format = select_format(pattern);

    match format {
        GramFormat::Node => serialize_node_pattern(pattern),
        GramFormat::Relationship => serialize_relationship_pattern(pattern),
        GramFormat::SubjectPattern => serialize_subject_pattern(pattern),
        GramFormat::Annotation => serialize_annotation_pattern(pattern),
        GramFormat::BareRecord => serialize_record(&pattern.value.properties),
    }
}

/// Serialize a sequence of patterns to gram notation.
///
/// Writes each pattern in sequence, joined by newlines.
///
/// # Arguments
///
/// * `patterns` - Patterns to serialize
///
/// # Returns
///
/// * `Ok(String)` - Valid Gram notation
pub fn to_gram(patterns: &[Pattern<Subject>]) -> Result<String, SerializeError> {
    patterns
        .iter()
        .map(to_gram_pattern)
        .collect::<Result<Vec<_>, _>>()
        .map(|lines| lines.join("\n"))
}

/// Serializes patterns with a leading header record.
///
/// Emits the header as a top-level record followed by the patterns,
/// joined by newlines.
///
/// # Arguments
///
/// * `header` - Header record to serialize
/// * `patterns` - Patterns to serialize
///
/// # Returns
///
/// * `Ok(String)` - Valid Gram notation with header
pub fn to_gram_with_header(
    header: crate::Record,
    patterns: &[Pattern<Subject>],
) -> Result<String, SerializeError> {
    let header_str = serialize_record(&header)?;
    let patterns_str = to_gram(patterns)?;

    if patterns_str.is_empty() {
        Ok(header_str)
    } else if header_str.is_empty() {
        Ok(patterns_str)
    } else {
        Ok(format!("{}\n{}", header_str, patterns_str))
    }
}

/// Format types for gram notation serialization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GramFormat {
    /// Node pattern: `(subject)` - 0 elements
    Node,
    /// Relationship pattern: `(left)-->(right)` - 2 atomic elements
    Relationship,
    /// Subject pattern: `[subject | elements]` - Other cases
    SubjectPattern,
    /// Annotation pattern: `@key(value) element` - 1 element with anonymous subject
    Annotation,
    /// Bare record: `{}` - 0 elements, no identity, no labels, has properties
    BareRecord,
}

/// Select appropriate gram notation format for a pattern
fn select_format(pattern: &Pattern<Subject>) -> GramFormat {
    let elem_count = pattern.elements.len();

    if elem_count == 0 {
        if pattern.value.identity.0.is_empty()
            && pattern.value.labels.is_empty()
            && !pattern.value.properties.is_empty()
        {
            GramFormat::BareRecord
        } else {
            GramFormat::Node
        }
    } else if elem_count == 1 {
        // Check if this is an annotation (anonymous subject with properties)
        if is_annotation_pattern(pattern) {
            GramFormat::Annotation
        } else {
            GramFormat::SubjectPattern
        }
    } else if elem_count == 2 {
        // Check if both elements are atomic (relationship notation)
        if is_relationship_pattern(pattern) {
            GramFormat::Relationship
        } else {
            GramFormat::SubjectPattern
        }
    } else {
        GramFormat::SubjectPattern
    }
}

/// Check if pattern qualifies for relationship notation
///
/// Relationship notation `(a)-->(b)` or `(a)-[edge]->(b)` is used when:
/// - Exactly 2 elements
/// - Both elements are atomic (0 elements each)
/// - Root is compatible with current parser capabilities
///
/// Subject pattern notation `[root | elements]` is used when:
/// - Root has labels (not yet supported in relationship notation by parser)
/// - Root has identifier without labels/properties (container pattern)
///
/// Examples:
/// - `(a)-->(b)` - anonymous root → relationship
/// - `(a)-[r {prop: val}]->(b)` - root with properties → relationship (if supported by parser)
/// - `[team | (a), (b)]` - root "team" without labels/props → subject pattern
/// - `[team:Group | (a), (b)]` - root with labels → subject pattern (parser doesn't support `-[:Label]->` yet)
fn is_relationship_pattern(pattern: &Pattern<Subject>) -> bool {
    // Must have exactly 2 atomic elements
    if pattern.elements.len() != 2
        || !pattern.elements[0].elements.is_empty()
        || !pattern.elements[1].elements.is_empty()
    {
        return false;
    }

    // The parser NOW supports:
    // - `(a)-->(b)` (anonymous)
    // - `(a)-[id]->(b)` (identifier only)
    // - `(a)-[:Label]->(b)` (labels only)
    // - `(a)-[{prop: val}]->(b)` (properties only)
    // - `(a)-[id:Label {prop: val}]->(b)` (all combined)
    //
    // So we can use relationship notation for all relationships!
    true
}

/// Check if pattern is an annotation
///
/// True if:
/// - Exactly 1 element
/// - Subject carries any annotation metadata
fn is_annotation_pattern(pattern: &Pattern<Subject>) -> bool {
    pattern.elements.len() == 1
        && (!pattern.value.identity.0.is_empty()
            || !pattern.value.labels.is_empty()
            || !pattern.value.properties.is_empty())
}

/// Serialize as node pattern: `(subject)`
fn serialize_node_pattern(pattern: &Pattern<Subject>) -> Result<String, SerializeError> {
    let subject_str = serialize_subject(&pattern.value)?;
    Ok(format!("({})", subject_str))
}

/// Serialize as relationship pattern: `(left)-->(right)`
fn serialize_relationship_pattern(pattern: &Pattern<Subject>) -> Result<String, SerializeError> {
    if pattern.elements.len() != 2 {
        return Err(SerializeError::invalid_structure(
            "Relationship pattern requires exactly 2 elements",
        ));
    }

    let left = serialize_node_pattern(&pattern.elements[0])?;
    let right = serialize_node_pattern(&pattern.elements[1])?;

    // Serialize the edge (relationship) subject if present
    let edge = if pattern.value.identity.0.is_empty()
        && pattern.value.labels.is_empty()
        && pattern.value.properties.is_empty()
    {
        // Empty edge: (a)-->(b)
        String::new()
    } else {
        // Edge with labels/properties: (a)-[:KNOWS {since: 2020}]->(b)
        let edge_str = serialize_subject(&pattern.value)?;
        format!("[{}]", edge_str)
    };

    Ok(format!("{}-{}->{}", left, edge, right))
}

/// Serialize as subject pattern: `[subject | elements]`
fn serialize_subject_pattern(pattern: &Pattern<Subject>) -> Result<String, SerializeError> {
    let subject_str = serialize_subject(&pattern.value)?;

    let elements_str = pattern
        .elements
        .iter()
        .map(to_gram_pattern)
        .collect::<Result<Vec<_>, _>>()?
        .join(", ");

    Ok(format!("[{} | {}]", subject_str, elements_str))
}

/// Serialize as annotation pattern: `@@id:Label @key(value) element`
fn serialize_annotation_pattern(pattern: &Pattern<Subject>) -> Result<String, SerializeError> {
    if pattern.elements.len() != 1 {
        return Err(SerializeError::invalid_structure(
            "Annotation pattern requires exactly 1 element",
        ));
    }

    let mut annotations = Vec::new();

    if !pattern.value.identity.0.is_empty() || !pattern.value.labels.is_empty() {
        let mut identified = String::from("@@");

        if !pattern.value.identity.0.is_empty() {
            identified.push_str(&quote_identifier(&pattern.value.identity.0));
        }

        if !pattern.value.labels.is_empty() {
            let mut labels: Vec<_> = pattern.value.labels.iter().collect();
            labels.sort();
            for label in labels {
                identified.push(':');
                identified.push_str(&quote_identifier(label));
            }
        }

        annotations.push(identified);
    }

    let mut property_annotations: Vec<String> = pattern
        .value
        .properties
        .iter()
        .map(|(key, value)| {
            let gram_value = value_from_pattern_value(value)?;
            let value_str = gram_value.to_gram_notation();
            Ok(format!("@{}({})", quote_identifier(key), value_str))
        })
        .collect::<Result<Vec<_>, SerializeError>>()?;

    property_annotations.sort();
    annotations.extend(property_annotations);

    let element_str = to_gram_pattern(&pattern.elements[0])?;

    Ok(format!("{} {}", annotations.join(" "), element_str))
}

/// Serialize a Subject (identifier + labels + properties)
fn serialize_subject(subject: &Subject) -> Result<String, SerializeError> {
    let mut parts = Vec::new();

    // Build identifier with labels (no spaces between them)
    let mut id_with_labels = String::new();

    // Serialize identifier
    if !subject.identity.0.is_empty() {
        id_with_labels.push_str(&quote_identifier(&subject.identity.0));
    }

    // Serialize labels (concatenate directly without spaces)
    if !subject.labels.is_empty() {
        let mut labels: Vec<_> = subject.labels.iter().collect();
        labels.sort(); // Consistent ordering
        for label in labels {
            id_with_labels.push(':');
            id_with_labels.push_str(&quote_identifier(label));
        }
    }

    // Add identifier+labels as a single part
    if !id_with_labels.is_empty() {
        parts.push(id_with_labels);
    }

    // Serialize properties (this goes as a separate part, with space before it)
    if !subject.properties.is_empty() {
        let record_str = serialize_record(&subject.properties)?;
        parts.push(record_str);
    }

    Ok(parts.join(" "))
}

/// Serialize property record: `{key1: value1, key2: value2}`
fn serialize_record(
    properties: &HashMap<String, pattern_core::Value>,
) -> Result<String, SerializeError> {
    if properties.is_empty() {
        return Ok(String::new());
    }

    let mut props: Vec<_> = properties.iter().collect();
    props.sort_by_key(|(k, _)| *k); // Consistent ordering

    let prop_strs: Vec<String> = props
        .iter()
        .map(|(key, value)| {
            // Convert pattern_core::Value to gram_codec::Value
            let gram_value = value_from_pattern_value(value)?;
            let value_str = gram_value.to_gram_notation();
            Ok(format!("{}: {}", quote_identifier(key), value_str))
        })
        .collect::<Result<Vec<_>, SerializeError>>()?;

    Ok(format!("{{{}}}", prop_strs.join(", ")))
}

/// Convert pattern_core::Value to gram_codec::Value
fn value_from_pattern_value(value: &pattern_core::Value) -> Result<Value, SerializeError> {
    match value {
        pattern_core::Value::VString(s) => Ok(Value::String(s.clone())),
        pattern_core::Value::VSymbol(s) => Ok(Value::String(s.clone())),
        pattern_core::Value::VInteger(i) => Ok(Value::Integer(*i)),
        pattern_core::Value::VDecimal(d) => Ok(Value::Decimal(*d)),
        pattern_core::Value::VBoolean(b) => Ok(Value::Boolean(*b)),
        pattern_core::Value::VArray(arr) => {
            let values = arr
                .iter()
                .map(value_from_pattern_value)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Value::Array(values))
        }
        pattern_core::Value::VRange(range) => {
            // Convert Option<f64> to i64 bounds
            // For now, only support bounded integer ranges
            let lower = range.lower.ok_or_else(|| {
                SerializeError::invalid_structure("Unbounded lower range not supported")
            })? as i64;
            let upper = range.upper.ok_or_else(|| {
                SerializeError::invalid_structure("Unbounded upper range not supported")
            })? as i64;
            Ok(Value::Range { lower, upper })
        }
        pattern_core::Value::VTaggedString { tag, content } => Ok(Value::TaggedString {
            tag: tag.clone(),
            content: content.clone(),
        }),
        pattern_core::Value::VMap(_map) => {
            // Maps are not supported in gram notation property values
            // They would need to be serialized as nested patterns
            Err(SerializeError::invalid_structure(
                "Map values not supported in gram notation properties",
            ))
        }
        pattern_core::Value::VMeasurement { .. } => {
            // Measurements are not supported in basic gram notation
            Err(SerializeError::invalid_structure(
                "Measurement values not supported in gram notation",
            ))
        }
    }
}

/// Quote identifier if needed (contains spaces, special chars, or starts with digit)
/// Uses backtick quoting per grammar: identifiers, labels, and keys use quoted_name (`)
fn quote_identifier(s: &str) -> String {
    if needs_quoting(s) {
        format!("`{}`", escape_backtick_string(s))
    } else {
        s.to_string()
    }
}

/// Determine if identifier needs backtick quoting
/// Valid unquoted forms per grammar:
///   symbol:  /[a-zA-Z_][0-9a-zA-Z_.\-@]*/
///   integer: /-?(0|[1-9]\d*)/
fn needs_quoting(s: &str) -> bool {
    if s.is_empty() {
        return true;
    }

    let first = s.chars().next().unwrap();

    if first.is_ascii_alphabetic() || first == '_' {
        // Symbol: first=[a-zA-Z_], rest=[0-9a-zA-Z_.-@]*
        return s[first.len_utf8()..]
            .chars()
            .any(|c| !c.is_ascii_alphanumeric() && !matches!(c, '_' | '.' | '-' | '@'));
    }

    if first.is_ascii_digit() || first == '-' {
        // Integer: optional leading '-' followed by pure digits
        let digits_part = if first == '-' { &s[1..] } else { s };
        return digits_part.is_empty() || !digits_part.chars().all(|c| c.is_ascii_digit());
    }

    // Anything else (unicode, @, special char at start) needs quoting
    true
}

/// Escape special characters in backtick-quoted identifiers
fn escape_backtick_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('`', "\\`")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
