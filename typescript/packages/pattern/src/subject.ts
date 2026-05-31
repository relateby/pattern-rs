// subject.ts — Self-describing entity with identity, labels, and properties
//
// SubjectLike is a plain interface using native JS types — no Effect dependency.
// Subject is the Effect-backed implementation using HashSet/HashMap for structural
// equality via Equal.equals. Internal fields are prefixed with _ to allow
// the native-typed getters to implement SubjectLike without name collision.
// Builder methods are immutable (return new instances).

import { Data, HashMap, HashSet } from "effect"
import type { Value } from "./value.js"

export interface SubjectLike {
  readonly identity:   string
  readonly labels:     ReadonlyArray<string>
  readonly properties: Readonly<Record<string, Value>>
}

export class Subject extends Data.Class<{
  readonly identity:    string
  readonly _labels:     HashSet.HashSet<string>
  readonly _properties: HashMap.HashMap<string, Value>
}> implements SubjectLike {
  static fromId(identity: string): Subject {
    return new Subject({ identity, _labels: HashSet.empty(), _properties: HashMap.empty() })
  }

  get labels(): ReadonlyArray<string> {
    return [...HashSet.values(this._labels)]
  }

  get properties(): Readonly<Record<string, Value>> {
    return Object.fromEntries(HashMap.entries(this._properties))
  }

  withLabel(label: string): Subject {
    return new Subject({ ...this, _labels: HashSet.add(this._labels, label) })
  }

  withProperty(name: string, value: Value): Subject {
    return new Subject({ ...this, _properties: HashMap.set(this._properties, name, value) })
  }
}
