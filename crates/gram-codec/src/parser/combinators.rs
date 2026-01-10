//! Parser combinator utilities for gram notation

use super::types::{Location, ParseResult, Span};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::multispace1,
    combinator::{recognize, rest, value},
    multi::many0,
    sequence::{delimited, pair},
    Parser,
};

/// Consume whitespace and comments
pub fn ws(input: &str) -> ParseResult<'_, ()> {
    value((), many0(alt((value((), multispace1), comment))))(input)
}

/// Parse a comment: // text until newline
pub fn comment(input: &str) -> ParseResult<'_, ()> {
    value(
        (),
        pair(
            tag("//"),
            alt((
                recognize(pair(take_until("\n"), tag("\n"))),
                rest, // Comment at end of file
            )),
        ),
    )(input)
}

/// Wrap a parser with optional leading/trailing whitespace
#[allow(dead_code)]
pub fn padded<'a, O, F>(parser: F) -> impl FnMut(&'a str) -> ParseResult<O>
where
    F: Parser<&'a str, O, nom::error::VerboseError<&'a str>>,
{
    delimited(ws, parser, ws)
}

/// Wrap a parser to track its location in the input
#[allow(dead_code)]
pub fn with_span<'a, O, F>(
    original_input: &'a str,
    mut parser: F,
) -> impl FnMut(&'a str) -> ParseResult<'a, (O, Span)> + 'a
where
    F: Parser<&'a str, O, nom::error::VerboseError<&'a str>> + 'a,
{
    move |input: &'a str| {
        let start_offset = original_input.len() - input.len();
        let (remaining, output) = parser.parse(input)?;
        let end_offset = original_input.len() - remaining.len();

        let span = Span {
            start: Location::from_offset(original_input, start_offset),
            end: Location::from_offset(original_input, end_offset),
        };

        Ok((remaining, (output, span)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::character::complete::alpha1;

    #[test]
    fn test_ws_empty() {
        let (remaining, _) = ws("hello").unwrap();
        assert_eq!(remaining, "hello");
    }

    #[test]
    fn test_ws_spaces() {
        let (remaining, _) = ws("   hello").unwrap();
        assert_eq!(remaining, "hello");
    }

    #[test]
    fn test_ws_with_comment() {
        let (remaining, _) = ws("// comment\nhello").unwrap();
        assert_eq!(remaining, "hello");
    }

    #[test]
    fn test_comment_basic() {
        let (remaining, _) = comment("// this is a comment\nrest").unwrap();
        assert_eq!(remaining, "rest");
    }

    #[test]
    fn test_comment_end_of_file() {
        let (remaining, _) = comment("// comment at end").unwrap();
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_padded() {
        let mut parser = padded(tag("hello"));
        let (remaining, output) = parser("  hello  world").unwrap();
        assert_eq!(output, "hello");
        assert_eq!(remaining, "world");
    }

    #[test]
    fn test_with_span() {
        let input = "hello world";
        let mut parser = with_span(input, alpha1);
        let (remaining, (output, span)) = parser(input).unwrap();

        assert_eq!(output, "hello");
        assert_eq!(remaining, " world");
        assert_eq!(span.start.line, 1);
        assert_eq!(span.start.column, 1);
        assert_eq!(span.start.offset, 0);
    }
}
