# Test Synchronization Utilities

This directory contains utilities for maintaining test parity between gram-rs and gram-hs reference implementation.

## Overview

Test synchronization ensures that gram-rs tests remain aligned with the gram-hs reference implementation, maintaining behavioral equivalence as features are ported.

## Test Case Format

Test cases are stored in JSON format following the schema defined in `specs/002-workspace-setup/contracts/test-sync-format.md`.

### JSON Schema

```json
{
  "version": "1.0",
  "test_cases": [
    {
      "name": "test_case_identifier",
      "description": "Human-readable description",
      "input": {
        "type": "gram_notation",
        "value": "(node)-[edge]->(target)"
      },
      "expected": {
        "type": "pattern",
        "value": { ... }
      },
      "operations": [
        {
          "op": "match",
          "against": "(pattern)",
          "expected_bindings": [ ... ]
        }
      ]
    }
  ]
}
```

## Utilities

### extract.sh / extract.rs

Extracts test cases from gram-hs reference implementation and converts them to the common JSON format.

**Usage** (placeholder - to be implemented):
```bash
./extract.sh ../gram-hs > tests/common/test_cases.json
```

**Alternative: Using gram-hs CLI**:

The `gram-hs` CLI tool can generate test suites directly:
```bash
# Generate test suite with 100 test cases
gram-hs generate --type suite --count 100 --seed 42 --complexity standard \
    --format json --value-only > tests/common/test_cases.json
```

See [gram-hs CLI Testing Guide](../../docs/gram-hs-cli-testing-guide.md) for comprehensive usage examples and integration patterns.

### compare.sh / compare.rs

Compares test cases between gram-hs and gram-rs implementations, identifying differences in coverage and behavior.

**Usage** (placeholder - to be implemented):
```bash
./compare.sh tests/common/test_cases.json
```

## Storage

Test cases are stored in `tests/common/test_cases.json` at the workspace root.

## Future Enhancements

- Full automation of test extraction from gram-hs
- Automated comparison and reporting
- Integration with CI/CD pipeline
- Test case validation and schema checking

## Implementation Status

- `extract.rs` - Test case extraction and validation utilities (implemented)
- `compare.rs` - Test case comparison utilities (implemented)
- JSON format validation - Validates test case format against schema

