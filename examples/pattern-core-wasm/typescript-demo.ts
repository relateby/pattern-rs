// pattern-core WASM TypeScript Example with Full Type Safety
//
// This demonstrates TypeScript's generic type inference with Pattern<V>
//
// Prerequisites:
//   1. Build WASM package: cd crates/pattern-core && wasm-pack build --target web --features wasm
//   2. Ensure TypeScript definitions are in pkg/
//   3. Type-check: tsc --noEmit typescript-demo.ts

import init, {
  Pattern,
  Subject,
  Value,
  ValidationRules,
} from "./pkg/pattern_core.js";
import type { Either } from "./pkg/pattern_core.js";

async function typeScriptDemo() {
  await init();

  // ===== Type Inference Demo =====

  // Pattern<string> - type inferred from literal
  const stringPattern: Pattern<string> = Pattern.point("hello");

  // Pattern<number> - type inferred from literal
  const numberPattern: Pattern<number> = Pattern.point(42);

  // Pattern<Subject> - explicitly typed
  const subject = Subject.new("n", ["Person"], {
    name: Value.string("Alice"),
  });
  const subjectPattern: Pattern<Subject> = Pattern.point(subject);

  // ===== Generic Type Flow Through Operations =====

  // Map: Pattern<number> -> Pattern<string>
  const numbers = Pattern.pattern(1, [Pattern.point(2), Pattern.point(3)]);
  const asStrings: Pattern<string> = numbers.map((n) => `Number: ${n}`);

  // Map: Pattern<string> -> Pattern<number>
  const lengths: Pattern<number> = stringPattern.map((s) => s.length);

  // Filter returns array of patterns
  const filtered: Pattern<number>[] = numbers.filter((p) => p.value > 1);

  // Fold produces output type
  const sum: number = numbers.fold(0, (acc, val) => acc + val);
  const concatenated: string = asStrings.fold("", (acc, val) => acc + val);

  // Para (paramorphism) - type inference from callback
  const paraSum: number = numbers.para((value, childResults) => {
    return value + childResults.reduce((a, b) => a + b, 0);
  });

  // ===== Nested Patterns: Pattern<Pattern<V>> =====

  const nested: Pattern<Pattern<number>> = Pattern.pattern(Pattern.point(1), [
    Pattern.point(Pattern.point(2)),
  ]);

  // Extract unwraps one level: Pattern<Pattern<V>> -> Pattern<V>
  const unwrapped: Pattern<number> = nested.extract();

  // ===== Comonad Operations with Type Safety =====

  // depthAt returns Pattern<number>
  const depths: Pattern<number> = numbers.depthAt();

  // sizeAt returns Pattern<number>
  const sizes: Pattern<number> = numbers.sizeAt();

  // indicesAt returns Pattern<number[]>
  const indices: Pattern<number[]> = numbers.indicesAt();

  // extend: replace each subpattern with function result
  const extended: Pattern<number> = numbers.extend((p) => p.size());

  // ===== Either-like Validation =====

  const rules = ValidationRules.new({ maxDepth: 2, maxElements: 10 });

  // validate returns Either<ValidationError, void>
  const validResult: Either<ValidationError, void> = numbers.validate(rules);

  // Pattern matching on Either
  if (validResult._tag === "Right") {
    console.log("Validation passed");
  } else {
    // TypeScript knows validResult.left exists here
    console.error("Validation failed:", validResult.left.message);
    console.error("Rule violated:", validResult.left.ruleViolated);
  }

  // ===== effect-ts Integration (if installed) =====

  // The Either shape is directly compatible with effect-ts:
  /*
    import { Either } from 'effect';

    const result = pattern.validate(rules);

    Either.match(result, {
        onLeft: (err) => {
            console.error(`Validation failed: ${err.message}`);
            if (err.ruleViolated) {
                console.error(`Rule: ${err.ruleViolated}`);
            }
        },
        onRight: () => console.log('Pattern is valid')
    });

    // Or use in Effect pipelines:
    pipe(
        pattern.validate(rules),
        Either.map(() => pattern),
        Either.flatMap(p => p.validate(otherRules)),
        Either.match({
            onLeft: handleError,
            onRight: processValidPattern
        })
    );
    */

  // ===== Query Operations with Type-Safe Callbacks =====

  // Predicates are type-checked
  const hasPositive: boolean = numbers.anyValue((v: number) => v > 0);
  const allEven: boolean = numbers.allValues((v: number) => v % 2 === 0);

  // Filter with pattern predicate returns array
  const largeValues: Pattern<number>[] = numbers.filter(
    (p: Pattern<number>) => p.value > 1,
  );
  console.log(`Filtered to ${largeValues.length} patterns with value > 1`);

  // findFirst returns Pattern<V> | null
  const first: Pattern<number> | null = numbers.findFirst((p) => p.value === 2);
  if (first) {
    console.log(`Found first match: ${first.value}`); // Type-safe access
  }

  // ===== Structure Analysis =====

  const analysis = numbers.analyzeStructure();
  console.log(`Summary: ${analysis.summary}`);
  console.log(`Depths: [${analysis.depthDistribution}]`);
  console.log(`Element counts: [${analysis.elementCounts}]`);

  // ===== Combination =====

  const combined: Pattern<string> = Pattern.point("hello").combine(
    Pattern.point(" world"),
  );
  console.log(`Combined pattern: ${combined.value}`);

  // ===== Complex Generic Example: Tree of Trees =====

  // Pattern<Pattern<Pattern<string>>>
  const deepTree: Pattern<Pattern<Pattern<string>>> = Pattern.pattern(
    Pattern.pattern(Pattern.point("leaf"), []),
    [],
  );

  // Extract unwraps one level at a time
  const level2: Pattern<Pattern<string>> = deepTree.extract();
  const level1: Pattern<string> = level2.extract();
  const leaf: string = level1.extract();
  console.log(`Extracted leaf value: ${leaf}`);

  console.log(
    "TypeScript type safety verified! All operations type-check correctly.",
  );
}

// Run if executed directly
if (import.meta.url === `file://${process.argv[1]}`) {
  typeScriptDemo().catch(console.error);
}

export { typeScriptDemo };
