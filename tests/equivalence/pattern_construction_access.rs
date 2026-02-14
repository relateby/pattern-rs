//! Behavioral equivalence tests for pattern construction and access functions
//!
//! This module contains tests that verify behavioral equivalence between
//! pattern-rs and gram-hs implementations for construction, access, and inspection functions.

use pattern_core::Pattern;
use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod construction_equivalence_tests {
    use super::*;

    #[test]
    fn test_point_construction_equivalence() {
        // T044 [US4] Port test cases from gram-hs for point() construction
        // In gram-hs: point "hello" creates Pattern "hello" []
        let pattern_rs = Pattern::point("hello".to_string());
        let pattern_expected = Pattern {
            value: "hello".to_string(),
            elements: vec![],
        };
        assert_eq!(pattern_rs, pattern_expected);
    }

    #[test]
    fn test_pattern_construction_equivalence() {
        // T045 [US4] Port test cases from gram-hs for pattern() construction
        // In gram-hs: pattern "parent" [point "child1", point "child2"]
        let pattern_rs = Pattern::pattern(
            "parent".to_string(),
            vec![
                Pattern::point("child1".to_string()),
                Pattern::point("child2".to_string()),
            ],
        );
        let pattern_expected = Pattern {
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
        assert_eq!(pattern_rs, pattern_expected);
    }

    #[test]
    fn test_from_list_construction_equivalence() {
        // T046 [US4] Port test cases from gram-hs for fromList() construction
        // In gram-hs: fromList "root" ["a", "b", "c"]
        let pattern_rs = Pattern::from_list(
            "root".to_string(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
        );
        let pattern_expected = Pattern {
            value: "root".to_string(),
            elements: vec![
                Pattern {
                    value: "a".to_string(),
                    elements: vec![],
                },
                Pattern {
                    value: "b".to_string(),
                    elements: vec![],
                },
                Pattern {
                    value: "c".to_string(),
                    elements: vec![],
                },
            ],
        };
        assert_eq!(pattern_rs, pattern_expected);
    }
}

#[cfg(test)]
mod accessor_equivalence_tests {
    use super::*;

    #[test]
    fn test_value_accessor_equivalence() {
        // T047 [US4] Port test cases from gram-hs for value accessor
        // In gram-hs: value (pattern "hello") == "hello"
        let pattern = Pattern::point("hello".to_string());
        assert_eq!(pattern.value(), "hello");
    }

    #[test]
    fn test_elements_accessor_equivalence() {
        // T048 [US4] Port test cases from gram-hs for elements accessor
        // In gram-hs: elements (pattern "parent" [point "child"]) == [point "child"]
        let pattern = Pattern::pattern(
            "parent".to_string(),
            vec![Pattern::point("child".to_string())],
        );
        let elements = pattern.elements();
        assert_eq!(elements.len(), 1);
        assert_eq!(elements[0].value(), "child");
    }
}

#[cfg(test)]
mod inspection_equivalence_tests {
    use super::*;

    #[test]
    fn test_length_inspection_equivalence() {
        // T049 [US4] Port test cases from gram-hs for length() inspection
        // In gram-hs: length (pattern "parent" [point "c1", point "c2"]) == 2
        let pattern = Pattern::pattern(
            "parent".to_string(),
            vec![
                Pattern::point("c1".to_string()),
                Pattern::point("c2".to_string()),
            ],
        );
        assert_eq!(pattern.length(), 2);
    }

    #[test]
    fn test_size_inspection_equivalence() {
        // T050 [US4] Port test cases from gram-hs for size() inspection
        // In gram-hs: size (pattern "atom") == 1
        // In gram-hs: size (pattern "root" [point "c1", point "c2"]) == 3
        let atomic = Pattern::point("atom".to_string());
        assert_eq!(atomic.size(), 1);

        let pattern = Pattern::pattern(
            "root".to_string(),
            vec![
                Pattern::point("c1".to_string()),
                Pattern::point("c2".to_string()),
            ],
        );
        assert_eq!(pattern.size(), 3);
    }

    #[test]
    fn test_depth_inspection_equivalence() {
        // T051 [US4] Port test cases from gram-hs for depth() inspection
        // In gram-hs: depth (pattern "atom") == 0 (atomic patterns have depth 0)
        // In gram-hs: depth (pattern "p" [pattern "c" [point "gc"]]) == 2
        let atomic = Pattern::point("atom".to_string());
        assert_eq!(atomic.depth(), 0);

        let nested = Pattern::pattern(
            "p".to_string(),
            vec![Pattern::pattern(
                "c".to_string(),
                vec![Pattern::point("gc".to_string())],
            )],
        );
        assert_eq!(nested.depth(), 2);
    }
}

#[cfg(test)]
mod equivalence_utilities_tests {
    use super::*;

    #[test]
    fn test_equivalence_checking_utilities() {
        // T052 [US4] Create equivalence checking utilities for comparing pattern-rs and gram-hs patterns
        // Basic equivalence: same structure should be equivalent
        let p1 = Pattern::point("test".to_string());
        let p2 = Pattern::point("test".to_string());
        assert_eq!(p1, p2);

        // Nested equivalence
        let p3 = Pattern::pattern(
            "parent".to_string(),
            vec![Pattern::point("child".to_string())],
        );
        let p4 = Pattern::pattern(
            "parent".to_string(),
            vec![Pattern::point("child".to_string())],
        );
        assert_eq!(p3, p4);
    }
}
