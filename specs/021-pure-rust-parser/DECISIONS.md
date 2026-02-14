# Design Decisions for pattern-rs AST Output

**Date**: January 9, 2026  
**Status**: Approved  
**Phase**: 7 (AST Output Implementation)

---

## Decision 1: AST Structure - Pattern<Subject> Only

### Decision
The AST mirrors the `Pattern<Subject>` structure exactly - no graph-specific concepts.

### Rationale
- Gram notation describes patterns, not graphs
- Path notation (`(a)-->(b)`) is syntactic sugar already desugared by parser
- Keeping AST pure to Pattern structure enables clean separation of concerns
- Graph interpretation happens in consumer code (gram-js, gram-py)

### Rejected Alternatives
- ❌ Include node/edge/relationship types in AST
- ❌ Add graph-specific metadata

### Impact
- ✅ Clean conceptual model
- ✅ Easy to understand and document
- ✅ No leaking of graph concepts into pattern layer

---

## Decision 2: Value Serialization - Mixed Approach

### Decision
Use **native JSON** for simple types, **tagged objects** for complex types.

### Rationale
1. **Simple cases are simple**: Most properties are strings, booleans, numbers
2. **No ambiguity**: Symbol vs String, Integer vs Decimal distinguishable
3. **Easy to consume**: Native types work directly in JavaScript/Python
4. **Preserves semantics**: Complex types retain their meaning

### Implementation

#### Native JSON (Simple Types)
```rust
Value::VNull => serde_json::Value::Null
Value::VBoolean(b) => serde_json::Value::Bool(*b)
Value::VString(s) => serde_json::Value::String(s)
Value::VArray(arr) => serde_json::Value::Array([...])
Value::VMap(map) => serde_json::Value::Object({...})
```

**JSON output**:
```json
{
  "name": "Alice",
  "active": true,
  "tags": ["dev", "admin"]
}
```

#### Tagged Objects (Complex Types)
```rust
Value::VSymbol(sym) => json!({"type": "Symbol", "value": sym.0})
Value::VInteger(i) => json!({"type": "Integer", "value": i})
Value::VDecimal(d) => json!({"type": "Decimal", "value": d})
Value::VRange(r) => json!({"type": "Range", "lower": ..., "upper": ...})
Value::VMeasurement {...} => json!({"type": "Measurement", ...})
Value::VTagged {...} => json!({"type": "Tagged", ...})
```

**JSON output**:
```json
{
  "id": {"type": "Symbol", "value": "user123"},
  "age": {"type": "Integer", "value": 30},
  "height": {"type": "Measurement", "value": 168, "unit": "cm"}
}
```

### Rejected Alternatives

#### All Native JSON
```json
{
  "id": "user123",        // Ambiguous: Symbol or String?
  "age": 30,              // Ambiguous: Integer or Decimal?
  "range": [1, 10]        // Ambiguous: Array or Range?
}
```
**Problem**: Loss of semantic information, can't round-trip

#### All Tagged
```json
{
  "name": {"type": "String", "value": "Alice"},
  "active": {"type": "Boolean", "value": true}
}
```
**Problem**: Overly verbose for common cases

### Trade-offs Accepted
- **Inconsistent**: Some values are primitives, some are objects
- **Type checking needed**: Consumers must check for `{type: ...}` pattern
- **Slightly verbose**: Tagged types take more space

**Why acceptable**: Optimizes for the common case (80% of values are simple), adds complexity only where semantic distinction matters.

---

## Decision 3: Single File-Level Pattern

### Decision
`parse_to_ast()` returns a **single `AstPattern`** representing the file-level pattern.

### Rationale
- Matches the semantic model: gram file = one pattern
- Optional leading `{}` becomes file-level properties
- All patterns in file become file-level elements
- Simpler API than returning `Vec<AstPattern>`

### Example
```rust
// Input: "{version: '1.0'} (alice) (bob)"
// Output: Single AstPattern
{
  subject: {
    identity: "",
    labels: [],
    properties: {version: "1.0"}
  },
  elements: [
    {subject: {identity: "alice", ...}, elements: []},
    {subject: {identity: "bob", ...}, elements: []}
  ]
}
```

### Empty File Behavior
Empty input returns empty pattern:
```rust
{
  subject: {identity: "", labels: [], properties: {}},
  elements: []
}
```

### Rejected Alternatives
- ❌ Return `Vec<AstPattern>` (more complex, doesn't match semantic model)
- ❌ Return `Option<AstPattern>` (complicates API for rare empty case)

---

## Decision 4: Error Handling - Simple Messages

### Decision
Keep error representation simple for Phase 7.

### Current Approach
```rust
Err(ParseError) => JsValue::from_str(&format!("Parse error: {}", e))
```

### Future Enhancement (Phase 8+)
```rust
{
  "error": "Unexpected token",
  "location": {"line": 3, "column": 15},
  "context": "(alice)-[KNOWS",
  "suggestion": "Did you mean '-[:KNOWS]->'?"
}
```

### Rationale
- AST implementation is additive, doesn't change errors
- Simple errors are sufficient for initial release
- Can enhance later without breaking changes
- Focus Phase 7 on getting AST working correctly

---

## Decision 5: No Streaming Support (Yet)

### Decision
Parse entire file to AST in one operation.

### Rationale
- Most gram files are small (< 1MB)
- Streaming complicates the API significantly
- Can add later as `parse_to_ast_stream()` without breaking changes
- YAGNI (You Aren't Gonna Need It) - wait for actual requirement

### When to Reconsider
- If users report parsing files > 10MB
- If memory usage becomes a problem
- If incremental parsing needed for editors

### Future API (if needed)
```rust
pub fn parse_to_ast_stream(input: &str) -> impl Iterator<Item = Result<AstEvent, ParseError>>
```

---

## Decision 6: No Structural Validation

### Decision
pattern-rs validates **syntax only**, not pattern structure.

### Rationale
- Parsing and validation are separate concerns
- Structural validation depends on use case
- Graph schemas vary by domain
- Better handled by separate tools or gram-js/gram-py

### What pattern-rs Validates
- ✅ Syntax: parentheses balanced, quotes closed, valid identifiers
- ✅ Grammar: follows gram notation rules

### What pattern-rs Does NOT Validate
- ❌ Required properties
- ❌ Property types (beyond Value type)
- ❌ Graph structure (cycles, connectivity)
- ❌ Domain-specific rules

### Future Validation Options
- JSON Schema for AST (external tool)
- gram-js/gram-py validation API
- Separate `gram-validate` tool

---

## Decision 7: Project Scope - Parser Only

### Decision
pattern-rs is responsible for **parsing only**, not Pattern operations.

### Scope

✅ **In Scope**:
- Parse gram notation to AST
- Validate syntax
- Serialize Pattern<Subject> back to gram
- Provide WASM/Python bindings for parsing

❌ **Out of Scope**:
- Pattern operations (map, fold, filter)
- Graph queries or traversals
- Type system or schema validation
- Persistence or storage
- Analytics or aggregations

### Why This Matters
- Keeps pattern-rs focused and maintainable
- Enables language-specific Pattern implementations (gram-js, gram-py)
- Avoids FFI overhead for Pattern operations
- Allows independent evolution of parsing vs. operations

### Future Projects
- **gram-js**: Native JavaScript Pattern API
- **gram-py**: Native Python Pattern API
- **pattern-frame**: DataFrame-like operations
- **pattern-store**: Persistence and queries

---

## Decision 8: Additive Changes Only

### Decision
Phase 7 adds AST output without removing or changing existing API.

### What's Added
- ✅ `pub fn parse_to_ast(input: &str) -> Result<AstPattern, ParseError>`
- ✅ `pub struct AstPattern` and `pub struct AstSubject`
- ✅ WASM binding: `parse_to_ast(input: string): object`
- ✅ Python binding: `parse_to_ast(input: str) -> dict`

### What's Unchanged
- ✅ `parse_gram()` still works (returns `Vec<Pattern<Subject>>`)
- ✅ `validate_gram()` still works
- ✅ `round_trip()` still works
- ✅ All existing tests still pass

### Migration Path
- Phase 7: Both APIs available (parse_gram and parse_to_ast)
- Phase 8+: Encourage AST usage in examples
- Phase 9+: Consider deprecation of metadata-only API (if needed)

### Rationale
- No breaking changes for existing users
- Gradual migration path
- Can validate new API before committing to it

---

## Summary of Decisions

| Decision | Choice | Key Rationale |
|----------|--------|---------------|
| **AST Structure** | Pattern<Subject> only | Clean conceptual model |
| **Value Serialization** | Mixed (native + tagged) | Optimize common case |
| **Return Type** | Single AstPattern | Matches semantic model |
| **Errors** | Simple messages | Sufficient for Phase 7 |
| **Streaming** | Not yet | YAGNI |
| **Validation** | Syntax only | Separate concern |
| **Project Scope** | Parser only | Enable specialization |
| **API Changes** | Additive only | No breaking changes |

---

**Status**: ✅ **ALL DECISIONS APPROVED**  
**Ready for**: Implementation (Phase 7)  
**Next Step**: T083 - Define AST types in Rust
