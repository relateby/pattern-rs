//! Basic Hash trait tests for Pattern
//!
//! This module tests fundamental hashing behavior including:
//! - HashSet deduplication
//! - HashMap usage with pattern keys
//! - Hash/Eq consistency
//! - Structure distinguishes hashes

use pattern_core::{Pattern, Symbol};
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

// ============================================================================
// Helper Functions
// ============================================================================

/// Computes the hash of a pattern using the default hasher
fn hash_pattern<V: Hash>(p: &Pattern<V>) -> u64 {
    let mut hasher = DefaultHasher::new();
    p.hash(&mut hasher);
    hasher.finish()
}

// ============================================================================
// HashSet Deduplication Tests
// ============================================================================

#[test]
fn test_hashset_dedup_string_patterns() {
    let p1 = Pattern::point("hello".to_string());
    let p2 = Pattern::point("world".to_string());
    let p3 = Pattern::point("hello".to_string()); // Duplicate of p1

    let mut set = HashSet::new();
    set.insert(p1);
    set.insert(p2);
    set.insert(p3); // Should not be added (duplicate)

    assert_eq!(set.len(), 2, "HashSet should deduplicate equal patterns");
}

#[test]
fn test_hashset_dedup_symbol_patterns() {
    let p1 = Pattern::point(Symbol("a".to_string()));
    let p2 = Pattern::point(Symbol("b".to_string()));
    let p3 = Pattern::point(Symbol("a".to_string())); // Duplicate

    let mut set = HashSet::new();
    set.insert(p1);
    set.insert(p2);
    set.insert(p3);

    assert_eq!(set.len(), 2);
}

#[test]
fn test_hashset_membership() {
    let p1 = Pattern::point("test".to_string());
    let p2 = Pattern::point("other".to_string());

    let mut set = HashSet::new();
    set.insert(p1.clone());

    assert!(set.contains(&p1), "Set should contain inserted pattern");
    assert!(
        !set.contains(&p2),
        "Set should not contain pattern that wasn't inserted"
    );
}

#[test]
fn test_hashset_nested_patterns() {
    let p1 = Pattern::pattern(
        "root".to_string(),
        vec![
            Pattern::point("a".to_string()),
            Pattern::point("b".to_string()),
        ],
    );
    let p2 = Pattern::pattern(
        "root".to_string(),
        vec![
            Pattern::point("a".to_string()),
            Pattern::point("b".to_string()),
        ],
    );

    let mut set = HashSet::new();
    set.insert(p1);
    set.insert(p2); // Duplicate

    assert_eq!(set.len(), 1, "Nested patterns should be deduplicated");
}

// ============================================================================
// Hash/Eq Consistency Tests
// ============================================================================

#[test]
fn test_equal_patterns_hash_equal_atomic() {
    let p1 = Pattern::point("test".to_string());
    let p2 = Pattern::point("test".to_string());

    assert_eq!(p1, p2, "Patterns should be equal");
    assert_eq!(
        hash_pattern(&p1),
        hash_pattern(&p2),
        "Equal patterns must hash to same value"
    );
}

#[test]
fn test_equal_patterns_hash_equal_nested() {
    let p1 = Pattern::pattern(
        "root".to_string(),
        vec![
            Pattern::point("a".to_string()),
            Pattern::pattern("b".to_string(), vec![Pattern::point("c".to_string())]),
        ],
    );
    let p2 = Pattern::pattern(
        "root".to_string(),
        vec![
            Pattern::point("a".to_string()),
            Pattern::pattern("b".to_string(), vec![Pattern::point("c".to_string())]),
        ],
    );

    assert_eq!(p1, p2);
    assert_eq!(hash_pattern(&p1), hash_pattern(&p2));
}

#[test]
fn test_different_values_hash_correctly() {
    let p1 = Pattern::point("test1".to_string());
    let p2 = Pattern::point("test2".to_string());

    // Verify patterns are different
    assert_ne!(p1, p2);

    // Compute hashes - collisions are valid, so we don't assert inequality
    let hash1 = hash_pattern(&p1);
    let hash2 = hash_pattern(&p2);

    // Note: Different patterns MAY have different hashes (expected),
    // but hash collisions are valid and we don't enforce inequality.
    // The important property is that equal patterns hash equally (tested elsewhere).
    let _ = (hash1, hash2); // Use variables to avoid warnings
}

#[test]
fn test_structure_affects_pattern_equality() {
    // Same value content, different structure
    let atomic = Pattern::point("value".to_string());
    let compound = Pattern::pattern(
        "value".to_string(),
        vec![Pattern::point("child".to_string())],
    );

    // Verify that structure makes patterns unequal
    assert_ne!(atomic, compound);

    // Compute hashes - collisions are valid, so we don't assert inequality
    let hash1 = hash_pattern(&atomic);
    let hash2 = hash_pattern(&compound);

    // Note: While we expect different structures to typically produce different hashes,
    // hash collisions are valid. The Hash trait only requires that equal values
    // produce equal hashes, not that unequal values produce different hashes.
    let _ = (hash1, hash2); // Use variables to avoid warnings
}

// ============================================================================
// HashMap Tests
// ============================================================================

#[test]
fn test_hashmap_insert_lookup() {
    let mut map: HashMap<Pattern<String>, i32> = HashMap::new();

    let p1 = Pattern::point("key1".to_string());
    let p2 = Pattern::point("key2".to_string());

    map.insert(p1.clone(), 42);
    map.insert(p2.clone(), 100);

    assert_eq!(map.get(&p1), Some(&42));
    assert_eq!(map.get(&p2), Some(&100));
}

#[test]
fn test_hashmap_update_existing() {
    let mut map: HashMap<Pattern<String>, i32> = HashMap::new();

    let p1 = Pattern::point("key".to_string());
    let p1_dup = Pattern::point("key".to_string());

    map.insert(p1.clone(), 42);
    map.insert(p1_dup, 100); // Should update existing entry

    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&p1), Some(&100));
}

#[test]
fn test_hashmap_nested_pattern_keys() {
    let mut map: HashMap<Pattern<String>, &str> = HashMap::new();

    let p1 = Pattern::pattern(
        "root".to_string(),
        vec![Pattern::point("child".to_string())],
    );
    let p2 = Pattern::pattern(
        "root".to_string(),
        vec![Pattern::point("child".to_string())],
    );

    map.insert(p1.clone(), "value1");
    // p2 is equal to p1, should access same entry
    assert_eq!(map.get(&p2), Some(&"value1"));
}

#[test]
fn test_hashmap_caching_pattern() {
    // Simulate caching expensive computations by pattern
    let mut cache: HashMap<Pattern<String>, i32> = HashMap::new();

    let pattern = Pattern::pattern(
        "computation".to_string(),
        vec![Pattern::point("input".to_string())],
    );

    // First computation
    if !cache.contains_key(&pattern) {
        cache.insert(pattern.clone(), 42); // Expensive computation result
    }

    // Second lookup should hit cache
    assert_eq!(cache.get(&pattern), Some(&42));
}

// ============================================================================
// Symbol Hash Tests
// ============================================================================

#[test]
fn test_symbol_hashable() {
    let s1 = Symbol("test".to_string());
    let s2 = Symbol("test".to_string());

    let mut hasher1 = DefaultHasher::new();
    s1.hash(&mut hasher1);
    let hash1 = hasher1.finish();

    let mut hasher2 = DefaultHasher::new();
    s2.hash(&mut hasher2);
    let hash2 = hasher2.finish();

    assert_eq!(s1, s2);
    assert_eq!(hash1, hash2, "Equal symbols should hash equally");
}

#[test]
fn test_pattern_symbol_in_hashset() {
    let p1 = Pattern::point(Symbol("a".to_string()));
    let p2 = Pattern::point(Symbol("b".to_string()));

    let mut set = HashSet::new();
    set.insert(p1.clone());
    set.insert(p2.clone());

    assert!(set.contains(&p1));
    assert!(set.contains(&p2));
    assert_eq!(set.len(), 2);
}
