//! Extension trait providing `from_gram` for StandardGraph.
//!
//! This lives in gram-codec (not pattern-core) because gram-codec depends on
//! pattern-core, but not vice versa.

use pattern_core::graph::StandardGraph;

use crate::ParseError;

/// Extension trait for constructing types from gram notation.
pub trait FromGram: Sized {
    fn from_gram(input: &str) -> Result<Self, ParseError>;
}

impl FromGram for StandardGraph {
    fn from_gram(input: &str) -> Result<Self, ParseError> {
        let patterns = crate::parse_gram(input)?;
        Ok(StandardGraph::from_patterns(patterns))
    }
}
