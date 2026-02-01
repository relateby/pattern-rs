/**
 * TypeScript type definitions for unified gram-wasm package.
 *
 * This module provides a single entry point for:
 * - Pattern<V>: Generic recursive pattern structure
 * - Subject: Self-descriptive values with identity, labels, and properties
 * - Value: Property value types
 * - Gram: Gram notation serialization/parsing namespace
 *
 * For converting arbitrary JavaScript data to Pattern<Subject>, users should
 * use the Pattern and Subject constructors directly, or implement their own
 * conversion strategies. A future `pattern-io` module will provide standardized
 * conversion utilities for common formats (JSON, CSV, etc.).
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

// ============================================================================
// Gram Namespace (Serialization/Parsing)
// ============================================================================

/**
 * Gram namespace provides gram notation serialization and parsing.
 *
 * **Core Operations**:
 * - `stringify()` - Serialize Pattern<Subject> to gram notation
 * - `parse()` - Parse gram notation to Pattern<Subject>[]
 * - `parseOne()` - Parse gram notation to first Pattern<Subject> or null
 *
 * **Scope**: This module focuses solely on gram notation codec (serialization/deserialization).
 * Data transformation (converting JSON, CSV, or other formats to Pattern<Subject>) is out of
 * scope and will be handled by a future `pattern-io` module.
 *
 * **Usage**:
 * ```typescript
 * import { Pattern, Subject, Value, Gram } from 'gram';
 *
 * // Build a pattern using constructors
 * const subject = new Subject("alice", ["Person"], {
 *   name: Value.string("Alice")
 * });
 * const pattern = Pattern.point(subject);
 *
 * // Serialize to gram notation
 * const gram = Gram.stringify(pattern);
 * console.log(gram); // "(alice:Person {name: "Alice"})"
 *
 * // Parse back
 * const parsed = Gram.parseOne(gram);
 * console.log(parsed?.value.identity); // "alice"
 * ```
 */
export namespace Gram {
  /**
   * Serialize a Pattern<Subject> to gram notation.
   *
   * Converts a single pattern with Subject values into a human-readable
   * gram notation string. The pattern must have Subject values.
   *
   * @param pattern - A Pattern with Subject values
   * @returns Gram notation string representation
   * @throws Error if serialization fails
   *
   * @example
   * ```typescript
   * const subject = new Subject("alice", ["Person"], {
   *   name: Value.string("Alice")
   * });
   * const pattern = Pattern.point(subject);
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
   * // Multiple patterns - returns first
   * const first = Gram.parseOne("(alice:Person) (bob:Person)");
   * console.log(first?.value.identity); // "alice"
   * ```
   */
  function parseOne(gram: string): Pattern<Subject> | null;
}
