// pattern-core WASM Node.js Example
//
// Prerequisites:
//   1. Build WASM package: cd crates/pattern-core && wasm-pack build --target nodejs --features wasm
//   2. Link package: ln -s ../../crates/pattern-core/pkg pkg
//   3. Run: node node.mjs

import { default as init, Pattern, Subject, Value, ValidationRules } from './pkg/pattern_core.js';

// Helper to format output sections
function log(title, content) {
    console.log(`\n=== ${title} ===`);
    console.log(content);
}

async function runExample() {
    // Initialize WASM module
    await init();
    log('Initialization', 'WASM module loaded successfully');

    // 1. Atomic Pattern
    const atomic = Pattern.point("hello");
    log('1. Atomic Pattern',
        `Pattern.point("hello"):\n` +
        `  value: ${atomic.value}\n` +
        `  elements: [${atomic.elements}]\n` +
        `  isAtomic: ${atomic.isAtomic()}\n` +
        `  length: ${atomic.length()}\n` +
        `  size: ${atomic.size()}\n` +
        `  depth: ${atomic.depth()}`
    );

    // 2. Nested Pattern
    const child1 = Pattern.point("child1");
    const child2 = Pattern.point("child2");
    const parent = Pattern.pattern("parent", [child1, child2]);
    log('2. Nested Pattern',
        `Pattern.pattern("parent", [child1, child2]):\n` +
        `  value: ${parent.value}\n` +
        `  length: ${parent.length()}\n` +
        `  size: ${parent.size()}\n` +
        `  depth: ${parent.depth()}\n` +
        `  values: [${parent.values().join(', ')}]`
    );

    // 3. fromValues (multiple atomic patterns)
    const atomics = Pattern.fromValues(["a", "b", "c"]);
    log('3. fromValues',
        `Pattern.fromValues(["a", "b", "c"]):\n` +
        `  returns array of ${atomics.length} atomic patterns\n` +
        `  values: [${atomics.map(p => p.value).join(', ')}]`
    );

    // 4. Pattern with Subject
    const subject = Subject.new("n", ["Person", "Employee"], {
        name: Value.string("Alice"),
        age: Value.int(30),
        salary: Value.decimal(75000.50)
    });
    const subjectPattern = Pattern.point(subject);
    log('4. Pattern with Subject',
        `Subject with identity "n", labels ["Person", "Employee"]:\n` +
        `  identity: ${subject.identity}\n` +
        `  labels: [${Array.from(subject.labels).join(', ')}]\n` +
        `  hasLabel("Person"): ${subject.hasLabel("Person")}\n` +
        `  properties: ${Object.keys(subject.properties).join(', ')}\n` +
        `  pattern.depth: ${subjectPattern.depth()}`
    );

    // 5. Map (Transformation)
    const numbers = Pattern.pattern(1, [Pattern.point(2), Pattern.point(3)]);
    const doubled = numbers.map(n => n * 2);
    log('5. Map Transformation',
        `Pattern.pattern(1, [2, 3]).map(n => n * 2):\n` +
        `  original values: [${numbers.values().join(', ')}]\n` +
        `  doubled values: [${doubled.values().join(', ')}]`
    );

    // 6. Filter (Query)
    const filtered = numbers.filter(p => p.value > 1);
    log('6. Filter Query',
        `numbers.filter(p => p.value > 1):\n` +
        `  original length: ${numbers.length()}\n` +
        `  filtered length: ${filtered.length()}\n` +
        `  filtered values: [${filtered.values().join(', ')}]`
    );

    // 7. Fold (Transformation)
    const sum = numbers.fold(0, (acc, val) => acc + val);
    const product = numbers.fold(1, (acc, val) => acc * val);
    log('7. Fold Transformation',
        `numbers.fold(0, (acc, val) => acc + val): ${sum}\n` +
        `numbers.fold(1, (acc, val) => acc * val): ${product}`
    );

    // 8. Paramorphism (bottom-up fold)
    const tree = Pattern.pattern("root", [
        Pattern.pattern("a", [Pattern.point("a1"), Pattern.point("a2")]),
        Pattern.point("b")
    ]);
    const paraResult = tree.para((value, childResults) => {
        if (childResults.length === 0) {
            return value.toString();
        }
        return `(${value} [${childResults.join(', ')}])`;
    });
    log('8. Paramorphism (para)',
        `Bottom-up fold with access to both value and child results:\n` +
        `  structure: root(a(a1, a2), b)\n` +
        `  result: ${paraResult}`
    );

    // 9. Comonad Operations
    const comonadPattern = Pattern.pattern("root", [
        Pattern.pattern("a", [Pattern.point("a1")]),
        Pattern.point("b")
    ]);
    const extracted = comonadPattern.extract();
    const depths = comonadPattern.depthAt();
    const sizes = comonadPattern.sizeAt();
    const indices = comonadPattern.indicesAt();

    log('9. Comonad Operations',
        `extract(): ${extracted}\n` +
        `depthAt() values: [${depths.values().join(', ')}]\n` +
        `sizeAt() values: [${sizes.values().join(', ')}]\n` +
        `indicesAt() first element: [${indices.elements[0].value}]`
    );

    // 10. Extended comonad operation
    const extended = comonadPattern.extend(p => p.size());
    log('10. Extend (Comonad)',
        `extend(p => p.size()) - replace each value with its size:\n` +
        `  values: [${extended.values().join(', ')}]`
    );

    // 11. Combination
    const p1 = Pattern.point("hello");
    const p2 = Pattern.point(" world");
    const combined = p1.combine(p2);
    log('11. Combination',
        `Pattern.point("hello").combine(Pattern.point(" world")):\n` +
        `  result: ${combined.value}`
    );

    // 12. Validation (Either-like return)
    const rules = ValidationRules.new({ maxDepth: 2, maxElements: 10 });
    const validResult = numbers.validate(rules);

    const deepPattern = Pattern.pattern("a", [
        Pattern.pattern("b", [
            Pattern.pattern("c", [Pattern.point("d")])
        ])
    ]);
    const invalidResult = deepPattern.validate(rules);

    log('12. Validation (Either-like)',
        `Validation returns Either-like: { _tag: 'Right' | 'Left', right/left: ... }\n\n` +
        `Valid pattern (depth 1):\n` +
        `  _tag: ${validResult._tag}\n` +
        `  isRight: ${validResult._tag === 'Right'}\n\n` +
        `Invalid pattern (depth 3, maxDepth 2):\n` +
        `  _tag: ${invalidResult._tag}\n` +
        `  isLeft: ${invalidResult._tag === 'Left'}\n` +
        `  error: ${invalidResult._tag === 'Left' ? invalidResult.left.message : 'N/A'}\n` +
        `  rule violated: ${invalidResult._tag === 'Left' ? invalidResult.left.ruleViolated : 'N/A'}`
    );

    // 13. Effect-ts compatibility example (conceptual)
    log('13. effect-ts Compatibility',
        `The Either-like return shape is directly compatible with effect-ts:\n\n` +
        `  import { Either } from 'effect';\n` +
        `  const result = pattern.validate(rules);\n` +
        `  Either.match(result, {\n` +
        `    onLeft: (err) => console.error('Failed:', err.message),\n` +
        `    onRight: () => console.log('Valid'),\n` +
        `  });\n\n` +
        `No wrapper needed - the { _tag, right/left } shape is native Either.`
    );

    // 14. Structure Analysis
    const analysis = deepPattern.analyzeStructure();
    log('14. Structure Analysis',
        `analyzeStructure() for deep pattern:\n` +
        `  summary: ${analysis.summary}\n` +
        `  depth distribution: [${analysis.depthDistribution.join(', ')}]\n` +
        `  element counts: [${analysis.elementCounts.join(', ')}]`
    );

    // 15. Query Operations
    const hasLargeValue = numbers.anyValue(v => v > 2);
    const allPositive = numbers.allValues(v => v > 0);
    const first = numbers.findFirst(p => p.value > 1);
    const containsTwo = numbers.contains(Pattern.point(2));
    const matchesPattern = numbers.matches(Pattern.pattern(1, [Pattern.point(2), Pattern.point(3)]));

    log('15. Query Operations',
        `anyValue(v => v > 2): ${hasLargeValue}\n` +
        `allValues(v => v > 0): ${allPositive}\n` +
        `findFirst(p => p.value > 1): ${first ? first.value : 'null'}\n` +
        `contains(Pattern.point(2)): ${containsTwo}\n` +
        `matches(exact pattern): ${matchesPattern}`
    );

    log('Complete', 'All examples executed successfully! ✓');
}

runExample().catch(err => {
    console.error('Error:', err);
    process.exit(1);
});
