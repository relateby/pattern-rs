/**
 * TypeScript type definitions for pattern-core WASM module.
 *
 * These hand-written definitions provide generic Pattern<V> types and
 * full coverage of the WASM API surface for TypeScript consumers.
 *
 * @module pattern_core
 */

// ============================================================================
// Either-like Types (for effect-ts compatibility)
// ============================================================================

/**
 * Right side of Either - represents success.
 * Compatible with effect-ts Either.right().
 */
export interface Right<T> {
  readonly _tag: "Right";
  readonly right: T;
}

/**
 * Left side of Either - represents failure.
 * Compatible with effect-ts Either.left().
 */
export interface Left<E> {
  readonly _tag: "Left";
  readonly left: E;
}

/**
 * Either type for fallible operations.
 * Trivially convertible to effect-ts Either.
 */
export type Either<E, T> = Left<E> | Right<T>;

// ============================================================================
// Validation Types
// ============================================================================

/**
 * Validation error returned from validate().
 */
export interface ValidationError {
  /** Human-readable error message */
  readonly message: string;
  /** Name of violated rule (e.g., "max_depth", "max_elements") */
  readonly ruleViolated: string;
  /** Path to violating node in pattern structure */
  readonly location: readonly string[];
}

/**
 * Configurable validation rules for pattern structure.
 *
 * All rules are optional - undefined/null means no limit.
 */
export class ValidationRules {
  /**
   * Create new validation rules.
   *
   * @param maxDepth - Maximum nesting depth allowed (undefined = no limit)
   * @param maxElements - Maximum element count allowed (undefined = no limit)
   */
  constructor(maxDepth?: number, maxElements?: number);

  /** Maximum nesting depth allowed (undefined = no limit) */
  readonly maxDepth?: number;
  /** Maximum element count allowed (undefined = no limit) */
  readonly maxElements?: number;
}

/**
 * Result of pattern structure analysis.
 */
export class StructureAnalysis {
  /** Human-readable summary of the structure */
  readonly summary: string;
  /** Count of nodes at each depth level (index = depth, value = count) */
  readonly depthDistribution: number[];
  /** Element counts at each level (index = level, value = element count) */
  readonly elementCounts: number[];
  /** Identified nesting patterns (e.g., "linear", "tree", "balanced") */
  readonly nestingPatterns: string[];
}

// ============================================================================
// Value Types
// ============================================================================

/**
 * Value represents property values with multiple type variants.
 *
 * This is a discriminated union that can hold:
 * - Primitives: string, int, decimal, boolean
 * - Structured: symbol, array, map, range, measurement, tagged string
 */
export type Value = any; // Runtime representation from WASM

/**
 * Value factory functions for creating typed values.
 *
 * Use these factories to create Value instances from JavaScript types.
 */
export namespace Value {
  /** Create a string value */
  function string(s: string): Value;

  /** Create an integer value */
  function int(i: number): Value;

  /** Create a decimal (floating point) value */
  function decimal(n: number): Value;

  /** Create a boolean value */
  function boolean(b: boolean): Value;

  /** Create a symbol value */
  function symbol(s: string): Value;

  /** Create an array value */
  function array(items: Value[]): Value;

  /** Create a map value */
  function map(entries: Record<string, Value>): Value;

  /**
   * Create a range value.
   *
   * @param lower - Optional lower bound (undefined = unbounded)
   * @param upper - Optional upper bound (undefined = unbounded)
   */
  function range(lower?: number, upper?: number): Value;

  /**
   * Create a measurement value.
   *
   * @param value - Numeric value
   * @param unit - Unit string (e.g., "kg", "m", "s")
   */
  function measurement(value: number, unit: string): Value;
}

/**
 * Symbol represents a unique identifier with compile-time type safety.
 *
 * This is a branded string type that provides compile-time distinction from
 * arbitrary strings while remaining a string at runtime for serialization
 * and WASM compatibility.
 *
 * Always use createSymbol() to construct Symbol values for type safety.
 *
 * @example
 * ```typescript
 * const userId = createSymbol("alice");
 * const subject = new Subject(userId, ["Person"], {});
 * ```
 */
export type Symbol = string & { readonly __brand: unique symbol };

/**
 * Create a typed Symbol from a string value.
 *
 * This factory function provides type-safe construction of Symbol values,
 * preventing accidental use of arbitrary strings where identity is required.
 *
 * At runtime, this is a no-op (returns the input string), but at compile time
 * it ensures proper typing through the branded type system.
 *
 * @param value - The string value for the symbol
 * @returns A branded Symbol type
 *
 * @example
 * ```typescript
 * const userId = createSymbol("alice");
 * const adminId = createSymbol("admin");
 *
 * // Type-safe: can only use Symbol where Symbol is expected
 * const subject = new Subject(userId, ["Person"], {});
 *
 * // Type error: can't pass arbitrary strings
 * // const subject = new Subject("alice", ["Person"], {}); // Error!
 * ```
 */
export function createSymbol(value: string): Symbol;

// ============================================================================
// Subject Type
// ============================================================================

/**
 * Subject represents a self-descriptive value with identity, labels, and properties.
 *
 * Subjects are commonly used as Pattern values to represent entities with metadata.
 */
export class Subject {
  /**
   * Create a new Subject.
   *
   * @param identity - Unique identifier symbol (use createSymbol() to construct)
   * @param labels - Array of label strings
   * @param properties - Map of property names to values
   *
   * @example
   * ```typescript
   * const subject = new Subject(
   *   createSymbol("alice"),
   *   ["Person", "User"],
   *   { name: Value.string("Alice"), age: Value.int(30) }
   * );
   * ```
   */
  constructor(
    identity: Symbol,
    labels: string[],
    properties: Record<string, Value>,
  );

  /** The identity symbol */
  readonly identity: Symbol;

  /** Array of label strings */
  readonly labels: string[];

  /** Map of property names to Value instances */
  readonly properties: Record<string, Value>;
}

// ============================================================================
// Pattern<V> Type (Generic Pattern over Value Type V)
// ============================================================================

/**
 * Pattern<V> represents a recursive, nested structure generic over value type V.
 *
 * A Pattern is a tree structure where each node has:
 * - A value of type V (the "decoration")
 * - Zero or more child patterns (the "elements")
 *
 * Common instantiations:
 * - Pattern<Subject> - patterns holding Subject entities
 * - Pattern<string> - patterns holding strings
 * - Pattern<number> - patterns holding numbers
 * - Pattern<Pattern<V>> - nested patterns
 *
 * Patterns support:
 * - Construction: point, of, pattern, fromValues
 * - Inspection: size, depth, length, isAtomic, values
 * - Query: anyValue, allValues, filter, findFirst, matches, contains
 * - Transformation: map, fold, para (paramorphism)
 * - Combination: combine
 * - Comonad operations: extract, extend, depthAt, sizeAt, indicesAt
 * - Validation: validate, analyzeStructure
 */
export class Pattern<V = any> {
  // ========================================================================
  // Static Constructors
  // ========================================================================

  /**
   * Create an atomic pattern from any value.
   *
   * An atomic pattern has a value but no child elements.
   * Accepts any JavaScript value: primitives, objects, Subject instances,
   * or even other Pattern instances (for nesting).
   *
   * @param value - Any JavaScript value
   * @returns A new atomic Pattern containing that value
   *
   * @example
   * ```typescript
   * const p1 = Pattern.point("hello");
   * const p2 = Pattern.point(42);
   * const subject = new Subject("alice", [], {});
   * const p3 = Pattern.point(subject);
   * ```
   */
  static point<V>(value: V): Pattern<V>;

  /**
   * Alias for point(). Create an atomic pattern from any value.
   *
   * This is identical to point() - just a different name following
   * functional programming convention where "of" is used to "lift"
   * a value into a functor.
   *
   * @param value - Any JavaScript value
   * @returns A new atomic Pattern containing that value
   */
  static of<V>(value: V): Pattern<V>;

  /**
   * Create a pattern with a value and optionally add elements later.
   *
   * Use addElement() to add child patterns after construction.
   *
   * @param value - Any JavaScript value
   * @returns A new Pattern with the value and no elements
   *
   * @example
   * ```typescript
   * const pattern = Pattern.pattern("parent");
   * pattern.addElement(Pattern.of("child1"));
   * pattern.addElement(Pattern.of("child2"));
   * ```
   */
  static pattern<V>(value: V): Pattern<V>;

  /**
   * Create an array of atomic patterns from an array of values.
   *
   * Each value in the input array is lifted to an atomic pattern using point().
   * Returns an array of Pattern instances, not a single nested pattern.
   *
   * @param values - Array of values
   * @returns Array of Pattern instances (one atomic pattern per value)
   *
   * @example
   * ```typescript
   * const patterns = Pattern.fromValues([1, 2, 3]);
   * // Returns [Pattern.point(1), Pattern.point(2), Pattern.point(3)]
   * console.log(patterns.length); // 3
   * console.log(patterns[0].value); // 1
   * ```
   */
  static fromValues<V>(values: V[]): Pattern<V>[];

  // ========================================================================
  // Accessors
  // ========================================================================

  /**
   * The value at the root of this pattern.
   *
   * Returns the JavaScript value stored in this pattern (can be any type).
   */
  readonly value: V;

  /**
   * The nested elements (sub-patterns) of this pattern as an array.
   *
   * Returns an array of Pattern instances representing the children.
   * Atomic patterns have an empty array.
   */
  readonly elements: Pattern<V>[];

  // ========================================================================
  // Instance Methods
  // ========================================================================

  /**
   * Add a child pattern element to this pattern.
   *
   * This method mutates the pattern by adding a new child element.
   *
   * @param element - A Pattern to add as a child
   */
  addElement(element: Pattern<V>): void;

  /**
   * Get a child element by index.
   *
   * @param index - The index of the element to retrieve (0-based)
   * @returns A Pattern if the index is valid, or undefined
   */
  getElement(index: number): Pattern<V> | undefined;

  // ========================================================================
  // Inspection Methods
  // ========================================================================

  /**
   * Get the number of direct child elements.
   *
   * @returns The number of elements (0 for atomic patterns)
   */
  length(): number;

  /**
   * Get the total number of nodes in the pattern structure.
   *
   * Counts this node plus all nested nodes recursively.
   *
   * @returns The total number of nodes (root + all nested nodes)
   *
   * @example
   * ```typescript
   * const atomic = Pattern.point("atom");
   * console.log(atomic.size()); // 1
   *
   * const pattern = Pattern.pattern("root");
   * pattern.addElement(Pattern.of("child1"));
   * pattern.addElement(Pattern.of("child2"));
   * console.log(pattern.size()); // 3 (root + 2 children)
   * ```
   */
  size(): number;

  /**
   * Get the maximum nesting depth of the pattern structure.
   *
   * @returns The maximum nesting depth (atomic patterns have depth 0)
   *
   * @example
   * ```typescript
   * const atomic = Pattern.point("hello");
   * console.log(atomic.depth()); // 0
   *
   * const nested = Pattern.pattern("parent");
   * const child = Pattern.pattern("child");
   * child.addElement(Pattern.of("grandchild"));
   * nested.addElement(child);
   * console.log(nested.depth()); // 2
   * ```
   */
  depth(): number;

  /**
   * Check if this pattern is atomic (has no children).
   *
   * @returns true if the pattern has no elements, false otherwise
   */
  isAtomic(): boolean;

  /**
   * Extract all values from the pattern as a flat array.
   *
   * Performs pre-order traversal (root first, then elements).
   *
   * @returns Array containing all values in pre-order
   *
   * @example
   * ```typescript
   * const pattern = Pattern.pattern("root");
   * pattern.addElement(Pattern.of("child1"));
   * pattern.addElement(Pattern.of("child2"));
   * const values = pattern.values();
   * // Returns ["root", "child1", "child2"]
   * ```
   */
  values(): V[];

  // ========================================================================
  // Query Methods
  // ========================================================================

  /**
   * Check if at least one value satisfies the given predicate.
   *
   * Traverses in pre-order and short-circuits on first match.
   *
   * @param predicate - Function that takes a value and returns boolean
   * @returns true if at least one value satisfies the predicate
   *
   * @example
   * ```typescript
   * const pattern = Pattern.pattern("hello");
   * pattern.addElement(Pattern.of("world"));
   * const hasWorld = pattern.anyValue(v => v === "world"); // true
   * ```
   */
  anyValue(predicate: (v: V) => boolean): boolean;

  /**
   * Check if all values satisfy the given predicate.
   *
   * Traverses in pre-order and short-circuits on first failure.
   *
   * @param predicate - Function that takes a value and returns boolean
   * @returns true if all values satisfy the predicate
   *
   * @example
   * ```typescript
   * const pattern = Pattern.pattern("hello");
   * pattern.addElement(Pattern.of("world"));
   * const allStrings = pattern.allValues(v => typeof v === "string"); // true
   * ```
   */
  allValues(predicate: (v: V) => boolean): boolean;

  /**
   * Filter subpatterns that satisfy the given pattern predicate.
   *
   * Traverses in pre-order and collects all matching patterns.
   *
   * @param predicate - Function that takes a Pattern and returns boolean
   * @returns Array of Pattern instances that satisfy the predicate
   *
   * @example
   * ```typescript
   * const pattern = Pattern.pattern("root");
   * pattern.addElement(Pattern.of("leaf1"));
   * pattern.addElement(Pattern.of("leaf2"));
   * const leaves = pattern.filter(p => p.isAtomic());
   * ```
   */
  filter(predicate: (p: Pattern<V>) => boolean): Pattern<V>[];

  /**
   * Find the first subpattern that satisfies the given predicate.
   *
   * Performs depth-first pre-order traversal and short-circuits on first match.
   *
   * @param predicate - Function that takes a Pattern and returns boolean
   * @returns The first matching Pattern, or null if no match found
   *
   * @example
   * ```typescript
   * const pattern = Pattern.pattern("root");
   * pattern.addElement(Pattern.of("target"));
   * const found = pattern.findFirst(p => p.value === "target");
   * ```
   */
  findFirst(predicate: (p: Pattern<V>) => boolean): Pattern<V> | null;

  /**
   * Check if two patterns have identical structure.
   *
   * Compares values and tree structure recursively.
   *
   * @param other - Another Pattern to compare with
   * @returns true if the patterns match, false otherwise
   */
  matches(other: Pattern<V>): boolean;

  /**
   * Check if this pattern contains another pattern as a subpattern.
   *
   * @param subpattern - The pattern to search for
   * @returns true if this pattern contains the subpattern
   */
  contains(subpattern: Pattern<V>): boolean;

  // ========================================================================
  // Transformation Methods
  // ========================================================================

  /**
   * Transform all values in the pattern using a mapping function.
   *
   * Creates a new pattern with the same structure but with values
   * transformed by the function. The function is applied to each
   * value in the pattern.
   *
   * @param fn - Function that takes a value and returns a new value
   * @returns A new Pattern with transformed values
   *
   * @example
   * ```typescript
   * const pattern = Pattern.pattern("hello");
   * pattern.addElement(Pattern.of("world"));
   * const upper = pattern.map(v =>
   *   typeof v === 'string' ? v.toUpperCase() : v
   * );
   * // Returns Pattern with values ["HELLO", "WORLD"]
   * ```
   */
  map<W>(fn: (v: V) => W): Pattern<W>;

  /**
   * Fold the pattern into a single value using an accumulator.
   *
   * Processes values in depth-first, root-first order (pre-order traversal).
   * The accumulator is threaded through all processing steps.
   *
   * @param init - Initial accumulator value
   * @param fn - Function that takes (accumulator, value) and returns new accumulator
   * @returns The final accumulated value
   *
   * @example
   * ```typescript
   * const pattern = Pattern.pattern(10);
   * pattern.addElement(Pattern.of(20));
   * pattern.addElement(Pattern.of(30));
   * const sum = pattern.fold(0, (acc, v) => acc + v); // 60
   * ```
   */
  fold<T>(init: T, fn: (acc: T, v: V) => T): T;

  /**
   * Paramorphism: bottom-up fold with access to both pattern and child results.
   *
   * This is a powerful recursion scheme that processes the pattern bottom-up,
   * giving each node access to both its value and the results of processing
   * its children. This is equivalent to Rust pattern-core para.
   *
   * @param fn - Function that takes (pattern, childResults array) and returns a result
   * @returns The result of the paramorphism
   *
   * @example
   * ```typescript
   * const pattern = Pattern.pattern("root");
   * pattern.addElement(Pattern.of("child1"));
   * pattern.addElement(Pattern.of("child2"));
   *
   * // Count nodes: each node returns 1 + sum of child counts
   * const count = pattern.para((p, childResults) => {
   *     return 1 + childResults.reduce((sum, r) => sum + r, 0);
   * });
   * // Returns 3 (root + 2 children)
   * ```
   */
  para<R>(fn: (pattern: Pattern<V>, elementResults: R[]) => R): R;

  // ========================================================================
  // Combination Methods
  // ========================================================================

  /**
   * Combine two patterns associatively.
   *
   * For JavaScript values, this uses a custom combiner function to combine
   * the values. Elements are concatenated (left first, then right).
   *
   * @param other - Another Pattern to combine with
   * @param combiner - Function that takes (value1, value2) and returns combined value
   * @returns A new Pattern with combined value and concatenated elements
   *
   * @example
   * ```typescript
   * const p1 = Pattern.pattern("hello");
   * p1.addElement(Pattern.of("a"));
   * const p2 = Pattern.pattern(" world");
   * p2.addElement(Pattern.of("b"));
   * const combined = p1.combine(p2, (v1, v2) => v1 + v2);
   * // Result: Pattern("hello world") with elements [a, b]
   * ```
   */
  combine(other: Pattern<V>, combiner: (v1: V, v2: V) => V): Pattern<V>;

  // ========================================================================
  // Comonad Methods
  // ========================================================================

  /**
   * Extract the decorative value at the current position.
   *
   * In Pattern's "decorated sequence" semantics, the value provides
   * information ABOUT the elements. This operation accesses that
   * decorative information.
   *
   * @returns The value at this position
   */
  extract(): V;

  /**
   * Compute new decorative information at each position based on subpattern context.
   *
   * This is a powerful comonad operation that gives each position access to
   * its full subpattern context, enabling context-aware computation of new decorations.
   *
   * @param fn - Function that takes a Pattern and returns a new value
   * @returns A new Pattern with the same structure but with computed decorative values
   *
   * @example
   * ```typescript
   * const p = Pattern.pattern("root");
   * p.addElement(Pattern.of("child1"));
   * p.addElement(Pattern.of("child2"));
   *
   * // Decorate each position with its size
   * const sizes = p.extend(subpattern => subpattern.size());
   * console.log(sizes.extract()); // 3 (root has 3 nodes)
   * ```
   */
  extend<W>(fn: (p: Pattern<V>) => W): Pattern<W>;

  /**
   * Decorate each position with its depth (maximum nesting level).
   *
   * Uses extend to compute the depth at every position.
   *
   * @returns A Pattern where each position's value is the depth of that subpattern
   *
   * @example
   * ```typescript
   * const p = Pattern.pattern("root");
   * const child = Pattern.pattern("child");
   * child.addElement(Pattern.of("grandchild"));
   * p.addElement(child);
   *
   * const depths = p.depthAt();
   * console.log(depths.extract()); // 2 (root has depth 2)
   * ```
   */
  depthAt(): Pattern<number>;

  /**
   * Decorate each position with its subtree size (total node count).
   *
   * Uses extend to compute the size at every position.
   *
   * @returns A Pattern where each position's value is the size of that subpattern
   *
   * @example
   * ```typescript
   * const p = Pattern.pattern("root");
   * p.addElement(Pattern.of("child1"));
   * p.addElement(Pattern.of("child2"));
   *
   * const sizes = p.sizeAt();
   * console.log(sizes.extract()); // 3 (root + 2 children)
   * ```
   */
  sizeAt(): Pattern<number>;

  /**
   * Decorate each position with its path from root (sequence of element indices).
   *
   * @returns A Pattern where each position's value is an array representing the path from root
   *
   * @example
   * ```typescript
   * const p = Pattern.pattern("root");
   * const child = Pattern.pattern("child");
   * child.addElement(Pattern.of("grandchild"));
   * p.addElement(child);
   *
   * const paths = p.indicesAt();
   * console.log(paths.extract()); // [] (root path)
   * ```
   */
  indicesAt(): Pattern<number[]>;

  // ========================================================================
  // Validation/Analysis Methods
  // ========================================================================

  /**
   * Validate pattern structure against configurable rules.
   *
   * Returns an Either-like value (does NOT throw):
   * - Success: { _tag: 'Right', right: undefined }
   * - Failure: { _tag: 'Left', left: ValidationError }
   *
   * This return shape is compatible with effect-ts Either type.
   *
   * @param rules - ValidationRules specifying constraints
   * @returns An Either-like value (does not throw)
   *
   * @example
   * ```typescript
   * const pattern = Pattern.pattern("root");
   * pattern.addElement(Pattern.of("child"));
   *
   * const rules = new ValidationRules(10, 100);
   * const result = pattern.validate(rules);
   *
   * if (result._tag === 'Left') {
   *     console.error('Validation failed:', result.left.message);
   * } else {
   *     console.log('Pattern is valid');
   * }
   * ```
   */
  validate(rules: ValidationRules): Either<ValidationError, void>;

  /**
   * Analyze the structural characteristics of the pattern.
   *
   * Returns detailed information about depth distribution, element counts,
   * nesting patterns, and a human-readable summary.
   *
   * @returns A StructureAnalysis object
   *
   * @example
   * ```typescript
   * const pattern = Pattern.pattern("root");
   * pattern.addElement(Pattern.of("child1"));
   * pattern.addElement(Pattern.of("child2"));
   *
   * const analysis = pattern.analyzeStructure();
   * console.log('Summary:', analysis.summary);
   * console.log('Depth distribution:', analysis.depthDistribution);
   * ```
   */
  analyzeStructure(): StructureAnalysis;
}
