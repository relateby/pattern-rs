#!/usr/bin/env node
/**
 * Node.js example for gram-codec WASM bindings
 * 
 * Prerequisites:
 *   1. Build WASM module: wasm-pack build --target nodejs crates/gram-codec -- --features wasm
 *   2. Install module: cd examples/wasm-node && npm install ../../pkg
 *   3. Run: node examples/wasm-node/index.js
 */

const { parse_gram, validate_gram, round_trip, version } = require('gram-codec');

console.log('=== Gram Codec WASM Examples (Node.js) ===\n');

// Example 1: Parse gram notation
console.log('1. Parse Gram Notation');
try {
    const result = parse_gram("(alice)-[:KNOWS]->(bob)");
    console.log(`   Pattern count: ${result.pattern_count}`);
    console.log(`   Identifiers: [${result.identifiers.join(', ')}]`);
} catch (e) {
    console.error(`   Error: ${e}`);
}

// Example 2: Validate gram notation
console.log('\n2. Validate Gram Notation');
const examples = [
    "(hello)",
    "(a)-->(b)",
    "[team | alice, bob]",
    "(unclosed",  // Invalid
];

examples.forEach(gram => {
    const isValid = validate_gram(gram);
    console.log(`   "${gram}" → ${isValid ? '✓ valid' : '✗ invalid'}`);
});

// Example 3: Round-trip test
console.log('\n3. Round-Trip Test');
const original = "(alice:Person {name: \"Alice\"})-[:KNOWS]->(bob:Person {name: \"Bob\"})";
console.log(`   Original: ${original}`);
try {
    const serialized = round_trip(original);
    console.log(`   Serialized: ${serialized}`);
    
    // Verify it's still valid
    const isValid = validate_gram(serialized);
    console.log(`   Still valid: ${isValid ? '✓' : '✗'}`);
} catch (e) {
    console.error(`   Error: ${e}`);
}

// Example 4: Parse multiple patterns
console.log('\n4. Multiple Patterns');
const multiPattern = "(alice) (bob) (charlie)";
console.log(`   Input: ${multiPattern}`);
try {
    const result = parse_gram(multiPattern);
    console.log(`   Parsed ${result.pattern_count} patterns`);
    console.log(`   Identifiers: [${result.identifiers.join(', ')}]`);
} catch (e) {
    console.error(`   Error: ${e}`);
}

// Example 5: Complex patterns
console.log('\n5. Complex Patterns');
const complexExamples = [
    "(a:Person)",
    '(a {name: "Alice", age: 30})',
    "(a)-[:KNOWS]->(b)",
    '[team:Team {name: "DevRel"} | (alice), (bob), (charlie)]',
    "[outer | [inner | (leaf)]]",
];

complexExamples.forEach(gram => {
    try {
        const result = parse_gram(gram);
        console.log(`   ✓ "${gram}" → ${result.pattern_count} pattern(s)`);
    } catch (e) {
        console.log(`   ✗ "${gram}" → Error: ${e}`);
    }
});

// Example 6: Error handling
console.log('\n6. Error Handling');
const invalidExamples = ["(unclosed", "{no_parens}", "(a {key: })"];

invalidExamples.forEach(gram => {
    try {
        parse_gram(gram);
        console.log(`   Unexpected success: "${gram}"`);
    } catch (e) {
        console.log(`   ✓ Expected error for "${gram}"`);
    }
});

// Example 7: Version information
console.log('\n7. Version Information');
console.log(`   Gram Codec version: ${version()}`);

// Example 8: Batch validation
console.log('\n8. Batch Validation');
const gramFiles = {
    "nodes.gram": "(alice) (bob) (charlie)",
    "relationships.gram": "(alice)-->(bob) (bob)-->(charlie)",
    "complex.gram": "[team | (alice), (bob)]",
    "invalid.gram": "(unclosed",
};

let validCount = 0;
let invalidCount = 0;

Object.entries(gramFiles).forEach(([filename, content]) => {
    const isValid = validate_gram(content);
    const status = isValid ? '✓' : '✗';
    console.log(`   ${status} ${filename}: ${isValid ? 'valid' : 'invalid'}`);
    isValid ? validCount++ : invalidCount++;
});

console.log(`   Summary: ${validCount} valid, ${invalidCount} invalid`);

console.log('\n=== Examples Complete ===');

