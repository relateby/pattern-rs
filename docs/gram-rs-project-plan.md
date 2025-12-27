# gram-rs Project Plan

## Project Overview

Port of gram-hs to Rust, maintaining both a faithful reference implementation and an optimized engine for UI applications. The library will compile to both native Rust and WASM targets. 

**Core Concept**: Patterns are the primary data structure (like objects in JavaScript), while gram notation is the serialization format (like JSON). The project name "gram-rs" honors the original inspiration, but internally everything is pattern-centric.

## Project Setup

### Repository Structure

```
gram-rs/
├── Cargo.toml                 # Workspace manifest
├── crates/
│   ├── pattern-core/          # Core Pattern data structures
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── pattern.rs    # Core Pattern ADT
│   │       ├── subject.rs    # Subject types
│   │       └── types.rs      # Supporting types
│   ├── pattern-ops/           # Pattern operations and algorithms
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── matching.rs   # Pattern matching
│   │       ├── transform.rs  # Pattern transformations
│   │       └── traverse.rs   # Pattern traversal
│   ├── gram-codec/            # Gram notation serialization/deserialization
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── decode.rs     # Gram → Pattern
│   │       ├── encode.rs     # Pattern → Gram
│   │       └── grammar.rs    # Gram syntax definition
│   ├── pattern-store/         # Optimized pattern storage and indexing
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── store.rs      # Columnar storage
│   │       ├── index.rs      # Pattern indices
│   │       └── query.rs      # Optimized queries
│   └── pattern-wasm/          # WASM bindings for patterns
│       ├── Cargo.toml
│       ├── package.json
│       └── src/
│           └── lib.rs
├── tests/
│   ├── common/                # Shared test data from gram-hs
│   │   └── test_cases.json
│   └── integration/
└── benches/                   # Benchmarks comparing implementations
```

### Initial Workspace Setup

```toml
# Cargo.toml
[workspace]
members = ["crates/*"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "MIT OR Apache-2.0"
repository = "https://github.com/gram-data/gram-rs"

[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "2.0"
```

### Development Setup

```bash
# Clone both repositories
git clone https://github.com/gram-data/gram-rs.git
git clone https://github.com/gram-data/gram-hs.git ../gram-hs

# Add gram-hs as a submodule for reference
cd gram-rs
git submodule add https://github.com/gram-data/gram-hs.git reference/gram-hs

# Setup Rust toolchain
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
cargo install cargo-watch
cargo install cargo-criterion

# Initial build
cargo build --all
cargo test --all
```

## Maintaining Reference to gram-hs

### Test Synchronization Strategy

1. **Shared Test Format**: Define a JSON schema for test cases that both gram-hs and gram-rs can use:

```json
{
  "version": "1.0",
  "test_cases": [
    {
      "name": "simple_node_pattern",
      "input_notation": "(node)",
      "expected_pattern": {
        "type": "Cons",
        "head": {"type": "Node", "labels": ["node"]},
        "tail": {"type": "Empty"}
      },
      "operations": [
        {
          "op": "match",
          "against": "(node)-[edge]->(target)",
          "expected_bindings": [...]
        }
      ]
    }
  ]
}
```

2. **Reference Sync Script**:

```rust
// scripts/sync_tests.rs
use std::process::Command;

fn sync_gram_hs_tests() {
    // Extract test cases from gram-hs
    Command::new("stack")
        .args(&["test", "--dump-test-json"])
        .current_dir("../gram-hs")
        .output()
        .expect("Failed to run gram-hs tests");
    
    // Convert to common format
    // Copy to tests/common/
}
```

3. **CI Integration**: Add GitHub Actions to verify compatibility on each commit.

## Updates Needed to gram-hs

### Proposed gram-hs Enhancements

1. **Test Export Module**:
```haskell
-- test/Export.hs
module Test.Export where

import Data.Aeson
import Test.QuickCheck

-- Export property test cases as JSON
exportTestCases :: IO ()
exportTestCases = do
  cases <- generateTestCases
  encodeFile "test-cases.json" cases

data TestCase = TestCase
  { tcName :: Text
  , tcInput :: Text  -- gram notation
  , tcExpectedPattern :: Pattern Subject
  , tcOperations :: [TestOperation]
  } deriving (Generic, ToJSON)
```

2. **Serialization for Core Types**:
```haskell
-- Add to gram-hs
instance ToJSON (Pattern a) where
  toJSON = genericToJSON defaultOptions

instance FromJSON (Pattern a) where
  parseJSON = genericParseJSON defaultOptions
```

3. **Behavioral Specification**:
```yaml
# gram-spec.yaml
version: 1.0
operations:
  match:
    signature: "Pattern -> Graph -> [Binding]"
    properties:
      - commutative: false
      - associative: true
  transform:
    signature: "Pattern -> Pattern -> Pattern"
    laws:
      - "transform Empty p = p"
      - "transform p Empty = p"
```

## Build Phases

### Phase 1: Foundation

**Goal**: Establish project structure and core Pattern types

1. Setup workspace and CI pipeline
2. Define core Pattern types (`Pattern`, `Subject`, `Record`)
3. Implement `Display` and `Debug` traits for patterns
4. Setup property-based testing with `proptest`
5. Create basic benchmarking suite

**Validation**: Pattern types match gram-hs semantics exactly

### Phase 2: Pattern Operations

**Goal**: Complete faithful port of pattern operations from gram-hs

1. Port pattern operations (fold, map, traverse)
2. Implement pattern matching algorithm
3. Port pattern transformation functions
4. Add pattern equivalence checking
5. Ensure all gram-hs tests pass

**Validation**: 100% test parity with gram-hs

### Phase 3: Gram Codec

**Goal**: Serialize/deserialize patterns to/from gram notation

1. Implement gram grammar using chosen parser library
2. Add error recovery and reporting
3. Support streaming decode for large gram files
4. Implement pretty-printing/formatting for patterns → gram
5. Round-trip testing (pattern → gram → pattern)

**Validation**: Correctly encode/decode all gram-hs examples

### Phase 4: Pattern Store

**Goal**: Build performance-optimized pattern storage

1. Design columnar storage for patterns
2. Implement string interning for pattern values
3. Add spatial indices for UI pattern queries
4. Create incremental pattern update system
5. Build pattern query optimizer

**Validation**: 10x performance improvement on pattern operations

### Phase 5: WASM Integration

**Goal**: Browser-ready WASM package

1. Create wasm-bindgen interfaces
2. Optimize for size (wee_alloc, compression)
3. Add TypeScript definitions
4. Create JavaScript test suite
5. Build example web application

**Validation**: <100KB WASM size, <10ms parse time for typical graphs

### Phase 6: Production Features

**Goal**: Production-ready library

1. Add comprehensive logging/telemetry
2. Implement error recovery strategies
3. Add migration tools from other formats
4. Create debugging/profiling tools
5. Documentation and tutorials

## Library Recommendations

### Data Structure Libraries

**Primary Recommendation: Build Custom Pattern Types**
- Polars is excellent but focused on DataFrames, not patterns
- Arrow-rs provides columnar format but may be overkill for patterns
- Better to start with lean pattern implementation and optimize based on usage

**Supporting Libraries for Pattern Store:**
```toml
# For optimized pattern storage
petgraph = "0.6"          # Graph algorithms for patterns
roaring = "0.10"          # Compressed bitmaps for pattern indices
rstar = "0.12"           # R-tree for spatial pattern queries
dashmap = "6.0"          # Concurrent pattern hashmaps
bytes = "1.5"            # Efficient byte buffers
smol_str = "0.3"         # Small string optimization for pattern values
```

### Parser Library Recommendation

**Primary: Winnow (for gram codec)**
```toml
winnow = "0.6"
```

**Rationale:**
- Zero-copy parsing (crucial for performance)
- Excellent error messages
- Streaming support built-in
- More maintainable than nom
- Active development

**Alternative: Pest (if gram grammar becomes complex)**
```toml
pest = "2.7"
pest_derive = "2.7"
```

Example winnow parser for gram → pattern:
```rust
use winnow::prelude::*;
use winnow::token::*;
use winnow::combinator::*;

fn decode_node(input: &mut &str) -> PResult<Pattern<Subject>> {
    seq!(
        '(',
        parse_identifier,
        opt(parse_labels),
        opt(parse_record),
        ')'
    ).map(|(_, id, labels, record, _)| {
        Pattern::cons(
            Subject::Node { id, labels, record },
            Pattern::empty()
        )
    }).parse_next(input)
}
```

### Core Dependencies

```toml
[workspace.dependencies]
# Serialization
serde = { version = "1.0", features = ["derive", "rc"] }
serde_json = "1.0"
rkyv = "0.8"              # Zero-copy deserialization

# Error handling
thiserror = "2.0"
anyhow = "1.0"

# Testing
proptest = "1.4"
criterion = "0.5"
insta = "1.34"            # Snapshot testing
test-case = "3.1"

# WASM
wasm-bindgen = "0.2"
wasm-bindgen-test = "0.3"
web-sys = "0.3"
js-sys = "0.3"
console_error_panic_hook = "0.1"
wee_alloc = "0.4"         # Small allocator
```

## Getting Started Checklist

### For the Lead Engineer

1. **Setup Phase** (Day 1)
   - [ ] Fork gram-hs and create gram-rs repo
   - [ ] Setup workspace structure with pattern-centric naming
   - [ ] Configure CI/CD (GitHub Actions recommended)
   - [ ] Create initial Pattern type definitions

2. **Pattern Core** (Week 1)
   - [ ] Port Pattern ADT from gram-hs
   - [ ] Port Subject types for patterns
   - [ ] Setup property testing for patterns
   - [ ] Create test synchronization

3. **Gram Codec** (Week 2)
   - [ ] Define gram grammar formally
   - [ ] Implement pattern → gram encoder
   - [ ] Implement gram → pattern decoder
   - [ ] Test round-tripping against gram-hs examples

4. **Pattern Store Planning** (Week 3+)
   - [ ] Profile pattern operations
   - [ ] Design optimized pattern storage
   - [ ] Prototype key pattern indices
   - [ ] Benchmark pattern queries

### Key Success Metrics

1. **Correctness**: Pattern operations match gram-hs 100%
2. **Performance**: <100ms for 10K pattern operations
3. **Size**: <100KB WASM bundle with patterns (compressed)
4. **Usability**: TypeScript types for patterns auto-generated
5. **Maintainability**: Pattern behavior stays aligned with gram-hs

## Next Steps

1. Review and refine this plan with stakeholders
2. Create GitHub issues for each phase
3. Set up weekly sync between gram-hs and gram-rs teams
4. Begin Phase 1 implementation

## Resources

- [gram-hs repository](https://github.com/gram-data/gram-hs)
- [Rust WASM Book](https://rustwasm.github.io/docs/book/)
- [Winnow Documentation](https://docs.rs/winnow/latest/winnow/)
- [Property Testing in Rust](https://proptest-rs.github.io/proptest/)
