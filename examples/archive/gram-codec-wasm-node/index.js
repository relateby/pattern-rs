#!/usr/bin/env node
/**
 * Node.js example for gram-codec WASM bindings
 * 
 * Prerequisites:
 *   1. Build WASM module: wasm-pack build --target nodejs crates/gram-codec -- --features wasm
 *   2. Install module: cd examples/wasm-node && npm install ../../pkg
 *   3. Run: node examples/wasm-node/index.js
 */

const { parse_gram, parse_to_ast, validate_gram, round_trip, version } = require('gram-codec');

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

// =============================================================================
// AST Output Examples
// =============================================================================

console.log('\n=== AST Output Examples ===');
console.log('\nThe parse_to_ast() function returns a JavaScript object representing');
console.log('the Abstract Syntax Tree (AST) of the parsed gram notation.');
console.log('This is the recommended way to access pattern data from JavaScript.\n');

// Example 1: Simple node with properties
console.log('Example 1: Simple node with properties');
console.log('-'.repeat(50));
let gramInput = '(alice:Person {name: "Alice", age: 30})';
console.log(`Input: ${gramInput}\n`);

let ast = parse_to_ast(gramInput);
console.log(`Identity:   ${ast.subject.identity}`);
console.log(`Labels:     ${JSON.stringify(ast.subject.labels)}`);
console.log(`Properties: ${JSON.stringify(ast.subject.properties, null, 2)}`);
console.log(`Elements:   ${ast.elements.length} children`);

// Example 2: Pattern with elements
console.log('\n\nExample 2: Pattern with elements');
console.log('-'.repeat(50));
gramInput = '[team:Team | (alice:Person), (bob:Person)]';
console.log(`Input: ${gramInput}\n`);

ast = parse_to_ast(gramInput);
console.log(`Parent identity: ${ast.subject.identity}`);
console.log(`Parent labels:   ${JSON.stringify(ast.subject.labels)}`);
console.log(`Number of elements: ${ast.elements.length}`);

ast.elements.forEach((elem, i) => {
    console.log(`\n  Element ${i+1}:`);
    console.log(`    Identity: ${elem.subject.identity}`);
    console.log(`    Labels:   ${JSON.stringify(elem.subject.labels)}`);
});

// Example 3: Complex properties with different value types
console.log('\n\nExample 3: Value type serialization');
console.log('-'.repeat(50));
gramInput = '(data {name: "Test", count: 42, active: true, tags: ["a", "b"]})';
console.log(`Input: ${gramInput}\n`);

ast = parse_to_ast(gramInput);
console.log('Property types:');
Object.entries(ast.subject.properties).forEach(([key, value]) => {
    if (typeof value === 'object' && value !== null && 'type' in value) {
        // Tagged value (e.g., Integer)
        console.log(`  ${key}: ${value.type} = ${value.value}`);
    } else {
        // Native JavaScript value (e.g., string, boolean)
        const typeName = Array.isArray(value) ? 'Array' : typeof value;
        console.log(`  ${key}: ${typeName} = ${JSON.stringify(value)}`);
    }
});

// Example 4: Navigating nested structures
console.log('\n\nExample 4: Navigating AST structure');
console.log('-'.repeat(50));
gramInput = '[org:Org {name: "ACME"} | [team:Team | (alice), (bob)]]';
console.log(`Input: ${gramInput}\n`);

ast = parse_to_ast(gramInput);

function printPattern(pattern, depth = 0) {
    const indent = '  '.repeat(depth);
    const identity = pattern.subject.identity || '(anonymous)';
    const labels = pattern.subject.labels.join(', ') || '(no labels)';
    const propsCount = Object.keys(pattern.subject.properties).length;
    
    console.log(`${indent}└─ ${identity} [${labels}] (${propsCount} properties)`);
    
    if (pattern.elements && pattern.elements.length > 0) {
        pattern.elements.forEach(elem => printPattern(elem, depth + 1));
    }
}

console.log('Pattern structure:');
printPattern(ast);

// Example 5: JSON serialization for storage/transmission
console.log('\n\nExample 5: JSON serialization');
console.log('-'.repeat(50));
gramInput = '(alice:Person {name: "Alice"})';
console.log(`Input: ${gramInput}\n`);

ast = parse_to_ast(gramInput);

// Serialize to compact JSON
const compactJson = JSON.stringify(ast);
console.log(`Compact JSON (${compactJson.length} bytes):`);
console.log(compactJson);

// Serialize to pretty JSON
const prettyJson = JSON.stringify(ast, null, 2);
console.log(`\nPretty JSON (${prettyJson.length} bytes):`);
console.log(prettyJson);

// Example 6: Working with TypeScript (type information)
console.log('\n\nExample 6: TypeScript usage hints');
console.log('-'.repeat(50));
console.log('For TypeScript projects, you can define interfaces:');
console.log(`
interface AstPattern {
  subject: {
    identity: string;
    labels: string[];
    properties: Record<string, any>;
  };
  elements: AstPattern[];
}

const ast: AstPattern = parse_to_ast("(hello)");
`);

console.log('\n=== Examples Complete ===');

