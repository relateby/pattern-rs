//! pattern-core - Core pattern data structures
//!
//! This crate provides the core pattern data structures for the gram-rs library.
//! It is a faithful port of the gram-hs reference implementation.
//!
//! # Overview
//!
//! The `pattern-core` crate defines two main types:
//!
//! - **[`Pattern<V>`](pattern::Pattern)**: A recursive, nested structure (s-expression-like)
//!   that is generic over value type `V`. This is the foundational data structure for
//!   representing nested, hierarchical data that may be interpreted as graphs.
//!
//! - **[`Subject`](subject::Subject)**: A self-descriptive value type with identity, labels,
//!   and properties. Designed to be used as the value type in `Pattern<Subject>`, which is
//!   a common use case for replacing object-graphs with nested patterns.
//!
//! # Quick Start
//!
//! ```rust
//! use pattern_core::{Pattern, Subject, Symbol, Value};
//! use std::collections::{HashSet, HashMap};
//!
//! // Create a simple pattern
//! let pattern = Pattern {
//!     value: "hello".to_string(),
//!     elements: vec![],
//! };
//!
//! // Create a pattern with Subject value
//! let subject = Subject {
//!     identity: Symbol("n".to_string()),
//!     labels: {
//!         let mut s = HashSet::new();
//!         s.insert("Person".to_string());
//!         s
//!     },
//!     properties: {
//!         let mut m = HashMap::new();
//!         m.insert("name".to_string(), Value::VString("Alice".to_string()));
//!         m
//!     },
//! };
//!
//! let pattern_with_subject: Pattern<Subject> = Pattern {
//!     value: subject,
//!     elements: vec![],
//! };
//! ```
//!
//! # WASM Compatibility
//!
//! All types in this crate are fully compatible with WebAssembly targets. Compile for WASM with:
//!
//! ```bash
//! cargo build --package pattern-core --target wasm32-unknown-unknown
//! ```
//!
//! # Reference Implementation
//!
//! This crate is ported from the gram-hs reference implementation:
//! - Pattern: `../gram-hs/libs/pattern/src/Pattern.hs`
//! - Subject: `../gram-hs/libs/subject/src/Subject/Core.hs`
//! - Feature Spec: `../gram-hs/specs/001-pattern-data-structure/`

pub mod pattern;
pub mod subject;
pub mod test_utils;

pub use pattern::Pattern;
pub use subject::{PropertyRecord, RangeValue, Subject, Symbol, Value};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Placeholder test - will be expanded as functionality is ported
        assert!(true);
    }
}
