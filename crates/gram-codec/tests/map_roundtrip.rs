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
