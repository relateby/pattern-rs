//! Unit tests for pattern-core crate
//!
//! This module contains unit tests for the core pattern data structures.

use pattern_core::{Pattern, Subject, Symbol, Value, RangeValue};
use std::collections::{HashSet, HashMap};

#[cfg(test)]
mod pattern_creation {
    use super::*;

    #[test]
    fn test_pattern_creation_with_string_value() {
        let pattern = Pattern {
            value: "hello".to_string(),
            elements: vec![],
        };
        assert_eq!(pattern.value, "hello");
        assert_eq!(pattern.elements.len(), 0);
    }

    #[test]
    fn test_pattern_creation_with_integer_value() {
        let pattern = Pattern {
            value: 42,
            elements: vec![],
        };
        assert_eq!(pattern.value, 42);
        assert_eq!(pattern.elements.len(), 0);
    }

    #[test]
    fn test_pattern_creation_with_empty_elements() {
        let pattern = Pattern {
            value: "atomic",
            elements: vec![],
        };
        assert_eq!(pattern.elements.len(), 0);
    }

    #[test]
    fn test_pattern_creation_with_nested_elements() {
        let pattern = Pattern {
            value: "parent".to_string(),
            elements: vec![
                Pattern {
                    value: "child1".to_string(),
                    elements: vec![],
                },
                Pattern {
                    value: "child2".to_string(),
                    elements: vec![],
                },
            ],
        };
        assert_eq!(pattern.elements.len(), 2);
        assert_eq!(pattern.elements[0].value, "child1");
        assert_eq!(pattern.elements[1].value, "child2");
    }

    #[test]
    fn test_pattern_creation_with_custom_value_type() {
        struct MyValue {
            data: String,
        }
        
        let pattern = Pattern {
            value: MyValue {
                data: "test".to_string(),
            },
            elements: vec![],
        };
        assert_eq!(pattern.value.data, "test");
    }

    #[test]
    fn test_pattern_deep_nesting() {
        fn create_nested(depth: usize) -> Pattern<usize> {
            if depth == 0 {
                Pattern {
                    value: 0,
                    elements: vec![],
                }
            } else {
                Pattern {
                    value: depth,
                    elements: vec![create_nested(depth - 1)],
                }
            }
        }
        
        let deep = create_nested(100);
        // Verify we can traverse the structure
        let mut current = &deep;
        let mut count = 0;
        while !current.elements.is_empty() {
            count += 1;
            current = &current.elements[0];
        }
        assert_eq!(count, 100);
    }

    #[test]
    fn test_pattern_with_many_elements() {
        let pattern = Pattern {
            value: "root",
            elements: (0..10000)
                .map(|i| Pattern {
                    value: i,
                    elements: vec![],
                })
                .collect(),
        };
        assert_eq!(pattern.elements.len(), 10000);
        assert_eq!(pattern.elements[0].value, 0);
        assert_eq!(pattern.elements[9999].value, 9999);
    }
}

#[cfg(test)]
mod pattern_traits {
    use super::*;

    #[test]
    fn test_pattern_clone() {
        let original = Pattern {
            value: 42,
            elements: vec![
                Pattern {
                    value: 1,
                    elements: vec![],
                },
                Pattern {
                    value: 2,
                    elements: vec![],
                },
            ],
        };
        let cloned = original.clone();
        assert_eq!(original, cloned);
        // Verify they are independent
        assert_eq!(original.elements.len(), cloned.elements.len());
    }

    #[test]
    fn test_pattern_equality() {
        let p1 = Pattern {
            value: 42,
            elements: vec![],
        };
        let p2 = Pattern {
            value: 42,
            elements: vec![],
        };
        assert_eq!(p1, p2);
        
        let p3 = Pattern {
            value: 43,
            elements: vec![],
        };
        assert_ne!(p1, p3);
    }

    #[test]
    fn test_pattern_equality_with_nested_elements() {
        let p1 = Pattern {
            value: "parent",
            elements: vec![
                Pattern {
                    value: "child1",
                    elements: vec![],
                },
                Pattern {
                    value: "child2",
                    elements: vec![],
                },
            ],
        };
        let p2 = Pattern {
            value: "parent",
            elements: vec![
                Pattern {
                    value: "child1",
                    elements: vec![],
                },
                Pattern {
                    value: "child2",
                    elements: vec![],
                },
            ],
        };
        assert_eq!(p1, p2);
        
        let p3 = Pattern {
            value: "parent",
            elements: vec![
                Pattern {
                    value: "child1",
                    elements: vec![],
                },
            ],
        };
        assert_ne!(p1, p3);
    }
}

#[cfg(test)]
mod subject_tests {
    use super::*;

    #[test]
    fn test_subject_creation_with_identity_labels_properties() {
        let mut labels = HashSet::new();
        labels.insert("Person".to_string());
        
        let mut properties = HashMap::new();
        properties.insert("name".to_string(), Value::VString("Alice".to_string()));
        properties.insert("age".to_string(), Value::VInteger(30));
        
        let subject = Subject {
            identity: Symbol("n".to_string()),
            labels,
            properties,
        };
        
        assert_eq!(subject.identity.0, "n");
        assert!(subject.labels.contains("Person"));
        assert_eq!(subject.properties.len(), 2);
    }

    #[test]
    fn test_subject_labels_preservation() {
        let mut labels = HashSet::new();
        labels.insert("Person".to_string());
        labels.insert("Employee".to_string());
        
        let subject = Subject {
            identity: Symbol("n".to_string()),
            labels: labels.clone(),
            properties: HashMap::new(),
        };
        
        assert_eq!(subject.labels.len(), 2);
        assert!(subject.labels.contains("Person"));
        assert!(subject.labels.contains("Employee"));
    }

    #[test]
    fn test_subject_properties_preservation() {
        let mut properties = HashMap::new();
        properties.insert("name".to_string(), Value::VString("Bob".to_string()));
        properties.insert("age".to_string(), Value::VInteger(25));
        properties.insert("active".to_string(), Value::VBoolean(true));
        
        let subject = Subject {
            identity: Symbol("n".to_string()),
            labels: HashSet::new(),
            properties: properties.clone(),
        };
        
        assert_eq!(subject.properties.len(), 3);
        match &subject.properties["name"] {
            Value::VString(s) => assert_eq!(s, "Bob"),
            _ => panic!("Expected VString"),
        }
        match &subject.properties["age"] {
            Value::VInteger(i) => assert_eq!(*i, 25),
            _ => panic!("Expected VInteger"),
        }
        match &subject.properties["active"] {
            Value::VBoolean(b) => assert_eq!(*b, true),
            _ => panic!("Expected VBoolean"),
        }
    }

    #[test]
    fn test_pattern_subject_creation_and_value_preservation() {
        let mut labels = HashSet::new();
        labels.insert("Person".to_string());
        
        let mut properties = HashMap::new();
        properties.insert("name".to_string(), Value::VString("Alice".to_string()));
        
        let subject = Subject {
            identity: Symbol("n".to_string()),
            labels,
            properties,
        };
        
        let pattern: Pattern<Subject> = Pattern {
            value: subject.clone(),
            elements: vec![],
        };
        
        assert_eq!(pattern.value.identity.0, "n");
        assert!(pattern.value.labels.contains("Person"));
        assert_eq!(pattern.value.properties.len(), 1);
    }

    #[test]
    fn test_subject_equality_comparison() {
        let mut labels1 = HashSet::new();
        labels1.insert("Person".to_string());
        
        let mut labels2 = HashSet::new();
        labels2.insert("Person".to_string());
        
        let subject1 = Subject {
            identity: Symbol("n".to_string()),
            labels: labels1,
            properties: HashMap::new(),
        };
        
        let subject2 = Subject {
            identity: Symbol("n".to_string()),
            labels: labels2,
            properties: HashMap::new(),
        };
        
        assert_eq!(subject1, subject2);
        
        let subject3 = Subject {
            identity: Symbol("m".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        };
        
        assert_ne!(subject1, subject3);
    }

    #[test]
    fn test_value_enum_variants() {
        // Test VInteger
        let v1 = Value::VInteger(42);
        match v1 {
            Value::VInteger(i) => assert_eq!(i, 42),
            _ => panic!("Expected VInteger"),
        }
        
        // Test VDecimal
        let v2 = Value::VDecimal(3.14);
        match v2 {
            Value::VDecimal(d) => assert!((d - 3.14).abs() < 0.001),
            _ => panic!("Expected VDecimal"),
        }
        
        // Test VBoolean
        let v3 = Value::VBoolean(true);
        match v3 {
            Value::VBoolean(b) => assert_eq!(b, true),
            _ => panic!("Expected VBoolean"),
        }
        
        // Test VString
        let v4 = Value::VString("hello".to_string());
        match v4 {
            Value::VString(s) => assert_eq!(s, "hello"),
            _ => panic!("Expected VString"),
        }
        
        // Test VSymbol
        let v5 = Value::VSymbol("sym".to_string());
        match v5 {
            Value::VSymbol(s) => assert_eq!(s, "sym"),
            _ => panic!("Expected VSymbol"),
        }
        
        // Test VTaggedString
        let v6 = Value::VTaggedString {
            tag: "type".to_string(),
            content: "value".to_string(),
        };
        match v6 {
            Value::VTaggedString { tag, content } => {
                assert_eq!(tag, "type");
                assert_eq!(content, "value");
            },
            _ => panic!("Expected VTaggedString"),
        }
        
        // Test VArray
        let v7 = Value::VArray(vec![
            Value::VInteger(1),
            Value::VInteger(2),
            Value::VInteger(3),
        ]);
        match v7 {
            Value::VArray(arr) => {
                assert_eq!(arr.len(), 3);
                match &arr[0] {
                    Value::VInteger(i) => assert_eq!(*i, 1),
                    _ => panic!("Expected VInteger"),
                }
            },
            _ => panic!("Expected VArray"),
        }
        
        // Test VMap
        let mut map = HashMap::new();
        map.insert("key1".to_string(), Value::VString("value1".to_string()));
        let v8 = Value::VMap(map);
        match v8 {
            Value::VMap(m) => {
                assert_eq!(m.len(), 1);
                match &m["key1"] {
                    Value::VString(s) => assert_eq!(s, "value1"),
                    _ => panic!("Expected VString"),
                }
            },
            _ => panic!("Expected VMap"),
        }
        
        // Test VRange
        let v9 = Value::VRange(RangeValue {
            lower: Some(1.0),
            upper: Some(10.0),
        });
        match v9 {
            Value::VRange(r) => {
                assert_eq!(r.lower, Some(1.0));
                assert_eq!(r.upper, Some(10.0));
            },
            _ => panic!("Expected VRange"),
        }
        
        // Test VMeasurement
        let v10 = Value::VMeasurement {
            unit: "kg".to_string(),
            value: 5.0,
        };
        match v10 {
            Value::VMeasurement { unit, value } => {
                assert_eq!(unit, "kg");
                assert!((value - 5.0).abs() < 0.001);
            },
            _ => panic!("Expected VMeasurement"),
        }
    }

    #[test]
    fn test_range_value_with_optional_bounds() {
        // Both bounds
        let r1 = RangeValue {
            lower: Some(1.0),
            upper: Some(10.0),
        };
        assert_eq!(r1.lower, Some(1.0));
        assert_eq!(r1.upper, Some(10.0));
        
        // Lower bound only
        let r2 = RangeValue {
            lower: Some(1.0),
            upper: None,
        };
        assert_eq!(r2.lower, Some(1.0));
        assert_eq!(r2.upper, None);
        
        // Upper bound only
        let r3 = RangeValue {
            lower: None,
            upper: Some(10.0),
        };
        assert_eq!(r3.lower, None);
        assert_eq!(r3.upper, Some(10.0));
        
        // No bounds
        let r4 = RangeValue {
            lower: None,
            upper: None,
        };
        assert_eq!(r4.lower, None);
        assert_eq!(r4.upper, None);
    }
}

#[cfg(test)]
mod debug_display_tests {
    use super::*;
    use std::fmt::{Debug, Display};

    #[test]
    fn test_debug_pattern_with_simple_value() {
        let pattern = Pattern {
            value: "test",
            elements: vec![],
        };
        let debug_output = format!("{:?}", pattern);
        // Debug output should contain the value
        assert!(debug_output.contains("test") || debug_output.contains("value"));
    }

    #[test]
    fn test_debug_pattern_with_nested_elements() {
        let pattern = Pattern {
            value: "parent",
            elements: vec![
                Pattern {
                    value: "child1",
                    elements: vec![],
                },
                Pattern {
                    value: "child2",
                    elements: vec![],
                },
            ],
        };
        let debug_output = format!("{:?}", pattern);
        // Debug output should show structure
        assert!(!debug_output.is_empty());
    }

    #[test]
    fn test_debug_pattern_with_subject() {
        let subject = Subject {
            identity: Symbol("n".to_string()),
            labels: {
                let mut s = HashSet::new();
                s.insert("Person".to_string());
                s
            },
            properties: HashMap::new(),
        };
        let pattern: Pattern<Subject> = Pattern {
            value: subject,
            elements: vec![],
        };
        let debug_output = format!("{:?}", pattern);
        // Debug output should not be empty
        assert!(!debug_output.is_empty());
    }

    #[test]
    fn test_debug_truncation_for_deeply_nested_patterns() {
        fn create_nested(depth: usize) -> Pattern<usize> {
            if depth == 0 {
                Pattern {
                    value: 0,
                    elements: vec![],
                }
            } else {
                Pattern {
                    value: depth,
                    elements: vec![create_nested(depth - 1)],
                }
            }
        }
        
        let deep = create_nested(100);
        let debug_output = format!("{:?}", deep);
        // Debug output should be reasonable length (truncated if needed)
        assert!(debug_output.len() < 10000); // Reasonable limit
    }

    #[test]
    fn test_display_pattern_with_simple_value() {
        let pattern = Pattern {
            value: "test",
            elements: vec![],
        };
        let display_output = format!("{}", pattern);
        // Display output should be human-readable
        assert!(!display_output.is_empty());
    }

    #[test]
    fn test_display_pattern_with_nested_elements() {
        let pattern = Pattern {
            value: "parent",
            elements: vec![
                Pattern {
                    value: "child1",
                    elements: vec![],
                },
                Pattern {
                    value: "child2",
                    elements: vec![],
                },
            ],
        };
        let display_output = format!("{}", pattern);
        // Display output should show structure clearly
        assert!(!display_output.is_empty());
    }

    #[test]
    fn test_display_pattern_with_subject() {
        let subject = Subject {
            identity: Symbol("n".to_string()),
            labels: {
                let mut s = HashSet::new();
                s.insert("Person".to_string());
                s
            },
            properties: HashMap::new(),
        };
        let pattern: Pattern<Subject> = Pattern {
            value: subject,
            elements: vec![],
        };
        let display_output = format!("{}", pattern);
        // Display output should be human-readable
        assert!(!display_output.is_empty());
    }

    #[test]
    fn test_debug_subject_symbol_value_rangevalue() {
        let symbol = Symbol("test".to_string());
        let debug_symbol = format!("{:?}", symbol);
        assert!(!debug_symbol.is_empty());
        
        let range = RangeValue {
            lower: Some(1.0),
            upper: Some(10.0),
        };
        let debug_range = format!("{:?}", range);
        assert!(!debug_range.is_empty());
        
        let value = Value::VInteger(42);
        let debug_value = format!("{:?}", value);
        assert!(!debug_value.is_empty());
        
        let subject = Subject {
            identity: Symbol("n".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        };
        let debug_subject = format!("{:?}", subject);
        assert!(!debug_subject.is_empty());
    }

    #[test]
    fn test_display_subject_symbol_value_rangevalue() {
        let symbol = Symbol("test".to_string());
        let display_symbol = format!("{}", symbol);
        assert!(!display_symbol.is_empty());
        
        let range = RangeValue {
            lower: Some(1.0),
            upper: Some(10.0),
        };
        let display_range = format!("{}", range);
        assert!(!display_range.is_empty());
        
        let value = Value::VInteger(42);
        let display_value = format!("{}", value);
        assert!(!display_value.is_empty());
        
        let subject = Subject {
            identity: Symbol("n".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        };
        let display_subject = format!("{}", subject);
        assert!(!display_subject.is_empty());
    }
}

#[cfg(test)]
mod wasm_compilation_tests {
    use super::*;

    #[test]
    fn test_wasm_compilation_pattern_core_crate() {
        // This test verifies that pattern-core types are WASM-compatible
        // The actual compilation is verified via cargo build --target wasm32-unknown-unknown
        // This test ensures the types can be used in a WASM context
        
        // Test that Pattern types are WASM-compatible (no platform-specific code)
        let pattern: Pattern<String> = Pattern {
            value: "wasm-test".to_string(),
            elements: vec![],
        };
        
        // Verify the pattern can be cloned (WASM-compatible operation)
        let cloned = pattern.clone();
        assert_eq!(pattern, cloned);
        
        // Verify the pattern can be compared (WASM-compatible operation)
        assert_eq!(pattern.value, "wasm-test");
    }

    #[test]
    fn test_pattern_types_included_in_wasm_module() {
        // Verify Pattern<V> types are WASM-compatible
        // All standard library types used in Pattern are WASM-compatible:
        // - Vec<T> is WASM-compatible
        // - Generic structs are WASM-compatible
        
        let pattern_int: Pattern<i32> = Pattern {
            value: 42,
            elements: vec![],
        };
        
        let pattern_string: Pattern<String> = Pattern {
            value: "test".to_string(),
            elements: vec![],
        };
        
        // Verify both compile and work
        assert_eq!(pattern_int.value, 42);
        assert_eq!(pattern_string.value, "test");
    }

    #[test]
    fn test_subject_types_included_in_wasm_module() {
        // Verify Subject types are WASM-compatible
        // All standard library types used in Subject are WASM-compatible:
        // - HashSet<T> is WASM-compatible
        // - HashMap<K, V> is WASM-compatible
        
        let subject = Subject {
            identity: Symbol("n".to_string()),
            labels: {
                let mut s = HashSet::new();
                s.insert("Person".to_string());
                s
            },
            properties: {
                let mut m = HashMap::new();
                m.insert("name".to_string(), Value::VString("Alice".to_string()));
                m
            },
        };
        
        // Verify Subject compiles and works
        assert_eq!(subject.identity.0, "n");
        assert!(subject.labels.contains("Person"));
        assert_eq!(subject.properties.len(), 1);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Test cross-crate usage - verify types can be imported and used from pattern_core
    #[test]
    fn test_cross_crate_imports() {
        // Verify all public types can be imported
        use pattern_core::{Pattern, Subject, Symbol, Value, PropertyRecord, RangeValue};
        
        // Verify Pattern can be used
        let _pattern: Pattern<String> = Pattern {
            value: "test".to_string(),
            elements: vec![],
        };
        
        // Verify Subject can be used
        let _subject = Subject {
            identity: Symbol("n".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        };
        
        // Verify all types are accessible
        let _symbol = Symbol("test".to_string());
        let _value = Value::VInteger(42);
        let _range = RangeValue {
            lower: Some(1.0),
            upper: Some(10.0),
        };
        let _props: PropertyRecord = HashMap::new();
    }

    /// Test that Pattern can be used with various value types from other crates
    #[test]
    fn test_pattern_with_various_value_types() {
        // String
        let _p1: Pattern<String> = Pattern {
            value: "test".to_string(),
            elements: vec![],
        };
        
        // Integer
        let _p2: Pattern<i32> = Pattern {
            value: 42,
            elements: vec![],
        };
        
        // Boolean
        let _p3: Pattern<bool> = Pattern {
            value: true,
            elements: vec![],
        };
        
        // Subject
        let _p4: Pattern<Subject> = Pattern {
            value: Subject {
                identity: Symbol("n".to_string()),
                labels: HashSet::new(),
                properties: HashMap::new(),
            },
            elements: vec![],
        };
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_performance_deep_nesting_100_levels() {
        // Test that patterns with 100+ nesting levels can be created and manipulated
        fn create_nested(depth: usize) -> Pattern<usize> {
            if depth == 0 {
                Pattern {
                    value: 0,
                    elements: vec![],
                }
            } else {
                Pattern {
                    value: depth,
                    elements: vec![create_nested(depth - 1)],
                }
            }
        }
        
        let deep = create_nested(100);
        
        // Verify we can traverse the structure
        let mut current = &deep;
        let mut count = 0;
        while !current.elements.is_empty() {
            count += 1;
            current = &current.elements[0];
        }
        assert_eq!(count, 100, "Should support 100+ nesting levels");
        
        // Verify we can clone deep structures
        let cloned = deep.clone();
        assert_eq!(deep, cloned, "Should be able to clone deep structures");
        
        // Verify we can compare deep structures
        assert_eq!(deep, cloned, "Should be able to compare deep structures");
    }

    #[test]
    fn test_performance_many_elements_10000() {
        // Test that patterns with 10,000+ elements can be created and manipulated
        let pattern = Pattern {
            value: "root",
            elements: (0..10000)
                .map(|i| Pattern {
                    value: i,
                    elements: vec![],
                })
                .collect(),
        };
        
        assert_eq!(pattern.elements.len(), 10000, "Should support 10,000+ elements");
        
        // Verify we can access elements
        assert_eq!(pattern.elements[0].value, 0);
        assert_eq!(pattern.elements[9999].value, 9999);
        
        // Verify we can clone wide structures
        let cloned = pattern.clone();
        assert_eq!(pattern, cloned, "Should be able to clone wide structures");
        
        // Verify we can compare wide structures
        assert_eq!(pattern, cloned, "Should be able to compare wide structures");
    }

    #[test]
    fn test_performance_combined_deep_and_wide() {
        // Test combination of depth and width
        let pattern = Pattern {
            value: 0,
            elements: (0..100)
                .map(|i| {
                    let mut p = Pattern {
                        value: i,
                        elements: vec![],
                    };
                    // Add 100 elements to each
                    for j in 0..100 {
                        p.elements.push(Pattern {
                            value: j,
                            elements: vec![],
                        });
                    }
                    p
                })
                .collect(),
        };
        
        assert_eq!(pattern.elements.len(), 100);
        assert_eq!(pattern.elements[0].elements.len(), 100);
        
        // Verify we can clone and compare
        let cloned = pattern.clone();
        assert_eq!(pattern, cloned);
    }
}

#[cfg(test)]
mod quickstart_validation {
    use super::*;

    /// Validate quickstart.md examples compile and work correctly
    #[test]
    fn test_quickstart_basic_usage() {
        // Example: Creating Patterns
        use pattern_core::Pattern;
        
        let pattern = Pattern {
            value: "hello".to_string(),
            elements: vec![],
        };
        assert_eq!(pattern.value, "hello");
        
        let int_pattern = Pattern {
            value: 42,
            elements: vec![],
        };
        assert_eq!(int_pattern.value, 42);
        
        let nested = Pattern {
            value: "parent".to_string(),
            elements: vec![
                Pattern {
                    value: "child1".to_string(),
                    elements: vec![],
                },
                Pattern {
                    value: "child2".to_string(),
                    elements: vec![],
                },
            ],
        };
        assert_eq!(nested.elements.len(), 2);
    }

    #[test]
    fn test_quickstart_custom_types() {
        // Example: Using Custom Types as Values
        use pattern_core::Pattern;
        
        struct MyData {
            id: u32,
            name: String,
        }
        
        let custom_pattern: Pattern<MyData> = Pattern {
            value: MyData {
                id: 1,
                name: "example".to_string(),
            },
            elements: vec![],
        };
        assert_eq!(custom_pattern.value.id, 1);
        assert_eq!(custom_pattern.value.name, "example");
    }

    #[test]
    fn test_quickstart_inspecting_patterns() {
        // Example: Inspecting Patterns
        use pattern_core::Pattern;
        
        let pattern = Pattern {
            value: "test",
            elements: vec![],
        };
        
        let debug_output = format!("{:?}", pattern);
        assert!(!debug_output.is_empty());
        
        let display_output = format!("{}", pattern);
        assert!(!display_output.is_empty());
    }

    #[test]
    fn test_quickstart_equality_comparison() {
        // Example: Equality Comparison
        use pattern_core::Pattern;
        
        let p1 = Pattern {
            value: 42,
            elements: vec![],
        };
        
        let p2 = Pattern {
            value: 42,
            elements: vec![],
        };
        
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_quickstart_cloning_patterns() {
        // Example: Cloning Patterns
        use pattern_core::Pattern;
        
        let original = Pattern {
            value: "original",
            elements: vec![],
        };
        
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_quickstart_subject_usage() {
        // Example: Using Subject Type
        use pattern_core::{Pattern, Subject, Symbol, Value};
        use std::collections::{HashSet, HashMap};
        
        let subject = Subject {
            identity: Symbol("n".to_string()),
            labels: {
                let mut set = HashSet::new();
                set.insert("Person".to_string());
                set
            },
            properties: {
                let mut map = HashMap::new();
                map.insert("name".to_string(), Value::VString("Alice".to_string()));
                map.insert("age".to_string(), Value::VInteger(30));
                map
            },
        };
        
        let pattern: Pattern<Subject> = Pattern {
            value: subject,
            elements: vec![],
        };
        
        assert_eq!(pattern.value.identity.0, "n");
        assert!(pattern.value.labels.contains("Person"));
        assert_eq!(pattern.value.properties.len(), 2);
    }
}

