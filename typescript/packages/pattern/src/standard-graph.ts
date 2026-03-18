import { Effect, Option, pipe } from "effect"
import { Gram } from "./gram.js"
import { GramParseError } from "./errors.js"
import { Pattern } from "./pattern.js"
import { Subject } from "./subject.js"

export interface StandardRelationship {
  readonly pattern: Pattern<Subject>
  readonly source: string
  readonly target: string
}

export class StandardGraph {
  private readonly _nodes = new Map<string, Pattern<Subject>>()
  private readonly _relationships = new Map<string, StandardRelationship>()
  private readonly _annotations = new Map<string, Pattern<Subject>>()
  private readonly _walks = new Map<string, Pattern<Subject>>()
  private readonly _other = new Map<string, Pattern<Subject>>()

  static fromPatterns(patterns: ReadonlyArray<Pattern<Subject>>): StandardGraph {
    const graph = new StandardGraph()
    for (const pattern of patterns) {
      graph.ingest(pattern)
    }
    return graph
  }

  static fromGram(input: string): Effect.Effect<StandardGraph, GramParseError> {
    return pipe(Gram.parse(input), Effect.map(StandardGraph.fromPatterns))
  }

  get nodeCount(): number {
    return this._nodes.size
  }

  get relationshipCount(): number {
    return this._relationships.size
  }

  get annotationCount(): number {
    return this._annotations.size
  }

  get walkCount(): number {
    return this._walks.size
  }

  get isEmpty(): boolean {
    return this.nodeCount === 0 &&
      this.relationshipCount === 0 &&
      this.annotationCount === 0 &&
      this.walkCount === 0 &&
      this._other.size === 0
  }

  get hasConflicts(): boolean {
    return false
  }

  nodes(): IterableIterator<[string, Pattern<Subject>]> {
    return this._nodes.entries()
  }

  relationships(): IterableIterator<[string, StandardRelationship]> {
    return this._relationships.entries()
  }

  annotations(): IterableIterator<[string, Pattern<Subject>]> {
    return this._annotations.entries()
  }

  walks(): IterableIterator<[string, Pattern<Subject>]> {
    return this._walks.entries()
  }

  other(): ReadonlyArray<Pattern<Subject>> {
    return [...this._other.values()]
  }

  node(id: string): Option.Option<Pattern<Subject>> {
    return liftOption(this._nodes.get(id))
  }

  relationship(id: string): Option.Option<StandardRelationship> {
    return liftOption(this._relationships.get(id))
  }

  annotation(id: string): Option.Option<Pattern<Subject>> {
    return liftOption(this._annotations.get(id))
  }

  walk(id: string): Option.Option<Pattern<Subject>> {
    return liftOption(this._walks.get(id))
  }

  source(id: string): Option.Option<Pattern<Subject>> {
    return pipe(
      this.relationship(id),
      Option.flatMap((relationship) => this.node(relationship.source))
    )
  }

  target(id: string): Option.Option<Pattern<Subject>> {
    return pipe(
      this.relationship(id),
      Option.flatMap((relationship) => this.node(relationship.target))
    )
  }

  neighbors(nodeId: string): ReadonlyArray<Pattern<Subject>> {
    const neighbors: Array<Pattern<Subject>> = []
    for (const relationship of this._relationships.values()) {
      if (relationship.source === nodeId) {
        const target = this._nodes.get(relationship.target)
        if (target) neighbors.push(target)
      } else if (relationship.target === nodeId) {
        const source = this._nodes.get(relationship.source)
        if (source) neighbors.push(source)
      }
    }
    return neighbors
  }

  degree(nodeId: string): number {
    return this.neighbors(nodeId).length
  }

  private ingest(pattern: Pattern<Subject>): void {
    switch (classifyPattern(pattern)) {
      case "node":
        this._nodes.set(pattern.value.identity, pattern)
        break
      case "relationship": {
        const [sourcePattern, targetPattern] = pattern.elements
        if (sourcePattern && targetPattern) {
          this.ingest(sourcePattern)
          this.ingest(targetPattern)
          this._relationships.set(pattern.value.identity, {
            pattern,
            source: sourcePattern.value.identity,
            target: targetPattern.value.identity,
          })
        }
        break
      }
      case "walk":
        for (const element of pattern.elements) {
          this.ingest(element)
        }
        this._walks.set(pattern.value.identity, pattern)
        break
      case "annotation": {
        const [inner] = pattern.elements
        if (inner) {
          this.ingest(inner)
        }
        this._annotations.set(pattern.value.identity, pattern)
        break
      }
      case "other":
        this._other.set(pattern.value.identity, pattern)
        break
    }
  }
}

type PatternClass = "node" | "relationship" | "annotation" | "walk" | "other"

function classifyPattern(pattern: Pattern<Subject>): PatternClass {
  if (pattern.elements.length === 0) {
    return "node"
  }
  if (pattern.elements.length === 1) {
    return "annotation"
  }
  if (pattern.elements.length === 2 && pattern.elements.every(isNodeLike)) {
    return "relationship"
  }
  if (pattern.elements.length >= 1 && pattern.elements.every(isRelationshipLike) && isValidWalk(pattern.elements)) {
    return "walk"
  }
  return "other"
}

function isNodeLike(pattern: Pattern<Subject>): boolean {
  return pattern.elements.length === 0
}

function isRelationshipLike(pattern: Pattern<Subject>): boolean {
  return pattern.elements.length === 2 && pattern.elements.every(isNodeLike)
}

function isValidWalk(patterns: ReadonlyArray<Pattern<Subject>>): boolean {
  if (patterns.length === 0) {
    return false
  }

  let active: Array<Pattern<Subject>> = []
  for (const pattern of patterns) {
    const [left, right] = pattern.elements
    if (!left || !right) {
      return false
    }

    if (active.length === 0) {
      active = [left, right]
      continue
    }

    const next: Array<Pattern<Subject>> = []
    if (active.some((candidate) => candidate.value.identity === left.value.identity)) {
      next.push(right)
    }
    if (active.some((candidate) => candidate.value.identity === right.value.identity)) {
      next.push(left)
    }
    active = next
  }

  return active.length > 0
}

function liftOption<A>(value: A | undefined): Option.Option<A> {
  return value === undefined ? Option.none() : Option.some(value)
}
