# TypeScript Definitions Contract: Unified Gram Package

**Feature**: 028-unified-gram-wasm  
**Date**: 2026-01-31

## Purpose

TypeScript declaration files MUST describe the unified public API: Pattern&lt;V&gt;, Subject, Value, and the **Gram** namespace. Consumers get full type safety with a single import (e.g. `import { Pattern, Subject, Value, Gram } from 'gram'`). Pattern and Subject types align with 027 typescript-types.md; this contract adds **Subject.fromValue**, **Gram**, and **FromOptions** (same shape as FromValueOptions where applicable).

## Single entry export

All public types and the Gram namespace MUST be exportable from one module (e.g. index.d.ts or gram.d.ts):

```ts
export { Pattern } from './pattern';
export { Subject } from './subject';
export { Value } from './value';
export { Gram } from './gram';
export type { ValidationRules, StructureAnalysis, Either, ValidationError } from './types';
```

(Exact file names may vary; the logical surface is one import.)

## Gram namespace

```ts
export namespace Gram {
  /** Serialize a Pattern<Subject> to gram notation */
  function stringify(pattern: Pattern<Subject>): string;

  /** Parse gram notation into Pattern<Subject>[]; empty/whitespace returns [] */
  function parse(gram: string): Pattern<Subject>[];

  /** Parse gram notation; returns first pattern or null */
  function parseOne(gram: string): Pattern<Subject> | null;

  /** Convert any JS value to Pattern<Subject> for gram serialization */
  function from(value: unknown): Pattern<Subject>;
}
```

## Subject.fromValue

```ts
// On Subject (static or namespace):
function fromValue(value: unknown, options?: FromValueOptions): Subject;
```

## FromValueOptions

```ts
export interface FromValueOptions {
  /** Label to apply to converted subjects (default: type-appropriate, e.g. "String", "Number", "Bool") */
  label?: string;
  /** Property name for the original value (default: "value") */
  valueProperty?: string;
  /** Index for auto-generated identity (default: 0, generates "_0", "_1", etc.) */
  index?: number;
}
```

**Note**: Custom identity generator functions are not yet supported. Identities are auto-generated as `_${index}`.

## Pattern&lt;Subject&gt; for stringify/parse

- `stringify` and `parse`/`parseOne` use `Pattern<Subject>`. TypeScript MUST express that stringify accepts only `Pattern<Subject>` (not generic `Pattern<V>`) so that from() is required for other value types.

## Parity

- Pattern, Subject, Value type shapes MUST match 027 typescript-types.md where re-exported. Subject.fromValue (and FromValueOptions), Gram, and FromOptions are net-new for 028.
