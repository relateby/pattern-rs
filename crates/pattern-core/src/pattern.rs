//! Pattern type definition
//!
//! This module provides the core `Pattern<V>` type, a recursive, nested structure
//! (s-expression-like) that is generic over value type `V`.

use std::fmt;

/// A recursive, nested structure (s-expression-like) that is generic over value type `V`.
///
/// The value provides "information about the elements" - they form an intimate pairing.
/// Elements are themselves patterns, creating the recursive structure.
///
/// Patterns are s-expression-like structures, not trees, though they may appear tree-like
/// and accept tree-like operations.
///
/// # Examples
///
/// ## Creating a simple pattern
///
/// ```rust
/// use pattern_core::Pattern;
///
/// let pattern = Pattern {
///     value: "hello".to_string(),
///     elements: vec![],
/// };
/// ```
///
/// ## Creating a nested pattern
///
/// ```rust
/// use pattern_core::Pattern;
///
/// let nested = Pattern {
///     value: "parent".to_string(),
///     elements: vec![
///         Pattern {
///             value: "child1".to_string(),
///             elements: vec![],
///         },
///         Pattern {
///             value: "child2".to_string(),
///             elements: vec![],
///         },
///     ],
/// };
/// ```
///
/// ## Using with Subject type
///
/// ```rust
/// use pattern_core::{Pattern, Subject, Symbol};
/// use std::collections::HashSet;
///
/// let subject = Subject {
///     identity: Symbol("n".to_string()),
///     labels: HashSet::new(),
///     properties: std::collections::HashMap::new(),
/// };
///
/// let pattern: Pattern<Subject> = Pattern {
///     value: subject,
///     elements: vec![],
/// };
/// ```
///
/// # Trait Implementations
///
/// - `Clone`: Patterns can be cloned when `V: Clone`
/// - `PartialEq`, `Eq`: Patterns can be compared for equality when `V: PartialEq` (or `Eq`)
/// - `Debug`: Structured representation for debugging (with truncation for deep nesting)
/// - `Display`: Human-readable representation
///
/// # Performance
///
/// Patterns support:
/// - At least 100 nesting levels without stack overflow
/// - At least 10,000 elements efficiently
/// - WASM compilation for web applications
#[derive(Clone, PartialEq, Eq)]
pub struct Pattern<V> {
    /// The value component, which provides information about the elements.
    ///
    /// The value and elements form an intimate pairing where the value provides
    /// "information about the elements".
    pub value: V,

    /// The nested collection of patterns that form the recursive structure.
    ///
    /// Elements are themselves `Pattern<V>`, creating the recursive nested structure.
    /// An empty vector represents an atomic pattern (a pattern with no nested elements).
    pub elements: Vec<Pattern<V>>,
}

impl<V: fmt::Debug> Pattern<V> {
    fn fmt_debug_with_depth(
        &self,
        f: &mut fmt::Formatter<'_>,
        depth: usize,
        max_depth: usize,
    ) -> fmt::Result {
        if depth > max_depth {
            return write!(f, "...");
        }

        f.debug_struct("Pattern")
            .field("value", &self.value)
            .field(
                "elements",
                &DebugElements {
                    elements: &self.elements,
                    depth,
                    max_depth,
                },
            )
            .finish()
    }
}

impl<V: fmt::Debug> fmt::Debug for Pattern<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_debug_with_depth(f, 0, 10) // Max depth of 10 for truncation
    }
}

struct DebugElements<'a, V> {
    elements: &'a Vec<Pattern<V>>,
    depth: usize,
    max_depth: usize,
}

impl<'a, V: fmt::Debug> fmt::Debug for DebugElements<'a, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.depth > self.max_depth {
            return write!(f, "[...]");
        }

        let mut list = f.debug_list();
        for (i, elem) in self.elements.iter().enumerate() {
            if i >= 5 && self.elements.len() > 10 {
                // Truncate if more than 10 elements
                list.entry(&format_args!("... ({} more)", self.elements.len() - 5));
                break;
            }
            list.entry(&DebugPattern {
                pattern: elem,
                depth: self.depth + 1,
                max_depth: self.max_depth,
            });
        }
        list.finish()
    }
}

struct DebugPattern<'a, V> {
    pattern: &'a Pattern<V>,
    depth: usize,
    max_depth: usize,
}

impl<'a, V: fmt::Debug> fmt::Debug for DebugPattern<'a, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pattern
            .fmt_debug_with_depth(f, self.depth, self.max_depth)
    }
}

impl<V: fmt::Display> Pattern<V> {
    fn fmt_display_with_depth(
        &self,
        f: &mut fmt::Formatter<'_>,
        depth: usize,
        max_depth: usize,
    ) -> fmt::Result {
        if depth > max_depth {
            return write!(f, "...");
        }

        write!(f, "(")?;
        write!(f, "{}", self.value)?;

        if !self.elements.is_empty() {
            write!(f, " ")?;
            for (i, elem) in self.elements.iter().enumerate() {
                if i > 0 {
                    write!(f, " ")?;
                }
                if i >= 5 && self.elements.len() > 10 {
                    write!(f, "... ({} more)", self.elements.len() - 5)?;
                    break;
                }
                elem.fmt_display_with_depth(f, depth + 1, max_depth)?;
            }
        }

        write!(f, ")")
    }
}

impl<V: fmt::Display> fmt::Display for Pattern<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_display_with_depth(f, 0, 10) // Max depth of 10 for truncation
    }
}
