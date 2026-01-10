//! Annotation parser for gram notation

use super::combinators::ws;
use super::types::ParseResult;
use super::value::identifier;
use nom::{
    character::complete::char,
    combinator::{cut, map, opt},
    sequence::{delimited, pair, preceded},
};

/// Intermediate type for annotations during parsing
#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    pub key: String,
    pub value: Option<String>,
}

/// Parse an annotation: @key or @key(value)
pub fn annotation(input: &str) -> ParseResult<'_, Annotation> {
    map(
        preceded(
            char('@'),
            pair(
                identifier,
                opt(delimited(
                    char('('),
                    delimited(ws, identifier, ws),
                    cut(char(')')),
                )),
            ),
        ),
        |(key, value)| Annotation { key, value },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_annotation_key_only() {
        let (remaining, ann) = annotation("@deprecated").unwrap();
        assert_eq!(ann.key, "deprecated");
        assert_eq!(ann.value, None);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_annotation_with_value() {
        let (remaining, ann) = annotation("@since(v1_0)").unwrap();
        assert_eq!(ann.key, "since");
        assert_eq!(ann.value, Some("v1_0".to_string()));
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_annotation_with_whitespace() {
        let (remaining, ann) = annotation("@key( value )").unwrap();
        assert_eq!(ann.key, "key");
        assert_eq!(ann.value, Some("value".to_string()));
        assert_eq!(remaining, "");
    }
}
