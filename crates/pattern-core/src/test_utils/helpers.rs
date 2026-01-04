//! Test helper utilities for pattern comparison and validation

use std::fmt::Debug;

/// Error type for pattern comparison failures
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternComparisonError {
    pub message: String,
    pub differences: Vec<Difference>,
    pub path: Vec<String>,
}

/// A single difference found during pattern comparison
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Difference {
    pub field: String,
    pub expected: String,
    pub actual: String,
}

/// Options for pattern comparison
#[derive(Debug, Clone)]
pub struct PatternComparisonOptions {
    pub deep: bool,
    pub ignore_fields: Vec<String>,
    pub approximate_equality: bool,
}

impl Default for PatternComparisonOptions {
    fn default() -> Self {
        Self {
            deep: true,
            ignore_fields: Vec::new(),
            approximate_equality: false,
        }
    }
}

/// Rules for pattern structure validation
#[derive(Debug, Clone, Default)]
pub struct ValidationRules {
    pub max_depth: Option<usize>,
    pub max_elements: Option<usize>,
    pub required_fields: Vec<String>,
}

/// Error type for pattern validation failures
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub message: String,
    pub rule_violated: String,
    pub location: Vec<String>,
}

/// Compare two patterns for equality with detailed error messages
///
/// # Arguments
///
/// * `actual` - Actual pattern value
/// * `expected` - Expected pattern value
/// * `msg` - Error message prefix if comparison fails
///
/// # Returns
///
/// `Result<(), PatternComparisonError>` - Ok if patterns are equal, Err with details if not
pub fn assert_patterns_equal<V>(
    _actual: &V,
    _expected: &V,
    _msg: &str,
) -> Result<(), PatternComparisonError>
where
    V: PartialEq + Debug,
{
    // Placeholder implementation
    // Will be fully implemented when Pattern types are defined in feature 004
    Ok(())
}

/// Validate that a pattern has valid structure
///
/// # Arguments
///
/// * `pattern` - Pattern to validate
/// * `rules` - Validation rules to apply
///
/// # Returns
///
/// `Result<(), ValidationError>` - Ok if pattern is valid, Err with details if not
pub fn assert_pattern_structure_valid<V>(
    _pattern: &V,
    _rules: &ValidationRules,
) -> Result<(), ValidationError>
where
    V: Debug,
{
    // Placeholder implementation
    // Will be fully implemented when Pattern types are defined in feature 004
    Ok(())
}

/// Compare patterns with equivalence checking options
///
/// # Arguments
///
/// * `pattern_a` - First pattern
/// * `pattern_b` - Second pattern
/// * `options` - Comparison options
///
/// # Returns
///
/// `Result<(), PatternComparisonError>` - Ok if patterns are equivalent, Err with details if not
pub fn assert_patterns_equivalent<V>(
    _pattern_a: &V,
    _pattern_b: &V,
    _options: &PatternComparisonOptions,
) -> Result<(), PatternComparisonError>
where
    V: PartialEq + Debug,
{
    // Placeholder implementation
    // Will be fully implemented when Pattern types are defined in feature 004
    Ok(())
}

// ====================================================================================
// Effect Counting Utilities (for traversable short-circuit verification)
// ====================================================================================

use std::sync::atomic::{AtomicUsize, Ordering};

/// Counter for tracking side effects during traversal
///
/// Used to verify short-circuit behavior: if traversal short-circuits on error,
/// the counter should show that not all values were processed.
#[derive(Debug)]
pub struct EffectCounter {
    count: AtomicUsize,
}

impl EffectCounter {
    /// Create a new effect counter starting at 0
    pub fn new() -> Self {
        Self {
            count: AtomicUsize::new(0),
        }
    }

    /// Increment the counter (called each time effectful function is invoked)
    pub fn increment(&self) {
        self.count.fetch_add(1, Ordering::SeqCst);
    }

    /// Get the current count
    pub fn count(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }

    /// Reset the counter to 0
    pub fn reset(&self) {
        self.count.store(0, Ordering::SeqCst);
    }
}

impl Default for EffectCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for EffectCounter {
    fn clone(&self) -> Self {
        Self {
            count: AtomicUsize::new(self.count()),
        }
    }
}

/// Helper function to create a counting effectful function for testing
///
/// Returns a closure that increments the counter each time it's called,
/// then applies the provided function.
///
/// # Example
///
/// ```ignore
/// let counter = EffectCounter::new();
/// let counting_fn = counting_effect(&counter, |x: &i32| {
///     if *x > 0 { Ok(*x) } else { Err("negative") }
/// });
///
/// // Use counting_fn in traverse
/// let result = pattern.traverse_result(counting_fn);
///
/// // Check how many times the function was called
/// assert_eq!(counter.count(), expected_count);
/// ```
pub fn counting_effect<'a, V, W, E, F>(
    counter: &'a EffectCounter,
    f: F,
) -> impl Fn(&V) -> Result<W, E> + 'a
where
    F: Fn(&V) -> Result<W, E> + 'a,
    V: 'a,
{
    move |v| {
        counter.increment();
        f(v)
    }
}
