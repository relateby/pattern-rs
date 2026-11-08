//! Maps in record properties must survive parse -> serialize -> parse, and
//! shapes outside the grammar's depth budget must be refused by the serializer.

use gram_codec::{parse_gram, to_gram, Pattern, Subject};
use pattern_core::{Symbol, Value};
use std::collections::HashMap;

#[test]
fn map_values_round_trip() {
    for input in [
        r#"(a {m: {city: "P"}})"#,
        r#"(a {m: {n: "x", aliases: ["a", "b"]}})"#,
        r#"(a {m: {}})"#,
        r#"(a {`odd key`: {`k 1`: 1}})"#,
    ] {
        let first = parse_gram(input).unwrap();
        let text = to_gram(&first).unwrap_or_else(|e| panic!("serialize {input}: {e}"));
        let second = parse_gram(&text).unwrap_or_else(|e| panic!("reparse {text}: {e}"));
        assert_eq!(first, second, "round trip of {input} via {text}");
    }
}

fn pattern_with(value: Value) -> Pattern<Subject> {
    let mut properties = HashMap::new();
    properties.insert("m".to_string(), value);
    Pattern::point(Subject {
        identity: Symbol("a".to_string()),
        labels: Default::default(),
        properties,
    })
}

#[test]
fn serializer_rejects_maps_deeper_than_the_grammar_allows() {
    let inner = Value::VMap(HashMap::from([("j".to_string(), Value::VInteger(1))]));
    let cases = [
        Value::VMap(HashMap::from([("k".to_string(), inner.clone())])),
        Value::VMap(HashMap::from([(
            "k".to_string(),
            Value::VArray(vec![inner.clone()]),
        )])),
        Value::VArray(vec![inner]),
        Value::VArray(vec![Value::VArray(vec![Value::VInteger(1)])]),
    ];
    for value in cases {
        assert!(
            to_gram(&[pattern_with(value.clone())]).is_err(),
            "{value:?}"
        );
    }
}

#[test]
fn measurement_values_round_trip() {
    for input in [
        "(a {h: 168cm})",
        "(a {n: -5m, z: 0m})",
        r#"(a {m: {len: 5Km}, list: [1m, 2m]})"#,
        "(a {xs: [10kg, 20kg]})",
    ] {
        let first = parse_gram(input).unwrap();
        let text = to_gram(&first).unwrap_or_else(|e| panic!("serialize {input}: {e}"));
        assert!(
            !text.contains(".0"),
            "integer magnitudes stay integers: {text}"
        );
        let second = parse_gram(&text).unwrap_or_else(|e| panic!("reparse {text}: {e}"));
        assert_eq!(first, second, "round trip of {input} via {text}");
        #[cfg(feature = "cst")]
        {
            let cst = gram_codec::parse_gram_cst(&text);
            assert!(cst.errors.is_empty(), "CST rejects {text}");
        }
    }
}

#[test]
fn serializer_rejects_measurements_the_grammar_cannot_express() {
    let bad = [
        Value::VMeasurement {
            unit: "kg".into(),
            value: 3.5,
        },
        Value::VMeasurement {
            unit: "".into(),
            value: 1.0,
        },
        Value::VMeasurement {
            unit: "m2".into(),
            value: 1.0,
        },
        Value::VMeasurement {
            unit: "µm".into(),
            value: 1.0,
        },
        Value::VMeasurement {
            unit: "m".into(),
            value: f64::NAN,
        },
        Value::VMeasurement {
            unit: "m".into(),
            value: 1e300,
        },
    ];
    for value in bad {
        assert!(
            to_gram(&[pattern_with(value.clone())]).is_err(),
            "{value:?}"
        );
    }
}

#[test]
fn integer_looking_keys_are_quoted() {
    for input in [r#"(a {m: {`1`: "x"}})"#, r#"(a {`1`: "x", `-2`: 3})"#] {
        let first = parse_gram(input).unwrap();
        let text = to_gram(&first).unwrap();
        assert!(!text.contains("{1:") && !text.contains(" 1:"), "{text}");
        let second = parse_gram(&text).unwrap_or_else(|e| panic!("reparse {text}: {e}"));
        assert_eq!(first, second, "{input} via {text}");
        #[cfg(feature = "cst")]
        assert!(
            gram_codec::parse_gram_cst(&text).errors.is_empty(),
            "{text}"
        );
    }
}

#[test]
fn serializer_rejects_measurements_that_read_back_differently() {
    for unit in ["xff", "xAc"] {
        let value = Value::VMeasurement {
            unit: unit.into(),
            value: 0.0,
        };
        assert!(to_gram(&[pattern_with(value)]).is_err(), "0{unit}");
    }
}
