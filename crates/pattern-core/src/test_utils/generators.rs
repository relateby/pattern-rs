//! Property-based test generators for patterns
//!
//! This module provides property-based test generators for patterns using proptest.
//! Generators produce patterns with various structures (atomic, nested, deep, wide)
//! suitable for property testing.

#[cfg(test)]
use proptest::prelude::*;

#[cfg(test)]
use crate::Pattern;

/// Generate an arbitrary pattern with integer values
///
/// Generates patterns with:
/// - Values between -100 and 100
/// - Up to 10 elements per level
/// - Up to 5 levels of nesting
#[cfg(test)]
pub fn arbitrary_pattern_i32() -> impl Strategy<Value = Pattern<i32>> {
    let leaf = any::<i32>().prop_map(Pattern::point);
    
    leaf.prop_recursive(
        5,   // max_depth
        256, // desired_size
        10,  // max_elements per level
        |inner| {
            (any::<i32>(), prop::collection::vec(inner, 0..10))
                .prop_map(|(value, elements)| Pattern::pattern(value, elements))
        },
    )
}

/// Generate an arbitrary pattern with string values
///
/// Generates patterns with:
/// - String values (alphanumeric)
/// - Up to 10 elements per level
/// - Up to 5 levels of nesting
#[cfg(test)]
pub fn arbitrary_pattern_string() -> impl Strategy<Value = Pattern<String>> {
    let leaf = "[a-zA-Z0-9]{1,10}".prop_map(|s: String| Pattern::point(s));
    
    leaf.prop_recursive(
        5,   // max_depth
        256, // desired_size
        10,  // max_elements per level
        |inner| {
            ("[a-zA-Z0-9]{1,10}", prop::collection::vec(inner, 0..10))
                .prop_map(|(value, elements)| Pattern::pattern(value, elements))
        },
    )
}

/// Generate a pattern with Option values (for sequence testing)
///
/// Some values will be Some, others None, allowing testing of sequence operations
#[cfg(test)]
pub fn arbitrary_pattern_option() -> impl Strategy<Value = Pattern<Option<i32>>> {
    let leaf = any::<Option<i32>>().prop_map(Pattern::point);
    
    leaf.prop_recursive(
        5,   // max_depth
        256, // desired_size
        10,  // max_elements per level
        |inner| {
            (any::<Option<i32>>(), prop::collection::vec(inner, 0..10))
                .prop_map(|(value, elements)| Pattern::pattern(value, elements))
        },
    )
}

/// Generate a pattern with Result values (for sequence testing)
///
/// Some values will be Ok, others Err, allowing testing of sequence operations
#[cfg(test)]
pub fn arbitrary_pattern_result() -> impl Strategy<Value = Pattern<Result<i32, String>>> {
    let leaf = any::<Result<i32, String>>().prop_map(Pattern::point);
    
    leaf.prop_recursive(
        5,   // max_depth
        256, // desired_size
        10,  // max_elements per level
        |inner| {
            (any::<Result<i32, String>>(), prop::collection::vec(inner, 0..10))
                .prop_map(|(value, elements)| Pattern::pattern(value, elements))
        },
    )
}

/// Generate a small atomic pattern (for quick tests)
#[cfg(test)]
pub fn atomic_pattern_i32() -> impl Strategy<Value = Pattern<i32>> {
    any::<i32>().prop_map(|v| Pattern::point(v))
}

/// Generate a shallow nested pattern (1-2 levels, for specific tests)
#[cfg(test)]
pub fn shallow_pattern_i32() -> impl Strategy<Value = Pattern<i32>> {
    let leaf = any::<i32>().prop_map(|v| Pattern::point(v));
    
    leaf.prop_recursive(
        2,   // max_depth (shallow)
        64,  // desired_size (small)
        5,   // max_elements per level
        |inner| {
            (any::<i32>(), prop::collection::vec(inner, 0..5))
                .prop_map(|(value, elements)| Pattern::pattern(value, elements))
        },
    )
}

/// Generate a deep nested pattern (for stack safety tests)
#[cfg(test)]
pub fn deep_pattern_i32() -> impl Strategy<Value = Pattern<i32>> {
    let leaf = any::<i32>().prop_map(|v| Pattern::point(v));
    
    leaf.prop_recursive(
        20,  // max_depth (deep!)
        100, // desired_size
        3,   // max_elements per level (fewer to keep depth high)
        |inner| {
            (any::<i32>(), prop::collection::vec(inner, 0..3))
                .prop_map(|(value, elements)| Pattern::pattern(value, elements))
        },
    )
}

/// Generate a wide pattern (many siblings, for breadth tests)
#[cfg(test)]
pub fn wide_pattern_i32() -> impl Strategy<Value = Pattern<i32>> {
    let leaf = any::<i32>().prop_map(|v| Pattern::point(v));
    
    leaf.prop_recursive(
        3,   // max_depth (shallow to keep wide)
        1000, // desired_size (large)
        50,  // max_elements per level (many siblings!)
        |inner| {
            (any::<i32>(), prop::collection::vec(inner, 0..50))
                .prop_map(|(value, elements)| Pattern::pattern(value, elements))
        },
    )
}
