# Branded Symbol Type Implementation

**Date**: 2026-01-31  
**Status**: ✅ COMPLETE  
**Feature Branch**: 027-wasm-pattern-typescript-parity

## Overview

Implemented a branded `Symbol` type in the TypeScript definitions to provide compile-time type safety for identity values while maintaining runtime string compatibility for serialization and WASM boundary crossing.

## Implementation

### Type Definition

```typescript
/**
 * Symbol represents a unique identifier with compile-time type safety.
 * 
 * This is a branded string type that provides compile-time distinction from
 * arbitrary strings while remaining a string at runtime for serialization
 * and WASM compatibility.
 */
export type Symbol = string & { readonly __brand: unique symbol };

/**
 * Create a typed Symbol from a string value.
 * 
 * This factory function provides type-safe construction of Symbol values,
 * preventing accidental use of arbitrary strings where identity is required.
 */
export function createSymbol(value: string): Symbol;
```

### Design Rationale

The branded type approach was chosen over JavaScript `symbol` primitives for several critical reasons:

1. **Serialization** ✅
   - Branded types remain strings at runtime
   - JSON serialization works without modification: `JSON.stringify({ id: createSymbol("alice") })` → `'{"id":"alice"}'`
   - No special handling needed for persistence, network transfer, or logging

2. **WASM Boundary Compatibility** ✅
   - Rust `Symbol(String)` maps directly to TypeScript `Symbol` (both strings at runtime)
   - Clean round-trip: Rust → JS → Rust maintains identity
   - No symbol registry or conversion logic needed

3. **Python Parity** ✅
   - Python bindings use strings for identity
   - Cross-language pattern compatibility maintained
   - Same logical behavior across runtimes

4. **Zero Runtime Cost** ✅
   - No wrapper objects or conversions
   - No performance overhead
   - Type safety is compile-time only

5. **Developer Experience** ✅
   - Can log/print directly: `console.log(createSymbol("alice"))` works
   - Can use in template strings: `` `User: ${userId}` ``
   - Comparison works naturally: `sym1 === sym2`

## What It Prevents

### ❌ Accidental String Usage

```typescript
// Error: Can't pass raw string to Subject constructor
const subject = new Subject("alice", ["Person"], {});
// Error: Argument of type 'string' is not assignable to parameter of type 'Symbol'

// Error: Can't assign raw string to Symbol variable
const id: Symbol = "alice";
// Error: Type 'string' is not assignable to type 'Symbol'

// Error: Can't pass string to function expecting Symbol
function requiresSymbol(id: Symbol): void { ... }
requiresSymbol("alice"); // Error!
```

### ✅ Forces Explicit Conversion

```typescript
// Must use createSymbol() for type safety
const id = createSymbol("alice");
const subject = new Subject(id, ["Person"], {});

// Functions clearly document intent
function createUser(idString: string): Subject {
  const id = createSymbol(idString); // Explicit conversion required
  return new Subject(id, ["User"], {});
}
```

## What It Allows

### ✅ Use Symbol Where String Is Expected

```typescript
const id: Symbol = createSymbol("alice");

// Can assign Symbol to string (Symbol extends string)
const str: string = id; // OK - useful for logging/display

// Can use in template strings
const msg = `User ID: ${id}`; // OK

// Can compare directly
const same = id === createSymbol("alice"); // OK - both strings at runtime
```

### ✅ Runtime Compatibility

```typescript
const id = createSymbol("alice");

// Serializes as plain string
JSON.stringify({ id }); // → '{"id":"alice"}'

// Can use as object key (with cast)
const map: Record<string, number> = {};
map[id as string] = 42;
```

## Updated API

### Subject Class

```typescript
export class Subject {
  constructor(
    identity: Symbol,      // Now requires branded Symbol (not string)
    labels: string[],
    properties: Record<string, Value>
  );
  
  readonly identity: Symbol; // Returns branded Symbol (not string)
  // ...
}
```

### Usage Pattern

```typescript
// Old (would cause type error now):
// const subject = new Subject("alice", [], {});

// New (type-safe):
const subject = new Subject(createSymbol("alice"), [], {});
const id: Symbol = subject.identity;

// Comparison:
if (subject.identity === createSymbol("alice")) {
  console.log("Found Alice!"); // Works - both strings at runtime
}
```

## Files Modified

1. **crates/pattern-core/typescript/pattern_core.d.ts**
   - Added branded `Symbol` type definition
   - Added `createSymbol()` factory function
   - Updated `Subject` class to use `Symbol` instead of `string`
   - Added comprehensive JSDoc documentation

2. **crates/pattern-core/typescript/consumer_sample.ts**
   - Updated all `new Subject()` calls to use `createSymbol()`
   - Updated variable type annotations from `string` to `Symbol`
   - Updated comparison logic to use branded symbols

3. **crates/pattern-core/typescript/symbol_type_safety_test.ts** (new)
   - Test file demonstrating type safety with `@ts-expect-error` directives
   - Shows what errors the branded type prevents

4. **crates/pattern-core/typescript/symbol_type_safety_demo.ts** (new)
   - Demonstration file showing allowed and prevented usage patterns
   - Documents design benefits and flexibility

## Verification

All TypeScript verification passed:

```bash
# Standard type checking
cd crates/pattern-core/typescript
tsc --noEmit consumer_sample.ts          # ✅ 0 errors
tsc --noEmit symbol_type_safety_test.ts  # ✅ 0 errors
tsc --noEmit symbol_type_safety_demo.ts  # ✅ 0 errors

# Strict mode verification
tsc --noEmit --strict consumer_sample.ts          # ✅ 0 errors
tsc --noEmit --strict symbol_type_safety_demo.ts  # ✅ 0 errors
```

## Benefits Summary

| Aspect | Benefit |
|--------|---------|
| **Type Safety** | Prevents accidental string/identity confusion at compile time |
| **Intent Clarity** | `createSymbol()` makes identity creation explicit |
| **Serialization** | Works with JSON.stringify() - no special handling |
| **WASM Boundary** | Clean Rust String ↔ JS string conversion |
| **Python Parity** | Maintains cross-language compatibility |
| **Performance** | Zero runtime overhead - type safety is compile-time only |
| **Developer UX** | Can log, print, compare naturally |
| **Flexibility** | Can use Symbol where string is expected (extends string) |

## Design Trade-offs

### Chosen: Branded Types

**Pros:**
- ✅ Zero runtime cost
- ✅ Perfect serialization
- ✅ WASM boundary compatible
- ✅ Cross-language compatibility

**Cons:**
- ⚠️ Symbol can be assigned to string (less strict than full isolation)
- ⚠️ Requires explicit `createSymbol()` call

### Alternative: JS `symbol` Primitive (Not Chosen)

**Pros:**
- ✅ True uniqueness guarantees
- ✅ Can't accidentally pass arbitrary values

**Cons:**
- ❌ Doesn't serialize to JSON
- ❌ WASM boundary complexity (need symbol registry)
- ❌ Breaks Python parity
- ❌ Comparison by reference only (not by value)

### Alternative: Plain `string` (Previous)

**Pros:**
- ✅ Simple and flexible

**Cons:**
- ❌ No type safety
- ❌ Easy to mix up identity with arbitrary strings
- ❌ Intent not clear from types

## Backward Compatibility

Since this is a new feature branch (027) that hasn't been published, there are **no backward compatibility concerns**. The branded type is the initial public API design.

## Future Considerations

1. **Helper Functions**: Could add utility functions like:
   ```typescript
   export function symbolEquals(a: Symbol, b: Symbol): boolean;
   export function symbolToString(s: Symbol): string; // Explicit cast
   ```

2. **Validation**: Could add runtime validation in `createSymbol()`:
   ```typescript
   export function createSymbol(value: string): Symbol {
     if (!value || typeof value !== 'string') {
       throw new Error('Symbol value must be a non-empty string');
     }
     return value as Symbol;
   }
   ```

3. **Symbol Namespaces**: Could support namespaced identities:
   ```typescript
   export function createSymbol(namespace: string, value: string): Symbol;
   // createSymbol("user", "alice") → "user:alice"
   ```

## Conclusion

The branded `Symbol` type provides excellent compile-time type safety while maintaining full runtime compatibility with strings. This design:

- Prevents accidental misuse of arbitrary strings as identities
- Requires explicit `createSymbol()` calls, making intent clear
- Works seamlessly with JSON serialization and WASM boundaries
- Maintains parity with Python bindings
- Has zero runtime performance cost

The implementation successfully balances type safety with pragmatic runtime concerns for a WASM boundary API.
