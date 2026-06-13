// fp.ts — Inline pipe and Option utilities
//
// Option<T> uses the same tagged-union shape as Effect's Option so that
// @relateby/pattern-effect can bridge to Effect with zero conversion.

export type Option<T> =
  | { readonly _tag: "Some"; readonly value: T }
  | { readonly _tag: "None" }

export const Option = {
  some: <T>(value: T): Option<T> => ({ _tag: "Some", value }),
  none: <T = never>(): Option<T> => ({ _tag: "None" }),
  isSome: <T>(self: Option<T>): self is { _tag: "Some"; value: T } => self._tag === "Some",
  isNone: <T>(self: Option<T>): self is { _tag: "None" } => self._tag === "None",
  getOrUndefined: <T>(self: Option<T>): T | undefined =>
    self._tag === "Some" ? self.value : undefined,
  getOrThrow: <T>(self: Option<T>): T => {
    if (self._tag === "Some") return self.value
    throw new Error("Option.getOrThrow called on None")
  },
  map:
    <A, B>(fn: (a: A) => B) =>
    (self: Option<A>): Option<B> =>
      self._tag === "Some" ? { _tag: "Some", value: fn(self.value) } : { _tag: "None" },
  flatMap:
    <A, B>(fn: (a: A) => Option<B>) =>
    (self: Option<A>): Option<B> =>
      self._tag === "Some" ? fn(self.value) : { _tag: "None" },
  getOrElse:
    <A>(fallback: () => A) =>
    (self: Option<A>): A =>
      self._tag === "Some" ? self.value : fallback(),
  orElse: <A>(self: Option<A>, that: () => Option<A>): Option<A> =>
    self._tag === "Some" ? self : that(),
}

// pipe — same overload shape as Effect's pipe
export function pipe<A>(a: A): A
export function pipe<A, B>(a: A, ab: (a: A) => B): B
export function pipe<A, B, C>(a: A, ab: (a: A) => B, bc: (b: B) => C): C
export function pipe<A, B, C, D>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D): D
export function pipe<A, B, C, D, E>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E): E
export function pipe<A, B, C, D, E, F>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E, ef: (e: E) => F): F
export function pipe(a: unknown, ...fns: Array<(x: unknown) => unknown>): unknown {
  return fns.reduce((v, fn) => fn(v), a)
}
