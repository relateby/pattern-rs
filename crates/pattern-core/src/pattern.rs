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
//! - [`Pattern::values`] - Extracts all values as a flat list (pre-order)
//!
//! # Query Functions
//!
//! - [`Pattern::any_value`] - Checks if at least one value satisfies a predicate (short-circuits)
//! - [`Pattern::all_values`] - Checks if all values satisfy a predicate (short-circuits)
//! - [`Pattern::filter`] - Extracts subpatterns that satisfy a pattern predicate
//! - [`Pattern::find_first`] - Finds the first subpattern that satisfies a pattern predicate (short-circuits)
//! - [`Pattern::matches`] - Checks if two patterns have identical structure
//! - [`Pattern::contains`] - Checks if a pattern contains another as a subpattern
//!
//! # Combination Operations
//!
//! - [`Pattern::combine`] - Combines two patterns associatively (value combination + element concatenation)

use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

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
/// - `PartialOrd`, `Ord`: Patterns can be ordered when `V: PartialOrd` (or `Ord`)
///   - Uses value-first lexicographic ordering: compares values, then elements
///   - Enables sorting, min/max operations, and use in ordered collections (BTreeSet, BTreeMap)
/// - `Hash`: Patterns can be hashed when `V: Hash` for use in HashMap/HashSet
///   - Enables pattern deduplication and caching
///   - Structure-preserving: different structures produce different hashes
///   - Note: `Pattern<Subject>` is NOT hashable (Subject contains f64)
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

    /// Checks if at least one value in the pattern satisfies the given predicate.
    ///
    /// This operation traverses the pattern structure in pre-order (root first, then elements)
    /// and applies the predicate to each value. Returns `true` as soon as a value satisfies
    /// the predicate (short-circuit evaluation - both predicate evaluation AND traversal stop),
    /// or `false` if no values match.
    ///
    /// Equivalent to Haskell's `anyValue :: (v -> Bool) -> Pattern v -> Bool`.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A function that takes a reference to a value and returns a boolean
    ///
    /// # Arguments
    ///
    /// * `predicate` - A function to test each value
    ///
    /// # Returns
    ///
    /// * `true` if at least one value satisfies the predicate
    /// * `false` if no values satisfy the predicate (including empty patterns)
    ///
    /// # Complexity
    ///
    /// * Time: O(n) worst case, O(1) to O(n) average (short-circuits on first match)
    /// * Space: O(1) heap, O(d) stack where d = maximum depth
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern(5, vec![
    ///     Pattern::point(10),
    ///     Pattern::point(3),
    /// ]);
    ///
    /// // Check if any value is greater than 8
    /// assert!(pattern.any_value(|v| *v > 8));  // true (10 > 8)
    ///
    /// // Check if any value is negative
    /// assert!(!pattern.any_value(|v| *v < 0)); // false (all positive)
    /// ```
    ///
    /// # Short-Circuit Behavior
    ///
    /// The operation stops traversal as soon as a matching value is found:
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern(1, vec![
    ///     Pattern::point(2),
    ///     Pattern::point(5), // Matches here - stops traversal
    ///     Pattern::point(3), // Not visited
    /// ]);
    ///
    /// assert!(pattern.any_value(|v| *v == 5));
    /// ```
    pub fn any_value<F>(&self, predicate: F) -> bool
    where
        F: Fn(&V) -> bool,
    {
        self.any_value_recursive(&predicate)
    }

    /// Helper function for any_value with early termination.
    fn any_value_recursive<F>(&self, predicate: &F) -> bool
    where
        F: Fn(&V) -> bool,
    {
        // Check current value (pre-order)
        if predicate(&self.value) {
            return true;
        }

        // Check elements recursively, stop on first match
        for element in &self.elements {
            if element.any_value_recursive(predicate) {
                return true;
            }
        }

        false
    }

    /// Checks if all values in the pattern satisfy the given predicate.
    ///
    /// This operation traverses the pattern structure in pre-order (root first, then elements)
    /// and applies the predicate to each value. Returns `false` as soon as a value is found
    /// that does not satisfy the predicate (short-circuit evaluation - both predicate evaluation
    /// AND traversal stop), or `true` if all values satisfy the predicate.
    ///
    /// Equivalent to Haskell's `allValues :: (v -> Bool) -> Pattern v -> Bool`.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A function that takes a reference to a value and returns a boolean
    ///
    /// # Arguments
    ///
    /// * `predicate` - A function to test each value
    ///
    /// # Returns
    ///
    /// * `true` if all values satisfy the predicate (vacuous truth for patterns with no values)
    /// * `false` if at least one value does not satisfy the predicate
    ///
    /// # Complexity
    ///
    /// * Time: O(n) worst case, O(1) to O(n) average (short-circuits on first failure)
    /// * Space: O(1) heap, O(d) stack where d = maximum depth
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern(5, vec![
    ///     Pattern::point(10),
    ///     Pattern::point(3),
    /// ]);
    ///
    /// // Check if all values are positive
    /// assert!(pattern.all_values(|v| *v > 0));  // true (all > 0)
    ///
    /// // Check if all values are greater than 8
    /// assert!(!pattern.all_values(|v| *v > 8)); // false (5 and 3 fail)
    /// ```
    ///
    /// # Short-Circuit Behavior
    ///
    /// The operation stops traversal as soon as a non-matching value is found:
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern(1, vec![
    ///     Pattern::point(2),
    ///     Pattern::point(-5), // Fails here - stops traversal
    ///     Pattern::point(3),  // Not visited
    /// ]);
    ///
    /// assert!(!pattern.all_values(|v| *v > 0));
    /// ```
    ///
    /// # Relationship to any_value
    ///
    /// The following equivalence holds: `all_values(p) ≡ !any_value(!p)`
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern(5, vec![Pattern::point(10)]);
    /// let predicate = |v: &i32| *v > 0;
    ///
    /// assert_eq!(
    ///     pattern.all_values(predicate),
    ///     !pattern.any_value(|v| !predicate(v))
    /// );
    /// ```
    pub fn all_values<F>(&self, predicate: F) -> bool
    where
        F: Fn(&V) -> bool,
    {
        self.all_values_recursive(&predicate)
    }

    /// Helper function for all_values with early termination.
    fn all_values_recursive<F>(&self, predicate: &F) -> bool
    where
        F: Fn(&V) -> bool,
    {
        // Check current value (pre-order)
        if !predicate(&self.value) {
            return false;
        }

        // Check elements recursively, stop on first failure
        for element in &self.elements {
            if !element.all_values_recursive(predicate) {
                return false;
            }
        }

        true
    }

    /// Filters subpatterns that satisfy the given pattern predicate.
    ///
    /// This operation traverses the pattern structure in pre-order (root first, then elements)
    /// and collects references to all patterns that satisfy the predicate. Unlike `any_value`
    /// and `all_values` which operate on values, this method operates on entire patterns,
    /// allowing predicates to test structural properties (length, depth, etc.) as well as values.
    ///
    /// Equivalent to Haskell's `filterPatterns :: (Pattern v -> Bool) -> Pattern v -> [Pattern v]`.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A function that takes a reference to a pattern and returns a boolean
    ///
    /// # Arguments
    ///
    /// * `predicate` - A function to test each pattern (including the root)
    ///
    /// # Returns
    ///
    /// A vector of immutable references to patterns that satisfy the predicate, in pre-order
    /// traversal order. Returns an empty vector if no patterns match.
    ///
    /// # Complexity
    ///
    /// * Time: O(n) where n is the number of nodes (must visit all patterns)
    /// * Space: O(m) heap where m is the number of matches, O(d) stack where d = maximum depth
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern(
    ///     "root",
    ///     vec![
    ///         Pattern::point("leaf1"),
    ///         Pattern::pattern("branch", vec![
    ///             Pattern::point("leaf2"),
    ///         ]),
    ///     ],
    /// );
    ///
    /// // Find all atomic (leaf) patterns
    /// let leaves = pattern.filter(|p| p.is_atomic());
    /// assert_eq!(leaves.len(), 2);  // leaf1, leaf2
    ///
    /// // Find all patterns with specific value
    /// let roots = pattern.filter(|p| p.value == "root");
    /// assert_eq!(roots.len(), 1);
    ///
    /// // Find patterns with elements (non-atomic)
    /// let branches = pattern.filter(|p| p.length() > 0);
    /// assert_eq!(branches.len(), 2);  // root, branch
    /// ```
    ///
    /// # Pre-Order Traversal
    ///
    /// Results are returned in pre-order traversal order (root first, then elements in order):
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern(1, vec![
    ///     Pattern::point(2),
    ///     Pattern::pattern(3, vec![Pattern::point(4)]),
    ///     Pattern::point(5),
    /// ]);
    ///
    /// let all = pattern.filter(|_| true);
    /// assert_eq!(all.len(), 5);
    /// assert_eq!(all[0].value, 1); // root
    /// assert_eq!(all[1].value, 2); // first element
    /// assert_eq!(all[2].value, 3); // second element
    /// assert_eq!(all[3].value, 4); // nested in second element
    /// assert_eq!(all[4].value, 5); // third element
    /// ```
    ///
    /// # Combining with Other Operations
    ///
    /// Filter can be combined with value predicates:
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern(5, vec![
    ///     Pattern::point(10),
    ///     Pattern::pattern(3, vec![]),
    /// ]);
    ///
    /// // Find patterns with large values
    /// let large = pattern.filter(|p| p.value > 8);
    /// assert_eq!(large.len(), 1); // Only point(10)
    ///
    /// // Find non-empty patterns with all values positive
    /// let branches = pattern.filter(|p| {
    ///     p.length() > 0 && p.all_values(|v| *v > 0)
    /// });
    /// ```
    ///
    /// # Lifetime and References
    ///
    /// The returned references borrow from the source pattern and have the same lifetime:
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern("a", vec![Pattern::point("b")]);
    /// let matches = pattern.filter(|_| true);
    /// // matches[0] and matches[1] borrow from pattern
    /// // Cannot move or drop pattern while matches exist
    /// ```
    pub fn filter<F>(&self, predicate: F) -> Vec<&Pattern<V>>
    where
        F: Fn(&Pattern<V>) -> bool,
    {
        let mut result = Vec::new();
        self.filter_recursive(&predicate, &mut result);
        result
    }

    /// Helper function for recursive filter implementation.
    ///
    /// This performs a pre-order traversal, checking the current pattern first,
    /// then recursively filtering elements.
    fn filter_recursive<'a, F>(&'a self, predicate: &F, result: &mut Vec<&'a Pattern<V>>)
    where
        F: Fn(&Pattern<V>) -> bool,
    {
        // Check current pattern (pre-order: root first)
        if predicate(self) {
            result.push(self);
        }

        // Recursively filter elements
        for element in &self.elements {
            element.filter_recursive(predicate, result);
        }
    }

    /// Finds the first subpattern (including self) that satisfies a predicate.
    ///
    /// This method performs a depth-first pre-order traversal of the pattern structure
    /// (checking the root first, then elements recursively from left to right) and
    /// returns the first pattern that satisfies the predicate.
    ///
    /// # Arguments
    ///
    /// * `predicate` - A function that takes a pattern reference and returns `true`
    ///   if it matches the search criteria. The predicate can examine both the
    ///   pattern's value and its structure (element count, depth, etc.).
    ///
    /// # Returns
    ///
    /// * `Some(&Pattern<V>)` - A reference to the first matching pattern
    /// * `None` - If no pattern in the structure satisfies the predicate
    ///
    /// # Traversal Order
    ///
    /// The method uses depth-first pre-order traversal:
    /// 1. Check the root pattern first
    /// 2. Then check elements from left to right
    /// 3. For each element, recursively apply the same order
    ///
    /// This ensures consistent, predictable ordering and matches the behavior
    /// of other pattern traversal methods (`filter`, `fold`, `map`).
    ///
    /// # Short-Circuit Evaluation
    ///
    /// Unlike `filter`, which collects all matches, `find_first` stops immediately
    /// upon finding the first match. This makes it more efficient when you only
    /// need to know if a match exists or when you want the first occurrence.
    ///
    /// # Time Complexity
    ///
    /// * Best case: O(1) - if root matches
    /// * Average case: O(k) - where k is position of first match
    /// * Worst case: O(n) - if no match exists or match is last
    ///
    /// # Space Complexity
    ///
    /// O(d) where d is the maximum nesting depth (recursion stack)
    ///
    /// # Examples
    ///
    /// ## Finding by value
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern("root", vec![
    ///     Pattern::point("child1"),
    ///     Pattern::point("target"),
    /// ]);
    ///
    /// let result = pattern.find_first(|p| p.value == "target");
    /// assert!(result.is_some());
    /// assert_eq!(result.unwrap().value, "target");
    /// ```
    ///
    /// ## Finding by structure
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern("root", vec![
    ///     Pattern::pattern("branch", vec![
    ///         Pattern::point("leaf"),
    ///     ]),
    ///     Pattern::point("leaf2"),
    /// ]);
    ///
    /// // Find first atomic pattern (no elements)
    /// let leaf = pattern.find_first(|p| p.is_atomic());
    /// assert!(leaf.is_some());
    /// assert_eq!(leaf.unwrap().value, "leaf");  // First in pre-order
    /// ```
    ///
    /// ## No match returns None
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::point("single");
    /// let result = pattern.find_first(|p| p.value == "other");
    /// assert!(result.is_none());
    /// ```
    ///
    /// ## Combining value and structural predicates
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern(5, vec![
    ///     Pattern::pattern(10, vec![
    ///         Pattern::point(3),
    ///         Pattern::point(7),
    ///     ]),
    ///     Pattern::point(15),
    /// ]);
    ///
    /// // Find first pattern with value > 8 AND has elements
    /// let result = pattern.find_first(|p| p.value > 8 && p.length() > 0);
    /// assert!(result.is_some());
    /// assert_eq!(result.unwrap().value, 10);
    /// ```
    ///
    /// ## Pre-order traversal demonstration
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern(1, vec![
    ///     Pattern::pattern(2, vec![
    ///         Pattern::point(3),
    ///     ]),
    ///     Pattern::point(4),
    /// ]);
    ///
    /// // Traversal order: 1 (root), 2, 3, 4
    /// // First pattern with value > 1 is 2 (not 3)
    /// let result = pattern.find_first(|p| p.value > 1);
    /// assert_eq!(result.unwrap().value, 2);
    /// ```
    ///
    /// ## Integration with other methods
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern(5, vec![
    ///     Pattern::pattern(10, vec![
    ///         Pattern::point(-3),
    ///         Pattern::point(7),
    ///     ]),
    ///     Pattern::pattern(2, vec![
    ///         Pattern::point(1),
    ///         Pattern::point(4),
    ///     ]),
    /// ]);
    ///
    /// // Find first pattern where all values are positive
    /// let result = pattern.find_first(|p| p.all_values(|v| *v > 0));
    /// assert!(result.is_some());
    /// assert_eq!(result.unwrap().value, 7);  // First point with all positive values
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic. All inputs are valid:
    /// - Works with atomic patterns (no elements)
    /// - Works with patterns with empty elements
    /// - Works with deeply nested structures (limited only by stack size)
    /// - Handles all predicate results gracefully
    ///
    /// # Relationship to Other Methods
    ///
    /// * `filter` - Returns all matches, `find_first` returns only the first
    /// * `any_value` - Operates on values only, `find_first` operates on whole patterns
    /// * Consistency: `find_first(p).is_some()` implies `filter(p).len() > 0`
    /// * Consistency: If `find_first(p) == Some(x)`, then `filter(p)[0] == x`
    pub fn find_first<F>(&self, predicate: F) -> Option<&Pattern<V>>
    where
        F: Fn(&Pattern<V>) -> bool,
    {
        self.find_first_recursive(&predicate)
    }

    /// Helper function for recursive find_first implementation.
    ///
    /// This performs a pre-order traversal with early termination.
    /// Checks the current pattern first, then recursively searches elements.
    ///
    /// Returns `Some(&Pattern<V>)` on first match, `None` if no match found.
    fn find_first_recursive<'a, F>(&'a self, predicate: &F) -> Option<&'a Pattern<V>>
    where
        F: Fn(&Pattern<V>) -> bool,
    {
        // Check current pattern first (pre-order: root first)
        if predicate(self) {
            return Some(self);
        }

        // Recursively search elements (with early termination)
        for element in &self.elements {
            if let Some(found) = element.find_first_recursive(predicate) {
                return Some(found);
            }
        }

        // No match found
        None
    }

    /// Checks if two patterns have identical structure.
    ///
    /// This method performs structural equality checking, comparing both values and
    /// element arrangement recursively. Two patterns match if and only if:
    /// - Their values are equal (using `PartialEq`)
    /// - They have the same number of elements
    /// - All corresponding elements match recursively
    ///
    /// This is distinct from the `Eq` trait implementation and is intended for
    /// structural pattern matching operations. While currently equivalent to `==`
    /// for patterns where `V: Eq`, this method may diverge in the future to support
    /// wildcards, partial matching, or other pattern matching semantics.
    ///
    /// # Type Constraints
    ///
    /// Requires `V: PartialEq` so that values can be compared for equality.
    ///
    /// # Mathematical Properties
    ///
    /// * **Reflexive**: `p.matches(&p)` is always `true`
    /// * **Symmetric**: `p.matches(&q) == q.matches(&p)`
    /// * **Structural**: Distinguishes patterns with same values but different structures
    ///
    /// # Time Complexity
    ///
    /// * Best case: O(1) - if root values differ
    /// * Average case: O(min(n, m) / 2) - short-circuits on first mismatch
    /// * Worst case: O(min(n, m)) - if patterns are identical or differ only at end
    ///
    /// Where n and m are the number of nodes in each pattern.
    ///
    /// # Space Complexity
    ///
    /// O(min(d1, d2)) where d1 and d2 are the maximum nesting depths
    /// (recursion stack usage).
    ///
    /// # Examples
    ///
    /// ## Identical patterns match
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let p1 = Pattern::pattern("root", vec![
    ///     Pattern::point("a"),
    ///     Pattern::point("b"),
    /// ]);
    /// let p2 = Pattern::pattern("root", vec![
    ///     Pattern::point("a"),
    ///     Pattern::point("b"),
    /// ]);
    ///
    /// assert!(p1.matches(&p2));
    /// ```
    ///
    /// ## Self-matching (reflexive)
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::point("a");
    /// assert!(pattern.matches(&pattern));
    /// ```
    ///
    /// ## Different values don't match
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let p1 = Pattern::point("a");
    /// let p2 = Pattern::point("b");
    /// assert!(!p1.matches(&p2));
    /// ```
    ///
    /// ## Different structures don't match
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let p1 = Pattern::pattern("a", vec![
    ///     Pattern::point("b"),
    ///     Pattern::point("c"),
    /// ]);
    /// let p2 = Pattern::pattern("a", vec![
    ///     Pattern::pattern("b", vec![
    ///         Pattern::point("c"),
    ///     ]),
    /// ]);
    ///
    /// // Same flattened values ["a", "b", "c"] but different structure
    /// assert!(!p1.matches(&p2));
    /// ```
    ///
    /// ## Symmetry property
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let p1 = Pattern::point(42);
    /// let p2 = Pattern::point(99);
    ///
    /// // p1.matches(&p2) == p2.matches(&p1)
    /// assert_eq!(p1.matches(&p2), p2.matches(&p1));
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic. All inputs are valid:
    /// - Works with atomic patterns
    /// - Works with patterns with empty elements
    /// - Works with deeply nested structures (limited only by stack size)
    ///
    /// # Relationship to Other Methods
    ///
    /// * **Eq trait**: Currently equivalent for `V: Eq`, may diverge in future
    /// * **contains**: `p.matches(&q)` implies `p.contains(&q)` (equality implies containment)
    pub fn matches(&self, other: &Pattern<V>) -> bool
    where
        V: PartialEq,
    {
        // Values must match
        if self.value != other.value {
            return false;
        }

        // Element counts must match
        if self.elements.len() != other.elements.len() {
            return false;
        }

        // All corresponding elements must match recursively
        self.elements
            .iter()
            .zip(other.elements.iter())
            .all(|(e1, e2)| e1.matches(e2))
    }

    /// Checks if this pattern contains another pattern as a subpattern.
    ///
    /// This method searches the entire pattern structure to determine if the given
    /// subpattern appears anywhere within it. A pattern contains a subpattern if:
    /// - The pattern matches the subpattern (using `matches`), OR
    /// - Any of its elements contains the subpattern (recursive search)
    ///
    /// This provides a structural containment check that goes beyond simple equality,
    /// allowing you to test whether a pattern appears as part of a larger structure.
    ///
    /// # Type Constraints
    ///
    /// Requires `V: PartialEq` because it uses `matches` internally for comparison.
    ///
    /// # Mathematical Properties
    ///
    /// * **Reflexive**: `p.contains(&p)` is always `true` (self-containment)
    /// * **Transitive**: If `a.contains(&b)` and `b.contains(&c)`, then `a.contains(&c)`
    /// * **Weaker than matches**: `p.matches(&q)` implies `p.contains(&q)`, but not vice versa
    /// * **Not symmetric**: `p.contains(&q)` does NOT imply `q.contains(&p)`
    ///
    /// # Time Complexity
    ///
    /// * Best case: O(1) - if root matches subpattern
    /// * Average case: O(n * m / 2) - where n = container size, m = subpattern size
    /// * Worst case: O(n * m) - if subpattern not found or found at end
    ///
    /// # Space Complexity
    ///
    /// O(d) where d is the maximum nesting depth (recursion stack usage).
    ///
    /// # Examples
    ///
    /// ## Self-containment
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern("root", vec![
    ///     Pattern::point("child"),
    /// ]);
    ///
    /// // Every pattern contains itself
    /// assert!(pattern.contains(&pattern));
    /// ```
    ///
    /// ## Direct element containment
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern("root", vec![
    ///     Pattern::point("a"),
    ///     Pattern::point("b"),
    /// ]);
    ///
    /// let subpattern = Pattern::point("a");
    /// assert!(pattern.contains(&subpattern));
    /// ```
    ///
    /// ## Nested descendant containment
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern("root", vec![
    ///     Pattern::pattern("branch", vec![
    ///         Pattern::point("leaf"),
    ///     ]),
    /// ]);
    ///
    /// let subpattern = Pattern::point("leaf");
    /// assert!(pattern.contains(&subpattern));
    /// ```
    ///
    /// ## Non-existent subpattern
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern("root", vec![
    ///     Pattern::point("a"),
    /// ]);
    ///
    /// let subpattern = Pattern::point("b");
    /// assert!(!pattern.contains(&subpattern));
    /// ```
    ///
    /// ## Transitivity
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let a = Pattern::pattern("a", vec![
    ///     Pattern::pattern("b", vec![
    ///         Pattern::point("c"),
    ///     ]),
    /// ]);
    /// let b = Pattern::pattern("b", vec![
    ///     Pattern::point("c"),
    /// ]);
    /// let c = Pattern::point("c");
    ///
    /// // If a contains b and b contains c, then a contains c
    /// assert!(a.contains(&b));
    /// assert!(b.contains(&c));
    /// assert!(a.contains(&c));  // Transitive
    /// ```
    ///
    /// ## Contains is weaker than matches
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern("root", vec![
    ///     Pattern::pattern("branch", vec![
    ///         Pattern::point("leaf"),
    ///     ]),
    /// ]);
    /// let subpattern = Pattern::point("leaf");
    ///
    /// // pattern contains subpattern, but they don't match
    /// assert!(pattern.contains(&subpattern));
    /// assert!(!pattern.matches(&subpattern));
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic. All inputs are valid:
    /// - Works with atomic patterns
    /// - Works with patterns with empty elements
    /// - Works with deeply nested structures (limited only by stack size)
    /// - Handles multiple occurrences correctly
    ///
    /// # Relationship to Other Methods
    ///
    /// * **matches**: `p.matches(&q)` implies `p.contains(&q)`, but not vice versa
    /// * **Short-circuit**: Returns `true` as soon as a match is found
    pub fn contains(&self, subpattern: &Pattern<V>) -> bool
    where
        V: PartialEq,
    {
        // Check if this pattern matches the subpattern
        if self.matches(subpattern) {
            return true;
        }

        // Recursively check if any element contains the subpattern
        self.elements
            .iter()
            .any(|element| element.contains(subpattern))
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

    // ====================================================================================
    // Traversable Operations
    // ====================================================================================

    /// Applies an effectful function returning `Option` to all values in the pattern.
    ///
    /// Traverses the pattern in depth-first, root-first order (pre-order traversal).
    /// If any transformation returns `None`, the entire operation returns `None`.
    /// If all transformations return `Some`, returns `Some(Pattern<W>)` with transformed values.
    ///
    /// This implements the Traversable pattern for Option, providing:
    /// - Structure preservation: Output pattern has same shape as input
    /// - Effect sequencing: Short-circuits on first None
    /// - All-or-nothing semantics: All values must be Some for success
    ///
    /// # Type Parameters
    ///
    /// * `W` - The type of transformed values
    /// * `F` - The transformation function type
    ///
    /// # Arguments
    ///
    /// * `f` - A function that transforms values of type `&V` to `Option<W>`
    ///
    /// # Returns
    ///
    /// * `Some(Pattern<W>)` if all transformations succeed
    /// * `None` if any transformation returns None (short-circuit)
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// // Successful traversal - all values parse
    /// let pattern = Pattern::pattern("1", vec![Pattern::point("2")]);
    /// let result = pattern.traverse_option(|s| s.parse::<i32>().ok());
    /// assert!(result.is_some());
    /// assert_eq!(result.unwrap().value, 1);
    ///
    /// // Failed traversal - one value doesn't parse
    /// let pattern = Pattern::pattern("1", vec![Pattern::point("invalid")]);
    /// let result = pattern.traverse_option(|s| s.parse::<i32>().ok());
    /// assert!(result.is_none());
    /// ```
    ///
    /// # Traversable Laws
    ///
    /// This implementation satisfies the traversable laws:
    /// - Identity: `pattern.traverse_option(|v| Some(*v)) == Some(pattern.clone())`
    /// - Structure preservation: If successful, output has same size, depth, and length
    ///
    /// # Performance
    ///
    /// - Time: O(n) where n is the number of nodes
    /// - Space: O(n) for the new pattern + O(d) stack for recursion depth d
    /// - Short-circuits on first None without processing remaining values
    pub fn traverse_option<W, F>(&self, f: F) -> Option<Pattern<W>>
    where
        F: Fn(&V) -> Option<W>,
    {
        self.traverse_option_with(&f)
    }

    /// Internal helper for `traverse_option` that takes function by reference.
    ///
    /// This enables efficient recursion without cloning the closure.
    /// Public `traverse_option` passes closure by value for ergonomics,
    /// internal `traverse_option_with` passes closure by reference for efficiency.
    fn traverse_option_with<W, F>(&self, f: &F) -> Option<Pattern<W>>
    where
        F: Fn(&V) -> Option<W>,
    {
        // Transform root value first (short-circuits on None via ?)
        let new_value = f(&self.value)?;

        // Transform elements recursively (left to right)
        // Iterator::collect() handles Option sequencing - stops on first None
        let new_elements: Option<Vec<Pattern<W>>> = self
            .elements
            .iter()
            .map(|elem| elem.traverse_option_with(f))
            .collect();

        // Construct new pattern with transformed value and elements
        Some(Pattern {
            value: new_value,
            elements: new_elements?,
        })
    }

    /// Applies an effectful function returning `Result` to all values in the pattern.
    ///
    /// Traverses the pattern in depth-first, root-first order (pre-order traversal).
    /// If any transformation returns `Err`, the entire operation returns `Err` (short-circuits).
    /// If all transformations return `Ok`, returns `Ok(Pattern<W>)` with transformed values.
    ///
    /// This implements the Traversable pattern for Result, providing:
    /// - Structure preservation: Output pattern has same shape as input
    /// - Effect sequencing: Short-circuits on first Err
    /// - All-or-nothing semantics: All values must be Ok for success
    /// - Error propagation: First error encountered is returned
    ///
    /// # Type Parameters
    ///
    /// * `W` - The type of transformed values
    /// * `E` - The error type
    /// * `F` - The transformation function type
    ///
    /// # Arguments
    ///
    /// * `f` - A function that transforms values of type `&V` to `Result<W, E>`
    ///
    /// # Returns
    ///
    /// * `Ok(Pattern<W>)` if all transformations succeed
    /// * `Err(E)` with the first error encountered (short-circuit)
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// // Successful traversal - all values parse
    /// let pattern = Pattern::pattern("1", vec![Pattern::point("2")]);
    /// let result: Result<Pattern<i32>, String> = pattern.traverse_result(|s| {
    ///     s.parse::<i32>().map_err(|e| format!("parse error: {}", e))
    /// });
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap().value, 1);
    ///
    /// // Failed traversal - one value doesn't parse
    /// let pattern = Pattern::pattern("1", vec![Pattern::point("invalid")]);
    /// let result: Result<Pattern<i32>, String> = pattern.traverse_result(|s| {
    ///     s.parse::<i32>().map_err(|e| format!("parse error: {}", e))
    /// });
    /// assert!(result.is_err());
    /// ```
    ///
    /// # Traversable Laws
    ///
    /// This implementation satisfies the traversable laws:
    /// - Identity: `pattern.traverse_result(|v| Ok(*v)) == Ok(pattern.clone())`
    /// - Structure preservation: If successful, output has same size, depth, and length
    ///
    /// # Short-Circuiting
    ///
    /// The traversal stops immediately when the first error is encountered:
    /// - Root value is checked first
    /// - Elements are processed left-to-right
    /// - Nested patterns are traversed depth-first
    /// - No further values are processed after an error
    ///
    /// # Performance
    ///
    /// - Time: O(n) where n is the number of nodes (best case: O(1) if root errors)
    /// - Space: O(n) for the new pattern + O(d) stack for recursion depth d
    /// - Short-circuits on first Err without processing remaining values
    pub fn traverse_result<W, E, F>(&self, f: F) -> Result<Pattern<W>, E>
    where
        F: Fn(&V) -> Result<W, E>,
    {
        self.traverse_result_with(&f)
    }

    /// Internal helper for `traverse_result` that takes function by reference.
    ///
    /// This enables efficient recursion without cloning the closure.
    /// Public `traverse_result` passes closure by value for ergonomics,
    /// internal `traverse_result_with` passes closure by reference for efficiency.
    fn traverse_result_with<W, E, F>(&self, f: &F) -> Result<Pattern<W>, E>
    where
        F: Fn(&V) -> Result<W, E>,
    {
        // Transform root value first (short-circuits on Err via ?)
        let new_value = f(&self.value)?;

        // Transform elements recursively (left to right)
        // Iterator::collect() handles Result sequencing - stops on first Err
        let new_elements: Result<Vec<Pattern<W>>, E> = self
            .elements
            .iter()
            .map(|elem| elem.traverse_result_with(f))
            .collect();

        // Construct new pattern with transformed value and elements
        Ok(Pattern {
            value: new_value,
            elements: new_elements?,
        })
    }
}

impl<V: Clone> Pattern<Option<V>> {
    /// Flips the layers of structure from `Pattern<Option<V>>` to `Option<Pattern<V>>`.
    ///
    /// This is the `sequence` operation for Option, which "sequences" or "flips" the
    /// nested structure layers. If all values in the pattern are `Some`, returns
    /// `Some(Pattern<V>)` with the unwrapped values. If any value is `None`, returns `None`.
    ///
    /// This operation is equivalent to `traverse_option` with the identity function,
    /// and demonstrates the relationship: `sequence = traverse(id)`.
    ///
    /// # Type Parameters
    ///
    /// * `V` - The type inside the Option values (must implement Clone)
    ///
    /// # Returns
    ///
    /// * `Some(Pattern<V>)` if all values are Some (unwrapped)
    /// * `None` if any value in the pattern is None
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// // All Some values → Some(Pattern)
    /// let pattern = Pattern::pattern(Some(1), vec![Pattern::point(Some(2))]);
    /// let result = pattern.sequence_option();
    /// assert!(result.is_some());
    /// assert_eq!(result.unwrap().value, 1);
    ///
    /// // Any None → None
    /// let pattern = Pattern::pattern(Some(1), vec![Pattern::point(None)]);
    /// let result = pattern.sequence_option();
    /// assert!(result.is_none());
    /// ```
    ///
    /// # All-or-Nothing Semantics
    ///
    /// - If all values are `Some`, returns `Some` with unwrapped pattern
    /// - If any value is `None`, returns `None` (short-circuits)
    /// - Preserves pattern structure when successful
    ///
    /// # Use Cases
    ///
    /// - Converting `Pattern<Option<V>>` from multiple optional lookups into `Option<Pattern<V>>`
    /// - Validating that all values in a pattern are present
    /// - Implementing all-or-nothing processing for optional data
    ///
    /// # Performance
    ///
    /// - Time: O(n) where n is the number of nodes (best case: O(1) if root is None)
    /// - Space: O(n) for the new pattern + O(d) stack for recursion depth d
    /// - Short-circuits on first None without processing remaining values
    pub fn sequence_option(self) -> Option<Pattern<V>> {
        self.traverse_option(|opt| opt.as_ref().cloned())
    }
}

impl<V, E> Pattern<Result<V, E>> {
    /// Flips the layers of structure from `Pattern<Result<V, E>>` to `Result<Pattern<V>, E>`.
    ///
    /// This is the `sequence` operation for Result, which "sequences" or "flips" the
    /// nested structure layers. If all values in the pattern are `Ok`, returns
    /// `Ok(Pattern<V>)` with the unwrapped values. If any value is `Err`, returns that `Err`.
    ///
    /// This operation is equivalent to `traverse_result` with the identity function,
    /// and demonstrates the relationship: `sequence = traverse(id)`.
    ///
    /// # Type Parameters
    ///
    /// * `V` - The success type inside the Result values
    /// * `E` - The error type
    ///
    /// # Returns
    ///
    /// * `Ok(Pattern<V>)` if all values are Ok (unwrapped)
    /// * `Err(E)` with the first error encountered
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// // All Ok values → Ok(Pattern)
    /// let pattern: Pattern<Result<i32, String>> = Pattern::pattern(
    ///     Ok(1),
    ///     vec![Pattern::point(Ok(2))]
    /// );
    /// let result = pattern.sequence_result();
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap().value, 1);
    ///
    /// // Any Err → Err
    /// let pattern: Pattern<Result<i32, String>> = Pattern::pattern(
    ///     Ok(1),
    ///     vec![Pattern::point(Err("error".to_string()))]
    /// );
    /// let result = pattern.sequence_result();
    /// assert!(result.is_err());
    /// ```
    ///
    /// # All-or-Nothing Semantics
    ///
    /// - If all values are `Ok`, returns `Ok` with unwrapped pattern
    /// - If any value is `Err`, returns first `Err` encountered (short-circuits)
    /// - Preserves pattern structure when successful
    ///
    /// # Use Cases
    ///
    /// - Converting `Pattern<Result<V, E>>` from multiple fallible operations into `Result<Pattern<V>, E>`
    /// - Validating that all operations in a pattern succeeded
    /// - Implementing all-or-nothing processing for fallible operations
    ///
    /// # Performance
    ///
    /// - Time: O(n) where n is the number of nodes (best case: O(1) if root is Err)
    /// - Space: O(n) for the new pattern + O(d) stack for recursion depth d
    /// - Short-circuits on first Err without processing remaining values
    pub fn sequence_result(self) -> Result<Pattern<V>, E>
    where
        V: Clone,
        E: Clone,
    {
        self.traverse_result(|res| res.clone())
    }
}

impl<V> Pattern<V> {
    /// Applies a validation function to all values and collects ALL errors.
    ///
    /// Unlike `traverse_result` which short-circuits on the first error, `validate_all`
    /// processes the entire pattern and collects all errors encountered. This is useful
    /// for comprehensive validation where you want to report all issues at once.
    ///
    /// Traverses in depth-first, root-first order (pre-order). If all validations succeed,
    /// returns `Ok(Pattern<W>)` with transformed values. If any validation fails, returns
    /// `Err(Vec<E>)` with ALL errors collected in traversal order.
    ///
    /// # Type Parameters
    ///
    /// * `W` - The type of transformed values
    /// * `E` - The error type
    /// * `F` - The validation function type
    ///
    /// # Arguments
    ///
    /// * `f` - A function that validates and transforms values of type `&V` to `Result<W, E>`
    ///
    /// # Returns
    ///
    /// * `Ok(Pattern<W>)` if all validations succeed
    /// * `Err(Vec<E>)` with ALL errors collected in traversal order
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// // All validations succeed
    /// let pattern = Pattern::pattern(1, vec![Pattern::point(2), Pattern::point(3)]);
    /// let result: Result<Pattern<i32>, Vec<String>> = pattern.validate_all(|v| {
    ///     if *v > 0 { Ok(*v * 10) } else { Err(format!("negative: {}", v)) }
    /// });
    /// assert!(result.is_ok());
    ///
    /// // Multiple validations fail - ALL errors collected
    /// let pattern = Pattern::pattern(-1, vec![Pattern::point(2), Pattern::point(-3)]);
    /// let result: Result<Pattern<i32>, Vec<String>> = pattern.validate_all(|v| {
    ///     if *v > 0 { Ok(*v * 10) } else { Err(format!("negative: {}", v)) }
    /// });
    /// assert!(result.is_err());
    /// let errors = result.unwrap_err();
    /// assert_eq!(errors.len(), 2); // Both -1 and -3 reported
    /// ```
    ///
    /// # Comparison with traverse_result
    ///
    /// **traverse_result**: Returns first error (short-circuits)
    /// - Use when: You want to fail fast and stop processing
    /// - Performance: O(1) to O(n) depending on error location
    /// - Behavior: Stops at first error
    ///
    /// **validate_all**: Returns ALL errors (no short-circuit)
    /// - Use when: You want comprehensive error reporting
    /// - Performance: Always O(n) - processes entire pattern
    /// - Behavior: Collects all errors, then returns
    ///
    /// # Error Ordering
    ///
    /// Errors are collected in traversal order (root first, then elements left to right).
    /// This provides predictable and consistent error reporting.
    ///
    /// # Performance
    ///
    /// - Time: Always O(n) where n is the number of nodes (no short-circuiting)
    /// - Space: O(n) for the new pattern + O(e) for error collection + O(d) stack depth
    /// - Processes every value regardless of errors
    pub fn validate_all<W, E, F>(&self, f: F) -> Result<Pattern<W>, Vec<E>>
    where
        F: Fn(&V) -> Result<W, E>,
    {
        self.validate_all_with(&f)
    }

    /// Internal helper for `validate_all` that takes function by reference.
    ///
    /// This enables efficient recursion without cloning the closure.
    /// Collects errors during traversal and returns them all at the end.
    ///
    /// Uses a single-pass approach:
    /// 1. Apply function to all values once, collecting both successes and errors
    /// 2. If no errors: Build the transformed pattern from collected successes
    /// 3. If errors: Return all collected errors
    fn validate_all_with<W, E, F>(&self, f: &F) -> Result<Pattern<W>, Vec<E>>
    where
        F: Fn(&V) -> Result<W, E>,
    {
        // Helper to recursively collect all results in pre-order (root first, then elements)
        fn collect_results<V, W, E, F>(
            pattern: &Pattern<V>,
            f: &F,
            successes: &mut Vec<W>,
            errors: &mut Vec<E>,
        ) where
            F: Fn(&V) -> Result<W, E>,
        {
            // Process root value first
            match f(&pattern.value) {
                Ok(w) => successes.push(w),
                Err(e) => errors.push(e),
            }

            // Process elements recursively (left to right)
            for elem in &pattern.elements {
                collect_results(elem, f, successes, errors);
            }
        }

        // Single pass: apply function to all values, collecting results
        let mut successes = Vec::new();
        let mut errors = Vec::new();
        collect_results(self, f, &mut successes, &mut errors);

        // If any errors occurred, return them all
        if !errors.is_empty() {
            return Err(errors);
        }

        // All validations succeeded - rebuild pattern from collected values
        // Values are in pre-order, so we consume them in the same order
        fn rebuild<V, W>(pattern: &Pattern<V>, values: &mut impl Iterator<Item = W>) -> Pattern<W> {
            // Get root value (next in pre-order sequence)
            let value = values
                .next()
                .expect("validate_all: insufficient transformed values");

            // Rebuild elements recursively
            let elements = pattern
                .elements
                .iter()
                .map(|elem| rebuild(elem, values))
                .collect();

            Pattern { value, elements }
        }

        Ok(rebuild(self, &mut successes.into_iter()))
    }
}

// ============================================================================
// Ordering Trait Implementations
// ============================================================================

/// `PartialOrd` implementation for `Pattern`.
///
/// Provides lexicographic ordering for patterns based on their structure.
/// Patterns are compared by their value first, then by their elements recursively.
///
/// This implementation follows the same semantics as the Haskell reference implementation
/// in `gram-hs/libs/pattern/src/Pattern/Core.hs`.
///
/// # Ordering Rules
///
/// 1. **Value-first comparison**: Compare pattern values first using the value type's `PartialOrd` instance
/// 2. **Element comparison**: If values are equal, compare element vectors lexicographically
/// 3. **Lexicographic elements**: Elements are compared left-to-right, stopping at first difference
/// 4. **Length comparison**: If all compared elements are equal, shorter < longer
///
/// # Examples
///
/// ```
/// use pattern_core::Pattern;
///
/// // Comparing atomic patterns
/// let p1 = Pattern::point(1);
/// let p2 = Pattern::point(2);
/// assert!(p1 < p2);
///
/// // Comparing patterns with same value but different elements
/// let p3 = Pattern::pattern(5, vec![Pattern::point(1)]);
/// let p4 = Pattern::pattern(5, vec![Pattern::point(2)]);
/// assert!(p3 < p4); // Values equal, first element 1 < 2
///
/// // Value takes precedence over elements
/// let p5 = Pattern::pattern(3, vec![Pattern::point(100)]);
/// let p6 = Pattern::pattern(4, vec![Pattern::point(1)]);
/// assert!(p5 < p6); // 3 < 4, elements not compared
/// ```
///
/// # Comparison with Haskell
///
/// This implementation is behaviorally equivalent to the Haskell `Ord` instance:
///
/// ```haskell
/// instance Ord v => Ord (Pattern v) where
///   compare (Pattern v1 es1) (Pattern v2 es2) =
///     case compare v1 v2 of
///       EQ -> compare es1 es2
///       other -> other
/// ```
impl<V: PartialOrd> PartialOrd for Pattern<V> {
    /// Compares two patterns, returning `Some(ordering)` if comparable, `None` otherwise.
    ///
    /// Uses value-first lexicographic comparison:
    /// 1. Compare pattern values using `V::partial_cmp`
    /// 2. If equal (or both None), compare element vectors lexicographically
    /// 3. If values differ, return that ordering
    ///
    /// # Returns
    ///
    /// - `Some(Ordering::Less)` if `self < other`
    /// - `Some(Ordering::Equal)` if `self == other`
    /// - `Some(Ordering::Greater)` if `self > other`
    /// - `None` if values cannot be compared (e.g., NaN in floats)
    ///
    /// # Performance
    ///
    /// - **Best case**: O(1) - Values differ, immediate return
    /// - **Average case**: O(log n) - Finds difference early in elements
    /// - **Worst case**: O(n) - Must compare all nodes where n = total nodes
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.value.partial_cmp(&other.value) {
            Some(Ordering::Equal) => self.elements.partial_cmp(&other.elements),
            other => other,
        }
    }
}

/// `Ord` implementation for `Pattern`.
///
/// Provides total ordering for patterns where the value type implements `Ord`.
/// This enables patterns to be used as keys in ordered data structures like
/// `BTreeMap`, `BTreeSet`, and `BinaryHeap`.
///
/// # Ordering Rules
///
/// Same as `PartialOrd`:
/// 1. Compare values first
/// 2. If equal, compare elements lexicographically
///
/// # Properties
///
/// This implementation satisfies all `Ord` trait requirements:
///
/// - **Reflexivity**: `x.cmp(&x) == Ordering::Equal` for all x
/// - **Antisymmetry**: if `x.cmp(&y) == Less` then `y.cmp(&x) == Greater`
/// - **Transitivity**: if `x < y` and `y < z` then `x < z`
/// - **Totality**: For all x, y, exactly one of `x < y`, `x == y`, `x > y` holds
/// - **Consistency with Eq**: `x == y` implies `x.cmp(&y) == Equal`
///
/// These properties are verified through property-based tests.
///
/// # Examples
///
/// ```
/// use pattern_core::Pattern;
/// use std::cmp::Ordering;
///
/// let p1 = Pattern::point(1);
/// let p2 = Pattern::point(2);
///
/// // Using cmp directly
/// assert_eq!(p1.cmp(&p2), Ordering::Less);
///
/// // Using comparison operators
/// assert!(p1 < p2);
/// assert!(p1 <= p2);
/// assert!(p2 > p1);
/// assert!(p2 >= p1);
///
/// // Sorting collections
/// let mut patterns = vec![p2.clone(), p1.clone()];
/// patterns.sort();
/// assert_eq!(patterns, vec![p1, p2]);
/// ```
///
/// # Usage in Data Structures
///
/// ```
/// use pattern_core::Pattern;
/// use std::collections::{BTreeMap, BTreeSet};
///
/// // As BTreeSet elements
/// let mut set = BTreeSet::new();
/// set.insert(Pattern::point(3));
/// set.insert(Pattern::point(1));
/// set.insert(Pattern::point(2));
/// // Iteration in sorted order: 1, 2, 3
///
/// // As BTreeMap keys
/// let mut map = BTreeMap::new();
/// map.insert(Pattern::point(1), "first");
/// map.insert(Pattern::point(2), "second");
/// ```
impl<V: Ord> Ord for Pattern<V> {
    /// Compares two patterns, returning their ordering.
    ///
    /// Uses value-first lexicographic comparison:
    /// 1. Compare pattern values using `V::cmp`
    /// 2. If equal, compare element vectors lexicographically
    /// 3. If values differ, return that ordering
    ///
    /// This method always returns a definitive ordering (never None).
    ///
    /// # Performance
    ///
    /// - **Best case**: O(1) - Values differ, immediate return
    /// - **Average case**: O(log n) - Finds difference early in elements
    /// - **Worst case**: O(n) - Must compare all nodes where n = total nodes
    ///
    /// # Short-Circuit Optimization
    ///
    /// Comparison stops as soon as a difference is found:
    /// - If values differ, elements are never compared
    /// - If elements differ, remaining elements are not compared
    ///
    /// This provides efficient comparison even for large patterns.
    fn cmp(&self, other: &Self) -> Ordering {
        match self.value.cmp(&other.value) {
            Ordering::Equal => self.elements.cmp(&other.elements),
            non_equal => non_equal,
        }
    }
}

// ============================================================================
// Pattern Combination
// ============================================================================

impl<V: crate::Combinable> Pattern<V> {
    /// Combines two patterns associatively.
    ///
    /// Creates a new pattern by:
    /// 1. Combining the values using `V::combine`
    /// 2. Concatenating the element vectors (left first, then right)
    ///
    /// The operation is associative: `(a.combine(b)).combine(c)` equals `a.combine(b.combine(c))`.
    ///
    /// # Parameters
    ///
    /// * `self` - The first pattern (consumed)
    /// * `other` - The second pattern to combine with (consumed)
    ///
    /// # Returns
    ///
    /// A new `Pattern<V>` with:
    /// * `value`: Result of `self.value.combine(other.value)`
    /// * `elements`: Concatenation of `self.elements` and `other.elements`
    ///
    /// # Examples
    ///
    /// ## Atomic Patterns
    ///
    /// ```rust
    /// use pattern_core::Pattern;
    ///
    /// let p1 = Pattern::point("hello".to_string());
    /// let p2 = Pattern::point(" world".to_string());
    /// let result = p1.combine(p2);
    ///
    /// assert_eq!(result.value(), "hello world");
    /// assert_eq!(result.length(), 0);  // No elements
    /// ```
    ///
    /// ## Patterns with Elements
    ///
    /// ```rust
    /// use pattern_core::Pattern;
    ///
    /// let p1 = Pattern::pattern("a".to_string(), vec![
    ///     Pattern::point("b".to_string()),
    ///     Pattern::point("c".to_string()),
    /// ]);
    ///
    /// let p2 = Pattern::pattern("d".to_string(), vec![
    ///     Pattern::point("e".to_string()),
    /// ]);
    ///
    /// let result = p1.combine(p2);
    ///
    /// assert_eq!(result.value(), "ad");
    /// assert_eq!(result.length(), 3);  // [b, c, e]
    /// ```
    ///
    /// ## Associativity
    ///
    /// ```rust
    /// use pattern_core::Pattern;
    ///
    /// let a = Pattern::point("a".to_string());
    /// let b = Pattern::point("b".to_string());
    /// let c = Pattern::point("c".to_string());
    ///
    /// let left = a.clone().combine(b.clone()).combine(c.clone());
    /// let right = a.combine(b.combine(c));
    ///
    /// assert_eq!(left, right);  // Associativity holds
    /// ```
    ///
    /// # Performance
    ///
    /// * **Time Complexity**: O(|elements1| + |elements2| + value_combine_cost)
    /// * **Space Complexity**: O(|elements1| + |elements2|)
    ///
    /// Element concatenation uses `Vec::extend` for efficiency.
    ///
    /// **Benchmark Results** (on typical hardware):
    /// * Atomic patterns: ~100 ns
    /// * 100 elements: ~11 µs
    /// * 1000 elements: ~119 µs
    /// * 100-pattern fold: ~17 µs
    ///
    /// All operations complete in microseconds, making combination suitable
    /// for performance-critical applications.
    pub fn combine(self, other: Self) -> Self {
        // Step 1: Combine values using V's Combinable implementation
        let combined_value = self.value.combine(other.value);

        // Step 2: Concatenate elements (left first, then right)
        let mut combined_elements = self.elements;
        combined_elements.extend(other.elements);

        // Step 3: Return new pattern
        Pattern {
            value: combined_value,
            elements: combined_elements,
        }
    }

    /// Creates patterns by combining three lists pointwise (zipWith3).
    ///
    /// Takes three lists of equal length and combines them element-wise to create
    /// new patterns. Each resulting pattern has:
    /// - **value**: From the `values` list
    /// - **elements**: A pair `[left, right]` from the corresponding positions
    ///
    /// This is useful for creating relationship patterns from separate lists of
    /// source nodes, target nodes, and relationship values.
    ///
    /// # Arguments
    ///
    /// * `left` - First list of patterns (e.g., source nodes)
    /// * `right` - Second list of patterns (e.g., target nodes)
    /// * `values` - List of values for the new patterns (e.g., relationship types)
    ///
    /// # Returns
    ///
    /// A vector of patterns where each pattern has value from `values` and
    /// elements `[left[i], right[i]]`.
    ///
    /// # Behavior
    ///
    /// - Stops at the length of the shortest input list
    /// - Consumes all three input vectors
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// // Create relationship patterns
    /// let sources = vec![
    ///     Pattern::point("Alice".to_string()),
    ///     Pattern::point("Bob".to_string()),
    /// ];
    /// let targets = vec![
    ///     Pattern::point("Company".to_string()),
    ///     Pattern::point("Project".to_string()),
    /// ];
    /// let rel_types = vec!["WORKS_FOR".to_string(), "MANAGES".to_string()];
    ///
    /// let relationships = Pattern::zip3(sources, targets, rel_types);
    ///
    /// assert_eq!(relationships.len(), 2);
    /// assert_eq!(relationships[0].value, "WORKS_FOR");
    /// assert_eq!(relationships[0].elements.len(), 2);
    /// ```
    pub fn zip3(left: Vec<Pattern<V>>, right: Vec<Pattern<V>>, values: Vec<V>) -> Vec<Pattern<V>> {
        left.into_iter()
            .zip(right)
            .zip(values)
            .map(|((l, r), v)| Pattern::pattern(v, vec![l, r]))
            .collect()
    }

    /// Creates patterns by applying a function to pairs from two lists (zipWith2).
    ///
    /// Takes two lists of patterns and applies a function to each pair to compute
    /// the value for the resulting pattern. Each resulting pattern has:
    /// - **value**: Computed by applying `value_fn` to the pair
    /// - **elements**: A pair `[left, right]` from the corresponding positions
    ///
    /// This is useful when relationship values are derived from the patterns being
    /// connected, rather than from a pre-computed list.
    ///
    /// # Arguments
    ///
    /// * `left` - First list of patterns (e.g., source nodes)
    /// * `right` - Second list of patterns (e.g., target nodes)
    /// * `value_fn` - Function that computes the value from each pair
    ///
    /// # Returns
    ///
    /// A vector of patterns where each pattern has value computed by `value_fn`
    /// and elements `[left[i], right[i]]`.
    ///
    /// # Behavior
    ///
    /// - Stops at the length of the shortest input list
    /// - Borrows patterns (uses references) to allow inspection without consuming
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let people = vec![
    ///     Pattern::point("Alice".to_string()),
    ///     Pattern::point("Bob".to_string()),
    /// ];
    /// let companies = vec![
    ///     Pattern::point("TechCorp".to_string()),
    ///     Pattern::point("StartupInc".to_string()),
    /// ];
    ///
    /// // Derive relationship type from patterns
    /// let relationships = Pattern::zip_with(people, companies, |person, company| {
    ///     format!("{}_WORKS_AT_{}", person.value, company.value)
    /// });
    ///
    /// assert_eq!(relationships[0].value, "Alice_WORKS_AT_TechCorp");
    /// ```
    pub fn zip_with<F>(
        left: Vec<Pattern<V>>,
        right: Vec<Pattern<V>>,
        value_fn: F,
    ) -> Vec<Pattern<V>>
    where
        F: Fn(&Pattern<V>, &Pattern<V>) -> V,
    {
        left.into_iter()
            .zip(right)
            .map(|(l, r)| {
                let value = value_fn(&l, &r);
                Pattern::pattern(value, vec![l, r])
            })
            .collect()
    }
}

// ============================================================================
// Default Trait Implementation - Identity Element for Monoid
// ============================================================================

/// Provides a default (identity) pattern for value types that implement `Default`.
///
/// The default pattern serves as the identity element for pattern combination,
/// completing the monoid algebraic structure (associative operation + identity).
/// The default pattern has the default value for type `V` and an empty elements list.
///
/// # Monoid Laws
///
/// When combined with the [`Combinable`] trait, patterns form a complete monoid
/// satisfying these identity laws:
///
/// - **Left Identity**: `Pattern::default().combine(p) == p` for all patterns `p`
/// - **Right Identity**: `p.combine(Pattern::default()) == p` for all patterns `p`
///
/// These laws ensure that the default pattern acts as a neutral element: combining
/// any pattern with the default pattern (on either side) yields the original pattern
/// unchanged.
///
/// # Implementation
///
/// The default pattern is created using [`Pattern::point`] with the default value
/// for type `V`. This results in:
/// ```text
/// Pattern {
///     value: V::default(),
///     elements: vec![]
/// }
/// ```
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use pattern_core::Pattern;
///
/// // Create default pattern for String
/// let empty: Pattern<String> = Pattern::default();
/// assert_eq!(empty.value(), "");
/// assert_eq!(empty.length(), 0);
///
/// // Create default pattern for Vec<i32>
/// let empty: Pattern<Vec<i32>> = Pattern::default();
/// let expected: Vec<i32> = vec![];
/// assert_eq!(empty.value(), &expected);
/// assert_eq!(empty.length(), 0);
///
/// // Create default pattern for unit type
/// let empty: Pattern<()> = Pattern::default();
/// assert_eq!(empty.value(), &());
/// assert_eq!(empty.length(), 0);
/// ```
///
/// ## Identity Laws
///
/// ```rust
/// use pattern_core::{Pattern, Combinable};
///
/// let p = Pattern::point("hello".to_string());
/// let empty = Pattern::<String>::default();
///
/// // Left identity: empty.combine(p) == p
/// assert_eq!(empty.clone().combine(p.clone()), p);
///
/// // Right identity: p.combine(empty) == p
/// assert_eq!(p.clone().combine(empty.clone()), p);
/// ```
///
/// ## Usage with Iterators
///
/// ```rust
/// use pattern_core::{Pattern, Combinable};
///
/// let patterns = vec![
///     Pattern::point("hello".to_string()),
///     Pattern::point(" ".to_string()),
///     Pattern::point("world".to_string()),
/// ];
///
/// // Fold with default as initial value
/// let result = patterns.into_iter()
///     .fold(Pattern::default(), |acc, p| acc.combine(p));
///
/// assert_eq!(result.value(), "hello world");
/// ```
///
/// ## Handling Empty Collections
///
/// ```rust
/// use pattern_core::{Pattern, Combinable};
///
/// let empty_vec: Vec<Pattern<String>> = vec![];
///
/// // Folding empty collection returns default
/// let result = empty_vec.into_iter()
///     .fold(Pattern::default(), |acc, p| acc.combine(p));
///
/// assert_eq!(result, Pattern::default());
/// ```
///
/// # See Also
///
/// - [`Pattern::point`] - Used internally to create the default pattern
/// - [`Pattern::combine`] - The associative combination operation
/// - [`Combinable`] - Trait for types supporting associative combination
///
/// [`Combinable`]: crate::Combinable
impl<V> Default for Pattern<V>
where
    V: Default,
{
    /// Creates a default pattern with the default value and empty elements.
    ///
    /// This is the identity element for pattern combination operations.
    ///
    /// # Returns
    ///
    /// A pattern with `V::default()` as the value and an empty elements vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pattern_core::Pattern;
    ///
    /// let empty: Pattern<String> = Pattern::default();
    /// assert_eq!(empty.value(), "");
    /// assert_eq!(empty.length(), 0);
    /// assert!(empty.is_atomic());
    /// ```
    fn default() -> Self {
        Pattern::point(V::default())
    }
}

// ============================================================================
// Hash Trait Implementation
// ============================================================================

/// Provides hashing support for patterns where the value type implements `Hash`.
///
/// This enables patterns to be used as keys in `HashMap` and elements in `HashSet`,
/// enabling efficient pattern deduplication, caching, and set-based operations.
///
/// # Hash/Eq Consistency
///
/// This implementation guarantees that equal patterns produce equal hashes:
/// - If `p1 == p2`, then `hash(p1) == hash(p2)`
/// - This consistency is required for correct HashMap/HashSet behavior
///
/// # Structure-Preserving Hashing
///
/// The hash incorporates both the value and the element structure recursively:
/// - Different patterns with the same values produce different hashes
/// - The nesting structure and element order affect the hash
/// - Atomic patterns hash differently from compound patterns
///
/// # Implementation
///
/// The implementation hashes both components of a pattern:
/// 1. Hash the value using `V::hash`
/// 2. Hash the elements vector (which recursively hashes nested patterns)
///
/// This approach leverages `Vec<T>`'s built-in `Hash` implementation, which
/// automatically handles recursive hashing of nested patterns correctly.
///
/// # Type Constraints
///
/// Only patterns where `V: Hash` can be hashed. This means:
/// - ✅ `Pattern<String>` is hashable (String implements Hash)
/// - ✅ `Pattern<Symbol>` is hashable (Symbol implements Hash)
/// - ✅ `Pattern<i32>` is hashable (integers implement Hash)
/// - ❌ `Pattern<Subject>` is NOT hashable (Subject contains f64)
/// - ❌ `Pattern<f64>` is NOT hashable (floats don't implement Hash)
///
/// This is correct behavior - the type system prevents hashing types that
/// shouldn't be hashed due to problematic equality semantics (e.g., NaN != NaN for floats).
///
/// # Examples
///
/// ## Using Patterns in HashSet (Deduplication)
///
/// ```rust
/// use pattern_core::Pattern;
/// use std::collections::HashSet;
///
/// let p1 = Pattern::point("hello".to_string());
/// let p2 = Pattern::point("world".to_string());
/// let p3 = Pattern::point("hello".to_string());  // Duplicate of p1
///
/// let mut set = HashSet::new();
/// set.insert(p1);
/// set.insert(p2);
/// set.insert(p3);  // Automatically deduplicated
///
/// assert_eq!(set.len(), 2);  // Only unique patterns
/// ```
///
/// ## Using Patterns as HashMap Keys (Caching)
///
/// ```rust
/// use pattern_core::Pattern;
/// use std::collections::HashMap;
///
/// let mut cache: HashMap<Pattern<String>, i32> = HashMap::new();
///
/// let p1 = Pattern::point("key1".to_string());
/// let p2 = Pattern::point("key2".to_string());
///
/// cache.insert(p1.clone(), 42);
/// cache.insert(p2.clone(), 100);
///
/// assert_eq!(cache.get(&p1), Some(&42));
/// assert_eq!(cache.get(&p2), Some(&100));
/// ```
///
/// ## Hash Consistency with Equality
///
/// ```rust
/// use pattern_core::Pattern;
/// use std::collections::hash_map::DefaultHasher;
/// use std::hash::{Hash, Hasher};
///
/// fn hash_pattern<V: Hash>(p: &Pattern<V>) -> u64 {
///     let mut hasher = DefaultHasher::new();
///     p.hash(&mut hasher);
///     hasher.finish()
/// }
///
/// let p1 = Pattern::point("test".to_string());
/// let p2 = Pattern::point("test".to_string());
///
/// // Equal patterns have equal hashes
/// assert_eq!(p1, p2);
/// assert_eq!(hash_pattern(&p1), hash_pattern(&p2));
/// ```
///
/// ## Structure Distinguishes Hashes
///
/// ```rust
/// use pattern_core::Pattern;
/// use std::collections::hash_map::DefaultHasher;
/// use std::hash::{Hash, Hasher};
///
/// fn hash_pattern<V: Hash>(p: &Pattern<V>) -> u64 {
///     let mut hasher = DefaultHasher::new();
///     p.hash(&mut hasher);
///     hasher.finish()
/// }
///
/// // Same values, different structures
/// let atomic = Pattern::point("value".to_string());
/// let compound = Pattern::pattern(
///     "value".to_string(),
///     vec![Pattern::point("child".to_string())]
/// );
///
/// // Different structures produce different hashes
/// assert_ne!(atomic, compound);
/// // Note: Hash inequality is not guaranteed but expected
/// // (hash collisions are possible but rare)
/// ```
///
/// # Performance
///
/// - **Time Complexity**: O(n) where n is the total number of nodes in the pattern
/// - **Space Complexity**: O(1) (hash computation uses constant space)
/// - Hashing is typically very fast (microseconds even for large patterns)
/// - Results are cached in HashMap/HashSet (computed once per pattern)
///
/// # Comparison with Haskell
///
/// This implementation is behaviorally equivalent to the Haskell `Hashable` instance:
///
/// ```haskell
/// instance Hashable v => Hashable (Pattern v) where
///   hashWithSalt salt (Pattern v es) =
///     salt `hashWithSalt` v `hashWithSalt` es
/// ```
///
/// Both implementations hash the value and elements in the same order, ensuring
/// equivalent hash values for equivalent patterns.
///
/// # See Also
///
/// - `HashMap` - For using patterns as keys in hash-based maps
/// - `HashSet` - For pattern deduplication and set operations
/// - [`Pattern::combine`] - Pattern combination (works well with cached patterns)
/// - [`Eq`] - Equality trait that Hash must be consistent with
impl<V: Hash> Hash for Pattern<V> {
    /// Hashes this pattern into the provided hasher.
    ///
    /// Computes the hash by:
    /// 1. Hashing the value component
    /// 2. Hashing the elements vector (recursively hashes nested patterns)
    ///
    /// This ensures that equal patterns produce equal hashes while different
    /// structures produce different hashes.
    ///
    /// # Parameters
    ///
    /// * `state` - The hasher to write the hash into
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pattern_core::Pattern;
    /// use std::collections::hash_map::DefaultHasher;
    /// use std::hash::{Hash, Hasher};
    ///
    /// let pattern = Pattern::point("test".to_string());
    ///
    /// let mut hasher = DefaultHasher::new();
    /// pattern.hash(&mut hasher);
    /// let hash_value = hasher.finish();
    /// ```
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.elements.hash(state);
    }
}

// ============================================================================
// Comonad Operations
// ============================================================================

pub mod comonad;
pub mod comonad_helpers;
