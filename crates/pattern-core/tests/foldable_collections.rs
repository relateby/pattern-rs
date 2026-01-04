//! Collection conversion tests for Pattern<V>
//!
//! Tests for the values() method and integration with standard library Iterator operations.

use pattern_core::Pattern;

// ============================================================================
// T015: Atomic pattern to Vec test
// ============================================================================

#[test]
fn values_atomic_pattern() {
    let pattern = Pattern::point("test");
    let values = pattern.values();

    assert_eq!(values.len(), 1);
    assert_eq!(values, vec![&"test"]);
}

#[test]
fn values_atomic_pattern_integer() {
    let pattern = Pattern::point(42);
    let values = pattern.values();

    assert_eq!(values.len(), 1);
    assert_eq!(values, vec![&42]);
}

// ============================================================================
// T016: Flat pattern to Vec test with order verification
// ============================================================================

#[test]
fn values_flat_pattern_preserves_order() {
    let pattern = Pattern::pattern(
        "first",
        vec![Pattern::point("second"), Pattern::point("third")],
    );

    let values = pattern.values();

    assert_eq!(values.len(), 3);
    assert_eq!(values, vec![&"first", &"second", &"third"]);
}

#[test]
fn values_flat_pattern_multiple_elements() {
    let pattern = Pattern::pattern(
        1,
        vec![
            Pattern::point(2),
            Pattern::point(3),
            Pattern::point(4),
            Pattern::point(5),
        ],
    );

    let values = pattern.values();

    assert_eq!(values.len(), 5);
    assert_eq!(values, vec![&1, &2, &3, &4, &5]);
}

#[test]
fn values_flat_pattern_root_first() {
    let pattern = Pattern::pattern("root", vec![Pattern::point("a"), Pattern::point("b")]);

    let values = pattern.values();

    // Root should be first
    assert_eq!(values[0], &"root");
    assert_eq!(values[1], &"a");
    assert_eq!(values[2], &"b");
}

// ============================================================================
// T017: Nested pattern to Vec test
// ============================================================================

#[test]
fn values_nested_pattern_depth_first_order() {
    // Pattern structure:
    //   "A"
    //   ├── "B"
    //   └── "C"
    //       └── "D"
    let pattern = Pattern::pattern(
        "A",
        vec![
            Pattern::point("B"),
            Pattern::pattern("C", vec![Pattern::point("D")]),
        ],
    );

    let values = pattern.values();

    // Should be in depth-first, root-first order: A, B, C, D
    assert_eq!(values.len(), 4);
    assert_eq!(values, vec![&"A", &"B", &"C", &"D"]);
}

#[test]
fn values_deeply_nested_pattern() {
    let level3 = Pattern::point(4);
    let level2 = Pattern::pattern(3, vec![level3]);
    let level1 = Pattern::pattern(2, vec![level2]);
    let pattern = Pattern::pattern(1, vec![level1]);

    let values = pattern.values();

    assert_eq!(values.len(), 4);
    assert_eq!(values, vec![&1, &2, &3, &4]);
}

#[test]
fn values_nested_pattern_multiple_branches() {
    // Pattern structure:
    //   1
    //   ├── 2
    //   │   └── 3
    //   └── 4
    //       └── 5
    let branch1 = Pattern::pattern(2, vec![Pattern::point(3)]);
    let branch2 = Pattern::pattern(4, vec![Pattern::point(5)]);
    let pattern = Pattern::pattern(1, vec![branch1, branch2]);

    let values = pattern.values();

    assert_eq!(values.len(), 5);
    assert_eq!(values, vec![&1, &2, &3, &4, &5]);
}

// ============================================================================
// T018: Values length equals size test
// ============================================================================

#[test]
fn values_length_equals_size_atomic() {
    let pattern = Pattern::point(42);
    let values = pattern.values();

    assert_eq!(values.len(), pattern.size());
}

#[test]
fn values_length_equals_size_flat() {
    let pattern = Pattern::pattern(
        "root",
        vec![
            Pattern::point("a"),
            Pattern::point("b"),
            Pattern::point("c"),
        ],
    );
    let values = pattern.values();

    assert_eq!(values.len(), pattern.size());
    assert_eq!(values.len(), 4);
}

#[test]
fn values_length_equals_size_nested() {
    let inner = Pattern::pattern(2, vec![Pattern::point(3)]);
    let pattern = Pattern::pattern(1, vec![inner, Pattern::point(4)]);

    let values = pattern.values();

    assert_eq!(values.len(), pattern.size());
    assert_eq!(values.len(), 4);
}

#[test]
fn values_length_equals_size_complex() {
    let level4 = Pattern::point(5);
    let level3 = Pattern::pattern(4, vec![level4]);
    let level2 = Pattern::pattern(3, vec![level3]);
    let level1 = Pattern::pattern(2, vec![level2]);
    let pattern = Pattern::pattern(1, vec![level1, Pattern::point(6)]);

    let values = pattern.values();

    assert_eq!(values.len(), pattern.size());
    assert_eq!(values.len(), 6);
}

// ============================================================================
// T019: Integration test with Iterator operations
// ============================================================================

#[test]
fn values_integrates_with_iterator_sum() {
    let pattern = Pattern::pattern(
        1,
        vec![Pattern::point(2), Pattern::point(3), Pattern::point(4)],
    );

    let sum: i32 = pattern.values().iter().map(|&&v| v).sum();

    assert_eq!(sum, 10); // 1 + 2 + 3 + 4
}

#[test]
fn values_integrates_with_iterator_filter() {
    let pattern = Pattern::pattern(
        1,
        vec![
            Pattern::point(2),
            Pattern::point(3),
            Pattern::point(4),
            Pattern::point(5),
        ],
    );

    // Filter for even numbers
    let evens: Vec<i32> = pattern
        .values()
        .iter()
        .filter(|&&v| v % 2 == 0)
        .map(|&&v| v)
        .collect();

    assert_eq!(evens, vec![2, 4]);
}

#[test]
fn values_integrates_with_iterator_all() {
    let pattern = Pattern::pattern(
        1,
        vec![Pattern::point(2), Pattern::point(3), Pattern::point(4)],
    );

    let all_positive = pattern.values().iter().all(|&&v| v > 0);
    assert!(all_positive);

    let all_large = pattern.values().iter().all(|&&v| v > 10);
    assert!(!all_large);
}

#[test]
fn values_integrates_with_iterator_any() {
    let pattern = Pattern::pattern(
        1,
        vec![Pattern::point(2), Pattern::point(3), Pattern::point(4)],
    );

    let any_large = pattern.values().iter().any(|&&v| v > 3);
    assert!(any_large);

    let any_negative = pattern.values().iter().any(|&&v| v < 0);
    assert!(!any_negative);
}

#[test]
fn values_integrates_with_iterator_find() {
    let pattern = Pattern::pattern(
        1,
        vec![Pattern::point(2), Pattern::point(3), Pattern::point(4)],
    );

    let values = pattern.values();
    let found = values.iter().find(|&&v| *v == 3);
    assert_eq!(found, Some(&&3));

    let values2 = pattern.values();
    let not_found = values2.iter().find(|&&v| *v == 10);
    assert_eq!(not_found, None);
}

#[test]
fn values_integrates_with_iterator_map() {
    let pattern = Pattern::pattern("hello", vec![Pattern::point("world"), Pattern::point("!")]);

    let lengths: Vec<usize> = pattern.values().iter().map(|s| s.len()).collect();

    assert_eq!(lengths, vec![5, 5, 1]);
}

#[test]
fn values_integrates_with_iterator_fold() {
    let pattern = Pattern::pattern(2, vec![Pattern::point(3), Pattern::point(4)]);

    // Use Iterator::fold on the values
    let product = pattern.values().iter().fold(1, |acc, &&v| acc * v);

    assert_eq!(product, 24); // 2 * 3 * 4
}

// ============================================================================
// Additional verification tests
// ============================================================================

#[test]
fn values_can_be_called_multiple_times() {
    let pattern = Pattern::pattern(1, vec![Pattern::point(2), Pattern::point(3)]);

    let values1 = pattern.values();
    let values2 = pattern.values();

    assert_eq!(values1, values2);
}

#[test]
fn values_returns_references_not_clones() {
    let pattern = Pattern::pattern("test", vec![Pattern::point("data")]);

    let values = pattern.values();

    // Values should be references
    assert_eq!(*values[0], "test");
    assert_eq!(*values[1], "data");
}

#[test]
fn values_works_with_custom_types() {
    #[derive(Clone, PartialEq, Debug)]
    struct Item {
        id: u32,
        name: String,
    }

    let item1 = Item {
        id: 1,
        name: "first".to_string(),
    };
    let item2 = Item {
        id: 2,
        name: "second".to_string(),
    };

    let pattern = Pattern::pattern(item1.clone(), vec![Pattern::point(item2.clone())]);

    let values = pattern.values();

    assert_eq!(values.len(), 2);
    assert_eq!(values[0].id, 1);
    assert_eq!(values[1].id, 2);
}
