# Test Synchronization Format Contract

**Feature**: 002-workspace-setup  
**Date**: 2025-01-27

## Overview

This contract defines the JSON schema and structure for test case synchronization between gram-hs and pattern-rs.

## Test Case Schema

### JSON Structure

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

### Schema Definition

#### Root Object

- **version** (string, required): Schema version (e.g., "1.0")
- **test_cases** (array, required): Array of test case objects

#### Test Case Object

- **name** (string, required): Unique identifier for the test case
- **description** (string, optional): Human-readable description
- **input** (object, required): Input data for the test
  - **type** (string, required): Type of input ("gram_notation", "pattern", etc.)
  - **value** (any, required): Input value (string for gram notation, object for pattern)
- **expected** (object, required): Expected output
  - **type** (string, required): Type of expected output
  - **value** (any, required): Expected value
- **operations** (array, optional): Array of operations to test
  - **op** (string, required): Operation name ("match", "transform", etc.)
  - **against** (any, required): Operation input
  - **expected_bindings** (array, optional): Expected bindings/result

## Requirements

### Extraction

**Requirements** (FR-018):
- MUST support extracting test cases from gram-hs reference implementation
- Extraction tools MUST produce valid JSON conforming to this schema
- Extraction can be manual initially, with automation as future enhancement

### Comparison

**Requirements** (FR-019):
- Comparison tools MUST accept test cases in this format
- Comparison MUST identify:
  - Missing test cases in pattern-rs
  - Test cases with different expected outputs
  - Test cases with behavioral differences
- Comparison reports MUST be human-readable

### Storage

- Test cases SHOULD be stored in `tests/common/test_cases.json` or similar
- Test cases MAY be organized by feature/category
- Test case files MUST be version-controlled

## Example Test Case

```json
{
  "version": "1.0",
  "test_cases": [
    {
      "name": "simple_node_pattern",
      "description": "Test parsing a simple node pattern",
      "input": {
        "type": "gram_notation",
        "value": "(node)"
      },
      "expected": {
        "type": "pattern",
        "value": {
          "type": "Cons",
          "head": {
            "type": "Node",
            "labels": ["node"]
          },
          "tail": {
            "type": "Empty"
          }
        }
      },
      "operations": []
    },
    {
      "name": "pattern_matching",
      "description": "Test pattern matching operation",
      "input": {
        "type": "gram_notation",
        "value": "(node)"
      },
      "expected": {
        "type": "pattern",
        "value": { ... }
      },
      "operations": [
        {
          "op": "match",
          "against": "(node)-[edge]->(target)",
          "expected_bindings": [
            {
              "variable": "node",
              "value": { ... }
            }
          ]
        }
      ]
    }
  ]
}
```

## Implementation Notes

- Initial implementation MAY be minimal (structure only)
- Full automation can be added in future features
- Schema can be extended as needed for additional test types
- Validation tools SHOULD verify schema compliance
