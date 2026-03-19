//! CST parser API and foundational syntax-node types.

mod lowering;
mod parser;
mod syntax_node;

pub use lowering::lower;
pub use parser::parse_gram_cst;
pub use syntax_node::{Annotation, ArrowKind, CstParseResult, SourceSpan, SyntaxKind, SyntaxNode};
