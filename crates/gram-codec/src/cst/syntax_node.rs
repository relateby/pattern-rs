//! Foundational CST data types shared by parsing and lowering.

use crate::{Pattern, Subject, Value};
use pattern_core::Symbol;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceSpan {
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArrowKind {
    Right,
    Left,
    Bidirectional,
    Undirected,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SyntaxKind {
    Document,
    Node,
    Relationship(ArrowKind),
    Subject,
    Annotated,
    Comment,
}

#[derive(Clone, Debug)]
pub enum Annotation {
    Property {
        key: String,
        value: Value,
    },
    Identified {
        identity: Option<Symbol>,
        labels: Vec<String>,
    },
}

#[derive(Clone, Debug)]
pub struct SyntaxNode {
    pub kind: SyntaxKind,
    pub subject: Option<Subject>,
    pub span: SourceSpan,
    pub annotations: Vec<Annotation>,
    pub text: Option<String>,
}

#[derive(Clone, Debug)]
pub struct CstParseResult {
    pub tree: Pattern<SyntaxNode>,
    pub errors: Vec<SourceSpan>,
}

impl CstParseResult {
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
}
