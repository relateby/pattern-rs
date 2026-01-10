//! Node pattern parser for gram notation

use super::combinators::ws;
use super::subject::subject;
use super::types::ParseResult;
use nom::{
    character::complete::char,
    combinator::{cut, map},
    sequence::delimited,
};
use pattern_core::{Pattern, Subject};

/// Parse a node pattern: (subject)
/// Node patterns have 0 elements
pub fn node(input: &str) -> ParseResult<'_, Pattern<Subject>> {
    map(
        delimited(char('('), delimited(ws, subject, ws), cut(char(')'))),
        Pattern::point,
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_simple() {
        let (remaining, pattern) = node("(hello)").unwrap();
        assert_eq!(pattern.value().identity.0, "hello");
        assert_eq!(pattern.elements().len(), 0);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_node_with_label() {
        let (remaining, pattern) = node("(alice:Person)").unwrap();
        assert_eq!(pattern.value().identity.0, "alice");
        assert!(pattern.value().labels.contains("Person"));
        assert_eq!(pattern.elements().len(), 0);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_node_with_properties() {
        let (remaining, pattern) = node(r#"(alice {name: "Alice"})"#).unwrap();
        assert_eq!(pattern.value().identity.0, "alice");
        assert_eq!(pattern.value().properties.len(), 1);
        assert_eq!(pattern.elements().len(), 0);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_node_full() {
        let (remaining, pattern) = node(r#"(alice:Person {name: "Alice", age: 30})"#).unwrap();
        assert_eq!(pattern.value().identity.0, "alice");
        assert!(pattern.value().labels.contains("Person"));
        assert_eq!(pattern.value().properties.len(), 2);
        assert_eq!(pattern.elements().len(), 0);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_node_empty() {
        let (remaining, pattern) = node("()").unwrap();
        assert_eq!(pattern.value().identity.0, "");
        assert_eq!(pattern.elements().len(), 0);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_node_with_whitespace() {
        let (remaining, pattern) = node("(  hello  )").unwrap();
        assert_eq!(pattern.value().identity.0, "hello");
        assert_eq!(remaining, "");
    }
}
