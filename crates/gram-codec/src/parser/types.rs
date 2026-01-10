//! Core types for the nom-based parser

use nom::error::VerboseError;
use nom::IResult;

/// Represents a location in the input text for error reporting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
    /// Byte offset from start (0-indexed)
    pub offset: usize,
}

impl Location {
    /// Create a new location
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self {
            line,
            column,
            offset,
        }
    }

    /// Create a location from a byte offset in the input
    pub fn from_offset(input: &str, offset: usize) -> Self {
        let offset = offset.min(input.len());
        let prefix = &input[..offset];

        let line = prefix.matches('\n').count() + 1;
        let column = prefix
            .rfind('\n')
            .map(|pos| offset - pos)
            .unwrap_or(offset + 1);

        Self {
            line,
            column,
            offset,
        }
    }

    /// Create a location at the start of input (line 1, column 1, offset 0)
    pub fn start() -> Self {
        Self {
            line: 1,
            column: 1,
            offset: 0,
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// Represents a span of text in the input
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Span {
    pub start: Location,
    pub end: Location,
}

impl Span {
    /// Create a new span
    #[allow(dead_code)]
    pub fn new(start: Location, end: Location) -> Self {
        Self { start, end }
    }

    /// Create a span for a single location (zero-width span)
    #[allow(dead_code)]
    pub fn single(location: Location) -> Self {
        Self {
            start: location,
            end: location,
        }
    }
}

/// Relationship arrow types from gram notation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrowType {
    // Single-line arrows
    /// Right arrow: -->
    Right,
    /// Left arrow: <--
    Left,
    /// Bidirectional arrow: <-->
    Bidirectional,
    /// Undirected arrow: --
    Undirected,

    // Double-line arrows
    /// Double undirected arrow: ==
    DoubleUndirected,
    /// Double right arrow: ==>
    DoubleRight,
    /// Double left arrow: <==
    DoubleLeft,
    /// Double bidirectional arrow: <==>
    DoubleBidirectional,

    // Squiggle arrows
    /// Squiggle undirected: ~~
    Squiggle,
    /// Squiggle right: ~~>
    SquiggleRight,
    /// Squiggle left: <~~
    SquiggleLeft,
    /// Squiggle bidirectional: <~~>
    SquiggleBidirectional,
}

impl ArrowType {
    /// Returns true if arrow implies left-to-right directionality
    #[allow(dead_code)]
    pub fn is_forward(&self) -> bool {
        matches!(
            self,
            ArrowType::Right | ArrowType::DoubleRight | ArrowType::SquiggleRight
        )
    }

    /// Returns true if arrow implies right-to-left directionality
    pub fn is_backward(&self) -> bool {
        matches!(
            self,
            ArrowType::Left | ArrowType::DoubleLeft | ArrowType::SquiggleLeft
        )
    }

    /// Returns true if arrow is bidirectional
    #[allow(dead_code)]
    pub fn is_bidirectional(&self) -> bool {
        matches!(
            self,
            ArrowType::Bidirectional
                | ArrowType::DoubleBidirectional
                | ArrowType::SquiggleBidirectional
        )
    }

    /// Returns true if arrow is undirected
    #[allow(dead_code)]
    pub fn is_undirected(&self) -> bool {
        matches!(
            self,
            ArrowType::Undirected | ArrowType::DoubleUndirected | ArrowType::Squiggle
        )
    }
}

impl std::fmt::Display for ArrowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ArrowType::Right => "-->",
            ArrowType::Left => "<--",
            ArrowType::Bidirectional => "<-->",
            ArrowType::Undirected => "--",
            ArrowType::DoubleUndirected => "==",
            ArrowType::DoubleRight => "==>",
            ArrowType::DoubleLeft => "<==",
            ArrowType::DoubleBidirectional => "<==>",
            ArrowType::Squiggle => "~~",
            ArrowType::SquiggleRight => "~~>",
            ArrowType::SquiggleLeft => "<~~",
            ArrowType::SquiggleBidirectional => "<~~>",
        };
        write!(f, "{}", s)
    }
}

/// Type alias for nom parser results with verbose errors
pub type ParseResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_from_offset() {
        let input = "line1\nline2\nline3";

        // Start of input
        let loc = Location::from_offset(input, 0);
        assert_eq!(loc.line, 1);
        assert_eq!(loc.column, 1);

        // After first newline
        let loc = Location::from_offset(input, 6);
        assert_eq!(loc.line, 2);
        assert_eq!(loc.column, 1);

        // Middle of second line
        let loc = Location::from_offset(input, 8);
        assert_eq!(loc.line, 2);
        assert_eq!(loc.column, 3);
    }

    #[test]
    fn test_arrow_type_predicates() {
        assert!(ArrowType::Right.is_forward());
        assert!(ArrowType::Left.is_backward());
        assert!(ArrowType::Bidirectional.is_bidirectional());
        assert!(ArrowType::Squiggle.is_undirected());
        assert!(ArrowType::SquiggleRight.is_forward());
    }
}
