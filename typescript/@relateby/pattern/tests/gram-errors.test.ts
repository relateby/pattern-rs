import { Effect, Either } from "effect"
import { describe, expect, it } from "vitest"
import { Gram } from "../src/gram.js"
import { GramParseError } from "../src/errors.js"

describe("Gram errors", () => {
  it("returns an Effect failure with GramParseError on invalid input", async () => {
    const input = "not valid gram ##!!"
    const result = await Effect.runPromise(Effect.either(Gram.parse(input)))

    expect(Either.isLeft(result)).toBe(true)
    if (Either.isLeft(result)) {
      expect(result.left).toBeInstanceOf(GramParseError)
      expect(result.left.input).toBe(input)
      expect(String(result.left.cause)).not.toHaveLength(0)
    }
  })

  it("returns an Effect success on valid input", async () => {
    const result = await Effect.runPromise(Effect.either(Gram.parse("(alice:Person)")))

    expect(Either.isRight(result)).toBe(true)
    if (Either.isRight(result)) {
      expect(result.right).toHaveLength(1)
      expect(result.right[0]?.value.identity).toBe("alice")
    }
  })

  it("validate succeeds for valid input and fails for invalid input", async () => {
    const valid = await Effect.runPromise(Effect.either(Gram.validate("(alice:Person)")))
    const invalid = await Effect.runPromise(Effect.either(Gram.validate("not valid gram ##!!")))

    expect(Either.isRight(valid)).toBe(true)
    expect(Either.isLeft(invalid)).toBe(true)
    if (Either.isLeft(invalid)) {
      expect(invalid.left).toBeInstanceOf(GramParseError)
      expect(invalid.left.input).toBe("not valid gram ##!!")
    }
  })
})
