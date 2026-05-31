// @relateby/pattern-effect — Effect interop for @relateby/pattern
//
// Re-exports the Effect-backed Subject and SubjectLike interface for
// consumers who want structural equality via Equal.equals. Also provides
// fromSubjectProps to flatten Subject properties into plain JS values for
// use with schema libraries (Effect Schema, Zod, Valibot, etc.).

export { Subject } from "@relateby/pattern"
export type { SubjectLike } from "@relateby/pattern"

export type { PropMap } from "./props.js"
export { fromSubjectProps } from "./props.js"
