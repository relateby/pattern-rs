# Benchmark Suite

This directory contains performance benchmarks using Criterion.

## Running Benchmarks

```bash
cargo bench
```

## Benchmark Organization

- `pattern_operations.rs` - Core pattern operation benchmarks

## WASM Compatibility

Benchmarks use conditional compilation for WASM targets due to timing limitations.

