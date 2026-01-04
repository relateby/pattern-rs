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

/// Configurable validation rules for pattern structure.
///
/// Rules can specify limits on nesting depth, element counts, or other structural properties.
/// Rules are optional (None means no limit).
///
/// # Examples
///
/// ```
/// use pattern_core::ValidationRules;
///
/// // No constraints (all patterns valid)
/// let rules = ValidationRules::default();
///
/// // Maximum depth constraint
/// let rules = ValidationRules {
///     max_depth: Some(10),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default)]
pub struct ValidationRules {
    /// Maximum nesting depth allowed (None = no limit)
    pub max_depth: Option<usize>,
    /// Maximum element count allowed (None = no limit)
    pub max_elements: Option<usize>,
    /// Required fields (reserved for future value-specific validation)
    pub required_fields: Vec<String>,
}

/// Error type for pattern validation failures.
///
/// Provides detailed information about what rule was violated and where
/// in the pattern structure the violation occurred.
///
/// # Examples
///
/// ```
/// use pattern_core::ValidationError;
///
/// let error = ValidationError {
///     message: "Pattern depth exceeds maximum".to_string(),
///     rule_violated: "max_depth".to_string(),
///     location: vec!["elements".to_string(), "0".to_string()],
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    /// Human-readable error message
    pub message: String,
    /// Name of violated rule (e.g., "max_depth", "max_elements")
    pub rule_violated: String,
    /// Path to violating node in pattern structure
    pub location: Vec<String>,
}

/// Results from structure analysis utilities.
///
/// Provides detailed information about pattern structural characteristics
/// including depth distribution, element counts, nesting patterns, and summaries.
///
/// # Examples
///
/// ```
/// use pattern_core::{Pattern, StructureAnalysis};
///
/// let pattern = Pattern::pattern("root".to_string(), vec![/* ... */]);
/// let analysis = pattern.analyze_structure();
///
/// println!("Depth distribution: {:?}", analysis.depth_distribution);
/// println!("Summary: {}", analysis.summary);
/// ```
#[derive(Debug, Clone)]
pub struct StructureAnalysis {
    /// Count of nodes at each depth level (index = depth, value = count)
    pub depth_distribution: Vec<usize>,
    /// Element counts at each level (index = level, value = count)
    pub element_counts: Vec<usize>,
    /// Identified structural patterns (e.g., "linear", "tree", "balanced")
    pub nesting_patterns: Vec<String>,
    /// Human-readable summary of structure
    pub summary: String,
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

    /// Maps a function over all values in the pattern, preserving structure.
    ///
    /// This is equivalent to Haskell's `fmap` for the Functor typeclass,
    /// but follows Rust naming conventions. The transformation applies to
    /// all values recursively while preserving the pattern structure
    /// (number of elements, nesting depth, element order).
    ///
    /// # Functor Laws
    ///
    /// This implementation satisfies the functor laws:
    /// - **Identity**: `pattern.map(|x| x.clone()) == pattern`
    /// - **Composition**: `pattern.map(|x| g(&f(x))) == pattern.map(f).map(g)`
    ///
    /// # Type Parameters
    ///
    /// * `W` - The output value type (can be different from `V`)
    /// * `F` - The transformation function type
    ///
    /// # Arguments
    ///
    /// * `f` - Transformation function that takes a reference to a value (`&V`)
    ///   and returns a new value (`W`)
    ///
    /// # Returns
    ///
    /// A new `Pattern<W>` with the same structure but transformed values
    ///
    /// # Examples
    ///
    /// ## String transformation
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::point("hello");
    /// let upper = pattern.map(|s| s.to_uppercase());
    /// assert_eq!(upper.value, "HELLO");
    /// ```
    ///
    /// ## Type conversion
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let numbers = Pattern::point(42);
    /// let strings = numbers.map(|n| n.to_string());
    /// assert_eq!(strings.value, "42");
    /// ```
    ///
    /// ## Nested patterns
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern("root", vec![
    ///     Pattern::point("child1"),
    ///     Pattern::point("child2"),
    /// ]);
    /// let upper = pattern.map(|s| s.to_uppercase());
    /// assert_eq!(upper.value, "ROOT");
    /// assert_eq!(upper.elements[0].value, "CHILD1");
    /// assert_eq!(upper.elements[1].value, "CHILD2");
    /// ```
    ///
    /// ## Composition
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let result = Pattern::point(5)
    ///     .map(|n| n * 2)
    ///     .map(|n| n + 1);
    /// assert_eq!(result.value, 11);
    /// ```
    ///
    /// # Performance
    ///
    /// - Time complexity: O(n) where n is the total number of nodes
    /// - Space complexity: O(n) for the new pattern + O(d) for recursion stack
    ///   where d is the maximum nesting depth
    /// - Handles patterns with 100+ nesting levels without stack overflow
    /// - Handles patterns with 10,000+ nodes efficiently
    pub fn map<W, F>(self, f: F) -> Pattern<W>
    where
        F: Fn(&V) -> W,
    {
        self.map_with(&f)
    }

    /// Internal helper for map that takes function by reference.
    /// This enables efficient recursion without cloning the closure.
    fn map_with<W, F>(self, f: &F) -> Pattern<W>
    where
        F: Fn(&V) -> W,
    {
        Pattern {
            value: f(&self.value),
            elements: self
                .elements
                .into_iter()
                .map(|elem| elem.map_with(f))
                .collect(),
        }
    }

    /// Folds the pattern into a single value by applying a function to each value with an accumulator.
    ///
    /// Processes values in depth-first, root-first order (pre-order traversal).
    /// The root value is processed first, then elements are processed left to right, recursively.
    /// Each value in the pattern is processed exactly once, and the accumulator is threaded through
    /// all processing steps.
    ///
    /// # Type Parameters
    ///
    /// * `B` - The accumulator type (can be different from `V`)
    /// * `F` - The folding function type
    ///
    /// # Arguments
    ///
    /// * `init` - Initial accumulator value
    /// * `f` - Folding function with signature `Fn(B, &V) -> B`
    ///   - First parameter: Accumulator (passed by value)
    ///   - Second parameter: Value reference (borrowed from pattern)
    ///   - Returns: New accumulator value
    ///
    /// # Returns
    ///
    /// The final accumulated value of type `B`
    ///
    /// # Examples
    ///
    /// ## Sum all integers
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern(10, vec![
    ///     Pattern::point(20),
    ///     Pattern::point(30),
    /// ]);
    /// let sum = pattern.fold(0, |acc, v| acc + v);
    /// assert_eq!(sum, 60);  // 10 + 20 + 30
    /// ```
    ///
    /// ## Count values
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern("root", vec![
    ///     Pattern::point("child1"),
    ///     Pattern::point("child2"),
    /// ]);
    /// let count = pattern.fold(0, |acc, _| acc + 1);
    /// assert_eq!(count, 3);  // root + 2 children
    /// assert_eq!(count, pattern.size());
    /// ```
    ///
    /// ## Concatenate strings
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern("Hello", vec![
    ///     Pattern::point(" "),
    ///     Pattern::point("World"),
    /// ]);
    /// let result = pattern.fold(String::new(), |acc, s| acc + s);
    /// assert_eq!(result, "Hello World");
    /// ```
    ///
    /// ## Type transformation (string lengths to sum)
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern("hello", vec![
    ///     Pattern::point("world"),
    ///     Pattern::point("!"),
    /// ]);
    /// let total_length: usize = pattern.fold(0, |acc, s| acc + s.len());
    /// assert_eq!(total_length, 11);  // 5 + 5 + 1
    /// ```
    ///
    /// ## Build a vector
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern(1, vec![
    ///     Pattern::point(2),
    ///     Pattern::point(3),
    /// ]);
    /// let values: Vec<i32> = pattern.fold(Vec::new(), |mut acc, v| {
    ///     acc.push(*v);
    ///     acc
    /// });
    /// assert_eq!(values, vec![1, 2, 3]);
    /// ```
    ///
    /// ## Verify traversal order
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern("A", vec![
    ///     Pattern::point("B"),
    ///     Pattern::pattern("C", vec![
    ///         Pattern::point("D"),
    ///     ]),
    /// ]);
    /// // Root first, then elements in order, depth-first
    /// let result = pattern.fold(String::new(), |acc, s| acc + s);
    /// assert_eq!(result, "ABCD");
    /// ```
    ///
    /// # Performance
    ///
    /// - Time complexity: O(n) where n is the total number of values
    /// - Space complexity: O(d) for recursion stack where d is the maximum nesting depth
    /// - Handles patterns with 100+ nesting levels without stack overflow
    /// - Handles patterns with 10,000+ nodes efficiently
    ///
    /// # Behavioral Guarantees
    ///
    /// 1. **Completeness**: Every value in the pattern is processed exactly once
    /// 2. **Order**: Values processed in depth-first, root-first order
    /// 3. **Non-destructive**: Pattern structure is not modified (borrows only)
    /// 4. **Reusability**: Pattern can be folded multiple times
    pub fn fold<B, F>(&self, init: B, f: F) -> B
    where
        F: Fn(B, &V) -> B,
    {
        self.fold_with(init, &f)
    }

    /// Internal helper for fold that takes function by reference.
    ///
    /// This enables efficient recursion without cloning the closure.
    /// Public `fold` passes closure by value for ergonomics,
    /// internal `fold_with` passes closure by reference for efficiency.
    fn fold_with<B, F>(&self, acc: B, f: &F) -> B
    where
        F: Fn(B, &V) -> B,
    {
        // Process root value first
        let acc = f(acc, &self.value);

        // Process elements recursively (left to right)
        self.elements
            .iter()
            .fold(acc, |acc, elem| elem.fold_with(acc, f))
    }

    /// Collects all values from the pattern into a vector in traversal order.
    ///
    /// Returns references to all values in the pattern, maintaining depth-first,
    /// root-first order (same as `fold`). The root value appears first in the vector,
    /// followed by element values in traversal order.
    ///
    /// This method uses `fold` internally and is a convenience for the common case
    /// of collecting all pattern values into a standard collection.
    ///
    /// # Returns
    ///
    /// A `Vec<&V>` containing references to all values in traversal order
    ///
    /// # Examples
    ///
    /// ## Get all values
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern(1, vec![
    ///     Pattern::point(2),
    ///     Pattern::point(3),
    /// ]);
    /// let values: Vec<&i32> = pattern.values();
    /// assert_eq!(values, vec![&1, &2, &3]);
    /// assert_eq!(values.len(), pattern.size());
    /// ```
    ///
    /// ## Verify order
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern(1, vec![
    ///     Pattern::point(2),
    ///     Pattern::pattern(3, vec![
    ///         Pattern::point(4),
    ///     ]),
    /// ]);
    /// let values = pattern.values();
    /// assert_eq!(values, vec![&1, &2, &3, &4]);
    /// ```
    ///
    /// ## Use with Iterator methods
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern(1, vec![
    ///     Pattern::point(2),
    ///     Pattern::point(3),
    ///     Pattern::point(4),
    /// ]);
    /// let sum: i32 = pattern.values().iter().map(|&&v| v).sum();
    /// assert_eq!(sum, 10);
    ///
    /// let all_positive = pattern.values().iter().all(|&&v| v > 0);
    /// assert!(all_positive);
    /// ```
    ///
    /// ## Nested patterns
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern(1, vec![
    ///     Pattern::pattern(2, vec![
    ///         Pattern::point(3),
    ///     ]),
    /// ]);
    /// let values: Vec<&i32> = pattern.values();
    /// assert_eq!(values, vec![&1, &2, &3]);
    /// ```
    ///
    /// # Performance
    ///
    /// - Time complexity: O(n) where n is the total number of values
    /// - Space complexity: O(n) for the result vector
    /// - Efficient single-pass collection using fold
    pub fn values(&self) -> Vec<&V> {
        // Collect all values into a vector using fold
        let mut result = Vec::with_capacity(self.size());
        self.collect_values(&mut result);
        result
    }

    /// Internal helper to recursively collect values into a vector.
    fn collect_values<'a>(&'a self, result: &mut Vec<&'a V>) {
        result.push(&self.value);
        for elem in &self.elements {
            elem.collect_values(result);
        }
    }

    /// Validates pattern structure against configurable rules and constraints.
    ///
    /// Returns `Ok(())` if the pattern is valid according to the rules,
    /// or `Err(ValidationError)` if validation fails.
    ///
    /// # Arguments
    ///
    /// * `rules` - Validation rules to apply
    ///
    /// # Returns
    ///
    /// * `Ok(())` if pattern is valid
    /// * `Err(ValidationError)` if validation fails, containing detailed error information
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::{Pattern, ValidationRules};
    ///
    /// let pattern = Pattern::pattern("root".to_string(), vec![/* ... */]);
    ///
    /// let rules = ValidationRules {
    ///     max_depth: Some(10),
    ///     ..Default::default()
    /// };
    ///
    /// match pattern.validate(&rules) {
    ///     Ok(()) => println!("Pattern is valid"),
    ///     Err(e) => println!("Validation failed: {} at {:?}", e.message, e.location),
    /// }
    /// ```
    ///
    /// # Performance
    ///
    /// This operation is O(n) where n is the number of nodes in the pattern.
    /// Must handle at least 100 nesting levels without stack overflow.
    pub fn validate(&self, rules: &ValidationRules) -> Result<(), ValidationError> {
        self.validate_recursive(rules, 0, &mut Vec::new())
    }

    /// Internal recursive validation helper
    fn validate_recursive(
        &self,
        rules: &ValidationRules,
        current_depth: usize,
        location: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        // Check max_depth constraint
        if let Some(max_depth) = rules.max_depth {
            if current_depth > max_depth {
                return Err(ValidationError {
                    message: format!(
                        "Pattern depth {} exceeds maximum allowed depth {}",
                        current_depth, max_depth
                    ),
                    rule_violated: "max_depth".to_string(),
                    location: location.clone(),
                });
            }
        }

        // Check max_elements constraint at current level
        if let Some(max_elements) = rules.max_elements {
            if self.elements.len() > max_elements {
                return Err(ValidationError {
                    message: format!(
                        "Pattern has {} elements, exceeding maximum allowed {}",
                        self.elements.len(),
                        max_elements
                    ),
                    rule_violated: "max_elements".to_string(),
                    location: location.clone(),
                });
            }
        }

        // Recursively validate all elements
        for (index, element) in self.elements.iter().enumerate() {
            location.push("elements".to_string());
            location.push(index.to_string());

            element.validate_recursive(rules, current_depth + 1, location)?;

            location.pop(); // Remove index
            location.pop(); // Remove "elements"
        }

        Ok(())
    }

    /// Analyzes pattern structure and returns detailed information about structural characteristics.
    ///
    /// Provides comprehensive structural analysis including depth distribution, element counts,
    /// nesting patterns, and a human-readable summary.
    ///
    /// # Returns
    ///
    /// `StructureAnalysis` containing:
    /// - Depth distribution: Count of nodes at each depth level
    /// - Element counts: Maximum element count at each level (for pattern identification)
    /// - Nesting patterns: Identified structural patterns
    /// - Summary: Human-readable text summary
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern("root".to_string(), vec![/* ... */]);
    /// let analysis = pattern.analyze_structure();
    ///
    /// println!("Depth distribution: {:?}", analysis.depth_distribution);
    /// println!("Element counts: {:?}", analysis.element_counts);
    /// println!("Nesting patterns: {:?}", analysis.nesting_patterns);
    /// println!("Summary: {}", analysis.summary);
    /// ```
    ///
    /// # Performance
    ///
    /// This operation is O(n) where n is the number of nodes in the pattern.
    /// Must handle at least 100 nesting levels without stack overflow.
    /// Must handle at least 10,000 elements efficiently.
    pub fn analyze_structure(&self) -> StructureAnalysis {
        let mut depth_distribution = Vec::new();
        let mut element_counts = Vec::new();

        self.analyze_recursive(0, &mut depth_distribution, &mut element_counts);

        // Trim trailing zeros from element_counts (leaf levels with 0 elements)
        // According to spec: atomic pattern should have [], 2-level tree should have [count], not [count, 0]
        while let Some(&0) = element_counts.last() {
            element_counts.pop();
        }

        // Identify nesting patterns
        let nesting_patterns = self.identify_nesting_patterns(&depth_distribution, &element_counts);

        // Generate summary
        let summary =
            self.generate_summary(&depth_distribution, &element_counts, &nesting_patterns);

        StructureAnalysis {
            depth_distribution,
            element_counts,
            nesting_patterns,
            summary,
        }
    }

    /// Internal recursive analysis helper
    /// Tracks maximum element count at each level for pattern identification
    fn analyze_recursive(
        &self,
        current_depth: usize,
        depth_distribution: &mut Vec<usize>,
        element_counts: &mut Vec<usize>,
    ) {
        // Ensure vectors are large enough
        while depth_distribution.len() <= current_depth {
            depth_distribution.push(0);
        }
        while element_counts.len() <= current_depth {
            element_counts.push(0);
        }

        // Count this node at current depth
        depth_distribution[current_depth] += 1;

        // Track maximum element count at current level
        // Maximum is used for linear/tree pattern detection (all nodes <= 1 vs any node > 1)
        // For balanced patterns, we compare maximums across levels, which works correctly
        // with the fixed balanced pattern logic (ratio between 0.5 and 2.0)
        let current_count = self.elements.len();
        if current_count > element_counts[current_depth] {
            element_counts[current_depth] = current_count;
        }

        // Recursively analyze elements
        for element in &self.elements {
            element.analyze_recursive(current_depth + 1, depth_distribution, element_counts);
        }
    }

    /// Identify structural patterns from depth distribution and element counts
    fn identify_nesting_patterns(
        &self,
        depth_distribution: &[usize],
        element_counts: &[usize],
    ) -> Vec<String> {
        let mut patterns = Vec::new();

        if depth_distribution.len() <= 1 {
            patterns.push("atomic".to_string());
            return patterns;
        }

        // Check for linear pattern (one element per level)
        let is_linear = element_counts.iter().all(|&count| count <= 1);
        if is_linear {
            patterns.push("linear".to_string());
        }

        // Check for tree-like pattern (multiple elements, decreasing with depth)
        let has_branching = element_counts.iter().any(|&count| count > 1);
        if has_branching {
            patterns.push("tree".to_string());
        }

        // Check for balanced pattern (similar element counts across levels)
        // Balanced means counts are within 50% of each other (ratio between 0.5 and 2.0)
        // Note: element_counts already has trailing zeros trimmed, so all entries are non-zero
        if element_counts.len() >= 2 {
            let first_count = element_counts[0];
            if first_count > 0 {
                // Check all levels (trailing zeros already trimmed)
                let similar_counts = element_counts.iter().skip(1).all(|&count| {
                    let ratio = count as f64 / first_count as f64;
                    // Balanced if ratio is between 0.5 and 2.0 (within 50% of first_count)
                    (0.5..=2.0).contains(&ratio)
                });
                if similar_counts && first_count > 1 {
                    patterns.push("balanced".to_string());
                }
            }
        }

        if patterns.is_empty() {
            patterns.push("irregular".to_string());
        }

        patterns
    }

    /// Generate human-readable summary of structure
    fn generate_summary(
        &self,
        depth_distribution: &[usize],
        _element_counts: &[usize], // Reserved for future use in summary generation
        nesting_patterns: &[String],
    ) -> String {
        let total_nodes: usize = depth_distribution.iter().sum();
        let max_depth = depth_distribution.len().saturating_sub(1);
        let pattern_desc = if nesting_patterns.is_empty() {
            "unknown"
        } else {
            &nesting_patterns[0]
        };

        format!(
            "Pattern with {} level{}, {} node{}, {}-like structure",
            max_depth + 1,
            if max_depth == 0 { "" } else { "s" },
            total_nodes,
            if total_nodes == 1 { "" } else { "s" },
            pattern_desc
        )
    }
}
