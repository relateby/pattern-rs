//! Pattern type definition
//!
//! This module provides the core `Pattern<V>` type, a recursive, nested structure
//! (s-expression-like) that is generic over value type `V`.
//!
//! # Construction Functions
//!
//! - [`Pattern::point`] - Creates an atomic pattern from a value (special case, matches gram-hs API)
//! - [`Pattern::pattern`] - Creates a pattern with elements (primary constructor, matches gram-hs API)
//! - [`Pattern::from_list`] - Creates a pattern from a list of values
//!
//! # Accessor Methods
//!
//! - [`Pattern::value`] - Returns a reference to the pattern's value
//! - [`Pattern::elements`] - Returns a slice of the pattern's elements
//!
//! # Inspection Utilities
//!
//! - [`Pattern::length`] - Returns the number of direct elements
//! - [`Pattern::size`] - Returns the total number of nodes
//! - [`Pattern::depth`] - Returns the maximum nesting depth
//! - [`Pattern::is_atomic`] - Checks if a pattern is atomic

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

// Construction Functions

impl<V> Pattern<V> {
    /// Creates an atomic pattern (a pattern with no elements) from a value.
    ///
    /// This is the special case constructor for atomic patterns.
    /// Equivalent to gram-hs `point :: v -> Pattern v`.
    ///
    /// # Arguments
    ///
    /// * `value` - The value component of the pattern
    ///
    /// # Returns
    ///
    /// A new atomic `Pattern<V>` instance with the specified value and empty elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let atomic = Pattern::point("atom".to_string());
    /// assert_eq!(atomic.value, "atom");
    /// assert_eq!(atomic.elements.len(), 0);
    /// ```
    pub fn point(value: V) -> Self {
        Pattern {
            value,
            elements: vec![],
        }
    }

    /// Creates a pattern with a value and elements.
    ///
    /// This is the primary constructor for creating patterns. Takes a decoration value
    /// and a list of pattern elements. The elements form the pattern itself; the value
    /// provides decoration about that pattern.
    ///
    /// Equivalent to gram-hs `pattern :: v -> [Pattern v] -> Pattern v`.
    ///
    /// # Arguments
    ///
    /// * `value` - The value component of the pattern
    /// * `elements` - The nested collection of patterns
    ///
    /// # Returns
    ///
    /// A new `Pattern<V>` instance with the specified value and elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern("root".to_string(), vec![
    ///     Pattern::point("child".to_string()),
    /// ]);
    /// assert_eq!(pattern.value, "root");
    /// assert_eq!(pattern.elements.len(), 1);
    /// ```
    #[allow(clippy::self_named_constructors)]
    pub fn pattern(value: V, elements: Vec<Pattern<V>>) -> Self {
        Pattern { value, elements }
    }

    /// Creates a pattern from a list of values.
    ///
    /// Creates a pattern where the first argument is the decoration value,
    /// and the list of values are converted to atomic patterns and used as elements.
    /// Equivalent to gram-hs `fromList :: v -> [v] -> Pattern v`.
    ///
    /// # Arguments
    ///
    /// * `value` - The decoration value for the pattern
    /// * `values` - List of values to convert to atomic patterns as elements
    ///
    /// # Returns
    ///
    /// A new `Pattern<V>` instance with value as decoration and values converted to atomic patterns as elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::from_list("root".to_string(), vec![
    ///     "a".to_string(),
    ///     "b".to_string(),
    ///     "c".to_string(),
    /// ]);
    /// assert_eq!(pattern.value, "root");
    /// assert_eq!(pattern.elements.len(), 3);
    /// ```
    pub fn from_list(value: V, values: Vec<V>) -> Self {
        Pattern {
            value,
            elements: values.into_iter().map(Pattern::point).collect(),
        }
    }

    /// Returns a reference to the pattern's value component.
    ///
    /// Equivalent to gram-hs `value :: Pattern v -> v` (record field accessor).
    ///
    /// # Returns
    ///
    /// An immutable reference to the pattern's value.
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::point("hello".to_string());
    /// let value = pattern.value(); // &String
    /// assert_eq!(value, "hello");
    /// ```
    pub fn value(&self) -> &V {
        &self.value
    }

    /// Returns a slice of the pattern's elements.
    ///
    /// Equivalent to gram-hs `elements :: Pattern v -> [Pattern v]` (record field accessor).
    ///
    /// # Returns
    ///
    /// An immutable slice of the pattern's nested elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern("parent".to_string(), vec![
    ///     Pattern::point("child1".to_string()),
    ///     Pattern::point("child2".to_string()),
    /// ]);
    /// let elements = pattern.elements();
    /// assert_eq!(elements.len(), 2);
    /// ```
    pub fn elements(&self) -> &[Pattern<V>] {
        &self.elements
    }

    /// Returns the number of direct elements in a pattern's sequence.
    ///
    /// Equivalent to gram-hs `length :: Pattern v -> Int`.
    ///
    /// # Returns
    ///
    /// The number of direct elements (not nested).
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern("parent".to_string(), vec![
    ///     Pattern::point("child1".to_string()),
    ///     Pattern::point("child2".to_string()),
    /// ]);
    /// assert_eq!(pattern.length(), 2);
    /// ```
    pub fn length(&self) -> usize {
        self.elements.len()
    }

    /// Returns the total number of nodes in a pattern structure, including all nested patterns.
    ///
    /// Equivalent to gram-hs `size :: Pattern v -> Int`.
    ///
    /// # Returns
    ///
    /// The total number of nodes (root + all nested nodes).
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let atomic = Pattern::point("atom".to_string());
    /// assert_eq!(atomic.size(), 1);
    ///
    /// let pattern = Pattern::pattern("root".to_string(), vec![
    ///     Pattern::point("child1".to_string()),
    ///     Pattern::point("child2".to_string()),
    /// ]);
    /// assert_eq!(pattern.size(), 3); // root + 2 children
    /// ```
    pub fn size(&self) -> usize {
        1 + self.elements.iter().map(|e| e.size()).sum::<usize>()
    }

    /// Returns the maximum nesting depth of a pattern structure.
    ///
    /// Equivalent to gram-hs `depth :: Pattern v -> Int`.
    ///
    /// # Returns
    ///
    /// The maximum nesting depth. Atomic patterns (patterns with no elements) have depth 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let atomic = Pattern::point("hello".to_string());
    /// assert_eq!(atomic.depth(), 0); // Atomic patterns have depth 0
    ///
    /// let nested = Pattern::pattern("parent".to_string(), vec![
    ///     Pattern::pattern("child".to_string(), vec![
    ///         Pattern::point("grandchild".to_string()),
    ///     ]),
    /// ]);
    /// assert_eq!(nested.depth(), 2);
    /// ```
    pub fn depth(&self) -> usize {
        if self.elements.is_empty() {
            0
        } else {
            1 + self.elements.iter().map(|e| e.depth()).max().unwrap_or(0)
        }
    }

    /// Checks if a pattern is atomic (has no elements).
    ///
    /// This is a convenience helper for pattern classification.
    ///
    /// # Returns
    ///
    /// `true` if the pattern has no elements, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let atomic = Pattern::point("hello".to_string());
    /// assert!(atomic.is_atomic());
    ///
    /// let nested = Pattern::pattern("parent".to_string(), vec![
    ///     Pattern::point("child".to_string()),
    /// ]);
    /// assert!(!nested.is_atomic());
    /// ```
    pub fn is_atomic(&self) -> bool {
        self.elements.is_empty()
    }
}
