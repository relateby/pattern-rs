// subject.ts — Self-describing entity with identity, labels, and properties
//
// Extends Data.Class for structural equality via Equal.equals.
// Uses effect's HashSet and HashMap so Equal.equals works correctly
// (JavaScript's native Set/Map don't implement the Equal protocol).
// Builder methods are immutable (return new instances).

import { Data, HashMap, HashSet } from "effect"
import type { Value } from "./value.js"

export class Subject extends Data.Class<{
  readonly identity:   string
  readonly labels:     HashSet.HashSet<string>
  readonly properties: HashMap.HashMap<string, Value>
}> {
  static fromId(identity: string): Subject {
    return new Subject({ identity, labels: HashSet.empty(), properties: HashMap.empty() })
  }

  withLabel(label: string): Subject {
    return new Subject({
      ...this,
      labels: HashSet.add(this.labels, label),
    })
  }

  withProperty(name: string, value: Value): Subject {
    return new Subject({
      ...this,
      properties: HashMap.set(this.properties, name, value),
    })
  }
}
