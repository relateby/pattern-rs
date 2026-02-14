# gram-data Multi-Project Architecture

**Date**: January 9, 2026  
**Status**: Design Document  
**Purpose**: Define the architecture and boundaries between gram-data projects

---

## Overview

The gram-data ecosystem consists of **multiple independent projects** with clear responsibilities:

```
┌─────────────┐
│   pattern-rs   │  Parser only (this project)
│  (Rust)     │  Text → AST
└──────┬──────┘
       │
       ├──────────────┬──────────────┬──────────────┐
       │              │              │              │
       v              v              v              v
┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐
│ gram-js  │   │ gram-py  │   │ gram-jvm │   │  other   │
│(JS/TS)   │   │(Python)  │   │ (Java)   │   │languages │
│Pattern   │   │Pattern   │   │Pattern   │   │          │
│API       │   │API       │   │API       │   │          │
└────┬─────┘   └────┬─────┘   └────┬─────┘   └────┬─────┘
     │              │              │              │
     └──────────────┴──────┬───────┴──────────────┘
                           │
                           v
                    ┌──────────────┐
                    │pattern-frame │  DataFrame-like
                    │(multi-lang)  │  operations
                    └──────┬───────┘
                           │
                           v
                    ┌──────────────┐
                    │pattern-store │  Persistence
                    │/ pattern-db  │  & queries
                    └──────────────┘
```

---

## Project 1: pattern-rs (This Repository)

### Responsibility

**Parse gram notation** - Text → AST

### Scope

✅ **What pattern-rs does**:
- Parse gram notation text to AST (Abstract Syntax Tree)
- Validate gram syntax
- Serialize Pattern<Subject> back to gram notation
- Provide WASM bindings for JavaScript
- Provide Python bindings via PyO3
- Provide native Rust API

❌ **What pattern-rs does NOT do**:
- Pattern operations (map, fold, filter, etc.)
- Graph operations (queries, traversals)
- Type system or schema validation
- Persistence or storage
- Analytics or aggregations

### Output Format

**AST (Abstract Syntax Tree)** - JSON-serializable representation of Pattern<Subject>:

```json
{
  "subject": {
    "identity": "alice",
    "labels": ["Person"],
    "properties": {"name": "Alice", "age": 30}
  },
  "elements": [...]
}
```

### API Surface

```rust
// Rust API
pub fn parse_to_ast(input: &str) -> Result<AstPattern, ParseError>
pub fn parse_gram(input: &str) -> Result<Vec<Pattern<Subject>>, ParseError>
pub fn to_gram_pattern(pattern: &Pattern<Subject>) -> Result<String, SerializeError>
```

```javascript
// WASM API
parse_to_ast(input: string): AstPattern
validate_gram(input: string): boolean
round_trip(input: string): string
version(): string
```

```python
# Python API
parse_to_ast(input: str) -> dict
validate_gram(input: str) -> bool
round_trip(input: str) -> str
version() -> str
```

### Dependencies

- ✅ `nom` - Parser combinators
- ✅ `serde` / `serde_json` - Serialization
- ✅ `wasm-bindgen` - WASM bindings
- ✅ `pyo3` - Python bindings
- ✅ `pattern-core` - Pattern/Subject types (internal)

### Deliverables

- Rust crate on crates.io
- WASM package on npm
- Python wheel on PyPI

---

## Project 2: gram-js (Future)

### Responsibility

**Native JavaScript Pattern implementation** with full FP API

### Scope

✅ **What gram-js does**:
- Implement Pattern<V> and Subject types in TypeScript
- Full FP API (map, fold, traverse, comonad operations)
- Pattern queries and transformations
- Pattern combination and validation
- Convert AST to Pattern
- TypeScript types and IntelliSense support

❌ **What gram-js does NOT do**:
- Parse gram notation (uses pattern-rs via WASM)
- Persistence (uses pattern-store)
- DataFrame operations (uses pattern-frame)

### API Surface

```typescript
// Core types
class Pattern<V> {
  constructor(value: V, elements: Pattern<V>[])
  
  // Accessors
  value(): V
  elements(): Pattern<V>[]
  
  // FP operations
  map<W>(f: (v: V) => W): Pattern<W>
  fold<B>(init: B, f: (acc: B, v: V) => B): B
  filter(predicate: (p: Pattern<V>) => boolean): Pattern<V>[]
  
  // Queries
  depth(): number
  size(): number
  findFirst(predicate: (p: Pattern<V>) => boolean): Pattern<V> | null
  
  // Combination
  combine(other: Pattern<V>): Pattern<V>  // For V: Combinable
  
  // Serialization
  toGram(): string
  
  // Static constructors
  static fromAst(ast: AstPattern): Pattern<Subject>
  static parse(gramText: string): Pattern<Subject>
}

class Subject {
  identity: Symbol
  labels: Set<string>
  properties: Map<string, any>
}
```

### Dependencies

- ✅ `@gram-data/codec` (pattern-rs WASM) - For parsing only
- ❌ No runtime dependencies for Pattern operations

### Architecture

```typescript
// gram-js only depends on pattern-rs at parse time
import { parse_to_ast } from '@gram-data/codec';

// Convert AST to native Pattern
const ast = parse_to_ast("(alice:Person)");
const pattern = Pattern.fromAst(ast);

// All operations are pure TypeScript - zero WASM calls
const ids = pattern.fold([], (acc, s) => [...acc, s.identity.value]);
```

### Package Structure

```
gram-js/
├── src/
│   ├── pattern.ts        # Pattern<V> implementation
│   ├── subject.ts        # Subject type
│   ├── symbol.ts         # Symbol type
│   ├── value.ts          # Value types
│   ├── combinable.ts     # Combinable trait
│   └── index.ts          # Public API
├── tests/
│   ├── pattern.test.ts
│   ├── functor.test.ts
│   ├── foldable.test.ts
│   └── round-trip.test.ts
├── examples/
│   ├── basic-usage.ts
│   └── advanced.ts
├── package.json
└── tsconfig.json
```

### Deliverables

- npm package `@gram-data/pattern` or `gram-js`
- TypeScript type definitions
- Complete documentation

---

## Project 3: gram-py (Future)

### Responsibility

**Native Python Pattern implementation** with full FP API

### Scope

✅ **What gram-py does**:
- Implement Pattern[V] and Subject types in Python
- Full FP API (map, fold, traverse, comonad operations)
- Pattern queries and transformations
- Pattern combination and validation
- Convert AST to Pattern
- Type hints for IDE support

❌ **What gram-py does NOT do**:
- Parse gram notation (uses pattern-rs via PyO3)
- Persistence (uses pattern-store)
- DataFrame operations (uses pattern-frame)

### API Surface

```python
# Core types
class Pattern(Generic[V]):
    def __init__(self, value: V, elements: List['Pattern[V]'] = None)
    
    # Accessors
    @property
    def value(self) -> V
    @property
    def elements(self) -> List['Pattern[V]']
    
    # FP operations
    def map(self, f: Callable[[V], W]) -> 'Pattern[W]'
    def fold(self, init: B, f: Callable[[B, V], B]) -> B
    def filter(self, predicate: Callable[['Pattern[V]'], bool]) -> List['Pattern[V]']
    
    # Queries
    def depth(self) -> int
    def size(self) -> int
    def find_first(self, predicate: Callable[['Pattern[V]'], bool]) -> Optional['Pattern[V]']
    
    # Combination
    def combine(self, other: 'Pattern[V]') -> 'Pattern[V]'  # For V: Combinable
    
    # Serialization
    def to_gram(self) -> str
    
    # Static constructors
    @staticmethod
    def from_ast(ast: dict) -> 'Pattern[Subject]'
    
    @staticmethod
    def parse(gram_text: str) -> 'Pattern[Subject]'

@dataclass(frozen=True)
class Subject:
    identity: Symbol
    labels: Set[str]
    properties: Dict[str, Any]
```

### Dependencies

- ✅ `gram-codec` (pattern-rs wheel) - For parsing only
- ❌ No runtime dependencies for Pattern operations

### Architecture

```python
# gram-py only depends on pattern-rs at parse time
from gram_codec import parse_to_ast
from gram import Pattern, Subject

# Convert AST to native Pattern
ast = parse_to_ast("(alice:Person)")
pattern = Pattern.from_ast(ast)

# All operations are pure Python - zero FFI calls
ids = pattern.fold([], lambda acc, s: acc + [s.identity.value])
```

### Package Structure

```
gram-py/
├── gram/
│   ├── __init__.py
│   ├── pattern.py         # Pattern[V] implementation
│   ├── subject.py         # Subject type
│   ├── symbol.py          # Symbol type
│   ├── value.py           # Value types
│   └── combinable.py      # Combinable protocol
├── tests/
│   ├── test_pattern.py
│   ├── test_functor.py
│   ├── test_foldable.py
│   └── test_round_trip.py
├── examples/
│   ├── basic_usage.py
│   └── advanced.py
├── pyproject.toml
└── README.md
```

### Deliverables

- PyPI package `gram` or `gram-py`
- Type stubs (.pyi files)
- Complete documentation

---

## Project 4: pattern-frame (Future)

### Responsibility

**DataFrame-like operations on pattern collections**

Think: Pandas/Polars for graph patterns

### Scope

✅ **What pattern-frame does**:
- Columnar storage for pattern collections
- Bulk operations (filter, map, group, aggregate)
- Query DSL for pattern matching
- Analytics and statistics
- Conversion to/from DataFrames (Pandas, Polars, Arrow)
- Efficient representation for large datasets

❌ **What pattern-frame does NOT do**:
- Parse gram notation (uses pattern-rs)
- Implement Pattern API (uses gram-js/gram-py)
- Persistence (uses pattern-store)

### Conceptual API

```typescript
// TypeScript
import { PatternFrame } from 'pattern-frame';
import { Pattern } from 'gram-js';

// Create frame from patterns
const frame = PatternFrame.fromPatterns(patterns);

// Query DSL
const results = frame
  .filter(p => p.labels.has("Person"))
  .groupBy(p => p.properties.get("age"))
  .aggregate({ count: "count", avgAge: "mean" });

// Convert to DataFrame
const df = frame.toDataFrame();  // Polars, Pandas, etc.
```

```python
# Python
from pattern_frame import PatternFrame
from gram import Pattern

# Create frame from patterns
frame = PatternFrame.from_patterns(patterns)

# Query DSL
results = (frame
    .filter(lambda p: "Person" in p.labels)
    .group_by(lambda p: p.properties["age"])
    .aggregate(count="count", avg_age="mean"))

# Convert to DataFrame
df = frame.to_pandas()  # or .to_polars()
```

### Dependencies

- ✅ `gram-js` or `gram-py` - For Pattern types
- ✅ Arrow/Parquet - For columnar storage
- ✅ Optional: Pandas, Polars integration

---

## Project 5: pattern-store / pattern-db (Future)

### Responsibility

**Persistent storage and database operations**

### Scope

✅ **What pattern-store does**:
- ACID transactions
- Indexes for fast queries
- Multi-process access
- Query optimization
- SQL interop
- Backup/restore

❌ **What pattern-store does NOT do**:
- Parse gram notation (uses pattern-rs)
- Implement Pattern API (uses gram-js/gram-py)
- DataFrame operations (uses pattern-frame)

### Conceptual API

```typescript
// TypeScript
import { PatternStore } from 'pattern-store';
import { Pattern } from 'gram-js';

const store = await PatternStore.open("./patterns.db");

// Persist patterns
await store.persist(patterns);

// Query
const results = await store.query("MATCH (p:Person) WHERE p.age > 30");

// Transactions
await store.transaction(async (tx) => {
  await tx.insert(pattern1);
  await tx.update(pattern2);
});
```

### Dependencies

- ✅ `gram-js` or `gram-py` - For Pattern types
- ✅ Database backend (SQLite, DuckDB, etc.)
- ✅ Optional: pattern-frame for bulk operations

---

## Interchange Format: AST

All projects communicate via the **AST (Abstract Syntax Tree)**:

```
┌─────────┐
│ pattern-rs │──┐
└─────────┘  │
             │  AST (JSON)
             v
      ┌──────────────┐
      │ AstPattern   │
      │              │
      │ { subject:   │
      │   { identity │
      │     labels   │
      │     props }  │
      │   elements   │
      │   [...] }    │
      └──────────────┘
             │
    ┌────────┼────────┐
    │        │        │
    v        v        v
┌────────┐ ┌────────┐ ┌────────┐
│gram-js │ │gram-py │ │gram-jvm│
└────────┘ └────────┘ └────────┘
```

### Why AST?

1. **Language-agnostic** - Pure JSON, works everywhere
2. **Simple** - Just subject + elements (recursive)
3. **Complete** - No information loss
4. **Efficient** - Can be zero-copy in some cases
5. **Versioned** - Can evolve independently

---

## Benefits of This Architecture

### 1. Clear Responsibilities

Each project has a single, well-defined purpose:
- pattern-rs: **Parse**
- gram-js/py: **Operate**
- pattern-frame: **Analyze**
- pattern-store: **Persist**

### 2. Independent Evolution

- pattern-rs can optimize parsing without affecting consumers
- gram-js/py can add features without changing parser
- Projects can version independently

### 3. Language-Idiomatic APIs

Each language implements Pattern following its best practices:
- gram-js: Immutable, functional, TypeScript
- gram-py: Pythonic, dataclasses, type hints
- gram-jvm: Object-oriented, null-safe

### 4. No FFI Tax for Operations

Pattern operations run in native language:
```javascript
// Pure JavaScript - zero WASM calls
pattern.map(f).fold(init, g).filter(pred)
```

### 5. Small Binaries

- pattern-rs WASM: 88KB (just parser)
- gram-js: Pure JS (no native code)
- gram-py: Pure Python (no native code)

### 6. Easy to Port

To add gram support to a new language:
1. Use pattern-rs parser (WASM or native bindings)
2. Implement Pattern<V> in target language
3. Write `Pattern.fromAst(ast)` converter

---

## Development Roadmap

### Phase 7: AST Output (Current)
- Add `parse_to_ast()` to pattern-rs
- Update WASM/Python bindings
- Document architecture

### Phase 8: gram-js Foundation
- Create gram-js repository
- Implement Pattern<V> and Subject
- Port FP operations from gram-hs
- Add comprehensive tests

### Phase 9: gram-py Foundation
- Create gram-py repository
- Implement Pattern[V] and Subject
- Port FP operations from gram-hs
- Add type hints and tests

### Phase 10: pattern-frame Prototype
- Design columnar layout
- Implement basic operations
- Integration with gram-js/py

### Phase 11: pattern-store Prototype
- Choose database backend
- Implement persistence layer
- Add query optimization

---

## References

- [gram-hs](https://github.com/relateby/pattern-hs) - Haskell reference implementation
- [Apache Arrow](https://arrow.apache.org/) - Inspiration for columnar format
- [Polars](https://www.pola.rs/) - Inspiration for DataFrame API
- [DuckDB](https://duckdb.org/) - Potential database backend

---

**Status**: ✅ **ARCHITECTURE APPROVED**  
**Current Phase**: Phase 7 (AST Output)  
**Next Phase**: Phase 8 (gram-js Foundation)
