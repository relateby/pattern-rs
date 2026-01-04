//! Custom aggregation tests for Pattern<V>
//!
//! Tests for fold with various custom accumulator types and aggregation logic.

use pattern_core::Pattern;
use std::collections::{HashMap, HashSet};

// ============================================================================
// T022: Counting aggregation test
// ============================================================================

#[test]
fn fold_counting_aggregation() {
    let pattern = Pattern::pattern(
        "a",
        vec![
            Pattern::point("b"),
            Pattern::pattern("c", vec![Pattern::point("d")]),
        ],
    );

    let count = pattern.fold(0usize, |acc, _| acc + 1);
    assert_eq!(count, 4);
    assert_eq!(count, pattern.size());
}

// ============================================================================
// T023: Max/min finding test
// ============================================================================

#[test]
fn fold_find_maximum() {
    let pattern = Pattern::pattern(
        10,
        vec![
            Pattern::point(25),
            Pattern::point(5),
            Pattern::point(42),
            Pattern::point(18),
        ],
    );

    let max = pattern.fold(i32::MIN, |acc, &v| acc.max(v));
    assert_eq!(max, 42);
}

#[test]
fn fold_find_minimum() {
    let pattern = Pattern::pattern(
        10,
        vec![
            Pattern::point(25),
            Pattern::point(5),
            Pattern::point(42),
            Pattern::point(18),
        ],
    );

    let min = pattern.fold(i32::MAX, |acc, &v| acc.min(v));
    assert_eq!(min, 5);
}

// ============================================================================
// T024: HashMap building test
// ============================================================================

#[test]
fn fold_build_hashmap() {
    #[derive(Clone, Debug)]
    struct Item {
        id: String,
        value: i32,
    }

    let item1 = Item {
        id: "a".to_string(),
        value: 10,
    };
    let item2 = Item {
        id: "b".to_string(),
        value: 20,
    };
    let item3 = Item {
        id: "c".to_string(),
        value: 30,
    };

    let pattern = Pattern::pattern(
        item1.clone(),
        vec![Pattern::point(item2.clone()), Pattern::point(item3.clone())],
    );

    let map = pattern.fold(HashMap::new(), |mut acc, item| {
        acc.insert(item.id.clone(), item.value);
        acc
    });

    assert_eq!(map.len(), 3);
    assert_eq!(map.get("a"), Some(&10));
    assert_eq!(map.get("b"), Some(&20));
    assert_eq!(map.get("c"), Some(&30));
}

// ============================================================================
// T025: HashSet building test
// ============================================================================

#[test]
fn fold_build_hashset() {
    let pattern = Pattern::pattern(
        "apple",
        vec![
            Pattern::point("banana"),
            Pattern::pattern("cherry", vec![Pattern::point("date")]),
        ],
    );

    let set = pattern.fold(HashSet::new(), |mut acc, s| {
        acc.insert(s.to_string());
        acc
    });

    assert_eq!(set.len(), 4);
    assert!(set.contains("apple"));
    assert!(set.contains("banana"));
    assert!(set.contains("cherry"));
    assert!(set.contains("date"));
}

#[test]
fn fold_build_hashset_deduplicates() {
    let pattern = Pattern::pattern(
        "a",
        vec![
            Pattern::point("b"),
            Pattern::point("a"),
            Pattern::point("c"),
            Pattern::point("b"),
        ],
    );

    let set = pattern.fold(HashSet::new(), |mut acc, s| {
        acc.insert(s.to_string());
        acc
    });

    assert_eq!(set.len(), 3); // Only unique values
    assert!(set.contains("a"));
    assert!(set.contains("b"));
    assert!(set.contains("c"));
}

// ============================================================================
// T026: Boolean validation (all/any) test
// ============================================================================

#[test]
fn fold_all_predicate() {
    let pattern = Pattern::pattern(
        2,
        vec![Pattern::point(4), Pattern::point(6), Pattern::point(8)],
    );

    let all_even = pattern.fold(true, |acc, &v| acc && (v % 2 == 0));
    assert!(all_even);
}

#[test]
fn fold_all_predicate_fails() {
    let pattern = Pattern::pattern(
        2,
        vec![Pattern::point(4), Pattern::point(5), Pattern::point(8)],
    );

    let all_even = pattern.fold(true, |acc, &v| acc && (v % 2 == 0));
    assert!(!all_even);
}

#[test]
fn fold_any_predicate() {
    let pattern = Pattern::pattern(
        1,
        vec![Pattern::point(2), Pattern::point(3), Pattern::point(10)],
    );

    let any_large = pattern.fold(false, |acc, &v| acc || (v > 5));
    assert!(any_large);
}

#[test]
fn fold_any_predicate_fails() {
    let pattern = Pattern::pattern(
        1,
        vec![Pattern::point(2), Pattern::point(3), Pattern::point(4)],
    );

    let any_large = pattern.fold(false, |acc, &v| acc || (v > 10));
    assert!(!any_large);
}

// ============================================================================
// T027: Type transformation test (fold string pattern to usize)
// ============================================================================

#[test]
fn fold_type_transformation_string_to_usize() {
    let pattern = Pattern::pattern(
        "hello",
        vec![Pattern::point("world"), Pattern::point("rust")],
    );

    let total_length = pattern.fold(0usize, |acc, s| acc + s.len());
    assert_eq!(total_length, 14); // 5 + 5 + 4
}

#[test]
fn fold_type_transformation_int_to_string() {
    let pattern = Pattern::pattern(1, vec![Pattern::point(2), Pattern::point(3)]);

    let concatenated = pattern.fold(String::new(), |acc, &v| {
        if acc.is_empty() {
            v.to_string()
        } else {
            format!("{},{}", acc, v)
        }
    });

    assert_eq!(concatenated, "1,2,3");
}

#[test]
fn fold_type_transformation_custom_to_simple() {
    #[derive(Clone, Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    let p1 = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    let p2 = Person {
        name: "Bob".to_string(),
        age: 25,
    };
    let p3 = Person {
        name: "Charlie".to_string(),
        age: 35,
    };

    let pattern = Pattern::pattern(p1, vec![Pattern::point(p2), Pattern::point(p3)]);

    // Transform to total age (u32)
    let total_age = pattern.fold(0u32, |acc, person| acc + person.age);
    assert_eq!(total_age, 90);

    // Transform to name list (Vec<String>)
    let names = pattern.fold(Vec::new(), |mut acc, person| {
        acc.push(person.name.clone());
        acc
    });
    assert_eq!(names, vec!["Alice", "Bob", "Charlie"]);
}

// ============================================================================
// T028: Custom struct accumulator test
// ============================================================================

#[test]
fn fold_custom_struct_accumulator() {
    #[derive(Debug, PartialEq)]
    struct Stats {
        count: usize,
        sum: i32,
        max: i32,
        min: i32,
    }

    let pattern = Pattern::pattern(
        5,
        vec![Pattern::point(10), Pattern::point(3), Pattern::point(15)],
    );

    let stats = pattern.fold(
        Stats {
            count: 0,
            sum: 0,
            max: i32::MIN,
            min: i32::MAX,
        },
        |mut acc, &v| {
            acc.count += 1;
            acc.sum += v;
            acc.max = acc.max.max(v);
            acc.min = acc.min.min(v);
            acc
        },
    );

    assert_eq!(stats.count, 4);
    assert_eq!(stats.sum, 33); // 5 + 10 + 3 + 15
    assert_eq!(stats.max, 15);
    assert_eq!(stats.min, 3);
}

#[test]
fn fold_custom_accumulator_with_state() {
    #[derive(Debug)]
    struct Counter {
        total: usize,
        evens: usize,
        odds: usize,
    }

    let pattern = Pattern::pattern(
        1,
        vec![
            Pattern::point(2),
            Pattern::point(3),
            Pattern::point(4),
            Pattern::point(5),
        ],
    );

    let counter = pattern.fold(
        Counter {
            total: 0,
            evens: 0,
            odds: 0,
        },
        |mut acc, &v| {
            acc.total += 1;
            if v % 2 == 0 {
                acc.evens += 1;
            } else {
                acc.odds += 1;
            }
            acc
        },
    );

    assert_eq!(counter.total, 5);
    assert_eq!(counter.evens, 2); // 2, 4
    assert_eq!(counter.odds, 3); // 1, 3, 5
}
