# Testing Infrastructure

This document describes the testing infrastructure for gram-rs, including property-based testing, equivalence checking, snapshot testing, benchmarks, and test helpers.

## Overview

The testing infrastructure provides comprehensive tools for ensuring correctness and behavioral equivalence with the gram-hs reference implementation.

## Components

### Property-Based Testing (proptest)

Property-based testing generates random test inputs automatically and validates properties that should always hold true.

**Usage**: See `specs/003-test-infrastructure/quickstart.md` for examples.

### Equivalence Checking

Utilities for comparing outputs from gram-rs and gram-hs implementations to ensure behavioral equivalence.

**Usage**: 
- See `crates/pattern-core/src/test_utils/equivalence.rs` for API documentation
- See [gram-hs CLI Testing Guide](gram-hs-cli-testing-guide.md) for using the `gram-hs` CLI tool with `--value-only`, `--deterministic`, and `--canonical` flags for reliable comparison

### Snapshot Testing (insta)

Snapshot testing captures outputs and detects changes to catch regressions.

**Usage**: See `crates/pattern-core/tests/snapshot/README.md` for workflow.

### Benchmark Suite (criterion)

Performance benchmarks for tracking performance over time and detecting regressions.

**Usage**: See `benches/README.md` for details.

### Test Helpers

Utilities for pattern comparison and validation that reduce boilerplate in tests.

**Usage**: See `crates/pattern-core/src/test_utils/helpers.rs` for API documentation.

## Quick Start

A new developer can write a property test within 15 minutes:

1. Add test dependencies to your crate's `Cargo.toml`
2. Create a test file in `tests/property/`
3. Write a property test using `proptest!` macro
4. Run with `cargo test`

See `specs/003-test-infrastructure/quickstart.md` for detailed examples.

## Integration

All testing infrastructure integrates with the Cargo workspace structure and works across all crates.

