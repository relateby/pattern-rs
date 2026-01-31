/**
 * Test file demonstrating branded Symbol type safety.
 *
 * This file contains intentional type errors to verify that the branded
 * Symbol type prevents accidental use of arbitrary strings.
 *
 * To test: Run `tsc --noEmit symbol_type_safety_test.ts`
 * Expected: Type errors on lines marked with "// Error:"
 */

import { Subject, Value, Symbol, createSymbol } from "./pattern_core";

// ============================================================================
// Test 1: createSymbol() is required
// ============================================================================

// ✅ Correct: Using createSymbol()
const validSymbol: Symbol = createSymbol("alice");
const validSubject = new Subject(validSymbol, ["Person"], {});

// ❌ Error: Can't assign string to Symbol without createSymbol()
// @ts-expect-error - Testing that this produces a type error
const invalidSymbol: Symbol = "alice"; // Error: Type 'string' is not assignable to type 'Symbol'

// ❌ Error: Can't pass string directly to Subject constructor
// @ts-expect-error - Testing that this produces a type error
const invalidSubject = new Subject("alice", ["Person"], {}); // Error: Argument of type 'string' is not assignable to parameter of type 'Symbol'

// ============================================================================
// Test 2: Symbol identity is distinct from string
// ============================================================================

const sym: Symbol = createSymbol("test");
const str: string = "test";

// ✅ Note: Symbol can be assigned to string (it IS a string at runtime)
// This is intentional - allows using Symbol where string is expected
const okAssignment1: string = sym; // OK - Symbol extends string

// ❌ Error: Can't assign string to Symbol variable without createSymbol()
// @ts-expect-error - Testing that this produces a type error
const wrongAssignment2: Symbol = str; // Error: Type 'string' is not assignable to type 'Symbol'

// ============================================================================
// Test 3: Functions requiring Symbol are type-safe
// ============================================================================

function processIdentity(id: Symbol): void {
  console.log("Processing:", id);
}

// ✅ Correct: Passing Symbol
processIdentity(createSymbol("valid"));

// ❌ Error: Can't pass string to function expecting Symbol
// @ts-expect-error - Testing that this produces a type error
processIdentity("invalid"); // Error: Argument of type 'string' is not assignable to parameter of type 'Symbol'

// ============================================================================
// Test 4: Subject.identity returns Symbol, not string
// ============================================================================

const subject = new Subject(createSymbol("bob"), ["Person"], {});

// ✅ Correct: Assigning to Symbol variable
const id1: Symbol = subject.identity;

// ✅ Note: Can assign Symbol to string (it IS a string at runtime)
// This allows interop with string-expecting APIs
const id2: string = subject.identity; // OK - Symbol extends string, useful for logging/display

// ============================================================================
// Test 5: Comparison still works (runtime compatibility)
// ============================================================================

// ✅ Correct: Can compare Symbols with === (both are strings at runtime)
const sym1 = createSymbol("alice");
const sym2 = createSymbol("alice");
const areEqual: boolean = sym1 === sym2; // true at runtime (both are "alice")

// ✅ Correct: Can use as object key (it's a string at runtime)
const map: Record<string, number> = {};
map[sym1 as string] = 42; // Need cast to use as index, showing type safety

// ============================================================================
// Test 6: Type narrowing works
// ============================================================================

function handleValue(value: string | Symbol): void {
  if (typeof value === "string") {
    // TypeScript knows this could be either (branded types are structural)
    // But the brand prevents accidental assignment
  }
}

handleValue(createSymbol("test"));
handleValue("test");

// ============================================================================
// Summary
// ============================================================================

console.log("This file demonstrates branded Symbol type safety.");
console.log("Lines with @ts-expect-error should produce type errors.");
console.log("Run: tsc --noEmit symbol_type_safety_test.ts");
console.log("Expected: Multiple type errors on marked lines");
