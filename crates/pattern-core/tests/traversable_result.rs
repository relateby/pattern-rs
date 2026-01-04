//! Tests for Pattern traverse_result operations
//!
//! This module tests the traverse_result method for Pattern<V>, which applies
//! an effectful function returning Result<W, E> to all values in the pattern.

use pattern_core::Pattern;

// ====================================================================================
// Basic Result Tests
// ====================================================================================

#[test]
fn test_traverse_result_atomic_ok() {
    let p = Pattern::point(10);
    let result: Result<Pattern<i32>, String> = p.traverse_result(|v| Ok(v * 2));
    assert_eq!(result, Ok(Pattern::point(20)));
}

#[test]
fn test_traverse_result_atomic_err() {
    let p = Pattern::point(10);
    let result: Result<Pattern<i32>, String> = p.traverse_result(|v| {
        if *v > 5 {
            Ok(v * 2)
        } else {
            Err("too small".to_string())
        }
    });
    assert_eq!(result, Ok(Pattern::point(20)));

    let p_err = Pattern::point(3);
    let result_err: Result<Pattern<i32>, String> = p_err.traverse_result(|v| {
        if *v > 5 {
            Ok(v * 2)
        } else {
            Err("too small".to_string())
        }
    });
    assert_eq!(result_err, Err("too small".to_string()));
}

#[test]
fn test_traverse_result_nested_all_ok() {
    let p = Pattern::pattern(1, vec![Pattern::point(2), Pattern::point(3)]);
    let result: Result<Pattern<i32>, String> = p.traverse_result(|v| Ok(v * 10));
    assert_eq!(
        result,
        Ok(Pattern::pattern(
            10,
            vec![Pattern::point(20), Pattern::point(30)]
        ))
    );
}

#[test]
fn test_traverse_result_nested_with_err() {
    let p = Pattern::pattern(
        10,
        vec![Pattern::point(20), Pattern::point(3)], // 3 will cause Err
    );
    let result: Result<Pattern<i32>, String> = p.traverse_result(|v| {
        if *v > 5 {
            Ok(v * 2)
        } else {
            Err(format!("value {} too small", v))
        }
    });
    assert_eq!(result, Err("value 3 too small".to_string()));
}

// ====================================================================================
// Short-Circuit Tests
// ====================================================================================

#[test]
fn test_traverse_result_short_circuit() {
    use pattern_core::test_utils::helpers::EffectCounter;

    // Create a pattern with 5 elements
    let p = Pattern::pattern(
        1,
        vec![
            Pattern::point(2),
            Pattern::point(3),
            Pattern::point(4), // This will error
            Pattern::point(5),
            Pattern::point(6),
        ],
    );

    let counter = EffectCounter::new();
    let result: Result<Pattern<i32>, String> = p.traverse_result(|v| {
        counter.increment();
        if *v < 4 {
            Ok(v * 10)
        } else {
            Err(format!("value {} too large", v))
        }
    });

    // Should fail at element 4
    assert!(result.is_err());
    assert_eq!(result, Err("value 4 too large".to_string()));

    // Should have called the function 3 times: root (1), child (2), child (3), child (4)
    // Should NOT have called it for elements 5 and 6 (short-circuit)
    assert_eq!(counter.count(), 4);
}

#[test]
fn test_traverse_result_root_error_immediate_short_circuit() {
    use pattern_core::test_utils::helpers::EffectCounter;

    // Create a pattern where the root will error
    let p = Pattern::pattern(
        10, // This will error
        vec![Pattern::point(2), Pattern::point(3)],
    );

    let counter = EffectCounter::new();
    let result: Result<Pattern<i32>, String> = p.traverse_result(|v| {
        counter.increment();
        if *v < 5 {
            Ok(v * 10)
        } else {
            Err(format!("value {} too large", v))
        }
    });

    // Should fail at root
    assert!(result.is_err());
    assert_eq!(result, Err("value 10 too large".to_string()));

    // Should have called the function only once (root), not for children
    assert_eq!(counter.count(), 1);
}

// ====================================================================================
// Type Transformation Tests
// ====================================================================================

#[test]
fn test_traverse_result_type_change() {
    let p = Pattern::pattern("1", vec![Pattern::point("2"), Pattern::point("3")]);
    let result: Result<Pattern<i32>, String> =
        p.traverse_result(|s| s.parse::<i32>().map_err(|e| format!("parse error: {}", e)));

    assert!(result.is_ok());
    let pattern = result.unwrap();
    assert_eq!(pattern.value, 1);
    assert_eq!(pattern.elements[0].value, 2);
    assert_eq!(pattern.elements[1].value, 3);
}

#[test]
fn test_traverse_result_type_change_with_error() {
    let p = Pattern::pattern("1", vec![Pattern::point("invalid"), Pattern::point("3")]);
    let result: Result<Pattern<i32>, String> =
        p.traverse_result(|s| s.parse::<i32>().map_err(|e| format!("parse error: {}", e)));

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("parse error"));
}

// ====================================================================================
// Deep Nesting Tests
// ====================================================================================

#[test]
fn test_traverse_result_deep_nesting() {
    // Create a deeply nested pattern: 1 -> 2 -> 3 -> 4 -> 5
    let p = Pattern::pattern(
        1,
        vec![Pattern::pattern(
            2,
            vec![Pattern::pattern(
                3,
                vec![Pattern::pattern(4, vec![Pattern::point(5)])],
            )],
        )],
    );

    let result: Result<Pattern<i32>, String> = p.traverse_result(|v| Ok(v * 10));
    assert!(result.is_ok());

    let pattern = result.unwrap();
    assert_eq!(pattern.value, 10);
    assert_eq!(pattern.elements[0].value, 20);
    assert_eq!(pattern.elements[0].elements[0].value, 30);
    assert_eq!(pattern.elements[0].elements[0].elements[0].value, 40);
    assert_eq!(
        pattern.elements[0].elements[0].elements[0].elements[0].value,
        50
    );
}

#[test]
fn test_traverse_result_deep_nesting_with_error() {
    // Create a deeply nested pattern: 1 -> 2 -> 3 -> 4 -> 5
    // Error on value 3
    let p = Pattern::pattern(
        1,
        vec![Pattern::pattern(
            2,
            vec![Pattern::pattern(
                3, // This will error
                vec![Pattern::pattern(4, vec![Pattern::point(5)])],
            )],
        )],
    );

    let result: Result<Pattern<i32>, String> = p.traverse_result(|v| {
        if *v != 3 {
            Ok(v * 10)
        } else {
            Err("found 3".to_string())
        }
    });

    assert_eq!(result, Err("found 3".to_string()));
}

// ====================================================================================
// Sequence Result Tests (T040-T041)
// ====================================================================================

/// T040: Pattern<Result<T, E>> with all Ok → Ok(Pattern<T>)
#[test]
fn sequence_result_all_ok() {
    let pattern: Pattern<Result<i32, String>> =
        Pattern::pattern(Ok(1), vec![Pattern::point(Ok(2)), Pattern::point(Ok(3))]);

    let result = pattern.sequence_result();

    assert!(result.is_ok());
    let unwrapped = result.unwrap();
    assert_eq!(unwrapped.value, 1);
    assert_eq!(unwrapped.elements[0].value, 2);
    assert_eq!(unwrapped.elements[1].value, 3);
}

/// T041: Pattern<Result<T, E>> with at least one Err → Err(E)
#[test]
fn sequence_result_with_err() {
    let pattern: Pattern<Result<i32, String>> = Pattern::pattern(
        Ok(1),
        vec![
            Pattern::point(Ok(2)),
            Pattern::point(Err("error here".to_string())), // This Err should propagate
            Pattern::point(Ok(3)),
        ],
    );

    let result = pattern.sequence_result();

    // All-or-nothing: any Err → Err
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "error here".to_string());
}

/// Test nested structure sequencing for Result
#[test]
fn sequence_result_nested_structure() {
    let pattern: Pattern<Result<i32, String>> = Pattern::pattern(
        Ok(1),
        vec![Pattern::pattern(
            Ok(2),
            vec![Pattern::point(Ok(3)), Pattern::point(Ok(4))],
        )],
    );

    let result = pattern.sequence_result();

    assert!(result.is_ok());
    let unwrapped = result.unwrap();
    assert_eq!(unwrapped.value, 1);
    assert_eq!(unwrapped.elements[0].value, 2);
    assert_eq!(unwrapped.elements[0].elements[0].value, 3);
    assert_eq!(unwrapped.elements[0].elements[1].value, 4);
}

/// Test that Err in root value short-circuits
#[test]
fn sequence_result_err_in_root() {
    let pattern: Pattern<Result<i32, String>> = Pattern::pattern(
        Err("root error".to_string()), // Root is Err
        vec![Pattern::point(Ok(2)), Pattern::point(Ok(3))],
    );

    let result = pattern.sequence_result();

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "root error".to_string());
}

/// Test atomic pattern with Ok
#[test]
fn sequence_result_atomic_ok() {
    let pattern: Pattern<Result<i32, String>> = Pattern::point(Ok(42));

    let result = pattern.sequence_result();

    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, 42);
}

/// Test atomic pattern with Err
#[test]
fn sequence_result_atomic_err() {
    let pattern: Pattern<Result<i32, String>> = Pattern::point(Err("error".to_string()));

    let result = pattern.sequence_result();

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "error".to_string());
}
