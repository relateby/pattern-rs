import type { Pattern } from "./interfaces.js";

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

export type Substitution =
  | { readonly tag: "DeleteContainer" }
  | { readonly tag: "SpliceGap" }
  | { readonly tag: "ReplaceWithSurrogate"; readonly surrogate: Pattern<unknown> };

export const DeleteContainer: Substitution = { tag: "DeleteContainer" };
export const SpliceGap: Substitution = { tag: "SpliceGap" };
export const ReplaceWithSurrogate = <V>(surrogate: Pattern<V>): Substitution =>
  ({ tag: "ReplaceWithSurrogate", surrogate });
