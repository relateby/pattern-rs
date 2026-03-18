//! Basic usage examples for the gram codec
//!
//! Run with: `cargo run --package gram-codec --example basic_usage`

use gram_codec::{parse_gram_notation, to_gram, to_gram_pattern};
use pattern_core::{Pattern, Subject, Symbol};
use std::collections::{HashMap, HashSet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Gram Codec Basic Usage Examples ===\n");

    // Example 1: Parse simple node
    println!("1. Parsing a simple node:");
    let gram_text = "(hello)";
    let patterns = parse_gram_notation(gram_text)?;
    println!("   Input: {}", gram_text);
    println!("   Parsed {} pattern(s)", patterns.len());
    println!("   Identifier: {}\n", patterns[0].value.identity.0);

    // Example 2: Parse node with label
    println!("2. Parsing a node with label:");
    let gram_text = "(alice:Person)";
    let patterns = parse_gram_notation(gram_text)?;
    println!("   Input: {}", gram_text);
    println!("   Identifier: {}", patterns[0].value.identity.0);
    println!("   Labels: {:?}\n", patterns[0].value.labels);

    // Example 3: Parse node with properties
    println!("3. Parsing a node with properties:");
    let gram_text = "(alice:Person {name: \"Alice\", age: 30})";
    let patterns = parse_gram_notation(gram_text)?;
    println!("   Input: {}", gram_text);
    println!(
        "   Properties: {} key-value pairs\n",
        patterns[0].value.properties.len()
    );

    // Example 4: Parse relationship
    println!("4. Parsing a relationship:");
    let gram_text = "(alice)-[:KNOWS]->(bob)";
    let patterns = parse_gram_notation(gram_text)?;
    println!("   Input: {}", gram_text);
    println!("   Elements: {}", patterns[0].elements.len());
    println!("   Edge label: {:?}\n", patterns[0].value.labels);

    // Example 5: Parse subject pattern
    println!("5. Parsing a subject pattern:");
    let gram_text = "[team:Team {name: \"DevRel\"} | (alice), (bob), (charlie)]";
    let patterns = parse_gram_notation(gram_text)?;
    println!("   Input: {}", gram_text);
    println!("   Subject: {}", patterns[0].value.identity.0);
    println!("   Elements: {}\n", patterns[0].elements.len());

    // Example 6: Parse multiple patterns
    println!("6. Parsing multiple patterns:");
    let gram_text = "(a) (b) (c)";
    let patterns = parse_gram_notation(gram_text)?;
    println!("   Input: {}", gram_text);
    println!("   Parsed {} patterns\n", patterns.len());

    // Example 7: Serialize a pattern
    println!("7. Serializing a pattern:");
    let subject = Subject {
        identity: Symbol("hello".to_string()),
        labels: HashSet::new(),
        properties: HashMap::new(),
    };
    let pattern = Pattern::point(subject);
    let gram_output = to_gram_pattern(&pattern)?;
    println!("   Output: {}\n", gram_output);

    // Example 8: Round-trip (parse → serialize → parse)
    println!("8. Round-trip correctness:");
    let original = "(alice:Person {name: \"Alice\"})";
    println!("   Original: {}", original);

    let parsed = parse_gram_notation(original)?;
    let serialized = to_gram_pattern(&parsed[0])?;
    println!("   Serialized: {}", serialized);

    let reparsed = parse_gram_notation(&serialized)?;
    println!("   Reparsed successfully!");
    println!(
        "   Structural equality: {}\n",
        parsed[0].value.identity == reparsed[0].value.identity
    );

    // Example 9: Serialize multiple patterns
    println!("9. Serializing multiple patterns:");
    let patterns = vec![
        Pattern::point(Subject {
            identity: Symbol("a".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        }),
        Pattern::point(Subject {
            identity: Symbol("b".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        }),
        Pattern::point(Subject {
            identity: Symbol("c".to_string()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        }),
    ];
    let gram_output = to_gram(&patterns)?;
    println!("   Output:\n{}\n", gram_output);

    // Example 10: Error handling
    println!("10. Error handling:");
    let invalid_gram = "(unclosed";
    match parse_gram_notation(invalid_gram) {
        Ok(_) => println!("   Unexpected success!"),
        Err(e) => {
            println!("   Parse error detected:");
            println!("   {}\n", e);
        }
    }

    println!("=== Examples Complete ===");
    Ok(())
}
