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
//! # Pattern Combination
//!
//! Patterns can be combined associatively using the `combine()` method when the value type
//! implements the `Combinable` trait. Combination merges two patterns by combining their values
//! and concatenating their elements.
//!
//! ```rust
//! use pattern_core::{Pattern, Combinable};
//!
//! // Combine atomic patterns (no elements)
//! let p1 = Pattern::point("hello".to_string());
//! let p2 = Pattern::point(" world".to_string());
//! let combined = p1.combine(p2);
//! assert_eq!(combined.value(), "hello world");
//! assert_eq!(combined.length(), 0);
//!
//! // Combine patterns with elements
//! let p3 = Pattern::pattern("a".to_string(), vec![
//!     Pattern::point("b".to_string()),
//!     Pattern::point("c".to_string()),
//! ]);
//! let p4 = Pattern::pattern("d".to_string(), vec![
//!     Pattern::point("e".to_string()),
//! ]);
//! let result = p3.combine(p4);
//! assert_eq!(result.value(), "ad");
//! assert_eq!(result.length(), 3); // [b, c, e]
//!
//! // Associativity: (a ⊕ b) ⊕ c = a ⊕ (b ⊕ c)
//! let a = Pattern::point("a".to_string());
//! let b = Pattern::point("b".to_string());
//! let c = Pattern::point("c".to_string());
//! let left = a.clone().combine(b.clone()).combine(c.clone());
//! let right = a.combine(b.combine(c));
//! assert_eq!(left, right);
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

#[cfg(feature = "python")]
pub mod python;

pub use pattern::{Pattern, StructureAnalysis, ValidationError, ValidationRules};
pub use subject::{PropertyRecord, RangeValue, Subject, Symbol, Value};

// Re-export comonad operations for convenient access
// These are defined in pattern::comonad and pattern::comonad_helpers modules
// All operations are methods on Pattern<V>, so no additional re-exports needed beyond Pattern itself

// ============================================================================
// Combinable Trait
// ============================================================================

/// Types that support associative combination.
///
/// Implementors must ensure that combination is associative:
/// `(a.combine(b)).combine(c)` must equal `a.combine(b.combine(c))` for all values.
///
/// This trait is used to enable pattern combination for `Pattern<V>` where `V: Combinable`.
///
/// # Laws
///
/// **Associativity**: For all values a, b, c of type Self:
/// ```text
/// (a.combine(b)).combine(c) == a.combine(b.combine(c))
/// ```
///
/// # Examples
///
/// ```rust
/// use pattern_core::Combinable;
///
/// let s1 = String::from("hello");
/// let s2 = String::from(" world");
/// let result = s1.combine(s2);
/// assert_eq!(result, "hello world");
/// ```
pub trait Combinable {
    /// Combines two values associatively.
    ///
    /// # Parameters
    ///
    /// * `self` - The first value (consumed)
    /// * `other` - The second value to combine with (consumed)
    ///
    /// # Returns
    ///
    /// A new value representing the combination of `self` and `other`.
    ///
    /// # Laws
    ///
    /// Must be associative: `(a.combine(b)).combine(c) == a.combine(b.combine(c))`
    fn combine(self, other: Self) -> Self;
}

// ============================================================================
// Standard Implementations
// ============================================================================

/// Combines two strings by concatenation.
///
/// String concatenation is associative: `(a + b) + c = a + (b + c)`
///
/// # Examples
///
/// ```rust
/// use pattern_core::Combinable;
///
/// let s1 = String::from("hello");
/// let s2 = String::from(" world");
/// let result = s1.combine(s2);
/// assert_eq!(result, "hello world");
/// ```
impl Combinable for String {
    fn combine(mut self, other: Self) -> Self {
        self.push_str(&other);
        self
    }
}

/// Combines two vectors by concatenation.
///
/// Vector concatenation is associative: `(a ++ b) ++ c = a ++ (b ++ c)`
///
/// # Examples
///
/// ```rust
/// use pattern_core::Combinable;
///
/// let v1 = vec![1, 2, 3];
/// let v2 = vec![4, 5];
/// let result = v1.combine(v2);
/// assert_eq!(result, vec![1, 2, 3, 4, 5]);
/// ```
impl<T> Combinable for Vec<T> {
    fn combine(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}

/// Combines two unit values (trivial).
///
/// Unit combination is trivially associative.
///
/// # Examples
///
/// ```rust
/// use pattern_core::Combinable;
///
/// let u1 = ();
/// let u2 = ();
/// let result = u1.combine(u2);
/// assert_eq!(result, ());
/// ```
impl Combinable for () {
    fn combine(self, _other: Self) -> Self {}
}

// ============================================================================
// Subject Combination Strategies
// ============================================================================

/// Combination strategy for Subject that merges labels and properties.
///
/// This strategy combines two subjects by:
/// - Using the identity from the first subject
/// - Taking the union of labels from both subjects
/// - Merging properties (values from the second subject overwrite the first)
///
/// # Semigroup Laws
///
/// This implementation satisfies associativity:
/// - Identity choice is associative (always picks leftmost)
/// - Label union is associative (set union is associative)
/// - Property merge is associative with right-bias (latter values win)
///
/// # Examples
///
/// ```rust
/// use pattern_core::{Subject, Symbol, Combinable};
/// use std::collections::{HashMap, HashSet};
///
/// let s1 = Subject {
///     identity: Symbol("n1".to_string()),
///     labels: {
///         let mut s = HashSet::new();
///         s.insert("Person".to_string());
///         s
///     },
///     properties: HashMap::new(),
/// };
///
/// let s2 = Subject {
///     identity: Symbol("n2".to_string()),
///     labels: {
///         let mut s = HashSet::new();
///         s.insert("Employee".to_string());
///         s
///     },
///     properties: HashMap::new(),
/// };
///
/// // Merge combines labels and uses first identity
/// let merged = s1.combine(s2);
/// assert_eq!(merged.identity.0, "n1");
/// assert!(merged.labels.contains("Person"));
/// assert!(merged.labels.contains("Employee"));
/// ```
impl Combinable for Subject {
    fn combine(self, other: Self) -> Self {
        // Keep first identity (leftmost in associative chain)
        let identity = self.identity;

        // Union of labels (set union is associative)
        let labels = self.labels.union(&other.labels).cloned().collect();

        // Merge properties (right overwrites left)
        let mut properties = self.properties;
        properties.extend(other.properties);

        Subject {
            identity,
            labels,
            properties,
        }
    }
}

/// Newtype wrapper for "first wins" combination strategy.
///
/// When combining two FirstSubject instances, the first subject is returned
/// and the second is discarded. This is useful for scenarios where you want
/// to keep the initial subject and ignore subsequent ones.
///
/// # Semigroup Laws
///
/// This satisfies associativity trivially: first(first(a, b), c) = first(a, first(b, c)) = a
///
/// # Examples
///
/// ```rust
/// use pattern_core::{Subject, Symbol, Combinable};
/// use std::collections::HashSet;
///
/// let s1 = Subject {
///     identity: Symbol("alice".to_string()),
///     labels: HashSet::new(),
///     properties: Default::default(),
/// };
///
/// let s2 = Subject {
///     identity: Symbol("bob".to_string()),
///     labels: HashSet::new(),
///     properties: Default::default(),
/// };
///
/// // First wins - s2 is discarded
/// let result = s1.combine(s2);
/// assert_eq!(result.identity.0, "alice");
/// ```
#[derive(Clone, PartialEq)]
pub struct FirstSubject(pub Subject);

impl Combinable for FirstSubject {
    fn combine(self, _other: Self) -> Self {
        self // Always return first, discard second
    }
}

/// Newtype wrapper for "last wins" combination strategy.
///
/// When combining two LastSubject instances, the second subject is returned
/// and the first is discarded. This is useful for scenarios where you want
/// the most recent subject to take precedence.
///
/// # Semigroup Laws
///
/// This satisfies associativity trivially: last(last(a, b), c) = last(a, last(b, c)) = c
///
/// # Examples
///
/// ```rust
/// use pattern_core::{Subject, Symbol, Combinable, LastSubject};
/// use std::collections::HashSet;
///
/// let s1 = LastSubject(Subject {
///     identity: Symbol("alice".to_string()),
///     labels: HashSet::new(),
///     properties: Default::default(),
/// });
///
/// let s2 = LastSubject(Subject {
///     identity: Symbol("bob".to_string()),
///     labels: HashSet::new(),
///     properties: Default::default(),
/// });
///
/// // Last wins - s1 is the last argument, so it wins
/// let result = s2.combine(s1);
/// assert_eq!(result.0.identity.0, "alice");
/// ```
#[derive(Clone, PartialEq)]
pub struct LastSubject(pub Subject);

impl Combinable for LastSubject {
    fn combine(self, other: Self) -> Self {
        other // Always return second, discard first
    }
}

/// Newtype wrapper for "empty" combination strategy that creates anonymous subjects.
///
/// When combining two EmptySubject instances, the result is always an anonymous
/// subject with no labels or properties. This serves as the identity element for
/// a Monoid-like structure.
///
/// # Semigroup Laws
///
/// This satisfies associativity trivially: empty(empty(a, b), c) = empty(a, empty(b, c)) = empty
///
/// # Monoid Laws
///
/// When used with Default, this provides monoid identity:
/// - Left identity: empty.combine(s) = empty
/// - Right identity: s.combine(empty) = empty
///
/// # Examples
///
/// ```rust
/// use pattern_core::{Subject, Symbol, Combinable, EmptySubject};
/// use std::collections::HashSet;
///
/// let s1 = EmptySubject(Subject {
///     identity: Symbol("alice".to_string()),
///     labels: {
///         let mut s = HashSet::new();
///         s.insert("Person".to_string());
///         s
///     },
///     properties: Default::default(),
/// });
///
/// let empty = EmptySubject(Subject {
///     identity: Symbol("_".to_string()),
///     labels: HashSet::new(),
///     properties: Default::default(),
/// });
///
/// // Always returns empty (anonymous)
/// let result = s1.combine(empty);
/// assert_eq!(result.0.identity.0, "_");
/// assert!(result.0.labels.is_empty());
/// ```
#[derive(Clone, PartialEq)]
pub struct EmptySubject(pub Subject);

impl Combinable for EmptySubject {
    fn combine(self, _other: Self) -> Self {
        // Always return anonymous empty subject
        EmptySubject(Subject {
            identity: Symbol("_".to_string()),
            labels: Default::default(),
            properties: Default::default(),
        })
    }
}

impl Default for EmptySubject {
    fn default() -> Self {
        EmptySubject(Subject {
            identity: Symbol("_".to_string()),
            labels: Default::default(),
            properties: Default::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn subject_merge_combines_labels_and_properties() {
        let s1 = Subject {
            identity: Symbol("n1".to_string()),
            labels: {
                let mut s = HashSet::new();
                s.insert("Person".to_string());
                s
            },
            properties: {
                let mut m = HashMap::new();
                m.insert("name".to_string(), Value::VString("Alice".to_string()));
                m
            },
        };

        let s2 = Subject {
            identity: Symbol("n2".to_string()),
            labels: {
                let mut s = HashSet::new();
                s.insert("Employee".to_string());
                s
            },
            properties: {
                let mut m = HashMap::new();
                m.insert("role".to_string(), Value::VString("Engineer".to_string()));
                m
            },
        };

        let merged = s1.combine(s2);

        assert_eq!(merged.identity.0, "n1");
        assert_eq!(merged.labels.len(), 2);
        assert!(merged.labels.contains("Person"));
        assert!(merged.labels.contains("Employee"));
        assert_eq!(merged.properties.len(), 2);
    }

    #[test]
    fn subject_merge_is_associative() {
        let s1 = Subject {
            identity: Symbol("a".to_string()),
            labels: {
                let mut s = HashSet::new();
                s.insert("L1".to_string());
                s
            },
            properties: HashMap::new(),
        };

        let s2 = Subject {
            identity: Symbol("b".to_string()),
            labels: {
                let mut s = HashSet::new();
                s.insert("L2".to_string());
                s
            },
            properties: HashMap::new(),
        };

        let s3 = Subject {
            identity: Symbol("c".to_string()),
            labels: {
                let mut s = HashSet::new();
                s.insert("L3".to_string());
                s
            },
            properties: HashMap::new(),
        };

        // (s1 + s2) + s3
        let left = s1.clone().combine(s2.clone()).combine(s3.clone());

        // s1 + (s2 + s3)
        let right = s1.combine(s2.combine(s3));

        assert_eq!(left.identity, right.identity);
        assert_eq!(left.labels, right.labels);
    }

    #[test]
    fn first_subject_keeps_first() {
        let s1 = FirstSubject(Subject {
            identity: Symbol("alice".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        });

        let s2 = FirstSubject(Subject {
            identity: Symbol("bob".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        });

        let result = s1.clone().combine(s2);
        assert_eq!(result.0.identity.0, "alice");
    }

    #[test]
    fn last_subject_keeps_last() {
        let s1 = LastSubject(Subject {
            identity: Symbol("alice".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        });

        let s2 = LastSubject(Subject {
            identity: Symbol("bob".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        });

        let result = s1.combine(s2.clone());
        assert_eq!(result.0.identity.0, "bob");
    }

    #[test]
    fn empty_subject_returns_anonymous() {
        let s1 = EmptySubject(Subject {
            identity: Symbol("alice".to_string()),
            labels: {
                let mut s = HashSet::new();
                s.insert("Person".to_string());
                s
            },
            properties: HashMap::new(),
        });

        let s2 = EmptySubject(Subject {
            identity: Symbol("bob".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        });

        let result = s1.combine(s2);
        assert_eq!(result.0.identity.0, "_");
        assert!(result.0.labels.is_empty());
        assert!(result.0.properties.is_empty());
    }

    #[test]
    fn empty_subject_is_identity() {
        let empty = EmptySubject::default();
        let result = empty.clone().combine(empty);
        assert_eq!(result.0.identity.0, "_");
    }
}
