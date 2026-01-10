# Canonical JSON Alignment: gram-rs vs gram-hs (Updated)

**Date**: January 10, 2026  
**Status**: ✅ **MOSTLY ALIGNED** (after gram-hs updates)  
**Priority**: P1 - Minor fixes needed

---

## 🎉 Great News!

gram-hs has been updated (commit `3b3bc9b`) to align with gram-rs naming:
- ✅ **Pattern field**: `value` → `subject` (now matches!)
- ✅ **Subject field**: `symbol` → `identity` (now matches!)

This means gram-rs and gram-hs are now **much more aligned**!

---

## 📊 Updated Comparison

### Pattern Structure ✅

| Field | gram-hs (Current) | gram-rs (Current) | Status |
|-------|-------------------|-------------------|--------|
| Pattern value | `subject` | `subject` | ✅ **MATCH** |
| Elements | `elements` | `elements` | ✅ Match |

### Subject Structure ✅

| Field | gram-hs (Current) | gram-rs (Current) | Status |
|-------|-------------------|-------------------|--------|
| Identity | `identity` | `identity` | ✅ **MATCH** |
| Labels | `labels` | `labels` | ✅ Match |
| Properties | `properties` | `properties` | ✅ Match |

### Value Type Discriminators ⚠️

| Type | gram-hs (Current) | gram-rs (Current) | Status |
|------|-------------------|-------------------|--------|
| Symbol | `{"type": "symbol", "value": "..."}` | `{"type": "Symbol", "value": "..."}` | ❌ **MISMATCH** (case) |
| Tagged String | `{"type": "tagged", "tag": "...", "content": "..."}` | `{"type": "Tagged", "tag": "...", "content": "..."}` | ❌ **MISMATCH** (case) |
| Range | `{"type": "range", "lower": n, "upper": n}` | `{"type": "Range", "lower": n, "upper": n}` | ❌ **MISMATCH** (case) |
| Measurement | `{"type": "measurement", "unit": "...", "value": n}` | `{"type": "Measurement", "unit": "...", "value": n}` | ❌ **MISMATCH** (case) |

### Simple Types ⚠️

| Type | gram-hs | gram-rs | Status |
|------|---------|---------|--------|
| Integer | Native JSON `number` | Tagged `{"type": "Integer", "value": n}` | ⚠️ **DIFFERENT APPROACH** |
| Decimal | Native JSON `number` | Tagged `{"type": "Decimal", "value": n}` | ⚠️ **DIFFERENT APPROACH** |
| Boolean | Native JSON `boolean` | Native JSON `boolean` | ✅ Match |
| String | Native JSON `string` | Native JSON `string` | ✅ Match |
| Array | Native JSON `array` | Native JSON `array` | ✅ Match |
| Map | Native JSON `object` | Native JSON `object` | ✅ Match |

---

## 🔍 Remaining Differences

### 1. Type Discriminator Case: Lowercase vs Capitalized

**gram-hs** (from JSON.hs line 79-84):
```haskell
valueToJSON (SubjectValue.VSymbol s) = object ["type" .= ("symbol" :: T.Text), "value" .= s]
valueToJSON (SubjectValue.VTaggedString tag content) = object ["type" .= ("tagged" :: T.Text), ...]
valueToJSON (SubjectValue.VRange rv) = rangeValueToJSON rv  -- uses "range"
valueToJSON (SubjectValue.VMeasurement unit val) = object ["type" .= ("measurement" :: T.Text), ...]
```

**gram-rs (current)**:
```rust
Value::VSymbol(sym) => serde_json::json!({
    "type": "Symbol",  // Should be "symbol"
    "value": sym.clone()
}),
```

**Impact**: Medium - Type discrimination will fail if consumers check for lowercase.

**Fix Required**: Change all type discriminators to lowercase:
- `"Symbol"` → `"symbol"`
- `"Tagged"` → `"tagged"`
- `"Range"` → `"range"`
- `"Measurement"` → `"measurement"`

---

### 2. Integer/Decimal Serialization: Native vs Tagged

**gram-hs** (from JSON.hs line 75-76):
```haskell
valueToJSON (SubjectValue.VInteger i) = toJSON i  -- Native JSON number
valueToJSON (SubjectValue.VDecimal d) = toJSON d   -- Native JSON number
```

**gram-rs (current)**:
```rust
Value::VInteger(i) => serde_json::json!({
    "type": "Integer",  // Should be native JSON
    "value": i
}),
Value::VDecimal(d) => serde_json::json!({
    "type": "Decimal",  // Should be native JSON
    "value": d
}),
```

**Impact**: High - This is a fundamental difference. gram-hs uses native JSON numbers, gram-rs tags them.

**Fix Required**: Remove tagging for integers and decimals, use native JSON numbers.

**Note**: This aligns with our original "mixed approach" design decision, but gram-hs uses pure native for numbers. We should align with gram-hs.

---

## ✅ What's Already Aligned

1. ✅ **Pattern field**: `subject` (both use it)
2. ✅ **Subject identity**: `identity` (both use it)
3. ✅ **Tagged String Structure**: `tag` and `content` fields match
4. ✅ **Range Structure**: `lower` and `upper` fields match
5. ✅ **Measurement Structure**: `unit` and `value` fields match
6. ✅ **Array/Map Recursion**: Both use native JSON
7. ✅ **Boolean/String**: Both use native JSON types

---

## 📋 Required Changes (Updated)

### High Priority

1. **Change Integer/Decimal to native JSON**
   - Remove tagging for integers and decimals
   - Use native JSON numbers
   - Update value conversion logic in `value_to_json()`

### Medium Priority

2. **Lowercase type discriminators**
   - `"Symbol"` → `"symbol"`
   - `"Tagged"` → `"tagged"`
   - `"Range"` → `"range"`
   - `"Measurement"` → `"measurement"`

### Low Priority

3. **Update examples** to show canonical format
4. **Update READMEs** to reference canonical format
5. **Add validation** against gram-hs JSON schema

---

## 🧪 Testing Strategy

After changes:

1. **Round-trip test**: gram-rs JSON → gram-hs parser → gram-rs parser
2. **Schema validation**: Validate gram-rs JSON against gram-hs schema
3. **Example comparison**: Compare outputs for same gram input

---

## 📚 References

- **gram-hs JSON Implementation**: `../gram-hs/libs/gram/src/Gram/JSON.hs`
- **gram-hs Commit**: `3b3bc9b` - "fix(json): align field names with semantic correctness"
- **gram-hs JSON Schema**: `../gram-hs/specs/029-canonical-json-pattern/contracts/json-schema.json`
- **gram-hs Spec**: `../gram-hs/specs/029-canonical-json-pattern/spec.md`

---

## 🎯 Recommendation

**Action**: Make the remaining 2 changes to fully align with gram-hs canonical format.

**Rationale**:
- Most alignment is done (field names match!)
- Only 2 remaining differences (type case + number serialization)
- Ensures full interoperability

**Estimated Effort**: 1-2 hours
- Value serialization changes: 45 minutes
- Type discriminator case: 15 minutes
- Testing and validation: 30 minutes

---

**Status**: ✅ **MOSTLY ALIGNED** (2 minor fixes needed)  
**Priority**: P1  
**Blocks**: Full interoperability
