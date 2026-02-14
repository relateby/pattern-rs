#!/usr/bin/env python3
"""
Gram Codec Python Examples

This script demonstrates how to use the relateby.gram Python module
for parsing, validating, and serializing gram notation.

Prerequisites:
    pip install relateby

Or from TestPyPI (pre-release):
    pip install --index-url https://test.pypi.org/simple/ relateby
"""

import json
import relateby.gram

print("=== Gram Codec Python Examples ===\n")

# Example 1: Parse gram notation
print("1. Parse Gram Notation")
try:
    result = relateby.gram.parse_gram("(alice)-[:KNOWS]->(bob)")
    print(f"   Pattern count: {result.pattern_count}")
    print(f"   Identifiers: {result.identifiers}")
    print(f"   Result object: {result}")
except ValueError as e:
    print(f"   Error: {e}")

# Example 2: Validate gram notation
print("\n2. Validate Gram Notation")
test_cases = [
    "(hello)",
    "(a)-->(b)",
    "[team | alice, bob]",
    "(unclosed",  # Invalid
    "{key: value}",
    "(a)-[:KNOWS]->(b)",
]

for gram in test_cases:
    is_valid = relateby.gram.validate_gram(gram)
    status = "✓ valid" if is_valid else "✗ invalid"
    print(f"   \"{gram}\" → {status}")

# Example 3: Round-trip test
print("\n3. Round-Trip Test")
test_patterns = [
    "(alice:Person {name: \"Alice\"})-[:KNOWS]->(bob:Person {name: \"Bob\"})",
    "[team:Team {name: \"DevRel\"} | (alice), (bob), (charlie)]",
    "(a)-->(b)-->(c)",
]

for pattern in test_patterns:
    try:
        serialized = relateby.gram.round_trip(pattern)
        is_same = pattern == serialized
        print(f"   Original:   {pattern}")
        print(f"   Serialized: {serialized}")
        print(f"   Match: {is_same}")
        print()
    except ValueError as e:
        print(f"   Error: {e}")

# Example 4: Complex patterns
print("4. Complex Patterns")
complex_patterns = [
    "(a:Person)",
    "(a {name: \"Alice\", age: 30})",
    "(a)-[:KNOWS]->(b)",
    "[]",
    "[team:Team | (alice), (bob)]",
]

for pattern in complex_patterns:
    try:
        result = relateby.gram.parse_gram(pattern)
        print(f"   ✓ \"{pattern}\" → {result.pattern_count} pattern(s)")
    except ValueError as e:
        print(f"   ✗ \"{pattern}\" → Error: {e}")

# Example 5: Error handling
print("\n5. Error Handling")
invalid_patterns = [
    "(unclosed",
    "{no_parens}",
    "(a {key: })",
    "not gram notation",
]

for pattern in invalid_patterns:
    try:
        relateby.gram.parse_gram(pattern)
        print(f"   Unexpected success for: {pattern}")
    except ValueError as e:
        print(f"   ✓ Expected error for \"{pattern}\"")

# Example 6: Version information
print("\n6. Version Information")
print(f"   Gram Codec version: {relateby.gram.version()}")

# Example 7: Batch validation
print("\n7. Batch Validation")
files = {
    "nodes.gram": "(alice:Person) (bob:Person)",
    "relationships.gram": "(alice)-[:KNOWS]->(bob)",
    "complex.gram": "[team | (alice), (bob), (charlie)]",
    "invalid.gram": "(unclosed",
}

valid_count = 0
invalid_count = 0

for filename, content in files.items():
    is_valid = relateby.gram.validate_gram(content)
    status = "✓ valid" if is_valid else "✗ invalid"
    print(f"   {filename}: {status}")
    
    if is_valid:
        valid_count += 1
    else:
        invalid_count += 1

print(f"   Summary: {valid_count} valid, {invalid_count} invalid")

# Example 8: Working with ParseResult
print("\n8. Working with ParseResult")
result = relateby.gram.parse_gram("(alice) (bob) (charlie)")
print(f"   Type: {type(result)}")
print(f"   Pattern count: {result.pattern_count}")
print(f"   Identifiers: {result.identifiers}")
print(f"   String representation: {str(result)}")
print(f"   Repr: {repr(result)}")

# Convert to dict
result_dict = result.to_dict()
print(f"   As dict: {result_dict}")

# Example 9: Integration with data processing
print("\n9. Data Processing Integration")

# Parse multiple gram files
gram_data = [
    "(alice:Person {name: \"Alice\"})",
    "(bob:Person {name: \"Bob\"})",
    "(alice)-[:KNOWS]->(bob)",
]

parsed_patterns = []
for gram in gram_data:
    try:
        result = relateby.gram.parse_gram(gram)
        parsed_patterns.append({
            "input": gram,
            "pattern_count": result.pattern_count,
            "identifiers": result.identifiers
        })
    except ValueError as e:
        parsed_patterns.append({
            "input": gram,
            "error": str(e)
        })

print(f"   Parsed {len(parsed_patterns)} inputs:")
for i, p in enumerate(parsed_patterns, 1):
    if "error" in p:
        print(f"   {i}. Error: {p['error']}")
    else:
        print(f"   {i}. {p['pattern_count']} pattern(s), {len(p['identifiers'])} identifier(s)")

# Example 10: Validation pipeline
print("\n10. Validation Pipeline")

def validate_and_normalize(gram_input):
    """Validate gram notation and return normalized form"""
    if not relateby.gram.validate_gram(gram_input):
        raise ValueError(f"Invalid gram notation: {gram_input}")
    
    return relateby.gram.round_trip(gram_input)

test_inputs = [
    "(alice)-->(bob)",
    "[team | (alice), (bob)]",
]

for input_gram in test_inputs:
    try:
        normalized = validate_and_normalize(input_gram)
        print(f"   Input:      {input_gram}")
        print(f"   Normalized: {normalized}")
    except ValueError as e:
        print(f"   Error: {e}")

# =============================================================================
# AST Output Examples
# =============================================================================

print("\n=== AST Output Examples ===")
print("\nThe parse_to_ast() function returns a Python dictionary representing")
print("the Abstract Syntax Tree (AST) of the parsed gram notation.")
print("This is the recommended way to access pattern data from Python.\n")

# Example 1: Simple node with properties
print("Example 1: Simple node with properties")
print("-" * 50)
gram_input = '(alice:Person {name: "Alice", age: 30})'
print(f"Input: {gram_input}\n")

ast = relateby.gram.parse_to_ast(gram_input)
print(f"Identity:   {ast['subject']['identity']}")
print(f"Labels:     {ast['subject']['labels']}")
print(f"Properties: {json.dumps(ast['subject']['properties'], indent=2)}")
print(f"Elements:   {len(ast['elements'])} children")

# Example 2: Pattern with elements
print("\n\nExample 2: Pattern with elements")
print("-" * 50)
gram_input = '[team:Team | (alice:Person), (bob:Person)]'
print(f"Input: {gram_input}\n")

ast = relateby.gram.parse_to_ast(gram_input)
print(f"Parent identity: {ast['subject']['identity']}")
print(f"Parent labels:   {ast['subject']['labels']}")
print(f"Number of elements: {len(ast['elements'])}")

for i, elem in enumerate(ast['elements']):
    print(f"\n  Element {i+1}:")
    print(f"    Identity: {elem['subject']['identity']}")
    print(f"    Labels:   {elem['subject']['labels']}")

# Example 3: Complex properties with different value types
print("\n\nExample 3: Value type serialization")
print("-" * 50)
gram_input = '(data {name: "Test", count: 42, active: true, tags: ["a", "b"]})'
print(f"Input: {gram_input}\n")

ast = relateby.gram.parse_to_ast(gram_input)
print("Property types:")
for key, value in ast['subject']['properties'].items():
    if isinstance(value, dict) and 'type' in value:
        # Tagged value (e.g., Integer)
        print(f"  {key}: {value['type']} = {value['value']}")
    else:
        # Native JSON value (e.g., String, Boolean)
        print(f"  {key}: {type(value).__name__} = {value}")

# Example 4: Navigating nested structures
print("\n\nExample 4: Navigating AST structure")
print("-" * 50)
gram_input = '[org:Org {name: "ACME"} | [team:Team | (alice), (bob)]]'
print(f"Input: {gram_input}\n")

ast = relateby.gram.parse_to_ast(gram_input)

def print_pattern(pattern, depth=0):
    """Recursively print pattern structure"""
    indent = "  " * depth
    identity = pattern['subject']['identity'] or "(anonymous)"
    labels = ", ".join(pattern['subject']['labels']) or "(no labels)"
    props_count = len(pattern['subject']['properties'])
    
    print(f"{indent}└─ {identity} [{labels}] ({props_count} properties)")
    
    if pattern['elements']:
        for elem in pattern['elements']:
            print_pattern(elem, depth + 1)

print("Pattern structure:")
print_pattern(ast)

# Example 5: JSON serialization for storage/transmission
print("\n\nExample 5: JSON serialization")
print("-" * 50)
gram_input = '(alice:Person {name: "Alice"})'
print(f"Input: {gram_input}\n")

ast = relateby.gram.parse_to_ast(gram_input)

# Serialize to compact JSON
compact_json = json.dumps(ast)
print(f"Compact JSON ({len(compact_json)} bytes):")
print(compact_json)

# Serialize to pretty JSON
pretty_json = json.dumps(ast, indent=2)
print(f"\nPretty JSON ({len(pretty_json)} bytes):")
print(pretty_json)

print("\n=== Examples Complete ===")
