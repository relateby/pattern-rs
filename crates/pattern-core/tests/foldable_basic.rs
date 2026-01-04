//! Basic fold operation tests for Pattern<V>
//!
//! Tests ported from gram-hs: ../gram-hs/libs/pattern/tests/Spec/Pattern/CoreSpec.hs
//! Lines 1054-1152: Foldable Instance tests

use pattern_core::Pattern;

// ============================================================================
// T005: Atomic pattern fold tests
// ============================================================================

#[test]
fn fold_atomic_pattern_with_integer() {
    let atom = Pattern::point(5);
    let result = atom.fold(0, |acc, v| acc + v);
    assert_eq!(result, 5);
}

#[test]
fn fold_atomic_pattern_with_string() {
    let atom = Pattern::point("test");
    let result = atom.fold(String::new(), |acc, s| acc + s);
    assert_eq!(result, "test");
}

#[test]
fn fold_atomic_pattern_count() {
    let atom = Pattern::point(42);
    let count = atom.fold(0, |acc, _| acc + 1);
    assert_eq!(count, 1);
}

// ============================================================================
// T006: Flat pattern fold tests (one level, multiple elements)
// ============================================================================

#[test]
fn fold_flat_pattern_with_integer_sum() {
    let elem1 = Pattern::point(10);
    let elem2 = Pattern::point(20);
    let elem3 = Pattern::point(30);
    let pattern = Pattern::pattern(100, vec![elem1, elem2, elem3]);

    // Should sum: 100 (pattern's value) + 10 + 20 + 30 = 160
    let result = pattern.fold(0, |acc, v| acc + v);
    assert_eq!(result, 160);
}

#[test]
fn fold_flat_pattern_with_string_concatenation() {
    let elem1 = Pattern::point("hello");
    let elem2 = Pattern::point("world");
    let pattern = Pattern::pattern("greeting", vec![elem1, elem2]);

    // Should concatenate: "greeting" + "hello" + "world"
    let result = pattern.fold(String::new(), |acc, s| acc + s);
    assert_eq!(result, "greetinghelloworld");
}

#[test]
fn fold_flat_pattern_count() {
    let elem1 = Pattern::point("a");
    let elem2 = Pattern::point("b");
    let elem3 = Pattern::point("c");
    let pattern = Pattern::pattern("root", vec![elem1, elem2, elem3]);

    // Should count: root + 3 elements = 4
    let count = pattern.fold(0, |acc, _| acc + 1);
    assert_eq!(count, 4);
}

#[test]
fn fold_flat_pattern_processes_pattern_value() {
    let elem1 = Pattern::point(10);
    let pattern = Pattern::pattern(5, vec![elem1]);

    // Should include the pattern's own value (5) + element (10) = 15
    let result = pattern.fold(0, |acc, v| acc + v);
    assert_eq!(result, 15);
}

// ============================================================================
// T007: Nested pattern fold tests (multiple levels)
// ============================================================================

#[test]
fn fold_nested_pattern_two_levels() {
    let inner = Pattern::point(1);
    let middle = Pattern::pattern(2, vec![inner]);
    let outer = Pattern::pattern(3, vec![middle]);
    let pattern = Pattern::pattern(4, vec![outer]);

    // Should sum: 4 + 3 + 2 + 1 = 10
    let result = pattern.fold(0, |acc, v| acc + v);
    assert_eq!(result, 10);
}

#[test]
fn fold_deeply_nested_pattern() {
    let level4 = Pattern::point(1);
    let level3 = Pattern::pattern(2, vec![level4]);
    let level2 = Pattern::pattern(3, vec![level3]);
    let level1 = Pattern::pattern(4, vec![level2]);
    let pattern = Pattern::pattern(5, vec![level1]);

    // Should sum: 5 + 4 + 3 + 2 + 1 = 15
    let result = pattern.fold(0, |acc, v| acc + v);
    assert_eq!(result, 15);
}

#[test]
fn fold_nested_pattern_with_multiple_branches() {
    let inner1 = Pattern::point(1);
    let inner2 = Pattern::point(2);
    let middle1 = Pattern::pattern(10, vec![inner1]);
    let middle2 = Pattern::pattern(20, vec![inner2]);
    let pattern = Pattern::pattern(100, vec![middle1, middle2]);

    // Should sum: 100 + 10 + 1 + 20 + 2 = 133
    let result = pattern.fold(0, |acc, v| acc + v);
    assert_eq!(result, 133);
}

#[test]
fn fold_nested_pattern_preserves_recursion() {
    let inner1 = Pattern::point(1);
    let inner2 = Pattern::point(2);
    let middle = Pattern::pattern(10, vec![inner1, inner2]);
    let pattern = Pattern::pattern(100, vec![middle]);

    // Should sum: 100 + 10 + 1 + 2 = 113
    let result = pattern.fold(0, |acc, v| acc + v);
    assert_eq!(result, 113);
}

// ============================================================================
// T008: Order verification test with string concatenation
// ============================================================================

#[test]
fn fold_preserves_depth_first_root_first_order() {
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

    // Should concatenate in depth-first, root-first order: A, B, C, D
    let result = pattern.fold(String::new(), |acc, s| acc + s);
    assert_eq!(result, "ABCD");
}

#[test]
fn fold_processes_root_before_elements() {
    let pattern = Pattern::pattern(
        "first",
        vec![Pattern::point("second"), Pattern::point("third")],
    );

    // Root value should be processed first
    let result = pattern.fold(String::new(), |acc, s| acc + s);
    assert_eq!(result, "firstsecondthird");
}

#[test]
fn fold_processes_elements_left_to_right() {
    let pattern = Pattern::pattern(
        1,
        vec![Pattern::point(2), Pattern::point(3), Pattern::point(4)],
    );

    // Elements should be processed in order: 2, 3, 4
    let result = pattern.fold(0, |acc, v| acc + v);
    assert_eq!(result, 10); // 1 + 2 + 3 + 4
}

// ============================================================================
// T009: Sum test (root + elements)
// ============================================================================

#[test]
fn fold_sum_includes_all_values() {
    let pattern = Pattern::pattern(
        2,
        vec![Pattern::point(3), Pattern::point(4), Pattern::point(5)],
    );

    let sum = pattern.fold(0, |acc, v| acc + v);
    assert_eq!(sum, 14); // 2 + 3 + 4 + 5
}

#[test]
fn fold_sum_with_nested_structure() {
    let inner = Pattern::pattern(5, vec![Pattern::point(6)]);
    let pattern = Pattern::pattern(1, vec![Pattern::point(2), inner]);

    let sum = pattern.fold(0, |acc, v| acc + v);
    assert_eq!(sum, 14); // 1 + 2 + 5 + 6
}

// ============================================================================
// T010: Count test (verify count equals size)
// ============================================================================

#[test]
fn fold_count_equals_size_atomic() {
    let pattern = Pattern::point(42);
    let count = pattern.fold(0, |acc, _| acc + 1);
    assert_eq!(count, pattern.size());
}

#[test]
fn fold_count_equals_size_flat() {
    let pattern = Pattern::pattern(
        "root",
        vec![
            Pattern::point("a"),
            Pattern::point("b"),
            Pattern::point("c"),
        ],
    );
    let count = pattern.fold(0, |acc, _| acc + 1);
    assert_eq!(count, pattern.size());
    assert_eq!(count, 4); // root + 3 elements
}

#[test]
fn fold_count_equals_size_nested() {
    let inner = Pattern::pattern(2, vec![Pattern::point(3)]);
    let pattern = Pattern::pattern(1, vec![inner, Pattern::point(4)]);

    let count = pattern.fold(0, |acc, _| acc + 1);
    assert_eq!(count, pattern.size());
    assert_eq!(count, 4); // 1, 2, 3, 4
}

#[test]
fn fold_count_equals_size_complex() {
    let level3 = Pattern::point(4);
    let level2 = Pattern::pattern(3, vec![level3]);
    let level1 = Pattern::pattern(2, vec![level2]);
    let pattern = Pattern::pattern(1, vec![level1, Pattern::point(5)]);

    let count = pattern.fold(0, |acc, _| acc + 1);
    assert_eq!(count, pattern.size());
    assert_eq!(count, 5); // 1, 2, 3, 4, 5
}

// ============================================================================
// Additional verification tests
// ============================================================================

#[test]
fn fold_pattern_unchanged_after_fold() {
    let pattern = Pattern::pattern(1, vec![Pattern::point(2), Pattern::point(3)]);
    let original = pattern.clone();

    // Perform fold
    let _ = pattern.fold(0, |acc, v| acc + v);

    // Pattern should be unchanged
    assert_eq!(pattern, original);
}

#[test]
fn fold_can_be_called_multiple_times() {
    let pattern = Pattern::pattern(2, vec![Pattern::point(3), Pattern::point(4)]);

    // Fold multiple times with different operations
    let sum = pattern.fold(0, |acc, v| acc + v);
    let product = pattern.fold(1, |acc, v| acc * v);
    let count = pattern.fold(0, |acc, _| acc + 1);

    assert_eq!(sum, 9); // 2 + 3 + 4
    assert_eq!(product, 24); // 2 * 3 * 4
    assert_eq!(count, 3);
}

#[test]
fn fold_with_custom_type() {
    #[derive(Clone, PartialEq, Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    let person1 = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    let person2 = Person {
        name: "Bob".to_string(),
        age: 25,
    };
    let person3 = Person {
        name: "Charlie".to_string(),
        age: 35,
    };

    let pattern = Pattern::pattern(
        person1.clone(),
        vec![
            Pattern::point(person2.clone()),
            Pattern::point(person3.clone()),
        ],
    );

    // Count people
    let count = pattern.fold(0, |acc, _| acc + 1);
    assert_eq!(count, 3);

    // Sum ages
    let total_age = pattern.fold(0u32, |acc, p| acc + p.age);
    assert_eq!(total_age, 90); // 30 + 25 + 35
}
