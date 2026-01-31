# TypeScript Type Definitions Contract

**Feature**: 027-wasm-pattern-typescript-parity  
**Date**: 2026-01-31

## Purpose

TypeScript declaration files (.d.ts) MUST describe the public WASM API and MUST include a generic `Pattern<V>` type so that `Pattern<Subject>` and other value types can be expressed (FR-010, FR-011). This document specifies the type shapes; the actual file(s) (e.g. `pattern_core.d.ts`) implement this contract.

## Generic Pattern&lt;V&gt;

```ts
export interface Pattern<V> {
  readonly value: V;
  readonly elements: Pattern<V>[];

  length(): number;
  size(): number;
  depth(): number;
  isAtomic(): boolean;
  values(): V[];

  anyValue(predicate: (v: V) => boolean): boolean;
  allValues(predicate: (v: V) => boolean): boolean;
  filter(predicate: (p: Pattern<V>) => boolean): Pattern<V>;
  findFirst(predicate: (p: Pattern<V>) => boolean): Pattern<V> | null;
  matches(other: Pattern<V>): boolean;
  contains(other: Pattern<V>): boolean;

  map<W>(fn: (v: V) => W): Pattern<W>;
  fold<T>(init: T, fn: (acc: T, v: V) => T): T;
  /** Paramorphism: bottom-up aggregation. fn(value, elementResults) -> result. Equivalent to Rust pattern-core para. */
  para<R>(fn: (value: V, elementResults: R[]) => R): R;

  combine(other: Pattern<V>): Pattern<V>;

  extract(): V;
  extend<W>(fn: (p: Pattern<V>) => W): Pattern<W>;
  depthAt(): Pattern<number>;
  sizeAt(): Pattern<number>;
  indicesAt(): Pattern<number[]>;

  /** Returns Either-like value (Right(undefined) on success, Left(ValidationError) on failure). Does NOT throw. Trivially convertible to effect-ts Either. */
  validate(rules: ValidationRules): Either<void, ValidationError>;
  analyzeStructure(): StructureAnalysis;
}
```

**Static constructors** (on Pattern or module):
- `Pattern.point<V>(value: V): Pattern<V>`
- `Pattern.of<V>(value: V): Pattern<V>`
- `Pattern.pattern<V>(value: V, elements: Pattern<V>[]): Pattern<V>`
- `Pattern.fromValues<V>(values: V[]): Pattern<V>[]`

## Subject

```ts
export interface Subject {
  readonly identity: Symbol;
  readonly labels: Set<string>;
  readonly properties: Record<string, Value>;

  addLabel(label: string): void;
  removeLabel(label: string): void;
  hasLabel(label: string): boolean;
  getProperty(name: string): Value | undefined;
  setProperty(name: string, value: Value): void;
  removeProperty(name: string): void;
}
```

Constructor: `Subject.new(identity: string, labels?: Iterable<string>, properties?: Record<string, Value>): Subject` (or equivalent).

## Symbol

```ts
export type Symbol = string;
// or
export interface Symbol { readonly value: string; }
```

Use whichever matches the runtime representation (string or wrapper).

## Value

Value MUST be a discriminated union or namespace so that TypeScript can narrow types and so that extractors are typed correctly.

**Option A – Union**:
```ts
export type Value =
  | { readonly kind: 'string'; asString(): string }
  | { readonly kind: 'int'; asInt(): number }
  | { readonly kind: 'decimal'; asDecimal(): number }
  | { readonly kind: 'boolean'; asBoolean(): boolean }
  | { readonly kind: 'symbol'; asSymbol(): string }
  | { readonly kind: 'array'; asArray(): Value[] }
  | { readonly kind: 'map'; asMap(): Record<string, Value> }
  | { readonly kind: 'range'; ... }
  | { readonly kind: 'measurement'; ... };
```

**Option B – Namespace with factories and type guard**:
```ts
export namespace Value {
  function string(s: string): Value;
  function int(i: number): Value;
  function decimal(n: number): Value;
  function boolean(b: boolean): Value;
  function symbol(s: string): Value;
  function array(items: Value[]): Value;
  function map(entries: Record<string, Value>): Value;
  function range(lower?: number, upper?: number): Value;
  function measurement(value: number, unit: string): Value;
}
export type Value = ... ; // union of variants
```

Each variant MUST expose the corresponding extractor(s); call sites use type guards or `kind` to narrow.

## ValidationRules

```ts
export interface ValidationRules {
  maxDepth?: number;
  maxElements?: number;
}
```

Constructor/factory as needed to match wasm-api.

## StructureAnalysis

```ts
export interface StructureAnalysis {
  readonly summary: string;
  readonly depthDistribution: number[];
  readonly elementCounts: number[];
  readonly nestingPatterns: string[];
}
```

## Result / Either types (fallible operations)

Fallible operations (e.g. `validate`) return a value trivially convertible to effect-ts Either. The .d.ts MUST declare a type compatible with Either (or use effect’s `Either` type if the package does not avoid a dependency; otherwise a minimal `Either<T, E>` interface matching effect-ts shape):

```ts
// Minimal shape (or re-export from "effect" if desired)
export type Either<T, E> =
  | { readonly _tag: 'Right'; readonly right: T }
  | { readonly _tag: 'Left'; readonly left: E };
```

`ValidationError` MUST be an interface so that `Either.left(error)` is well-typed:

```ts
export interface ValidationError {
  readonly message: string;
  readonly ruleViolated?: string;
}
```

## Requirements

- All public WASM exports that are callable from JS MUST have corresponding TypeScript signatures in the .d.ts file(s).
- Generic `Pattern<V>` MUST be the primary type for patterns; method return types MUST use it (e.g. `map<W>(fn: (v: V) => W): Pattern<W>`).
- No implementation details (Rust, WASM) in the type names or JSDoc beyond what is needed for usage. Types MUST be verifiable by running `tsc --noEmit` on a consumer file that uses `Pattern<Subject>` and the main operations.
