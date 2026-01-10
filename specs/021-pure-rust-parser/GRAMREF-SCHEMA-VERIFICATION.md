# gramref Schema Verification

**Date**: January 10, 2026  
**Status**: ✅ **SCHEMA GENERATOR IS CORRECT**

---

## ✅ Verification Results

### Schema Generation Code (`Gram.Schema.JSONSchema.hs`)

**Pattern Definition** (line 107):
```haskell
"subject" .= object ["$ref" .= ("#/$defs/Subject" :: T.Text)]
```

**Subject Definition** (line 131):
```haskell
"identity" .= object
  [ "type" .= ("string" :: T.Text)
  , "description" .= ("Identity symbol for the subject" :: T.Text)
  ]
```

**Required Fields** (line 114, 149):
```haskell
-- Pattern
"required" .= (["subject", "elements"] :: [T.Text])

-- Subject  
"required" .= (["identity", "labels", "properties"] :: [T.Text])
```

### ✅ Confirmed: Schema Generator Uses Correct Field Names

The `generatePatternSchema` function in `Gram.Schema.JSONSchema.hs` correctly uses:
- ✅ `subject` (not `value`)
- ✅ `identity` (not `symbol`)

This matches:
- ✅ gram-hs JSON implementation (`Gram.JSON.hs`)
- ✅ gram-rs AST implementation (`ast.rs`)

---

## ⚠️ Static Schema File is Outdated

**File**: `specs/029-canonical-json-pattern/contracts/json-schema.json`

This static file still shows old field names:
- ❌ Uses `"value"` instead of `"subject"`
- ❌ Uses `"symbol"` instead of `"identity"`

**Status**: The static file needs to be regenerated from the schema generator.

**Action**: The static file should be updated by running:
```bash
gramref schema --format json-schema > specs/029-canonical-json-pattern/contracts/json-schema.json
```

---

## 📊 Type Discriminators Verification

**Schema Generator** (lines 51, 59, 68, 87):
```haskell
-- Symbol
"type" .= object ["const" .= ("symbol" :: T.Text)]

-- TaggedString
"type" .= object ["const" .= ("tagged" :: T.Text)]

-- Range
"type" .= object ["const" .= ("range" :: T.Text)]

-- Measurement
"type" .= object ["const" .= ("measurement" :: T.Text)]
```

✅ **Confirmed**: Schema generator uses **lowercase** type discriminators.

---

## 📊 Value Type Definitions Verification

**Schema Generator** (lines 165-179):
```haskell
-- Integer
object ["type" .= ("integer" :: T.Text)]

-- Decimal
object ["type" .= ("number" :: T.Text)]

-- Boolean
object ["type" .= ("boolean" :: T.Text)]

-- String
object ["type" .= ("string" :: T.Text)]
```

✅ **Confirmed**: Schema generator uses **native JSON types** for integers and decimals (not tagged objects).

---

## 🎯 Summary

| Component | Schema Generator | Static File | gram-rs | Status |
|-----------|------------------|-------------|---------|--------|
| Pattern field | ✅ `subject` | ❌ `value` | ✅ `subject` | Generator correct |
| Subject identity | ✅ `identity` | ❌ `symbol` | ✅ `identity` | Generator correct |
| Type discriminators | ✅ lowercase | ✅ lowercase | ❌ Capitalized | gram-rs needs fix |
| Integer/Decimal | ✅ native JSON | ✅ native JSON | ❌ Tagged | gram-rs needs fix |

---

## ✅ Conclusion

**gramref schema generator is correct!** It produces:
- ✅ `subject` and `identity` (matches gram-rs)
- ✅ Lowercase type discriminators (gram-rs needs to match)
- ✅ Native JSON for integers/decimals (gram-rs needs to match)

**Action Required**:
1. Update static schema file (regenerate from gramref)
2. Fix gram-rs to use lowercase type discriminators
3. Fix gram-rs to use native JSON for integers/decimals

---

**Status**: ✅ **SCHEMA GENERATOR VERIFIED CORRECT**  
**Next**: Fix gram-rs to match schema generator output
