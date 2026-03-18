#!/usr/bin/env python3
"""Quick start example for the public relateby.gram package."""

from relateby.gram import GramParseError, gram_stringify, gram_validate, parse_gram, round_trip

print("1. Parse a relationship:")
patterns = parse_gram("(alice)-[:KNOWS]->(bob)")
print(f"   Parsed patterns: {len(patterns)}")
print(f"   Root identities: {[pattern.value.identity for pattern in patterns]}\n")

print("2. Validate gram notation:")
print(f"   (hello) errors: {gram_validate('(hello)')}")
print(f"   (unclosed errors: {gram_validate('(unclosed')}\n")

print("3. Round-trip test:")
original = "(alice)-->(bob)"
serialized = round_trip(original)
print(f"   Original:   {original}")
print(f"   Serialized: {serialized}\n")

print("4. Serialize parsed native patterns:")
print(f"   Stringified: {gram_stringify(patterns)}\n")

print("5. Error handling:")
try:
    parse_gram("(invalid syntax")
except GramParseError as error:
    print(f"   Caught error: {error}")
