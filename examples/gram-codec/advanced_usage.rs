//! Advanced usage examples for the gram codec
//!
//! Run with: `cargo run --package gram-codec --example advanced_usage`

use gram_codec::{parse_gram_notation, serialize_pattern};
use pattern_core::{Pattern, Subject, Symbol};
use std::collections::{HashMap, HashSet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Gram Codec Advanced Usage Examples ===\n");

    // Example 1: Complex relationship with properties
    println!("1. Complex relationship:");
    let gram_text = "(alice:Person {name: \"Alice\", age: 30})-[:KNOWS {since: 2020}]->(bob:Person {name: \"Bob\", age: 25})";
    let patterns = parse_gram_notation(gram_text)?;
    println!("   Parsed complex relationship");
    println!("   Left node: {}", patterns[0].elements[0].value.identity.0);
    println!("   Edge labels: {:?}", patterns[0].value.labels);
    println!(
        "   Right node: {}\n",
        patterns[0].elements[1].value.identity.0
    );

    // Example 2: Nested subject patterns
    println!("2. Nested subject patterns:");
    let gram_text = "[outer:Group | [inner:Team | (alice), (bob)], (charlie)]";
    let patterns = parse_gram_notation(gram_text)?;
    println!("   Outer pattern: {}", patterns[0].value.identity.0);
    println!("   Outer elements: {}", patterns[0].elements.len());
    println!(
        "   First element is nested: {}\n",
        !patterns[0].elements[0].elements.is_empty()
    );

    // Example 3: Path patterns (chained relationships)
    println!("3. Path patterns:");
    let gram_text = "(a)-->(b)-->(c)-->(d)";
    let patterns = parse_gram_notation(gram_text)?;
    println!("   Input: {}", gram_text);
    println!("   Creates nested relationship structure");
    println!("   Top-level patterns: {}\n", patterns.len());

    // Example 4: Working with different value types
    println!("4. Different property value types:");
    let mut subject = Subject {
        identity: Symbol("node".to_string()),
        labels: HashSet::new(),
        properties: HashMap::new(),
    };

    // Add various value types
    subject.properties.insert(
        "name".to_string(),
        pattern_core::Value::VString("Alice".to_string()),
    );
    subject
        .properties
        .insert("age".to_string(), pattern_core::Value::VInteger(30));
    subject
        .properties
        .insert("score".to_string(), pattern_core::Value::VDecimal(95.5));
    subject
        .properties
        .insert("active".to_string(), pattern_core::Value::VBoolean(true));
    subject.properties.insert(
        "tags".to_string(),
        pattern_core::Value::VArray(vec![
            pattern_core::Value::VString("rust".to_string()),
            pattern_core::Value::VString("wasm".to_string()),
        ]),
    );

    let pattern = Pattern::point(subject);
    let gram_output = serialize_pattern(&pattern)?;
    println!("   Serialized with mixed types:");
    println!("   {}\n", gram_output);

    // Example 5: Annotations
    println!("5. Annotated patterns:");
    let gram_text = "@type(node) @depth(2) (leaf)";
    let patterns = parse_gram_notation(gram_text)?;
    println!("   Input: {}", gram_text);
    println!(
        "   Annotation properties: {}",
        patterns[0].value.properties.len()
    );
    println!("   Element count: {}\n", patterns[0].elements.len());

    // Example 6: Multiple labels
    println!("6. Multiple labels:");
    let gram_text = "(alice:Person:Employee:Manager)";
    let patterns = parse_gram_notation(gram_text)?;
    println!("   Input: {}", gram_text);
    println!("   Labels: {:?}\n", patterns[0].value.labels);

    // Example 7: Unicode and special characters
    println!("7. Unicode support:");
    let gram_text = "(\"世界\" {greeting: \"こんにちは\"})";
    let patterns = parse_gram_notation(gram_text)?;
    println!("   Input: {}", gram_text);
    println!("   Identifier: {}", patterns[0].value.identity.0);
    println!(
        "   Unicode property present: {}\n",
        patterns[0].value.properties.contains_key("greeting")
    );

    // Example 8: Building patterns programmatically
    println!("8. Building patterns programmatically:");

    // Create two nodes
    let alice = Pattern::point(Subject {
        identity: Symbol("alice".to_string()),
        labels: {
            let mut labels = HashSet::new();
            labels.insert("Person".to_string());
            labels
        },
        properties: HashMap::new(),
    });

    let bob = Pattern::point(Subject {
        identity: Symbol("bob".to_string()),
        labels: {
            let mut labels = HashSet::new();
            labels.insert("Person".to_string());
            labels
        },
        properties: HashMap::new(),
    });

    // Create a relationship
    let relationship = Pattern::pattern(
        Subject {
            identity: Symbol(String::new()),
            labels: {
                let mut labels = HashSet::new();
                labels.insert("KNOWS".to_string());
                labels
            },
            properties: HashMap::new(),
        },
        vec![alice, bob],
    );

    let gram_output = serialize_pattern(&relationship)?;
    println!("   Built relationship: {}\n", gram_output);

    // Example 9: Comments and whitespace
    println!("9. Handling comments:");
    let gram_text = "// This is a comment\n(alice) // Alice node\n// Another comment\n(bob)";
    let patterns = parse_gram_notation(gram_text)?;
    println!("   Input has comments");
    println!("   Parsed {} patterns (comments ignored)\n", patterns.len());

    // Example 10: Error recovery
    println!("10. Error recovery:");
    let invalid_gram = "(a {key: }) (b)";
    match parse_gram_notation(invalid_gram) {
        Ok(_) => println!("   Parsed despite issues"),
        Err(e) => {
            println!("   Parse error: {}", e);
        }
    }

    println!("\n=== Advanced Examples Complete ===");
    Ok(())
}
