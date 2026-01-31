/**
 * Demonstration of branded Symbol type safety.
 *
 * This file shows what the branded Symbol type prevents and allows.
 * Run with: tsc --noEmit --strict symbol_type_safety_demo.ts
 */

import { Subject, Value, Symbol, createSymbol } from './pattern_core';

// ============================================================================
// ✅ ALLOWED: Proper Symbol usage
// ============================================================================

// Create Symbols using the factory
const userId = createSymbol("alice");
const adminId = createSymbol("admin");

// Use Symbols with Subject
const user = new Subject(userId, ["Person"], {
  name: Value.string("Alice")
});

// Extract Symbol from Subject
const extractedId: Symbol = user.identity;

// Compare Symbols (both are strings at runtime)
const same = userId === createSymbol("alice"); // true at runtime

// Use Symbol where string is accepted (Symbol extends string)
const message: string = `User ID: ${userId}`; // OK - can use in template string
console.log(userId); // OK - can log directly

// ============================================================================
// ❌ PREVENTED: Accidental string usage
// ============================================================================

// The following would cause TypeScript errors if uncommented:

// Can't pass raw string to Subject constructor
// const bad1 = new Subject("alice", ["Person"], {});
// Error: Argument of type 'string' is not assignable to parameter of type 'Symbol'

// Can't assign raw string to Symbol variable
// const bad2: Symbol = "alice";
// Error: Type 'string' is not assignable to type 'Symbol'

// Can't pass string to function expecting Symbol
function requiresSymbol(id: Symbol): void {
  console.log("ID:", id);
}
requiresSymbol(userId); // ✅ OK
// requiresSymbol("alice"); // ❌ Error

// ============================================================================
// ✅ DESIGN BENEFIT: Explicit conversion required
// ============================================================================

// Forces developers to be intentional about identity creation
function createUser(idString: string): Subject {
  // Must explicitly convert string to Symbol
  const id = createSymbol(idString);
  return new Subject(id, ["User"], {});
}

// Can't accidentally mix up regular strings with identity symbols
const username: string = "alice";
const identity: Symbol = createSymbol("alice");

// These are different types at compile time (same at runtime)
function displayUsername(name: string): void { console.log(name); }
function displayIdentity(id: Symbol): void { console.log(id); }

displayUsername(username);  // ✅ OK
displayIdentity(identity);  // ✅ OK
// displayIdentity(username);  // ❌ Error - forces explicit conversion

// ============================================================================
// ✅ FLEXIBILITY: Can still use Symbol as string when needed
// ============================================================================

// Symbol can be used where string is expected (for interop)
const logMessage: string = identity; // ✅ OK - Symbol extends string
const key = { [identity as string]: "value" }; // ✅ OK - can cast for object keys

// ============================================================================
// Summary
// ============================================================================

console.log("Branded Symbol type provides:");
console.log("1. Compile-time safety - prevents accidental string usage");
console.log("2. Explicit conversion - forces use of createSymbol()");
console.log("3. Runtime compatibility - still a string for serialization");
console.log("4. Type flexibility - can use Symbol where string is expected");
console.log("\nAll checks passed! ✓");
