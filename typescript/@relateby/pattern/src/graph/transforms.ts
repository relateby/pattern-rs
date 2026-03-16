import type { CategoryMappers, GraphQuery, GraphView, Pattern } from "./interfaces.js";
import type { GraphClass, Substitution } from "./adts.js";

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

export const mapGraph =
  <V>(mappers: CategoryMappers<V>) =>
  (view: GraphView<V>): GraphView<V> => ({
    viewQuery: view.viewQuery,
    viewElements: view.viewElements.map(
      ([cls, p]) => [cls, applyMapper(cls, p, mappers)] as const
    ),
  });

export const mapAllGraph =
  <V>(f: (p: Pattern<V>) => Pattern<V>) =>
  (view: GraphView<V>): GraphView<V> => ({
    viewQuery: view.viewQuery,
    viewElements: view.viewElements.map(([cls, p]) => [cls, f(p)] as const),
  });

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

    if (cls.tag === "GWalk" || cls.tag === "GAnnotation") {
      const removedChildren = p.elements.filter((e) =>
        removedIdentities.has(e.identity)
      );

      if (removedChildren.length > 0) {
        switch (subst.tag) {
          case "DeleteContainer":
            continue;
          case "SpliceGap": {
            const remaining = p.elements.filter(
              (e) => !removedIdentities.has(e.identity)
            );
            const spliced: Pattern<V> = { ...p, elements: remaining };
            result.push([cls, spliced] as const);
            break;
          }
          case "ReplaceWithSurrogate": {
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

export const filterGraph =
  <V>(
    keep: (cls: GraphClass, p: Pattern<V>) => boolean,
    subst: Substitution
  ) =>
  (view: GraphView<V>): GraphView<V> => {
    const removedIdentities = new Set<string | undefined>();
    for (const [cls, p] of view.viewElements) {
      if (!keep(cls, p)) {
        removedIdentities.add(p.identity);
      }
    }

    const filtered = applySubstitution(view.viewElements, removedIdentities, subst);

    return {
      viewQuery: view.viewQuery,
      viewElements: filtered,
    };
  };

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

export const mapWithContext =
  <V>(f: (query: GraphQuery<V>, p: Pattern<V>) => Pattern<V>) =>
  (view: GraphView<V>): GraphView<V> => ({
    viewQuery: view.viewQuery,
    viewElements: view.viewElements.map(
      ([cls, p]) => [cls, f(view.viewQuery, p)] as const
    ),
  });

export const paraGraph =
  <V, R>(
    f: (query: GraphQuery<V>, p: Pattern<V>, subResults: readonly R[]) => R
  ) =>
  (view: GraphView<V>): ReadonlyMap<string, R> => {
    const topoOrder = view.viewQuery.nodes
      ? view.viewQuery.nodes()
      : [];

    const results = new Map<string, R>();
    const processOrder = topoOrder.length > 0
      ? topoOrder
      : view.viewElements.map(([, p]) => p);

    for (const p of processOrder) {
      if (p.identity === undefined) continue;
      const subResults: R[] = p.elements
        .filter((e) => e.identity !== undefined && results.has(e.identity!))
        .map((e) => results.get(e.identity!)!);

      const result = f(view.viewQuery, p, subResults);
      results.set(p.identity, result);
    }

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

export const paraGraphFixed =
  <V, R>(
    conv: (prev: R, next: R) => boolean,
    f: (query: GraphQuery<V>, p: Pattern<V>, subResults: readonly R[]) => R,
    init: R
  ) =>
  (view: GraphView<V>): ReadonlyMap<string, R> => {
    let current = new Map<string, R>();
    for (const [, p] of view.viewElements) {
      if (p.identity !== undefined) {
        current.set(p.identity, init);
      }
    }

    const runPara = paraGraph(f);

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

export const unfoldGraph =
  <S, V>(
    expand: (seed: S) => readonly Pattern<V>[],
    build: (patterns: readonly Pattern<V>[]) => import("./interfaces.js").PatternGraph<V>
  ) =>
  (seeds: readonly S[]): import("./interfaces.js").PatternGraph<V> => {
    const allPatterns = seeds.flatMap((seed) => [...expand(seed)]);
    return build(allPatterns);
  };
