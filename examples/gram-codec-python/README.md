# Gram Codec - Python Bindings

Python bindings for the gram-codec Rust library, enabling parsing, validation, and serialization of gram notation in Python.

> **Note**: This module currently shows the basic validation API. For full data access, see the **AST Output** section below (coming in Phase 7).

## Installation

### From PyPI (when published)

```bash
pip install gram-codec
```

### From Wheel (Local Development)

```bash
# Build the wheel
cd crates/gram-codec
maturin build --release --features python

# Install the wheel
pip install ../../target/wheels/gram_codec-0.1.0-*.whl
```

### From Source

```bash
# Install maturin
pip install maturin

# Build and install
cd crates/gram-codec
maturin develop --features python
```

## Quick Start

```python
import gram_codec

# Parse gram notation
result = gram_codec.parse_gram("(alice)-[:KNOWS]->(bob)")
print(f"Parsed {result.pattern_count} patterns")

# Validate gram notation
is_valid = gram_codec.validate_gram("(hello)")  # True

# Round-trip (parse and serialize)
serialized = gram_codec.round_trip("(a)-->(b)")  # "(a)-->(b)"

# Get version
print(gram_codec.version())  # "0.1.0"
```

## Current API (Validation Only)

### `parse_gram(input: str) -> ParseResult`

Parse gram notation and return information about the parsed patterns.

**Parameters:**
- `input` (str): Gram notation string to parse

**Returns:**
- `ParseResult`: Object containing:
  - `pattern_count` (int): Number of top-level patterns
  - `identifiers` (list[str]): List of root identifiers

**Raises:**
- `ValueError`: If the gram notation is invalid

**Example:**
```python
result = gram_codec.parse_gram("(alice)-[:KNOWS]->(bob)")
print(result.pattern_count)  # 1
print(result.identifiers)  # []
```

### `validate_gram(input: str) -> bool`

Validate gram notation without parsing.

**Parameters:**
- `input` (str): Gram notation string to validate

**Returns:**
- `bool`: True if valid, False otherwise

**Example:**
```python
gram_codec.validate_gram("(hello)")  # True
gram_codec.validate_gram("(unclosed")  # False
```

### `round_trip(input: str) -> str`

Parse gram notation, serialize it back, and return the serialized form.

**Parameters:**
- `input` (str): Gram notation string

**Returns:**
- `str`: Serialized gram notation

**Raises:**
- `ValueError`: If parsing or serialization fails

**Example:**
```python
serialized = gram_codec.round_trip("(alice)-->(bob)")
print(serialized)  # "(alice)-->(bob)"
```

### `version() -> str`

Get the version of gram-codec.

**Returns:**
- `str`: Version string (e.g., "0.1.0")

**Example:**
```python
print(gram_codec.version())  # "0.1.0"
```

### `ParseResult` Class

Result object from `parse_gram()`.

**Attributes:**
- `pattern_count` (int): Number of patterns parsed
- `identifiers` (list[str]): List of root pattern identifiers

**Methods:**
- `to_dict() -> dict`: Convert to dictionary
- `__str__() -> str`: String representation
- `__repr__() -> str`: Repr representation

**Example:**
```python
result = gram_codec.parse_gram("(hello)")
print(result.pattern_count)  # 1
print(result.to_dict())  # {'pattern_count': 1, 'identifiers': []}
```

## Examples

See `example.py` for comprehensive examples demonstrating:

1. Parse gram notation
2. Validate syntax
3. Round-trip testing
4. Complex patterns
5. Error handling
6. Version information
7. Batch validation
8. Working with ParseResult
9. Data processing integration
10. Validation pipelines

### Running the Examples

```bash
# Make sure gram_codec is installed
pip install ../../target/wheels/gram_codec-*.whl

# Run the examples
python example.py
```

## Common Use Cases

### File Validation

```python
import gram_codec

def validate_gram_file(filepath):
    """Validate a gram file"""
    with open(filepath) as f:
        content = f.read()
    
    return gram_codec.validate_gram(content)

# Usage
is_valid = validate_gram_file("data.gram")
print(f"File is {'valid' if is_valid else 'invalid'}")
```

### Batch Processing

```python
import gram_codec
import glob

def process_gram_files(pattern="**/*.gram"):
    """Process all gram files matching a pattern"""
    results = []
    
    for filepath in glob.glob(pattern, recursive=True):
        with open(filepath) as f:
            content = f.read()
        
        try:
            result = gram_codec.parse_gram(content)
            results.append({
                "file": filepath,
                "valid": True,
                "pattern_count": result.pattern_count
            })
        except ValueError as e:
            results.append({
                "file": filepath,
                "valid": False,
                "error": str(e)
            })
    
    return results

# Usage
results = process_gram_files()
for r in results:
    print(f"{r['file']}: {r}")
```

### Data Pipeline Integration

```python
import gram_codec
import pandas as pd

def parse_gram_column(df, column_name):
    """Parse gram notation in a DataFrame column"""
    def parse_safe(gram):
        try:
            result = gram_codec.parse_gram(gram)
            return result.pattern_count
        except ValueError:
            return None
    
    df['pattern_count'] = df[column_name].apply(parse_safe)
    return df

# Usage
df = pd.DataFrame({
    'gram': ['(alice)', '(bob)', '(invalid']
})
df = parse_gram_column(df, 'gram')
print(df)
```

### CLI Tool

```python
#!/usr/bin/env python3
import sys
import gram_codec

def main():
    if len(sys.argv) < 3:
        print("Usage: gram-tool <parse|validate> <input>")
        sys.exit(1)
    
    command = sys.argv[1]
    input_gram = sys.argv[2]
    
    if command == "parse":
        try:
            result = gram_codec.parse_gram(input_gram)
            print(f"Valid: {result.pattern_count} pattern(s)")
        except ValueError as e:
            print(f"Invalid: {e}")
            sys.exit(1)
    
    elif command == "validate":
        is_valid = gram_codec.validate_gram(input_gram)
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
import gram_codec
import time

gram = "(alice)-[:KNOWS]->(bob)"
iterations = 10000

start = time.time()
for _ in range(iterations):
    gram_codec.parse_gram(gram)
end = time.time()

elapsed = end - start
per_call = (elapsed / iterations) * 1000
print(f"Parsed {iterations} times in {elapsed:.2f}s")
print(f"Average: {per_call:.3f}ms per parse")
```

## Type Hints

The package includes type hints for better IDE support:

```python
from gram_codec import parse_gram, validate_gram, round_trip, ParseResult

# Type-checked usage
result: ParseResult = parse_gram("(hello)")
is_valid: bool = validate_gram("(world)")
serialized: str = round_trip("(a)-->(b)")
```

## Error Handling

All parsing errors are raised as `ValueError`:

```python
import gram_codec

try:
    result = gram_codec.parse_gram("(unclosed")
except ValueError as e:
    print(f"Parse error: {e}")
    # Parse error: Syntax error at ...
```

## Testing

```python
import gram_codec
import unittest

class TestGramCodec(unittest.TestCase):
    def test_parse_valid(self):
        result = gram_codec.parse_gram("(hello)")
        self.assertEqual(result.pattern_count, 1)
    
    def test_parse_invalid(self):
        with self.assertRaises(ValueError):
            gram_codec.parse_gram("(unclosed")
    
    def test_validate(self):
        self.assertTrue(gram_codec.validate_gram("(hello)"))
        self.assertFalse(gram_codec.validate_gram("(unclosed"))
    
    def test_round_trip(self):
        original = "(alice)-->(bob)"
        serialized = gram_codec.round_trip(original)
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
from gram_codec import parse_to_ast

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
gram-rs (this project)
  └─> parse_to_ast() → AST (dict)
       └─> gram-py (separate project)
            └─> Pattern.from_ast(ast) → Full Pattern API
                 └─> map(), fold(), filter(), etc.
```

**gram-rs** responsibilities:
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

If you get `ModuleNotFoundError: No module named 'gram_codec'`:

1. Make sure the package is installed: `pip list | grep gram-codec`
2. Check Python version compatibility: `python --version` (requires >= 3.8)
3. Reinstall: `pip install --force-reinstall gram-codec`

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

Issues and pull requests welcome at https://github.com/gram-data/gram-rs
