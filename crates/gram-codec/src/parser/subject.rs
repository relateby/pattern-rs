//! Subject parsers for gram notation identifiers, labels, and records

use super::combinators::ws;
use super::types::ParseResult;
use super::value::{identifier, value_parser};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{cut, map, opt},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded, tuple},
};
use pattern_core::Subject;
use pattern_core::Value;
use std::collections::{HashMap, HashSet};

/// Parse a label: :Label
#[allow(dead_code)]
pub fn label(input: &str) -> ParseResult<'_, String> {
    preceded(char(':'), identifier)(input)
}

/// Parse multiple labels: Label1:Label2:Label3 (without leading :)
#[allow(dead_code)]
fn labels(input: &str) -> ParseResult<'_, Vec<String>> {
    separated_list1(char(':'), identifier)(input)
}

/// Parse a property record: {key: value, key2: value2}
pub fn record(input: &str) -> ParseResult<'_, HashMap<String, Value>> {
    delimited(
        char('{'),
        delimited(
            ws,
            map(
                separated_list0(delimited(ws, char(','), ws), property_pair),
                |pairs: Vec<(String, Value)>| pairs.into_iter().collect::<HashMap<String, Value>>(),
            ),
            ws,
        ),
        cut(char('}')),
    )(input)
}

/// Parse a key-value pair: key: value or key :: value (declare)
fn property_pair(input: &str) -> ParseResult<'_, (String, Value)> {
    map(
        tuple((
            delimited(ws, identifier, ws),
            alt((
                tag("::"), // Declare separator (must come before single :)
                tag(":"),  // Regular separator
            )),
            delimited(ws, value_parser, ws),
        )),
        |(key, _separator, value)| (key, value),
    )(input)
}

/// Parse a subject: identifier:labels {record}
/// All components are optional, but at least one must be present
pub fn subject(input: &str) -> ParseResult<'_, Subject> {
    map(
        tuple((
            opt(identifier),
            opt(preceded(char(':'), separated_list1(char(':'), identifier))),
            opt(preceded(ws, record)),
        )),
        |(id, label_list, props)| {
            let identity = id
                .map(pattern_core::Symbol)
                .unwrap_or_else(|| pattern_core::Symbol(String::new()));

            let labels = label_list
                .map(|list| list.into_iter().collect::<HashSet<_>>())
                .unwrap_or_default();

            let properties = props.unwrap_or_default();

            Subject {
                identity,
                labels,
                properties,
            }
        },
    )(input)
}

// Note: subject_pattern is defined in parser/mod.rs to avoid circular dependencies
// It needs to call gram_pattern recursively for nested patterns

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label() {
        let (remaining, lbl) = label(":Person").unwrap();
        assert_eq!(lbl, "Person");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_labels_multiple() {
        let (remaining, lbls) = labels("Person:User").unwrap();
        assert_eq!(lbls, vec!["Person", "User"]);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_property_pair() {
        let (remaining, (key, val)) = property_pair(r#"name: "Alice""#).unwrap();
        assert_eq!(key, "name");
        match val {
            Value::VString(s) => assert_eq!(s, "Alice"),
            _ => panic!("Expected string value"),
        }
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_record() {
        let (remaining, rec) = record(r#"{name: "Alice", age: 30}"#).unwrap();
        assert_eq!(rec.len(), 2);
        assert!(rec.contains_key("name"));
        assert!(rec.contains_key("age"));
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_record_empty() {
        let (remaining, rec) = record("{}").unwrap();
        assert_eq!(rec.len(), 0);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_subject_identifier_only() {
        let (remaining, subj) = subject("hello").unwrap();
        assert_eq!(subj.identity.0, "hello");
        assert!(subj.labels.is_empty());
        assert!(subj.properties.is_empty());
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_subject_with_label() {
        let (remaining, subj) = subject("alice:Person").unwrap();
        assert_eq!(subj.identity.0, "alice");
        assert!(subj.labels.contains("Person"));
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_subject_with_multiple_labels() {
        let (remaining, subj) = subject("alice:Person:User").unwrap();
        assert_eq!(subj.identity.0, "alice");
        assert!(subj.labels.contains("Person"));
        assert!(subj.labels.contains("User"));
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_subject_with_properties() {
        let (remaining, subj) = subject(r#"alice {name: "Alice", age: 30}"#).unwrap();
        assert_eq!(subj.identity.0, "alice");
        assert_eq!(subj.properties.len(), 2);
        assert!(subj.properties.contains_key("name"));
        assert!(subj.properties.contains_key("age"));
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_subject_full() {
        let (remaining, subj) = subject(r#"alice:Person {name: "Alice"}"#).unwrap();
        assert_eq!(subj.identity.0, "alice");
        assert!(subj.labels.contains("Person"));
        assert_eq!(subj.properties.len(), 1);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_subject_label_and_props_no_id() {
        let (remaining, subj) = subject(r#":Person {name: "Alice"}"#).unwrap();
        assert_eq!(subj.identity.0, "");
        assert!(subj.labels.contains("Person"));
        assert_eq!(subj.properties.len(), 1);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_subject_properties_only() {
        let (remaining, subj) = subject(r#"{name: "Alice"}"#).unwrap();
        assert_eq!(subj.identity.0, "");
        assert!(subj.labels.is_empty());
        assert_eq!(subj.properties.len(), 1);
        assert_eq!(remaining, "");
    }
}
