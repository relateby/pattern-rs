# gram-hs CLI Testing Guide for gram-rs

**Purpose**: Guide for using the updated `gram-hs` CLI tool for testing gram-rs  
**Last Updated**: 2025-12-28

## Overview

The `gram-hs` CLI tool has been updated with several improvements that make it highly suitable for automated testing and equivalence checking with gram-rs. This guide demonstrates how to use these features effectively.

## Key Improvements Implemented

### ✅ High Priority Features (Implemented)

1. **`--value-only`**: Output only the result value without metadata
2. **`--deterministic`**: Use fixed values for metadata (timestamps, hashes)
3. **`--canonical`**: Sort JSON keys alphabetically at all nesting levels
4. **`--type suite`**: Generate test cases in test suite format

### ⚠️ Features Not Yet Implemented

- `--batch`: Batch processing mode
- `--select`: Output filtering/selection
- `--type property`: Property test data generation (listed but returns error)

---

## Usage Examples for Testing gram-rs

### 1. Equivalence Checking with `--value-only`

**Problem**: The default JSON output includes `Meta.Hash` and `Meta.Timestamp` that change on every run, making comparison difficult.

**Solution**: Use `--value-only` to get just the pattern value:

```bash
# Get reference output from gram-hs (just the value)
echo '(node1)' | gram-hs parse --format json --value-only
# Output: {"elements":[],"value":{"labels":[],"properties":{},"symbol":"node1"}}

# Compare with gram-rs output (after implementing gram-rs CLI)
# Both outputs can now be directly compared as JSON
```

**Use Case**: Direct comparison in equivalence checking utilities (`check_equivalence` function).

**Example Integration**:
```rust
// In equivalence checking utility
let gram_hs_output = std::process::Command::new("gram-hs")
    .args(&["parse", "--format", "json", "--value-only"])
    .stdin(std::process::Stdio::piped())
    .output()?;
    
let gram_hs_value: serde_json::Value = serde_json::from_slice(&gram_hs_output.stdout)?;
// Now compare directly with gram_rs_output
```

---

### 2. Deterministic Output with `--deterministic`

**Problem**: Even with `--value-only`, if metadata is needed, timestamps and hashes change.

**Solution**: Use `--deterministic` to get fixed metadata:

```bash
# Deterministic output with fixed timestamp and hash
echo '(node1)' | gram-hs parse --format json --deterministic
# Meta.Timestamp: "1970-01-01T00:00:00+0000"
# Meta.Hash: "0000000000000000000000000000000000000000000000000000000000000000"
```

**Use Case**: Snapshot testing where full output structure is needed but must be deterministic.

**Example**:
```bash
# Generate deterministic snapshot
echo '(node1)' | gram-hs parse --format json --deterministic > snapshot.json

# This snapshot will be identical on every run, suitable for insta snapshots
```

---

### 3. Canonical JSON with `--canonical`

**Problem**: JSON key ordering may vary, making byte-for-byte comparison unreliable.

**Solution**: Use `--canonical` to ensure sorted keys:

```bash
# Canonical output with sorted keys
echo '(node1)-[edge]->(node2)' | gram-hs parse --format json --canonical --value-only
```

**Use Case**: Reliable comparison where exact JSON string matching is required.

**Note**: `--canonical` automatically sorts keys at all nesting levels, ensuring equivalent data structures produce identical JSON strings.

---

### 4. Test Suite Generation with `--type suite`

**Problem**: Need to extract test cases from gram-hs for gram-rs testing.

**Solution**: Use `--type suite` to generate test cases in the correct format:

```bash
# Generate test suite with 10 test cases
gram-hs generate --type suite --count 10 --seed 42 --format json > test_cases.json

# Output format matches test-sync-format.md:
# {
#   "version": "1.0",
#   "test_cases": [
#     {
#       "name": "test_case_001",
#       "description": "...",
#       "input": { "type": "gram_notation", "value": "..." },
#       "expected": { "type": "pattern", "value": {...} },
#       "operations": null
#     }
#   ]
# }
```

**Use Case**: Automated test case extraction (User Story 4: Test Data Extraction).

**Example Integration**:
```rust
// In test extraction utility
let output = std::process::Command::new("gram-hs")
    .args(&[
        "generate",
        "--type", "suite",
        "--count", "100",
        "--seed", "42",
        "--format", "json",
        "--value-only"  // Optional: exclude metadata
    ])
    .output()?;
    
let test_suite: TestSuite = serde_json::from_slice(&output.stdout)?;
// Use test_suite.test_cases for equivalence checking
```

**Complexity Levels**:
- `--complexity minimal`: Simple patterns
- `--complexity basic`: Basic patterns (default)
- `--complexity standard`: Standard complexity patterns
- `--complexity complex`: Complex patterns
- `--complexity adversarial`: Edge cases and adversarial inputs

---

## Combining Flags

Flags can be combined for optimal testing scenarios:

### Scenario 1: Direct Value Comparison
```bash
# Best for equivalence checking - just the value, deterministic
echo '(node1)' | gram-hs parse --format json --value-only --deterministic
```

### Scenario 2: Snapshot Testing
```bash
# Full structure but deterministic - good for insta snapshots
echo '(node1)' | gram-hs parse --format json --deterministic --canonical
```

### Scenario 3: Test Suite Generation
```bash
# Generate test suite without metadata
gram-hs generate --type suite --count 50 --seed 42 --format json --value-only
```

### Scenario 4: Canonical Value Output
```bash
# Value only, with sorted keys for reliable comparison
echo '(node1)-[edge]->(node2)' | gram-hs parse --format json --value-only --canonical
```

---

## Integration with gram-rs Testing Infrastructure

### Equivalence Checking

Update `check_equivalence` to use `--value-only`:

```rust
// In crates/pattern-core/src/test_utils/equivalence.rs

pub fn get_gram_hs_reference(input: &str) -> Result<serde_json::Value, String> {
    let output = std::process::Command::new("gram-hs")
        .args(&["parse", "--format", "json", "--value-only", "--canonical"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn gram-hs: {}", e))?;
    
    // Write input to stdin
    use std::io::Write;
    if let Some(mut stdin) = output.stdin {
        stdin.write_all(input.as_bytes())
            .map_err(|e| format!("Failed to write to gram-hs: {}", e))?;
    }
    
    let result = output.wait_with_output()
        .map_err(|e| format!("Failed to get gram-hs output: {}", e))?;
    
    if !result.status.success() {
        return Err(format!("gram-hs failed: {}", String::from_utf8_lossy(&result.stderr)));
    }
    
    serde_json::from_slice(&result.stdout)
        .map_err(|e| format!("Failed to parse gram-hs JSON: {}", e))
}
```

### Test Data Extraction

Update `extract_test_cases_from_json` to use `--type suite`:

```rust
// In scripts/sync-tests/extract.rs

pub fn generate_test_suite(count: usize, seed: u64, complexity: &str) -> Result<Value, String> {
    let output = std::process::Command::new("gram-hs")
        .args(&[
            "generate",
            "--type", "suite",
            "--count", &count.to_string(),
            "--seed", &seed.to_string(),
            "--complexity", complexity,
            "--format", "json",
            "--value-only",  // Exclude metadata
        ])
        .output()
        .map_err(|e| format!("Failed to run gram-hs: {}", e))?;
    
    if !output.status.success() {
        return Err(format!("gram-hs generate failed: {}", 
            String::from_utf8_lossy(&output.stderr)));
    }
    
    serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Failed to parse test suite JSON: {}", e))
}
```

---

## Testing Workflow Examples

### Workflow 1: Generate Test Cases

```bash
# Generate test suite
gram-hs generate --type suite --count 100 --seed 42 --complexity standard \
    --format json --value-only > tests/common/test_cases.json

# Validate format
cargo run --bin test-validator tests/common/test_cases.json
```

### Workflow 2: Compare Single Pattern

```bash
# Get gram-hs reference
echo '(node1)-[edge]->(node2)' | gram-hs parse --format json --value-only --canonical > hs_output.json

# Get gram-rs output (when implemented)
echo '(node1)-[edge]->(node2)' | cargo run --bin gram-rs parse --format json --value-only > rs_output.json

# Compare
diff hs_output.json rs_output.json
```

### Workflow 3: Snapshot Testing

```bash
# Generate deterministic snapshot
echo '(node1)' | gram-hs parse --format json --deterministic --canonical > snapshot.json

# Use in insta snapshot test
# The snapshot will be identical on every run
```

---

## Benefits for gram-rs Testing

1. **Simplified Equivalence Checking**: `--value-only` eliminates metadata comparison issues
2. **Deterministic Testing**: `--deterministic` ensures reproducible test outputs
3. **Reliable Comparison**: `--canonical` ensures consistent JSON formatting
4. **Automated Test Generation**: `--type suite` generates test cases in the correct format
5. **Reduced Boilerplate**: Flags can be combined for optimal testing scenarios

---

## Next Steps

1. **Update Equivalence Checking Utilities**: Use `--value-only` and `--canonical` flags
2. **Update Test Extraction**: Use `--type suite` for automated test case generation
3. **Update Documentation**: Reference these flags in testing infrastructure docs
4. **Create Helper Functions**: Wrap gram-hs invocations in convenient Rust functions

---

## Related Documentation

- **Test Sync Format**: `specs/002-workspace-setup/contracts/test-sync-format.md`
- **Test Utilities API**: `specs/003-test-infrastructure/contracts/test-utilities-api.md`
- **Testing Infrastructure**: `specs/003-test-infrastructure/`

---

**Last Updated**: 2025-12-28

