# Phase 3 Implementation Issues and Corrections Needed

**Date**: 2026-01-31  
**Status**: Issues identified; corrections needed before Phase 3 can be marked complete

## Critical Issue: Pattern<V> Generic Type Not Respected

### The Problem

The current WASM implementation **fundamentally misunderstands** the Pattern<V> generic type model:

**Current (Wrong) Implementation**:
```rust
#[wasm_bindgen]
pub struct WasmPattern {
    inner: Pattern<Subject>,  // ❌ HARDCODED to Subject only
}
```

This hardcodes Pattern to only work with Subject values, which violates the core generic Pattern<V> design.

### What Rust Does (Correct)

```rust
pub struct Pattern<V> {  // ✅ Generic over ANY value type V
    pub value: V,
    pub elements: Vec<Pattern<V>>,
}

pub fn point(value: V) -> Self {  // ✅ Accepts ANY V
    Pattern { value, elements: vec![] }
}
```

### What Python Does (Correct)

```python
fn point(py: Python, value: &Bound<'_, PyAny>) -> PyResult<Self> {
    // ✅ Accepts ANY Python value (PyAny)
    Ok(Self {
        value: value.clone().unbind(),
        elements: vec![],
    })
}

fn of(py: Python, value: &Bound<'_, PyAny>) -> PyResult<Self> {
    Self::point(py, value)  // ✅ Just an alias, accepts any value
}
```

Key observations:
- Python accepts `PyAny` - **literally any Python object**
- Can be primitives, dicts, lists, Subject, **even other Patterns** (nesting)
- `of()` is just an alias for `point()` - identical behavior

### What WASM Should Do (Needs Fixing)

```rust
// Option A: Store JsValue directly (like Python stores PyAny)
#[wasm_bindgen]
pub struct WasmPattern {
    inner: Pattern<JsValue>,  // ✅ Generic over JsValue
}

#[wasm_bindgen]
impl WasmPattern {
    pub fn point(value: JsValue) -> WasmPattern {  // ✅ Accept ANY JsValue
        WasmPattern {
            inner: Pattern::point(value),
        }
    }

    pub fn of(value: JsValue) -> WasmPattern {  // ✅ Alias for point
        Self::point(value)
    }
}
```

## Specific Issues in Current Implementation

### Issue 1: `point()` only accepts WasmSubject

**Current**:
```rust
pub fn point(value: WasmSubject) -> WasmPattern  // ❌ Restricts to WasmSubject only
```

**Should be**:
```rust
pub fn point(value: JsValue) -> WasmPattern  // ✅ Accept any JS value
```

**Impact**: 
- Users cannot create `Pattern.point(42)` or `Pattern.point("hello")`
- Users cannot create `Pattern.point(otherPattern)` for nesting
- Breaks parity with Python which accepts `PyAny`

### Issue 2: `of()` is NOT an alias for `point()`

**Current**:
```rust
pub fn of(value: JsValue) -> Result<WasmPattern, JsValue> {
    // ❌ Different implementation: only accepts primitives
    // ❌ Converts primitives to minimal Subjects (data loss)
    // ❌ Different signature (Result vs direct return)
    if let Some(s) = value.as_string() {
        // Creates minimal Subject...
    } else if let Some(n) = value.as_f64() {
        // ...
    }
}
```

**Should be**:
```rust
pub fn of(value: JsValue) -> WasmPattern {
    Self::point(value)  // ✅ Just delegate to point
}
```

**Impact**:
- Breaks spec: "Pattern.of(value: JsValue) -> Pattern — alias for point"
- Breaks parity with Python where `of()` literally just calls `point()`
- API confusion: users don't know which to use
- Type restriction: only accepts primitives vs point accepts WasmSubject

### Issue 3: Value conversion forces Subject type

**Current design issue**: The implementation assumes all Pattern values must be Subject. This shows up in:
- `of()` converting primitives to Subject
- No way to store raw JsValue in patterns
- Incompatible with Python's "store any PyAny" approach

### Issue 4: Missing support for Pattern nesting

**Current**: Cannot do `Pattern.point(anotherPattern)` because `point()` only accepts `WasmSubject`

**Python allows**:
```python
# Python allows Pattern<Pattern<T>> nesting
p1 = Pattern.point("atom")
p2 = Pattern.point(p1)  # Pattern<Pattern<str>>
```

**WASM should allow**:
```javascript
const p1 = Pattern.point("atom");
const p2 = Pattern.point(p1);  // Should work - Pattern<Pattern<V>>
```

### Issue 5: `pattern()` constructor is incomplete

**Current**:
```rust
pub fn pattern(value: WasmSubject) -> WasmPattern {
    // ❌ Only creates empty pattern, requires addElement() calls
    WasmPattern {
        inner: Pattern::pattern(value.into_inner(), vec![]),
    }
}
```

**Contract says**:
```
Pattern.pattern(value: JsValue, elements: Pattern[]) -> Pattern
```

**Should accept elements array** like Python:
```rust
pub fn pattern(value: JsValue, elements: /* array of patterns */) -> WasmPattern
```

### Issue 6: `fromValues()` is missing

**Contract says**:
```
Pattern.fromValues(values: JsValue[]) -> Pattern[]
```

**Current**: Has `fromSubjects()` instead, which:
- ❌ Wrong signature: returns single Pattern, not array of Patterns
- ❌ Only works with string children (not any values)
- ❌ Wrong semantics: creates nested pattern, not array of atomic patterns

**Should be**:
```rust
pub fn from_values(values: &JsValue) -> Result<Vec<WasmPattern>, JsValue> {
    // Convert array of JsValue to array of Pattern.point(value)
    // Returns Vec<WasmPattern>, not a single nested pattern
}
```

### Issue 7: `elements` accessor is missing

**Contract says**:
```
pattern.elements — array of Pattern
```

**Current**: Has `getElement(index)` but not `elements` property

**Should have**:
```rust
#[wasm_bindgen(getter)]
pub fn elements(&self) -> /* JS array of WasmPattern */
```

This is a getter, not a method. Users should access `pattern.elements`, not `pattern.getElement(i)` in a loop.

## Impact on Spec Compliance

### FR-001: Pattern constructors
- ❌ **FAIL**: `point()` doesn't accept any value (only WasmSubject)
- ❌ **FAIL**: `of()` is not an alias (different behavior)
- ❌ **FAIL**: `pattern()` doesn't accept elements array
- ❌ **FAIL**: `fromValues()` has wrong signature and semantics

### FR-003: Pattern accessors
- ⚠️ **PARTIAL**: Has `value` and `length` but missing `elements` array

### FR-014: Structure preservation
- ❌ **FAIL**: Cannot preserve arbitrary value types, forces Subject

### Parity with Python
- ❌ **CRITICAL FAIL**: Python accepts `PyAny` (any value), WASM restricts to Subject/primitives

## Root Cause Analysis

The fundamental misunderstanding was:
1. **Saw** `Pattern<Subject>` in examples
2. **Assumed** Pattern can only hold Subject values
3. **Missed** that Pattern<V> is generic over **any** V
4. **Missed** that Python stores PyAny directly (no type restriction)
5. **Invented** a "convenience" conversion from primitives to Subject (not in spec)

The correct understanding:
- Pattern<V> is **fully generic**
- WASM equivalent of "any value" is `JsValue` (like Python's `PyAny`)
- `WasmPattern` should wrap `Pattern<JsValue>`, not `Pattern<Subject>`
- Subject is just **one possible value type** for patterns, not the only one
- `point()` and `of()` must accept **any** JsValue and store it directly

## Required Corrections

### High Priority (Blocking Phase 3 completion)

1. **Change WasmPattern to store JsValue**:
   ```rust
   pub struct WasmPattern {
       inner: Pattern<JsValue>,  // Not Pattern<Subject>
   }
   ```

2. **Fix `point()` to accept any JsValue**:
   ```rust
   pub fn point(value: JsValue) -> WasmPattern
   ```

3. **Fix `of()` to be a true alias**:
   ```rust
   pub fn of(value: JsValue) -> WasmPattern {
       Self::point(value)
   }
   ```

4. **Fix `pattern()` to accept elements**:
   ```rust
   pub fn pattern(value: JsValue, elements: &JsValue) -> Result<WasmPattern, JsValue>
   // Parse elements as array of WasmPattern
   ```

5. **Implement `fromValues()` correctly**:
   ```rust
   pub fn from_values(values: &JsValue) -> Result<Vec<WasmPattern>, JsValue>
   // Map each value to Pattern.point(value)
   ```

6. **Add `elements` getter**:
   ```rust
   #[wasm_bindgen(getter)]
   pub fn elements(&self) -> /* JS array */
   ```

### Medium Priority (Can defer to Phase 4)

7. Remove `addElement()` and `fromSubjects()` - these are not in the spec
8. Consider how Subject fits: it's a **value type**, not the only value type
9. Add WasmSubject ↔ JsValue conversions so Subject can be **one of many** value types

## Design Pattern to Follow

**Python's approach (correct model)**:
- Store PyAny (any Python object) in Pattern
- PySubject is a specific Python class you can store
- User decides what value type to use: `Pattern.point(subject)`, `Pattern.point(42)`, `Pattern.point(pattern)`

**WASM should mirror this**:
- Store JsValue (any JavaScript value) in Pattern
- WasmSubject is a specific WASM class you can store
- User decides: `Pattern.point(subject)`, `Pattern.point(42)`, `Pattern.point(pattern)`

## Next Steps

1. Review and approve this analysis
2. Update plan.md to be explicit about Pattern<JsValue> not Pattern<Subject>
3. Revise Phase 3 implementation with corrections
4. Add explicit tests for:
   - Pattern.point(primitive)
   - Pattern.point(subject)
   - Pattern.point(pattern) - nesting
   - Pattern.of === Pattern.point
   - Pattern.fromValues returns array of atomic patterns
