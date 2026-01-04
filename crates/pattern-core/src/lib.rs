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
//! // Create an atomic pattern (special case)
//! let atomic = Pattern::point("hello".to_string());
//!
//! // Create a pattern with elements (primary constructor)
//! let pattern = Pattern::pattern("parent".to_string(), vec![
//!     Pattern::point("child1".to_string()),
//!     Pattern::point("child2".to_string()),
//! ]);
//!
//! // Access pattern components
//! assert_eq!(atomic.value(), "hello");
//! assert_eq!(pattern.length(), 2);
//! assert_eq!(pattern.depth(), 1);
//!
//! // Transform pattern values (Functor)
//! let upper = pattern.clone().map(|s| s.to_uppercase());
//! assert_eq!(upper.value(), "PARENT");
//!
//! // Validate pattern structure
//! use pattern_core::ValidationRules;
//! let rules = ValidationRules {
//!     max_depth: Some(10),
//!     ..Default::default()
//! };
//! assert!(pattern.validate(&rules).is_ok());
//!
//! // Analyze pattern structure
//! let analysis = pattern.analyze_structure();
//! println!("Structure: {}", analysis.summary);
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
//! let pattern_with_subject: Pattern<Subject> = Pattern::point(subject);
//! ```
//!
//! # Pattern Ordering
//!
//! Patterns implement `Ord` and `PartialOrd` for types that support ordering,
//! enabling sorting, comparison, and use in ordered data structures.
//!
//! ```rust
//! use pattern_core::Pattern;
//! use std::collections::{BTreeSet, BTreeMap};
//!
//! // Compare patterns
//! let p1 = Pattern::point(1);
//! let p2 = Pattern::point(2);
//! assert!(p1 < p2);
//!
//! // Value-first ordering: values compared before elements
//! let p3 = Pattern::pattern(3, vec![Pattern::point(100)]);
//! let p4 = Pattern::pattern(4, vec![Pattern::point(1)]);
//! assert!(p3 < p4); // 3 < 4, elements not compared
//!
//! // Sort patterns
//! let mut patterns = vec![
//!     Pattern::point(5),
//!     Pattern::point(2),
//!     Pattern::point(8),
//! ];
//! patterns.sort();
//! assert_eq!(patterns[0], Pattern::point(2));
//!
//! // Find min/max
//! let min = patterns.iter().min().unwrap();
//! let max = patterns.iter().max().unwrap();
//! assert_eq!(min, &Pattern::point(2));
//! assert_eq!(max, &Pattern::point(8));
//!
//! // Use in BTreeSet (maintains sorted order)
//! let mut set = BTreeSet::new();
//! set.insert(Pattern::point(5));
//! set.insert(Pattern::point(2));
//! set.insert(Pattern::point(8));
//! let sorted: Vec<_> = set.iter().map(|p| p.value).collect();
//! assert_eq!(sorted, vec![2, 5, 8]);
//!
//! // Use as BTreeMap keys
//! let mut map = BTreeMap::new();
//! map.insert(Pattern::point(1), "first");
//! map.insert(Pattern::point(2), "second");
//! assert_eq!(map.get(&Pattern::point(1)), Some(&"first"));
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

pub use pattern::{Pattern, StructureAnalysis, ValidationError, ValidationRules};
pub use subject::{PropertyRecord, RangeValue, Subject, Symbol, Value};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // Placeholder test - will be expanded as functionality is ported
        assert!(true);
    }
}
