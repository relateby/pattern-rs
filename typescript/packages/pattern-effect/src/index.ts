// @relateby/pattern-effect — Effect interop for @relateby/pattern
//
// Provides Effect-wrapped Gram operations and utilities for consumers
// who want full Effect ecosystem integration. Subject types are re-exported
// directly — they are already Effect-compatible (same tagged-union shape).

export { Subject } from "@relateby/pattern"
export type { SubjectLike } from "@relateby/pattern"

export type { PropMap } from "./props.js"
export { fromSubjectProps } from "./props.js"

export { Gram } from "./gram.js"
