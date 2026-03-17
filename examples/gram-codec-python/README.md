# relateby.gram Examples

These examples use the supported public package boundary from the combined Python distribution:

```bash
pip install relateby-pattern
```

Use:

```python
from relateby.gram import parse_gram, round_trip, validate_gram
```

`parse_gram()` returns a `ParseResult` object with attributes, not a dictionary.

## Quick Start

```python
from relateby.gram import parse_gram, round_trip, validate_gram

result = parse_gram("(alice)-[:KNOWS]->(bob)")
print(result.pattern_count)
print(result.identifiers)

print(validate_gram("(alice:Person)"))
print(round_trip("(alice:Person)"))
```

## Public API

- `parse_gram(input: str) -> ParseResult`
- `validate_gram(input: str) -> bool`
- `round_trip(input: str) -> str`

## Running the examples

```bash
python examples/gram-codec-python/quickstart.py
python examples/gram-codec-python/demo.py
```

Both examples expect `relateby-pattern` to be installed and use only `relateby.gram`.
        print("Usage: gram-tool <parse|validate> <input>")
        sys.exit(1)
    
    command = sys.argv[1]
    input_gram = sys.argv[2]
    
    if command == "parse":
        try:
            result = relateby.gram.parse_gram(input_gram)
            print(f"Valid: {result.pattern_count} pattern(s)")
        except ValueError as e:
            print(f"Invalid: {e}")
            sys.exit(1)
    
    elif command == "validate":
        is_valid = relateby.gram.validate_gram(input_gram)
        print("Valid" if is_valid else "Invalid")
        sys.exit(0 if is_valid else 1)
    
    else:
        print(f"Unknown command: {command}")
        sys.exit(1)

if __name__ == "__main__":
    main()
```

## Performance

The Python bindings are powered by native Rust code, providing:

- **Fast parsing**: Near-native Rust performance
- **Low overhead**: Minimal Python/Rust boundary crossing
- **Memory efficient**: Patterns are processed and returned efficiently

### Benchmarking

```python
import relateby.gram
import time

gram = "(alice)-[:KNOWS]->(bob)"
iterations = 10000

start = time.time()
for _ in range(iterations):
    relateby.gram.parse_gram(gram)
end = time.time()

elapsed = end - start
per_call = (elapsed / iterations) * 1000
print(f"Parsed {iterations} times in {elapsed:.2f}s")
print(f"Average: {per_call:.3f}ms per parse")
```

## Type Hints

The package includes type hints for better IDE support:

```python
from relateby.gram import parse_gram, validate_gram, round_trip, ParseResult

# Type-checked usage
result: ParseResult = parse_gram("(hello)")
is_valid: bool = validate_gram("(world)")
serialized: str = round_trip("(a)-->(b)")
```

## Error Handling

All parsing errors are raised as `ValueError`:

```python
import relateby.gram

try:
    result = relateby.gram.parse_gram("(unclosed")
except ValueError as e:
    print(f"Parse error: {e}")
    # Parse error: Syntax error at ...
```

## Testing

```python
import relateby.gram
import unittest

class TestGramCodec(unittest.TestCase):
    def test_parse_valid(self):
        result = relateby.gram.parse_gram("(hello)")
        self.assertEqual(result.pattern_count, 1)
    
    def test_parse_invalid(self):
        with self.assertRaises(ValueError):
            relateby.gram.parse_gram("(unclosed")
    
    def test_validate(self):
        self.assertTrue(relateby.gram.validate_gram("(hello)"))
        self.assertFalse(relateby.gram.validate_gram("(unclosed"))
    
    def test_round_trip(self):
        original = "(alice)-->(bob)"
        serialized = relateby.gram.round_trip(original)
        self.assertEqual(original, serialized)

if __name__ == "__main__":
    unittest.main()
```

## AST Output (Coming in Phase 7)

For building Python applications that need full access to pattern data, we're adding **AST output**. The AST provides the complete `Pattern<Subject>` structure as a Python dictionary.

### Why AST?

The current API returns only metadata (pattern count, identifiers). The AST will provide:
- ✅ **Complete pattern structure** - subjects, elements, properties
- ✅ **Pythonic** - Regular dicts and lists, no opaque objects
- ✅ **Ready for gram-py** - Native Python Pattern library (separate project)
- ✅ **Serializable** - Can pickle, json.dump, or store as needed

### Future Usage (Phase 7)

```python
from relateby.gram import parse_to_ast

# Parse to AST
ast = parse_to_ast("(alice:Person {name: 'Alice', age: 30})")

# Access pattern data
print(ast['subject']['identity'])    # "alice"
print(ast['subject']['labels'])      # ["Person"]
print(ast['subject']['properties'])  # {'name': 'Alice', 'age': 30}
print(ast['elements'])               # [] (no child patterns)

# AST is just a dict - serialize it
import json
json_str = json.dumps(ast)
```

### Architecture

```
pattern-rs (this project)
  └─> parse_to_ast() → AST (dict)
       └─> gram-py (separate project)
            └─> Pattern.from_ast(ast) → Full Pattern API
                 └─> map(), fold(), filter(), etc.
```

**pattern-rs** responsibilities:
- ✅ Parse gram notation to AST
- ✅ Validate syntax
- ✅ Serialize patterns back to gram

**gram-py** responsibilities (future):
- ✅ Native Python Pattern[V] implementation (using dataclasses)
- ✅ Full FP API (map, fold, traverse, comonad operations)
- ✅ Pattern queries and transformations
- ✅ Type hints and IDE support

This separation keeps the native extension focused on parsing while enabling full Pattern operations in pure Python (zero FFI overhead).

## Requirements

- Python >= 3.8
- No external dependencies (pure Rust native extension)

## Platform Support

- ✅ macOS (x86_64, arm64)
- ✅ Linux (x86_64, aarch64)
- ✅ Windows (x86_64)

## Troubleshooting

### Import Error

If you get `ModuleNotFoundError: No module named 'relateby.gram'`:

1. Make sure the package is installed: `pip list | grep relateby-pattern`
2. Check Python version compatibility: `python --version` (requires >= 3.8)
3. Reinstall: `pip install --force-reinstall relateby-pattern`

### Build Errors

If building from source fails:

1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Install maturin: `pip install maturin`
3. Try building with verbose output: `maturin build --features python -v`

### Performance Issues

For best performance:

1. Use `validate_gram()` for quick checks before parsing
2. Parse once and cache results
3. Process in batches when possible

## Next Steps

- See `example.py` for comprehensive examples
- Check the main README for gram notation syntax
- Explore the Rust API documentation

## License

Apache-2.0

## Contributing

Issues and pull requests welcome at https://github.com/relateby/pattern-rs
