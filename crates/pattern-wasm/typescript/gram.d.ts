/**
 * TypeScript type definitions for unified gram-wasm package.
 *
 * This module provides a single entry point for:
 * - Pattern<V>: Generic recursive pattern structure
 * - Subject: Self-descriptive values with identity, labels, and properties
 * - Value: Property value types
 * - Gram: Gram notation serialization/parsing namespace
 *
 * @module gram
 */

// ============================================================================
// Re-export Core Types from pattern_core
// ============================================================================

export {
  Pattern,
  Subject,
  Value,
  Symbol,
  createSymbol,
  ValidationRules,
  ValidationError,
  StructureAnalysis,
  Either,
  Left,
  Right,
} from "./pattern_core";

import { Pattern } from "./pattern_core";
import { Subject } from "./pattern_core";
import { Value } from "./pattern_core";

// ============================================================================
// Conventional Conversion Options
// ============================================================================

/**
 * Options for converting conventional JavaScript values to Subject instances.
 *
 * Used by both Subject.fromValue() and Gram.from() to control how values
 * are mapped to Subject structures.
 */
export interface FromValueOptions {
  /**
   * Label to apply to converted subjects.
   *
   * Default behavior (when undefined):
   * - number → "Number"
   * - string → "String"
   * - boolean → "Bool"
   * - Subject → passthrough (no conversion)
   *
   * For arrays and objects, use Gram.from() which applies:
   * - array → "List" (elements become pattern children)
   * - object → "Map" (key-value pairs become pattern children)
   */
  label?: string;

  /**
   * Property name for storing the original value.
   *
   * Default: "value"
   *
   * The converted Subject will have a property with this name
   * containing the original primitive value.
   *
   * @example
   * ```typescript
   * // Default behavior
   * Subject.fromValue(42)
   * // → Subject with identity="_0", labels=["Number"], properties={value: 42}
   *
   * // Custom property name
   * Subject.fromValue(42, { valueProperty: "data" })
   * // → Subject with identity="_0", labels=["Number"], properties={data: 42}
   * ```
   */
  valueProperty?: string;

  /**
   * Custom identity generator function.
   *
   * Default: Auto-generates identities as "_0", "_1", "_2", ...
   *
   * The function receives the value being converted and an index (for array contexts).
   *
   * @param value - The value being converted
   * @param index - Index in array context (or 0 for single values)
   * @returns A string to use as the Subject identity
   *
   * @example
   * ```typescript
   * // Custom identity based on value
   * Subject.fromValue("alice", {
   *   identity: (v) => `user_${v}`
   * })
   * // → Subject with identity="user_alice"
   *
   * // Custom identity based on index
   * const patterns = [1, 2, 3].map((v, i) =>
   *   Pattern.point(Subject.fromValue(v, {
   *     identity: (_, idx) => `item_${idx}`
   *   }))
   * );
   * ```
   */
  identity?: (value: unknown, index: number) => string;
}

/**
 * Options for converting Pattern<V> to Pattern<Subject>.
 *
 * This is the same interface as FromValueOptions - Gram.from() passes
 * these options through to Subject.fromValue() for each value in the pattern.
 */
export interface FromOptions extends FromValueOptions {}

// ============================================================================
// Extended Subject Interface with fromValue
// ============================================================================

/**
 * Augment the Subject class with the fromValue static method.
 */
declare module "./pattern_core" {
  interface SubjectConstructor {
    /**
     * Convert a conventional JavaScript value to a Subject instance.
     *
     * Supports primitives only (string, number, boolean, Subject).
     * For arrays and objects, use Gram.from() which creates proper
     * Pattern structures with List/Map labels.
     *
     * **Mapping Rules (pattern-lisp compatible)**:
     * - `number` → Subject with label "Number", property {value: n}
     * - `string` → Subject with label "String", property {value: s}
     * - `boolean` → Subject with label "Bool", property {value: b}
     * - `Subject` → passthrough (returns the Subject unchanged)
     * - `array` → **Error** (use Gram.from() instead)
     * - `object` → **Error** (use Gram.from() instead)
     *
     * @param value - The value to convert (primitive or Subject)
     * @param options - Optional conversion options
     * @returns A Subject instance
     * @throws Error if value is an array or object (use Gram.from() instead)
     *
     * @example
     * ```typescript
     * // Convert primitives
     * const numSubject = Subject.fromValue(42);
     * // → Subject { identity: "_0", labels: ["Number"], properties: {value: 42} }
     *
     * const strSubject = Subject.fromValue("hello");
     * // → Subject { identity: "_0", labels: ["String"], properties: {value: "hello"} }
     *
     * const boolSubject = Subject.fromValue(true);
     * // → Subject { identity: "_0", labels: ["Bool"], properties: {value: true} }
     *
     * // Passthrough existing Subject
     * const existing = new Subject("alice", ["Person"], {});
     * const same = Subject.fromValue(existing);
     * // → Returns the same Subject instance
     *
     * // Custom options
     * const custom = Subject.fromValue(42, {
     *   label: "Integer",
     *   valueProperty: "data",
     *   identity: (v, i) => `num_${i}`
     * });
     * // → Subject { identity: "num_0", labels: ["Integer"], properties: {data: 42} }
     *
     * // Arrays and objects not supported
     * try {
     *   Subject.fromValue([1, 2, 3]); // Throws error
     * } catch (e) {
     *   console.error("Use Gram.from() for arrays/objects");
     * }
     * ```
     */
    fromValue(value: unknown, options?: FromValueOptions): Subject;
  }

  const Subject: SubjectConstructor;
}

// ============================================================================
// Gram Namespace (Serialization/Parsing/Conversion)
// ============================================================================

/**
 * Gram namespace provides gram notation serialization, parsing, and conversion.
 *
 * **Core Operations**:
 * - `stringify()` - Serialize Pattern<Subject> to gram notation
 * - `parse()` - Parse gram notation to Pattern<Subject>[]
 * - `parseOne()` - Parse gram notation to first Pattern<Subject> or null
 * - `from()` - Convert Pattern<V> to Pattern<Subject> with conventional mapping
 *
 * **Usage**:
 * ```typescript
 * import { Pattern, Subject, Gram } from 'gram';
 *
 * // Build a pattern
 * const pattern = Pattern.pattern(new Subject("root", ["Node"], {}));
 * pattern.addElement(Pattern.of(new Subject("child", ["Leaf"], {})));
 *
 * // Serialize to gram notation
 * const gram = Gram.stringify(pattern);
 * console.log(gram); // "(root:Node)-[]->(child:Leaf)"
 *
 * // Parse back
 * const parsed = Gram.parseOne(gram);
 * console.log(parsed?.value.identity); // "root"
 *
 * // Convert conventional values
 * const dataPattern = Pattern.pattern([1, 2, 3]);
 * const subjectPattern = Gram.from(dataPattern);
 * const gramNotation = Gram.stringify(subjectPattern);
 * ```
 */
export namespace Gram {
  /**
   * Serialize a Pattern<Subject> to gram notation.
   *
   * Converts a single pattern with Subject values into a human-readable
   * gram notation string. The pattern must have Subject values - use
   * Gram.from() to convert Pattern<V> with other value types first.
   *
   * @param pattern - A Pattern with Subject values
   * @returns Gram notation string representation
   * @throws Error if pattern has non-Subject values
   *
   * @example
   * ```typescript
   * const subject = new Subject("alice", ["Person"], {
   *   name: Value.string("Alice")
   * });
   * const pattern = Pattern.of(subject);
   *
   * const gram = Gram.stringify(pattern);
   * // Returns: "(alice:Person {name: "Alice"})"
   * ```
   */
  function stringify(pattern: Pattern<Subject>): string;

  /**
   * Parse gram notation into an array of Pattern<Subject> instances.
   *
   * Parses a gram notation string and returns all top-level patterns.
   * Empty or whitespace-only input returns an empty array (does not throw).
   *
   * @param gram - Gram notation string
   * @returns Array of Pattern<Subject> instances (empty array for empty input)
   * @throws Error with clear message if gram notation is invalid
   *
   * @example
   * ```typescript
   * // Parse single pattern
   * const patterns = Gram.parse("(alice:Person)");
   * console.log(patterns.length); // 1
   * console.log(patterns[0].value.identity); // "alice"
   *
   * // Parse multiple patterns
   * const multi = Gram.parse("(alice:Person) (bob:Person)");
   * console.log(multi.length); // 2
   *
   * // Empty input returns empty array
   * const empty = Gram.parse("");
   * console.log(empty.length); // 0
   *
   * // Whitespace-only returns empty array
   * const whitespace = Gram.parse("   \n  ");
   * console.log(whitespace.length); // 0
   *
   * // Invalid input throws clear error
   * try {
   *   Gram.parse("(invalid");
   * } catch (e) {
   *   console.error("Parse error:", e.message);
   * }
   * ```
   */
  function parse(gram: string): Pattern<Subject>[];

  /**
   * Parse gram notation and return the first pattern or null.
   *
   * Convenience method for parsing single patterns. Returns null for
   * empty or whitespace-only input (does not throw).
   *
   * @param gram - Gram notation string
   * @returns First Pattern<Subject> or null if empty
   * @throws Error with clear message if gram notation is invalid
   *
   * @example
   * ```typescript
   * // Parse single pattern
   * const pattern = Gram.parseOne("(alice:Person)");
   * if (pattern) {
   *   console.log(pattern.value.identity); // "alice"
   * }
   *
   * // Empty input returns null
   * const empty = Gram.parseOne("");
   * console.log(empty); // null
   *
   * // Whitespace-only returns null
   * const whitespace = Gram.parseOne("   ");
   * console.log(whitespace); // null
   *
   * // Multiple patterns - returns first
   * const first = Gram.parseOne("(alice:Person) (bob:Person)");
   * console.log(first?.value.identity); // "alice"
   *
   * // Invalid input throws clear error
   * try {
   *   Gram.parseOne("(invalid");
   * } catch (e) {
   *   console.error("Parse error:", e.message);
   * }
   * ```
   */
  function parseOne(gram: string): Pattern<Subject> | null;

  /**
   * Convert Pattern<V> to Pattern<Subject> using conventional value mapping.
   *
   * Recursively transforms a pattern with arbitrary value types into a pattern
   * with Subject values, making it serializable via Gram.stringify().
   *
   * This is implemented as `pattern.map(v => Subject.fromValue(v, options))`
   * with special handling for collections to create proper pattern structures.
   *
   * **Mapping Rules (pattern-lisp compatible)**:
   * - Primitives (number, string, boolean) → Subject with type-appropriate label
   * - Arrays → Pattern with "List" label and elements as pattern children
   * - Objects → Pattern with "Map" label and alternating key-value elements
   * - Subject instances → passthrough (no conversion)
   *
   * **Pattern Structure for Collections**:
   * - Arrays serialize as: `[:List | elem1, elem2, ...]`
   * - Objects serialize as: `[:Map | key1, val1, key2, val2, ...]`
   *
   * @param pattern - Pattern with any value type
   * @param options - Optional conversion options (passed to Subject.fromValue)
   * @returns Pattern with Subject values (gram-serializable)
   *
   * @example
   * ```typescript
   * // Convert pattern of primitives
   * const numbers = Pattern.pattern(42);
   * numbers.addElement(Pattern.of(100));
   * numbers.addElement(Pattern.of(200));
   *
   * const subjectPattern = Gram.from(numbers);
   * const gram = Gram.stringify(subjectPattern);
   * // Can now parse back:
   * const parsed = Gram.parseOne(gram);
   *
   * // Convert pattern with arrays (creates List structure)
   * const arrayPattern = Pattern.of([1, 2, 3]);
   * const listPattern = Gram.from(arrayPattern);
   * // Creates: Pattern with List subject and 3 Number elements
   *
   * // Convert pattern with objects (creates Map structure)
   * const objPattern = Pattern.of({ name: "Alice", age: 30 });
   * const mapPattern = Gram.from(objPattern);
   * // Creates: Pattern with Map subject and alternating key-value elements
   *
   * // With custom options
   * const custom = Gram.from(numbers, {
   *   label: "Integer",
   *   valueProperty: "data",
   *   identity: (v, i) => `num_${i}`
   * });
   *
   * // Mixed types
   * const mixed = Pattern.pattern("hello");
   * mixed.addElement(Pattern.of(42));
   * mixed.addElement(Pattern.of(true));
   * mixed.addElement(Pattern.of([1, 2]));
   *
   * const converted = Gram.from(mixed);
   * // Each value converted appropriately:
   * // - "hello" → String subject
   * // - 42 → Number subject
   * // - true → Bool subject
   * // - [1, 2] → List subject with Number elements
   * ```
   */
  function from<V>(pattern: Pattern<V>, options?: FromOptions): Pattern<Subject>;
}
