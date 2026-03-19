//! Annotation parser for gram notation

use super::combinators::ws;
use super::types::ParseResult;
use super::value::identifier;
use super::value::value_parser;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{cut, map, opt},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded},
};

/// Intermediate type for annotations during parsing
#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    pub key: String,
    pub value: Option<pattern_core::Value>,
}

/// Identity/label annotation syntax: @@id, @@:label, @@id:label
#[derive(Debug, Clone, PartialEq)]
pub struct IdentifiedAnnotation {
    pub identity: Option<String>,
    pub labels: Vec<String>,
}

/// Parse a property annotation: @key or @key(value)
pub fn property_annotation(input: &str) -> ParseResult<'_, Annotation> {
    map(
        preceded(
            char('@'),
            pair(
                identifier,
                opt(delimited(
                    char('('),
                    delimited(ws, value_parser, ws),
                    cut(char(')')),
                )),
            ),
        ),
        |(key, value)| Annotation {
            key,
            value: Some(value.map_or_else(
                || pattern_core::Value::VBoolean(true),
                normalize_annotation_value,
            )),
        },
    )(input)
}

/// Parse an identified annotation: @@id, @@:label, or @@id:label
pub fn identified_annotation(input: &str) -> ParseResult<'_, IdentifiedAnnotation> {
    map(
        preceded(
            tag("@@"),
            pair(
                opt(identifier),
                opt(many1(preceded(alt((tag("::"), tag(":"))), identifier))),
            ),
        ),
        |(identity, labels)| IdentifiedAnnotation {
            identity,
            labels: labels.unwrap_or_default(),
        },
    )(input)
}

/// Parse the full annotation sequence accepted by the v0.3.4 grammar.
pub fn annotations(
    input: &str,
) -> ParseResult<'_, (Option<IdentifiedAnnotation>, Vec<Annotation>)> {
    alt((
        map(
            pair(
                delimited(ws, identified_annotation, ws),
                many0(delimited(ws, property_annotation, ws)),
            ),
            |(identified, properties)| (Some(identified), properties),
        ),
        map(
            many1(delimited(ws, property_annotation, ws)),
            |properties| (None, properties),
        ),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_annotation_key_only() {
        let (remaining, ann) = property_annotation("@deprecated").unwrap();
        assert_eq!(ann.key, "deprecated");
        assert_eq!(ann.value, Some(pattern_core::Value::VBoolean(true)));
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_annotation_with_value() {
        let (remaining, ann) = property_annotation("@since(v1_0)").unwrap();
        assert_eq!(ann.key, "since");
        assert_eq!(
            ann.value,
            Some(pattern_core::Value::VString("v1_0".to_string()))
        );
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_annotation_with_whitespace() {
        let (remaining, ann) = property_annotation("@key( value )").unwrap();
        assert_eq!(ann.key, "key");
        assert_eq!(
            ann.value,
            Some(pattern_core::Value::VString("value".to_string()))
        );
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_identified_annotation_with_identity_and_label() {
        let (remaining, ann) = identified_annotation("@@p:L").unwrap();
        assert_eq!(ann.identity, Some("p".to_string()));
        assert_eq!(ann.labels, vec!["L".to_string()]);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_identified_annotation_with_double_colon_label() {
        let (remaining, ann) = identified_annotation("@@::Label").unwrap();
        assert_eq!(ann.identity, None);
        assert_eq!(ann.labels, vec!["Label".to_string()]);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_annotations_accept_identified_then_properties() {
        let (remaining, (identified, properties)) =
            annotations("@@p:L @since(v1_0) @deprecated").unwrap();
        assert!(identified.is_some());
        assert_eq!(properties.len(), 2);
        assert_eq!(remaining, "");
    }
}

fn normalize_annotation_value(value: pattern_core::Value) -> pattern_core::Value {
    match value {
        pattern_core::Value::VString(value) => pattern_core::Value::VString(value),
        pattern_core::Value::VSymbol(value) => pattern_core::Value::VString(value),
        pattern_core::Value::VInteger(value) => pattern_core::Value::VInteger(value),
        pattern_core::Value::VDecimal(value) => pattern_core::Value::VDecimal(value),
        pattern_core::Value::VBoolean(value) => pattern_core::Value::VBoolean(value),
        pattern_core::Value::VArray(values) => pattern_core::Value::VArray(
            values
                .into_iter()
                .map(normalize_annotation_value)
                .collect::<Vec<_>>(),
        ),
        pattern_core::Value::VRange(range) => match (range.lower, range.upper) {
            (Some(lower), Some(upper)) if lower.fract() == 0.0 && upper.fract() == 0.0 => {
                pattern_core::Value::VRange(range)
            }
            _ => pattern_core::Value::VString(format!("{range}")),
        },
        pattern_core::Value::VTaggedString { tag, content } => {
            pattern_core::Value::VTaggedString { tag, content }
        }
        pattern_core::Value::VMap(map) => {
            pattern_core::Value::VString(pattern_core::Value::VMap(map).to_string())
        }
        pattern_core::Value::VMeasurement { unit, value } => {
            pattern_core::Value::VString(format!("{value}{unit}"))
        }
    }
}
