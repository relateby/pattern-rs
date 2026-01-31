/**
 * Sample TypeScript consumer to test pattern_core.d.ts type definitions.
 *
 * This file exercises the full public API surface to verify that:
 * - Pattern<V> generic types work correctly
 * - Type inference is correct for all operations
 * - Either-like return types are properly typed
 * - Subject, Value, and other types are accessible
 *
 * Run with: tsc --noEmit consumer_sample.ts
 */

import {
  Pattern,
  Subject,
  Value,
  ValidationRules,
  StructureAnalysis,
  ValidationError,
  Either,
  Left,
  Right,
  Symbol,
  createSymbol,
} from "./pattern_core";

// ============================================================================
// Test 1: Basic Pattern Construction (User Story 1)
// ============================================================================

// Atomic patterns with primitives
const p1: Pattern<string> = Pattern.point("hello");
const p2: Pattern<number> = Pattern.point(42);
const p3: Pattern<boolean> = Pattern.point(true);

// Using 'of' alias
const p4: Pattern<string> = Pattern.of("world");

// Accessing value and elements
const value1: string = p1.value;
const elements1: Pattern<string>[] = p1.elements;

// Pattern with Subject value
const subject = new Subject(createSymbol("alice"), ["Person", "User"], {
  name: Value.string("Alice"),
  age: Value.int(30),
});

const subjectPattern: Pattern<Subject> = Pattern.point(subject);
const subjectValue: Subject = subjectPattern.value;
const identitySymbol: Symbol = subjectValue.identity;
console.log(subjectValue.identity); // Symbol("alice") - typed as Symbol, runtime string
console.log(subjectValue.labels); // ["Person", "User"]

// Pattern with nested structure
const parent = Pattern.pattern("parent");
parent.addElement(Pattern.of("child1"));
parent.addElement(Pattern.of("child2"));

const child: Pattern<string> | undefined = parent.getElement(0);

// fromValues
const patterns: Pattern<number>[] = Pattern.fromValues([1, 2, 3]);
const firstPattern: Pattern<number> = patterns[0];
const firstValue: number = firstPattern.value;

// ============================================================================
// Test 2: Inspection Methods
// ============================================================================

const len: number = parent.length(); // 2
const sz: number = parent.size(); // 3
const d: number = parent.depth(); // 1
const atomic: boolean = p1.isAtomic(); // true
const vals: string[] = parent.values(); // ["parent", "child1", "child2"]

// ============================================================================
// Test 3: Query Methods
// ============================================================================

// anyValue
const hasWorld: boolean = parent.anyValue((v) => v === "world");

// allValues
const allStrings: boolean = parent.allValues((v) => typeof v === "string");

// filter
const leaves: Pattern<string>[] = parent.filter((p) => p.isAtomic());

// findFirst
const found: Pattern<string> | null = parent.findFirst(
  (p) => p.value === "child1",
);

// matches
const p5 = Pattern.pattern("test");
const p6 = Pattern.pattern("test");
const match: boolean = p5.matches(p6);

// contains
const containsChild: boolean = parent.contains(Pattern.of("child1"));

// ============================================================================
// Test 4: Transformation Methods
// ============================================================================

// map - transforms values
const upper: Pattern<string> = parent.map((v) =>
  typeof v === "string" ? v.toUpperCase() : v,
);
const upperValue: string = upper.value; // "PARENT"

// map - transforms to different type
const lengths: Pattern<number> = parent.map((v) =>
  typeof v === "string" ? v.length : 0,
);
const lengthValue: number = lengths.value;

// fold
const numPattern = Pattern.pattern(10);
numPattern.addElement(Pattern.of(20));
numPattern.addElement(Pattern.of(30));
const sum: number = numPattern.fold(0, (acc, v) => acc + v); // 60

// para - paramorphism
const count: number = parent.para((p, childResults: number[]) => {
  return 1 + childResults.reduce((sum, r) => sum + r, 0);
});

// ============================================================================
// Test 5: Combination Methods
// ============================================================================

const p7 = Pattern.pattern("hello");
p7.addElement(Pattern.of("a"));
const p8 = Pattern.pattern(" world");
p8.addElement(Pattern.of("b"));

const combined: Pattern<string> = p7.combine(p8, (v1, v2) => v1 + v2);
const combinedValue: string = combined.value; // "hello world"

// ============================================================================
// Test 6: Comonad Methods
// ============================================================================

// extract
const extracted: string = p1.extract(); // "hello"

// extend
const sizes: Pattern<number> = parent.extend((subpattern) => subpattern.size());
const sizeValue: number = sizes.extract();

// depthAt
const depths: Pattern<number> = parent.depthAt();
const depthValue: number = depths.extract();

// sizeAt
const sizesAt: Pattern<number> = parent.sizeAt();
const sizeAtValue: number = sizesAt.extract();

// indicesAt
const paths: Pattern<number[]> = parent.indicesAt();
const pathValue: number[] = paths.extract();

// ============================================================================
// Test 7: Validation (Either-like return values)
// ============================================================================

const rules = new ValidationRules(10, 100);
const result: Either<ValidationError, void> = parent.validate(rules);

// Pattern matching on Either
if (result._tag === "Left") {
  const error: ValidationError = result.left;
  console.error("Validation failed:", error.message);
  console.error("Rule violated:", error.ruleViolated);
  console.error("Location:", error.location);
} else {
  // result._tag === 'Right'
  const success: void = result.right;
  console.log("Pattern is valid");
}

// Type narrowing works
const processResult = (r: Either<ValidationError, void>) => {
  if (r._tag === "Left") {
    // TypeScript knows r.left is ValidationError
    const msg: string = r.left.message;
    return msg;
  } else {
    // TypeScript knows r.right is void
    return "Success";
  }
};

// ============================================================================
// Test 8: Structure Analysis
// ============================================================================

const analysis: StructureAnalysis = parent.analyzeStructure();
const summary: string = analysis.summary;
const depthDist: number[] = analysis.depthDistribution;
const elemCounts: number[] = analysis.elementCounts;
const nestingPatterns: string[] = analysis.nestingPatterns;

// ============================================================================
// Test 9: Value Factories and Types
// ============================================================================

const vString: Value = Value.string("hello");
const vInt: Value = Value.int(42);
const vDecimal: Value = Value.decimal(3.14);
const vBoolean: Value = Value.boolean(true);
const vSymbol: Value = Value.symbol("sym");
const vArray: Value = Value.array([Value.int(1), Value.int(2)]);
const vMap: Value = Value.map({ key: Value.string("value") });
const vRange: Value = Value.range(0, 10);
const vMeasurement: Value = Value.measurement(5.5, "kg");

// ============================================================================
// Test 10: Subject Construction and Access
// ============================================================================

const subject2 = new Subject(createSymbol("bob"), ["Person"], {
  name: Value.string("Bob"),
  age: Value.int(25),
  tags: Value.array([Value.string("developer"), Value.string("rust")]),
});

const identity: Symbol = subject2.identity;
const labels: string[] = subject2.labels;
const properties: Record<string, Value> = subject2.properties;

// ============================================================================
// Test 11: Pattern<Pattern<V>> (Nested Patterns)
// ============================================================================

const innerPattern: Pattern<string> = Pattern.point("inner");
const outerPattern: Pattern<Pattern<string>> = Pattern.point(innerPattern);
const nestedValue: Pattern<string> = outerPattern.value;
const innerValue: string = nestedValue.value; // "inner"

// ============================================================================
// Test 12: Complex Workflow (User Story integration)
// ============================================================================

// Create a pattern of subjects
const alice = new Subject(createSymbol("alice"), ["Person"], {
  name: Value.string("Alice"),
});
const bob = new Subject(createSymbol("bob"), ["Person"], {
  name: Value.string("Bob"),
});

const people = Pattern.pattern(alice);
people.addElement(Pattern.of(bob));

// Transform pattern - extract identities as branded Symbols
const names: Pattern<Symbol> = people.map((subj) => {
  if (typeof subj === "object" && "identity" in subj) {
    return (subj as Subject).identity;
  }
  return createSymbol("unknown");
});

// Query pattern - compare branded Symbols
const hasBob: boolean = people.anyValue((subj) => {
  if (typeof subj === "object" && "identity" in subj) {
    // At runtime, both are strings, so === works correctly
    return (subj as Subject).identity === createSymbol("bob");
  }
  return false;
});

// Validate pattern
const validationResult = people.validate(new ValidationRules(5, 50));
if (validationResult._tag === "Right") {
  console.log("People pattern is valid");
}

// Analyze structure
const peopleAnalysis = people.analyzeStructure();
console.log("Analysis:", peopleAnalysis.summary);

// ============================================================================
// Test 13: Type Inference
// ============================================================================

// TypeScript should infer types correctly
const inferred1 = Pattern.point("test"); // Pattern<string>
const inferred2 = inferred1.map((s) => s.length); // Pattern<number>
const inferred3 = inferred2.map((n) => n > 5); // Pattern<boolean>

// Check that types flow through transformations
const checkInference = () => {
  const start: Pattern<string> = Pattern.point("hello");
  const step1: Pattern<number> = start.map((s) => s.length);
  const step2: Pattern<boolean> = step1.map((n) => n > 3);
  const final: boolean = step2.extract();
  return final;
};

console.log("Type inference test passed");
