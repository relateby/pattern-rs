// subject.ts — Self-describing entity with identity, labels, and properties
//
// SubjectLike is a plain interface using native JS types.
// Subject uses identity-based equality: two subjects are equal iff their
// identity strings are equal. Use Value.equals() / matches() for structural
// comparison of properties or patterns.
//
// Public construction: Subject.fromId(id) or Subject.from(subjectLike)
// then .withLabel() / .withProperty() for immutable mutation.

import type { Value } from "./value.js"

export interface SubjectLike {
  readonly identity:   string
  readonly labels:     ReadonlyArray<string>
  readonly properties: Readonly<Record<string, Value>>
}

export class Subject implements SubjectLike {
  readonly identity: string
  private readonly _labels: ReadonlyArray<string>
  private readonly _properties: Readonly<Record<string, Value>>

  constructor(
    identity: string,
    labels: ReadonlyArray<string>,
    properties: Readonly<Record<string, Value>>,
  ) {
    this.identity = identity
    this._labels = labels
    this._properties = properties
  }

  static fromId(identity: string): Subject {
    return new Subject(identity, [], {})
  }

  static from({ identity, labels, properties }: SubjectLike): Subject {
    return new Subject(identity, [...labels], { ...properties })
  }

  get labels(): ReadonlyArray<string> {
    return this._labels
  }

  get properties(): Readonly<Record<string, Value>> {
    return this._properties
  }

  withLabel(label: string): Subject {
    if (this._labels.includes(label)) return this
    return new Subject(this.identity, [...this._labels, label], this._properties)
  }

  withProperty(name: string, value: Value): Subject {
    return new Subject(this.identity, this._labels, { ...this._properties, [name]: value })
  }

  /** Identity equality: two subjects are equal when their identity strings match. */
  equals(other: Subject): boolean {
    return this.identity === other.identity
  }

  /** Merge labels and properties from other into this subject (this wins on conflicts). */
  merge(other: Subject): Subject {
    const merged = new Set([...this._labels, ...other._labels])
    return new Subject(
      this.identity,
      [...merged],
      { ...other._properties, ...this._properties },
    )
  }
}
