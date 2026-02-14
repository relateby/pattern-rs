#!/usr/bin/env python3
"""
Quick start example for gram-codec Python bindings

Install: pip install relateby
Or from TestPyPI: pip install --index-url https://test.pypi.org/simple/ relateby
"""

from relateby.gram import parse_gram, validate_gram, round_trip, version

# Print version
print(f"Gram Codec version: {version()}\n")

# Example 1: Parse a simple relationship
print("1. Parse a relationship:")
result = parse_gram("(alice)-[:KNOWS]->(bob)")
print(f"   Patterns: {result['pattern_count']}")
print(f"   Identifiers: {result['identifiers']}\n")

# Example 2: Validate gram notation
print("2. Validate gram notation:")
print(f"   (hello) is valid: {validate_gram('(hello)')}")
print(f"   (unclosed is valid: {validate_gram('(unclosed')}\n")

# Example 3: Round-trip test
print("3. Round-trip test:")
original = "(alice)-->(bob)"
serialized = round_trip(original)
print(f"   Original:   {original}")
print(f"   Serialized: {serialized}\n")

# Example 4: Parse complex pattern
print("4. Parse complex pattern:")
gram = '[team:Team {name: "DevRel"} | (alice), (bob), (charlie)]'
result = parse_gram(gram)
print(f"   Input: {gram}")
print(f"   Patterns: {result['pattern_count']}")
print(f"   Root identifier: {result['identifiers'][0]}\n")

# Example 5: Error handling
print("5. Error handling:")
try:
    parse_gram("(invalid syntax")
except ValueError as e:
    print(f"   Caught error: {e}")

