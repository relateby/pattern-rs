// ADT definitions for @relateby/graph.
// GraphClass and Substitution discriminated unions with smart constructors.

import type { Pattern } from "./interfaces.js";

// ---------------------------------------------------------------------------
// GraphClass discriminated union
// ---------------------------------------------------------------------------

/**
 * Discriminated union mirroring Haskell's GraphClass ADT.
 * Used as the class discriminant in transform callbacks.
 */
export type GraphClass =
  | { readonly tag: "GNode" }
  | { readonly tag: "GRelationship" }
  | { readonly tag: "GWalk" }
  | { readonly tag: "GAnnotation" }
  | { readonly tag: "GOther"; readonly extra: unknown };

export const GNode: GraphClass = { tag: "GNode" };
export const GRelationship: GraphClass = { tag: "GRelationship" };
export const GWalk: GraphClass = { tag: "GWalk" };
export const GAnnotation: GraphClass = { tag: "GAnnotation" };
export const GOther = (extra: unknown): GraphClass => ({ tag: "GOther", extra });

// ---------------------------------------------------------------------------
// Substitution discriminated union
// ---------------------------------------------------------------------------

/**
 * Governs how container integrity is maintained when filterGraph removes
 * an element from inside a walk or annotation.
 *
 * - DeleteContainer: remove the entire containing walk/annotation
 * - SpliceGap: remove the element and close the gap
 * - ReplaceWithSurrogate: replace the removed element with a surrogate pattern
 */
export type Substitution =
  | { readonly tag: "DeleteContainer" }
  | { readonly tag: "SpliceGap" }
  | { readonly tag: "ReplaceWithSurrogate"; readonly surrogate: Pattern<unknown> };

export const DeleteContainer: Substitution = { tag: "DeleteContainer" };
export const SpliceGap: Substitution = { tag: "SpliceGap" };
export const ReplaceWithSurrogate = <V>(surrogate: Pattern<V>): Substitution =>
  ({ tag: "ReplaceWithSurrogate", surrogate });
