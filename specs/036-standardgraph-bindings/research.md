# Research: StandardGraph Bindings

**Feature**: 036-standardgraph-bindings | **Date**: 2026-03-15

## R1: WASM &mut self chaining for StandardGraph

**Decision**: Use `&mut self` returning `&mut Self` for all mutating methods on `WasmStandardGraph`. Return `this` in JavaScript via wasm-bindgen's support for mutable references.

**Rationale**: The existing `WasmPattern::addElement(&mut self, element: &WasmPattern)` already uses `&mut self` successfully. `wasm_bindgen` handles `&mut self` transparently — the JS object retains ownership and mutation works in place. This is natural in JS/TS (`graph.addNode(n).addRelationship(r, a, b)`).

**Alternatives considered**:
- Consuming `self` (Builder pattern): Not viable — wasm-bindgen invalidates the JS object after the first consuming call.
- Clone-on-write: Unnecessary overhead and confusing semantics (each call returns a "new" graph).

## R2: SubjectBuilder ownership model at WASM boundary

**Decision**: `WasmSubjectBuilder` accumulates state internally (`identity: String`, `labels: Vec<String>`, `properties: HashMap<String, Value>`) using `&mut self` methods. The Rust `SubjectBuilder` is only constructed at `.done()` time.

**Rationale**: The Rust `SubjectBuilder` uses consuming `self` (`fn label(self, ...) -> Self`), which doesn't work with wasm-bindgen. By accumulating state in the WASM wrapper and constructing the Rust builder only at `.done()`, we avoid the consuming-self problem entirely.

**Alternatives considered**:
- Wrap Rust `SubjectBuilder` directly: Not viable due to consuming `self`.
- Single factory method `Subject.build(identity, labels, properties)`: Works but loses fluent builder ergonomics.

## R3: SubjectBuilder ownership model at Python boundary

**Decision**: Same as WASM — `PySubjectBuilder` accumulates state internally with `&mut self` methods. PyO3 handles `&mut self` naturally via its cell protocol.

**Rationale**: Consistent approach across targets. The existing `PySubject.add_label(&mut self)` and `PySubject.set_property(&mut self)` already use this pattern successfully.

**Alternatives considered**:
- Wrap Rust `SubjectBuilder` directly: Not viable due to consuming `self` in Rust.

## R4: Symbol handling at FFI boundary

**Decision**: All identity parameters are `&str` at the binding boundary. The WASM/Python wrapper converts `&str → Symbol` internally using `Symbol::from(str)`.

**Rationale**: The existing bindings already do this — `WasmSubject.identity` returns `String`, and all constructors take `&str`. Users never see `Symbol` in TypeScript or Python.

**Alternatives considered**:
- Expose Symbol as a separate type: Unnecessary complexity for binding users.

## R5: from_gram implementation location

**Decision**: `fromGram`/`from_gram` MUST be implemented outside `pattern-core` due to circular dependency constraints (`gram-codec` depends on `pattern-core` for types, so `pattern-core` cannot depend on `gram-codec`).

- **WASM**: Implement `fromGram` in `crates/pattern-wasm/src/lib.rs` using `#[wasm_bindgen(js_class = "StandardGraph")]` to attach it to the exported class. The `pattern-wasm` crate already depends on both `pattern-core` and `gram-codec`.
- **Python**: Implement `from_gram` as a pure-Python classmethod in `python/relateby/pattern/__init__.py` (the unified package layer), bridging gram's `parse_gram()` and `StandardGraph.from_patterns()`.

**Rationale**: The existing `Gram.parse()` in `pattern-wasm/src/gram.rs` demonstrates this cross-crate pattern for WASM. For Python, the unified `relateby` package can import from both subpackages. No need to expose the `FromGram` trait.

**Alternatives considered**:
- Implement in `pattern-core` with optional `gram` feature: Rejected — Cargo does not allow circular dependencies even with optional features.
- Expose `FromGram` trait as a separate binding: Over-engineered for one method.
- Separate `Gram.toStandardGraph()` method: Adds indirection users don't need.

## R6: Iterator materialization strategy

**Decision**: All iteration methods return materialized arrays/lists. WASM returns `js_sys::Array`, Python returns `list`.

**Rationale**: The existing `WasmPatternGraph.nodes()` already returns `js_sys::Array`. Graph sizes in practice are small enough that materialization is fine. Lazy iteration across FFI boundaries adds complexity with negligible benefit.

**Alternatives considered**:
- Python `__iter__` protocol: Could be added later if profiling warrants it.
- WASM generators: Not well-supported in wasm-bindgen.

## R7: Error mapping for from_gram

**Decision**: Map `ParseError` to JavaScript `Error` (WASM) via `JsValue::from_str()` and to Python `ValueError` (Python) via `PyValueError`.

**Rationale**: The existing `Gram.parse()` in pattern-wasm already maps parse errors to JS errors. The existing Python bindings use `PyValueError` for invalid input.

**Alternatives considered**:
- Custom error types: Over-engineered for parse errors.

## R8: Return type for element access and queries

**Decision**:
- WASM: `node(id)` returns `Option<WasmPattern>` (mapped to `Pattern | undefined` in TS). `neighbors()` returns `js_sys::Array` of `WasmPattern`.
- Python: `node(id)` returns `Option<PyPattern>` (mapped to `PatternSubject | None`). `neighbors()` returns `list[PatternSubject]`.

**Rationale**: Follows existing patterns. `WasmGraphQuery.nodeById()` already returns `Option<WasmPattern>`. Python uses `Option<T>` → `T | None` naturally.

**Alternatives considered**:
- Return `(id, pattern)` tuples for single access: Unnecessary — the user already has the id.

## R9: Python escape hatches scope

**Decision**: Defer all Python escape hatches (`as_query`, `as_pattern_graph`, `as_snapshot`) until the abstract graph layer has Python bindings.

**Rationale**: There is no `PyPatternGraph` or `PyGraphQuery` yet. Exposing escape hatches that return types users can't use is confusing. The proposal explicitly defers this.

**Alternatives considered**:
- Implement PyPatternGraph/PyGraphQuery as part of this feature: Scope creep. Those deserve their own feature.

## R10: Conversion helpers reuse

**Decision**: Reuse existing conversion helpers (`subject_pattern_to_wasm`, `wasm_pattern_to_subject_pattern`, `js_value_to_subject_pattern` for WASM; `python_to_value`, `value_to_python` for Python) for all StandardGraph boundary operations.

**Rationale**: All boundary types (`Subject`, `Pattern<Subject>`, `Value`) are already wrapped. StandardGraph methods just compose these existing conversions.

**Alternatives considered**: None — this is the only reasonable approach.
