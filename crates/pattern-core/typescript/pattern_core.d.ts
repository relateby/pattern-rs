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
 * Rules for pattern validation.
 */
export interface ValidationRules {
  readonly maxDepth?: number;
  readonly maxElements?: number;
  readonly maxSize?: number;
}

/**
 * Result of pattern structure analysis.
 */
export interface StructureAnalysis {
  readonly summary: string;
  readonly depthDistribution: number[];
  readonly elementCounts: number[];
  readonly nestingPatterns: string[];
}

// ============================================================================
// Placeholder Types (to be expanded in Phase 3+)
// ============================================================================

// TODO: T018 - Add Pattern<V> interface and static constructors
// TODO: T019 - Add Subject, Value, Symbol types and Value factories
// TODO: T020 - Add full type definitions for all operations
