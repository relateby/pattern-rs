# gramref CLI Quick Reference

**Last Updated**: 2025-01-04

## What is gramref?

`gramref` (formerly `gram-hs`) is a CLI tool for generating test patterns and validating outputs.

**Location**: `/Users/akollegger/.cabal/bin/gramref` (or in PATH)

**Not to be confused with**: The `../gram-hs/` library which contains the Haskell source code to port.

## Important Distinction

| Aspect | `../gram-hs/` Library | `gramref` CLI Tool |
|--------|----------------------|-------------------|
| **What** | Haskell source code | Executable program |
| **Location** | `../gram-hs/libs/` | `/Users/akollegger/.cabal/bin/gramref` |
| **Purpose** | Reference implementation to port | Testing and validation tool |
| **Use for** | Reading code, understanding algorithms | Generating test data, checking outputs |
| **When** | During implementation | During testing |

## Common Commands

### Generate Test Suite

```bash
# Generate 100 test cases with standard complexity
gramref generate --type suite --count 100 --seed 42 --format json --value-only

# With full metadata
gramref generate --type suite --count 50 --seed 42 --format json

# Different complexity levels
gramref generate --type suite --count 10 --complexity minimal --format json
gramref generate --type suite --count 10 --complexity basic --format json
gramref generate --type suite --count 10 --complexity standard --format json
gramref generate --type suite --count 10 --complexity complex --format json
gramref generate --type suite --count 10 --complexity adversarial --format json
```

### Parse Pattern for Testing

```bash
# Basic parsing
echo '(node1)' | gramref parse --format json

# Value only (no metadata)
echo '(node1)' | gramref parse --format json --value-only

# Canonical format (sorted keys)
echo '(node1)-[edge]->(node2)' | gramref parse --format json --canonical

# Deterministic output (fixed timestamps/hashes)
echo '(node1)' | gramref parse --format json --deterministic

# Combined flags for reliable comparison
echo '(node1)' | gramref parse --format json --value-only --canonical
```

## Key Flags

### Output Control

- `--format json` - Output in JSON format
- `--value-only` - Output only the pattern value (no metadata)
- `--canonical` - Sort JSON keys alphabetically for consistent output
- `--deterministic` - Use fixed timestamps and hashes for reproducible outputs

### Generation Control

- `--type suite` - Generate test suite format
- `--count N` - Number of test cases to generate
- `--seed N` - Random seed for reproducible generation
- `--complexity LEVEL` - Complexity level: minimal, basic, standard, complex, adversarial

## Use Cases in gram-rs Testing

### 1. Equivalence Testing

Generate reference outputs to compare with gram-rs:

```bash
# Get reference output
echo '(node1)-[edge]->(node2)' | gramref parse --format json --value-only --canonical > ref_output.json

# Compare with gram-rs output
diff ref_output.json rs_output.json
```

### 2. Test Case Generation

Create large test suites with various complexity levels:

```bash
# Generate test suite
gramref generate --type suite --count 100 --seed 42 --complexity standard \
    --format json --value-only > tests/common/test_cases.json
```

### 3. Snapshot Testing

Generate deterministic outputs for insta snapshots:

```bash
# Generate snapshot
echo '(node1)' | gramref parse --format json --deterministic --canonical > snapshot.json
```

### 4. Property Testing

Generate diverse patterns for property-based tests:

```bash
# Generate adversarial test cases
gramref generate --type suite --count 50 --complexity adversarial \
    --format json --value-only
```

## Integration with Rust Tests

### In Test Code

```rust
use std::process::Command;

// Get reference output from gramref
let output = Command::new("gramref")
    .args(&["parse", "--format", "json", "--value-only", "--canonical"])
    .stdin(Stdio::piped())
    .output()?;

let ref_value: serde_json::Value = serde_json::from_slice(&output.stdout)?;
```

### In Test Scripts

```bash
#!/bin/bash
# Generate test data
gramref generate --type suite --count 100 --seed 42 \
    --format json --value-only > test_data.json

# Run tests
cargo test --test equivalence
```

## Common Workflows

### Workflow 1: Generate and Validate

```bash
# 1. Generate test suite
gramref generate --type suite --count 100 --seed 42 --format json --value-only > tests.json

# 2. Validate format
cargo run --bin test-validator tests.json

# 3. Run equivalence tests
cargo test --test equivalence
```

### Workflow 2: Compare Implementations

```bash
# 1. Get gramref reference
echo '(n1)-[e]->(n2)' | gramref parse --format json --value-only --canonical > ref.json

# 2. Get gram-rs output (when CLI is implemented)
echo '(n1)-[e]->(n2)' | cargo run --bin gram-rs parse --format json --value-only > rs.json

# 3. Compare
diff ref.json rs.json
```

### Workflow 3: Snapshot Testing

```bash
# 1. Generate deterministic snapshot
echo '(node1)' | gramref parse --format json --deterministic --canonical > snapshot.json

# 2. Use in test
cargo test --test snapshot_tests
```

## Tips and Best Practices

1. **Always use `--value-only` for comparisons** - Eliminates metadata differences
2. **Use `--canonical` for reliable diffs** - Ensures consistent key ordering
3. **Use `--deterministic` for snapshots** - Makes outputs reproducible
4. **Set `--seed` for reproducible generation** - Same seed = same test cases
5. **Start with `--complexity minimal`** - Easier to debug issues

## Troubleshooting

### gramref not found

```bash
# Check if gramref is in PATH
which gramref

# If not, use full path
/Users/akollegger/.cabal/bin/gramref --help
```

### Output format issues

```bash
# Use --canonical to ensure consistent formatting
gramref parse --format json --canonical --value-only
```

### Reproducibility issues

```bash
# Use --deterministic and --seed together
gramref generate --seed 42 --deterministic --format json
```

## Related Documentation

- **Detailed Guide**: [gramref CLI Testing Guide](gramref-cli-testing-guide.md)
- **Porting Guide**: [porting-guide.md](porting-guide.md) - Library vs CLI distinction
- **Test Infrastructure**: [specs/003-test-infrastructure/](../specs/003-test-infrastructure/)

---

**Remember**: 
- Read from `../gram-hs/libs/` when implementing
- Execute `gramref` when testing

