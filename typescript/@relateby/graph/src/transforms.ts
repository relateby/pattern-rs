// Pure TypeScript graph transform functions for @relateby/graph.
// All transforms are curried to enable point-free pipeline composition.
// No runtime dependency on WASM.

import type { Pattern, GraphQuery, GraphView, CategoryMappers } from "./interfaces.js";
import type { GraphClass, Substitution } from "./adts.js";

// ---------------------------------------------------------------------------
// Internal dispatch helper
// ---------------------------------------------------------------------------

function applyMapper<V>(
  cls: GraphClass,
  p: Pattern<V>,
  mappers: CategoryMappers<V>
): Pattern<V> {
  switch (cls.tag) {
    case "GNode":
      return mappers.mapNode?.(p) ?? p;
    case "GRelationship":
      return mappers.mapRelationship?.(p) ?? p;
    case "GWalk":
      return mappers.mapWalk?.(p) ?? p;
    case "GAnnotation":
      return mappers.mapAnnotation?.(p) ?? p;
    case "GOther":
      return mappers.mapOther?.(cls, p) ?? p;
  }
}

// ---------------------------------------------------------------------------
// mapGraph
// ---------------------------------------------------------------------------

/**
 * Transform each element of a graph view using separate mapping functions per class.
 * Unspecified classes are passed through unchanged (identity function).
 *
 * Curried: mapGraph(mappers)(view)
 *
 * Haskell reference: mapGraph classifier fNode fRel fWalk fAnnot fOther view
 */
export const mapGraph =
  <V>(mappers: CategoryMappers<V>) =>
  (view: GraphView<V>): GraphView<V> => ({
    viewQuery: view.viewQuery,
    viewElements: view.viewElements.map(
      ([cls, p]) => [cls, applyMapper(cls, p, mappers)] as const
    ),
  });

// ---------------------------------------------------------------------------
// mapAllGraph
// ---------------------------------------------------------------------------

/**
 * Transform every element of a graph view with a single uniform function,
 * regardless of class.
 *
 * Curried: mapAllGraph(f)(view)
 *
 * Haskell reference: mapAllGraph f view
 */
export const mapAllGraph =
  <V>(f: (p: Pattern<V>) => Pattern<V>) =>
  (view: GraphView<V>): GraphView<V> => ({
    viewQuery: view.viewQuery,
    viewElements: view.viewElements.map(([cls, p]) => [cls, f(p)] as const),
  });

// ---------------------------------------------------------------------------
// filterGraph
// ---------------------------------------------------------------------------

/**
 * Apply Substitution strategy when an element inside a walk/annotation is removed.
 */
function applySubstitution<V>(
  elements: ReadonlyArray<readonly [GraphClass, Pattern<V>]>,
  removedIdentities: Set<string | undefined>,
  subst: Substitution
): Array<readonly [GraphClass, Pattern<V>]> {
  const result: Array<readonly [GraphClass, Pattern<V>]> = [];

  for (const [cls, p] of elements) {
    if (removedIdentities.has(p.identity)) {
      continue;
    }

    // For walks and annotations, check if any child element was removed
    if (cls.tag === "GWalk" || cls.tag === "GAnnotation") {
      const removedChildren = p.elements.filter((e) =>
        removedIdentities.has(e.identity)
      );

      if (removedChildren.length > 0) {
        switch (subst.tag) {
          case "DeleteContainer":
            // Remove the entire container
            continue;
          case "SpliceGap": {
            // Remove removed children, keep remaining
            const remaining = p.elements.filter(
              (e) => !removedIdentities.has(e.identity)
            );
            const spliced: Pattern<V> = { ...p, elements: remaining };
            result.push([cls, spliced] as const);
            break;
          }
          case "ReplaceWithSurrogate": {
            // Replace removed children with surrogate
            const replaced = p.elements.map((e) =>
              removedIdentities.has(e.identity)
                ? (subst.surrogate as Pattern<V>)
                : e
            );
            const substituted: Pattern<V> = { ...p, elements: replaced };
            result.push([cls, substituted] as const);
            break;
          }
        }
      } else {
        result.push([cls, p] as const);
      }
    } else {
      result.push([cls, p] as const);
    }
  }

  return result;
}

/**
 * Remove elements from a graph view that do not satisfy the predicate.
 * The substitution strategy governs how container integrity is maintained
 * when an element inside a walk or annotation is removed.
 *
 * Curried: filterGraph(keep, subst)(view)
 *
 * Haskell reference: filterGraph classifier keep subst view
 */
export const filterGraph =
  <V>(
    keep: (cls: GraphClass, p: Pattern<V>) => boolean,
    subst: Substitution
  ) =>
  (view: GraphView<V>): GraphView<V> => {
    // First pass: identify removed elements
    const removedIdentities = new Set<string | undefined>();
    for (const [cls, p] of view.viewElements) {
      if (!keep(cls, p)) {
        removedIdentities.add(p.identity);
      }
    }

    // Second pass: apply substitution for containers with removed children
    const filtered = applySubstitution(view.viewElements, removedIdentities, subst);

    return {
      viewQuery: view.viewQuery,
      viewElements: filtered,
    };
  };

// ---------------------------------------------------------------------------
// foldGraph
// ---------------------------------------------------------------------------

/**
 * Reduce a graph view to a single value.
 * The (empty, combine) pair mirrors Haskell's Monoid constraint.
 *
 * Curried: foldGraph(f, empty, combine)(view)
 *
 * Haskell reference: foldGraph classifier f mempty mappend view
 */
export const foldGraph =
  <V, M>(
    f: (cls: GraphClass, p: Pattern<V>) => M,
    empty: M,
    combine: (a: M, b: M) => M
  ) =>
  (view: GraphView<V>): M =>
    view.viewElements.reduce(
      (acc, [cls, p]) => combine(acc, f(cls, p)),
      empty
    );

// ---------------------------------------------------------------------------
// mapWithContext
// ---------------------------------------------------------------------------

/**
 * Transform each element while receiving a snapshot GraphQuery.
 * The snapshot reflects the graph state at the start of the transformation;
 * later elements do not see mutations from earlier ones.
 *
 * Curried: mapWithContext(f)(view)
 *
 * Haskell reference: mapWithContext classifier f view
 */
export const mapWithContext =
  <V>(f: (query: GraphQuery<V>, p: Pattern<V>) => Pattern<V>) =>
  (view: GraphView<V>): GraphView<V> => ({
    viewQuery: view.viewQuery,
    viewElements: view.viewElements.map(
      ([cls, p]) => [cls, f(view.viewQuery, p)] as const
    ),
  });

// ---------------------------------------------------------------------------
// paraGraph
// ---------------------------------------------------------------------------

/**
 * Bottom-up structural fold. Each element receives the pre-computed results
 * of its structural dependencies (sub-results).
 *
 * Calls view.viewQuery's underlying PatternGraph.topoSort() once (one WASM
 * crossing when backed by NativePatternGraph) to determine processing order,
 * then iterates entirely in TypeScript.
 *
 * Returns ReadonlyMap<string, R> mapping identity string → result.
 *
 * Curried: paraGraph(f)(view)
 *
 * Haskell reference: paraGraph classifier f view  →  Map (Id v) r
 */
export const paraGraph =
  <V, R>(
    f: (query: GraphQuery<V>, p: Pattern<V>, subResults: readonly R[]) => R
  ) =>
  (view: GraphView<V>): ReadonlyMap<string, R> => {
    // Get topological order (one WASM crossing if backed by NativePatternGraph)
    const topoOrder = view.viewQuery.nodes
      ? view.viewQuery.nodes()
      : [];

    // Build identity → element map from viewElements
    const elemMap = new Map<string, readonly [GraphClass, Pattern<V>]>();
    for (const [cls, p] of view.viewElements) {
      if (p.identity !== undefined) {
        elemMap.set(p.identity, [cls, p]);
      }
    }

    const results = new Map<string, R>();

    // Process in topological order (bottom-up)
    const processOrder = topoOrder.length > 0
      ? topoOrder
      : view.viewElements.map(([, p]) => p);

    for (const p of processOrder) {
      if (p.identity === undefined) continue;
      // Collect sub-results for direct children
      const subResults: R[] = p.elements
        .filter((e) => e.identity !== undefined && results.has(e.identity!))
        .map((e) => results.get(e.identity!)!);

      const result = f(view.viewQuery, p, subResults);
      results.set(p.identity, result);
    }

    // Process any remaining elements not in topoOrder
    for (const [, p] of view.viewElements) {
      if (p.identity !== undefined && !results.has(p.identity)) {
        const subResults: R[] = p.elements
          .filter((e) => e.identity !== undefined && results.has(e.identity!))
          .map((e) => results.get(e.identity!)!);
        results.set(p.identity, f(view.viewQuery, p, subResults));
      }
    }

    return results;
  };

// ---------------------------------------------------------------------------
// paraGraphFixed
// ---------------------------------------------------------------------------

/**
 * Iterate paraGraph until a convergence predicate is satisfied.
 * init is the initial result for all elements before the first pass.
 * conv(prev, next) returns true when the result has converged.
 *
 * Curried: paraGraphFixed(conv, f, init)(view)
 *
 * Haskell reference: paraGraphFixed classifier conv f init view
 */
export const paraGraphFixed =
  <V, R>(
    conv: (prev: R, next: R) => boolean,
    f: (query: GraphQuery<V>, p: Pattern<V>, subResults: readonly R[]) => R,
    init: R
  ) =>
  (view: GraphView<V>): ReadonlyMap<string, R> => {
    // Initialize all elements with init value
    let current = new Map<string, R>();
    for (const [, p] of view.viewElements) {
      if (p.identity !== undefined) {
        current.set(p.identity, init);
      }
    }

    const runPara = paraGraph(f);

    // Iterate until convergence
    let converged = false;
    while (!converged) {
      const next = runPara(view) as Map<string, R>;

      converged = true;
      for (const [id, nextVal] of next) {
        const prevVal = current.get(id) ?? init;
        if (!conv(prevVal, nextVal)) {
          converged = false;
          break;
        }
      }

      current = next;
    }

    return current;
  };

// ---------------------------------------------------------------------------
// unfoldGraph
// ---------------------------------------------------------------------------

/**
 * Expand a set of seed values into a PatternGraph.
 * expand(seed) returns the patterns produced by that seed.
 * build(patterns) constructs the PatternGraph from all expanded patterns.
 *
 * Curried: unfoldGraph(expand, build)(seeds)
 *
 * Haskell reference: unfoldGraph expand build seeds
 */
export const unfoldGraph =
  <S, V>(
    expand: (seed: S) => readonly Pattern<V>[],
    build: (patterns: readonly Pattern<V>[]) => import("./interfaces.js").PatternGraph<V>
  ) =>
  (seeds: readonly S[]): import("./interfaces.js").PatternGraph<V> => {
    const allPatterns = seeds.flatMap((seed) => [...expand(seed)]);
    return build(allPatterns);
  };
