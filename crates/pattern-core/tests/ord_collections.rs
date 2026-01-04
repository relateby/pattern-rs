//! Ordered collections tests for Pattern Ord implementation
//!
//! This module tests using Pattern as keys/elements in standard library
//! ordered data structures like BTreeSet, BTreeMap, and BinaryHeap.

use pattern_core::Pattern;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

// ============================================================================
// T047: Insert patterns into BTreeSet and verify ordering
// ============================================================================

#[test]
fn test_btreeset_ordering() {
    let mut set = BTreeSet::new();

    // Insert in random order
    set.insert(Pattern::point(5));
    set.insert(Pattern::point(2));
    set.insert(Pattern::point(8));
    set.insert(Pattern::point(1));
    set.insert(Pattern::point(4));

    // Iteration should be in sorted order
    let values: Vec<i32> = set.iter().map(|p| p.value).collect();
    assert_eq!(values, vec![1, 2, 4, 5, 8]);

    // Verify set maintains ordering
    let mut prev: Option<&Pattern<i32>> = None;
    for pattern in set.iter() {
        if let Some(p) = prev {
            assert!(p < pattern, "Set not in sorted order");
        }
        prev = Some(pattern);
    }
}

// ============================================================================
// T048: BTreeSet prevents duplicate patterns
// ============================================================================

#[test]
fn test_btreeset_duplicates() {
    let mut set = BTreeSet::new();

    // Insert same pattern multiple times
    assert!(set.insert(Pattern::point(5)));
    assert!(!set.insert(Pattern::point(5))); // Should return false (already exists)
    assert!(!set.insert(Pattern::point(5)));

    // Set should only contain one element
    assert_eq!(set.len(), 1);

    // Insert different patterns
    assert!(set.insert(Pattern::point(3)));
    assert!(set.insert(Pattern::point(7)));

    assert_eq!(set.len(), 3);

    // Try inserting duplicates again
    assert!(!set.insert(Pattern::point(3)));
    assert!(!set.insert(Pattern::point(7)));

    assert_eq!(set.len(), 3);
}

// ============================================================================
// T049: BTreeSet membership queries work correctly
// ============================================================================

#[test]
fn test_btreeset_membership() {
    let mut set = BTreeSet::new();

    set.insert(Pattern::point(1));
    set.insert(Pattern::point(3));
    set.insert(Pattern::point(5));
    set.insert(Pattern::point(7));

    // Test contains
    assert!(set.contains(&Pattern::point(3)));
    assert!(set.contains(&Pattern::point(7)));
    assert!(!set.contains(&Pattern::point(2)));
    assert!(!set.contains(&Pattern::point(10)));

    // Test range queries
    let range: Vec<i32> = set
        .range(Pattern::point(2)..Pattern::point(6))
        .map(|p| p.value)
        .collect();
    assert_eq!(range, vec![3, 5]);

    // Test first and last
    assert_eq!(set.first().unwrap(), &Pattern::point(1));
    assert_eq!(set.last().unwrap(), &Pattern::point(7));

    // Test remove
    assert!(set.remove(&Pattern::point(5)));
    assert!(!set.remove(&Pattern::point(5))); // Already removed
    assert_eq!(set.len(), 3);
}

// ============================================================================
// T050: Use patterns as BTreeMap keys with insertion and retrieval
// ============================================================================

#[test]
fn test_btreemap_keys() {
    let mut map = BTreeMap::new();

    // Insert patterns as keys
    map.insert(Pattern::point(5), "five");
    map.insert(Pattern::point(2), "two");
    map.insert(Pattern::point(8), "eight");
    map.insert(Pattern::point(1), "one");

    // Retrieve by key
    assert_eq!(map.get(&Pattern::point(5)), Some(&"five"));
    assert_eq!(map.get(&Pattern::point(2)), Some(&"two"));
    assert_eq!(map.get(&Pattern::point(10)), None);

    // Update existing key
    map.insert(Pattern::point(5), "FIVE");
    assert_eq!(map.get(&Pattern::point(5)), Some(&"FIVE"));
    assert_eq!(map.len(), 4); // Size unchanged

    // Remove by key
    assert_eq!(map.remove(&Pattern::point(2)), Some("two"));
    assert_eq!(map.remove(&Pattern::point(2)), None);
    assert_eq!(map.len(), 3);
}

// ============================================================================
// T051: BTreeMap range queries with pattern keys
// ============================================================================

#[test]
fn test_btreemap_range_queries() {
    let mut map = BTreeMap::new();

    for i in 0..10 {
        map.insert(Pattern::point(i), i * 10);
    }

    // Range query [3, 7)
    let range: Vec<(i32, i32)> = map
        .range(Pattern::point(3)..Pattern::point(7))
        .map(|(k, v)| (k.value, *v))
        .collect();

    assert_eq!(range, vec![(3, 30), (4, 40), (5, 50), (6, 60)]);

    // Range from start
    let from_start: Vec<i32> = map
        .range(..Pattern::point(3))
        .map(|(k, _)| k.value)
        .collect();

    assert_eq!(from_start, vec![0, 1, 2]);

    // Range to end
    let to_end: Vec<i32> = map
        .range(Pattern::point(7)..)
        .map(|(k, _)| k.value)
        .collect();

    assert_eq!(to_end, vec![7, 8, 9]);
}

// ============================================================================
// T052: BTreeMap iteration yields patterns in sorted order
// ============================================================================

#[test]
fn test_btreemap_iteration_order() {
    let mut map = BTreeMap::new();

    // Insert in random order
    map.insert(Pattern::point(5), "e");
    map.insert(Pattern::point(2), "b");
    map.insert(Pattern::point(8), "h");
    map.insert(Pattern::point(1), "a");
    map.insert(Pattern::point(9), "i");
    map.insert(Pattern::point(3), "c");

    // Iteration should be in sorted key order
    let keys: Vec<i32> = map.keys().map(|p| p.value).collect();
    assert_eq!(keys, vec![1, 2, 3, 5, 8, 9]);

    // Values should correspond to sorted keys
    let values: Vec<&str> = map.values().copied().collect();
    assert_eq!(values, vec!["a", "b", "c", "e", "h", "i"]);

    // Reverse iteration
    let rev_keys: Vec<i32> = map.keys().rev().map(|p| p.value).collect();
    assert_eq!(rev_keys, vec![9, 8, 5, 3, 2, 1]);
}

// ============================================================================
// T053: BinaryHeap with patterns (max-heap behavior)
// ============================================================================

#[test]
fn test_binaryheap_max_heap() {
    let mut heap = BinaryHeap::new();

    // Insert patterns
    heap.push(Pattern::point(5));
    heap.push(Pattern::point(2));
    heap.push(Pattern::point(8));
    heap.push(Pattern::point(1));
    heap.push(Pattern::point(9));
    heap.push(Pattern::point(3));

    // Peek should return maximum
    assert_eq!(heap.peek().unwrap(), &Pattern::point(9));

    // Verify heap size
    assert_eq!(heap.len(), 6);

    // Peek doesn't remove
    assert_eq!(heap.peek().unwrap(), &Pattern::point(9));
    assert_eq!(heap.len(), 6);
}

// ============================================================================
// T054: BinaryHeap pop returns patterns in descending order
// ============================================================================

#[test]
fn test_binaryheap_pop_order() {
    let mut heap = BinaryHeap::new();

    // Insert patterns
    let values = vec![5, 2, 8, 1, 9, 3, 7, 4, 6];
    for v in values {
        heap.push(Pattern::point(v));
    }

    // Pop should return in descending order (max-heap)
    let mut popped = Vec::new();
    while let Some(pattern) = heap.pop() {
        popped.push(pattern.value);
    }

    assert_eq!(popped, vec![9, 8, 7, 6, 5, 4, 3, 2, 1]);

    // Heap should be empty
    assert!(heap.is_empty());
    assert_eq!(heap.peek(), None);
}

// ============================================================================
// T055: Large-scale BTreeSet operations (10,000 patterns)
// ============================================================================

#[test]
fn test_btreeset_large_scale() {
    let mut set = BTreeSet::new();

    // Insert 10,000 patterns
    for i in 0..10000 {
        set.insert(Pattern::point(i));
    }

    assert_eq!(set.len(), 10000);

    // Verify ordering (spot check)
    assert_eq!(set.first().unwrap(), &Pattern::point(0));
    assert_eq!(set.last().unwrap(), &Pattern::point(9999));

    // Test membership
    assert!(set.contains(&Pattern::point(5000)));
    assert!(!set.contains(&Pattern::point(10000)));

    // Range query
    let range: Vec<i32> = set
        .range(Pattern::point(1000)..Pattern::point(1010))
        .map(|p| p.value)
        .collect();

    assert_eq!(range.len(), 10);
    assert_eq!(range[0], 1000);
    assert_eq!(range[9], 1009);

    // Remove some elements
    for i in 5000..5100 {
        assert!(set.remove(&Pattern::point(i)));
    }

    assert_eq!(set.len(), 9900);
    assert!(!set.contains(&Pattern::point(5050)));
}

// ============================================================================
// T056: Large-scale BTreeMap operations (10,000 patterns)
// ============================================================================

#[test]
fn test_btreemap_large_scale() {
    let mut map = BTreeMap::new();

    // Insert 10,000 patterns
    for i in 0..10000 {
        map.insert(Pattern::point(i), i * 2);
    }

    assert_eq!(map.len(), 10000);

    // Verify retrieval
    assert_eq!(map.get(&Pattern::point(5000)), Some(&10000));
    assert_eq!(map.get(&Pattern::point(7500)), Some(&15000));

    // Range query
    let range_sum: i32 = map
        .range(Pattern::point(1000)..Pattern::point(2000))
        .map(|(_, v)| v)
        .sum();

    // Sum of 1000*2 + 1001*2 + ... + 1999*2 = 2 * (1000 + 1001 + ... + 1999)
    let expected_sum = 2 * (1000..2000).sum::<i32>();
    assert_eq!(range_sum, expected_sum);

    // Update values
    for i in 0..100 {
        map.insert(Pattern::point(i), i * 100);
    }

    assert_eq!(map.len(), 10000); // Size unchanged
    assert_eq!(map.get(&Pattern::point(50)), Some(&5000)); // Updated value

    // Remove elements
    for i in 9000..10000 {
        assert_eq!(map.remove(&Pattern::point(i)), Some(i * 2));
    }

    assert_eq!(map.len(), 9000);
}

// ============================================================================
// Additional collection tests
// ============================================================================

#[test]
fn test_nested_patterns_in_btreeset() {
    let mut set = BTreeSet::new();

    set.insert(Pattern::pattern(5, vec![Pattern::point(1)]));
    set.insert(Pattern::pattern(3, vec![Pattern::point(2)]));
    set.insert(Pattern::pattern(5, vec![Pattern::point(2)]));
    set.insert(Pattern::pattern(3, vec![Pattern::point(1)]));

    // Should be sorted by value first, then elements
    let values: Vec<(i32, i32)> = set.iter().map(|p| (p.value, p.elements[0].value)).collect();

    assert_eq!(values, vec![(3, 1), (3, 2), (5, 1), (5, 2)]);
}

#[test]
fn test_nested_patterns_in_btreemap() {
    let mut map = BTreeMap::new();

    let k1 = Pattern::pattern(1, vec![Pattern::point(2)]);
    let k2 = Pattern::pattern(1, vec![Pattern::point(3)]);
    let k3 = Pattern::pattern(2, vec![Pattern::point(1)]);

    map.insert(k1.clone(), "first");
    map.insert(k2.clone(), "second");
    map.insert(k3.clone(), "third");

    assert_eq!(map.get(&k1), Some(&"first"));
    assert_eq!(map.get(&k2), Some(&"second"));
    assert_eq!(map.get(&k3), Some(&"third"));

    // Verify ordering
    let keys: Vec<(i32, i32)> = map.keys().map(|p| (p.value, p.elements[0].value)).collect();

    assert_eq!(keys, vec![(1, 2), (1, 3), (2, 1)]);
}

#[test]
fn test_binaryheap_with_nested_patterns() {
    let mut heap = BinaryHeap::new();

    heap.push(Pattern::pattern(5, vec![Pattern::point(1)]));
    heap.push(Pattern::pattern(3, vec![Pattern::point(2)]));
    heap.push(Pattern::pattern(5, vec![Pattern::point(2)]));
    heap.push(Pattern::pattern(7, vec![Pattern::point(1)]));

    // Max should be value 7
    assert_eq!(heap.peek().unwrap().value, 7);

    // Pop in descending value order
    let p1 = heap.pop().unwrap();
    assert_eq!(p1.value, 7);

    let p2 = heap.pop().unwrap();
    assert_eq!(p2.value, 5);

    let p3 = heap.pop().unwrap();
    assert_eq!(p3.value, 5);

    let p4 = heap.pop().unwrap();
    assert_eq!(p4.value, 3);
}

#[test]
fn test_collections_interoperability() {
    // Create patterns in a Vec
    let patterns = [
        Pattern::point(5),
        Pattern::point(2),
        Pattern::point(8),
        Pattern::point(2), // Duplicate
        Pattern::point(5), // Duplicate
    ];

    // Convert to BTreeSet (removes duplicates)
    let set: BTreeSet<_> = patterns.iter().cloned().collect();
    assert_eq!(set.len(), 3); // Duplicates removed

    // Convert to BTreeMap
    let map: BTreeMap<_, usize> = patterns
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect();

    assert_eq!(map.len(), 3); // Duplicates removed (last index kept)

    // Convert to BinaryHeap
    let heap: BinaryHeap<_> = patterns.iter().cloned().collect();
    assert_eq!(heap.len(), 5); // All elements kept
    assert_eq!(heap.peek().unwrap(), &Pattern::point(8)); // Max
}
