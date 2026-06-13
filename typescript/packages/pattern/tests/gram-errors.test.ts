import { describe, expect, it } from "vitest"
import { Gram } from "../src/gram.js"
import { GramParseError } from "../src/errors.js"
import { Pattern } from "../src/pattern.js"
import { Subject } from "../src/subject.js"
import { Value } from "../src/value.js"

describe("Gram errors", () => {
  it("rejects with GramParseError on invalid input", async () => {
    const input = "not valid gram ##!!"
    await expect(Gram.parse(input)).rejects.toBeInstanceOf(GramParseError)
    await expect(Gram.parse(input)).rejects.toMatchObject({ input })
    const err = await Gram.parse(input).catch(e => e as GramParseError)
    expect(String(err.cause)).not.toHaveLength(0)
  })

  it("resolves with patterns on valid input", async () => {
    const result = await Gram.parse("(alice:Person)")
    expect(result).toHaveLength(1)
    expect(result[0]?.value.identity).toBe("alice")
  })

  it("validate resolves for valid input and rejects for invalid input", async () => {
    await expect(Gram.validate("(alice:Person)")).resolves.toBeUndefined()
    const err = await Gram.validate("not valid gram ##!!").catch(e => e as GramParseError)
    expect(err).toBeInstanceOf(GramParseError)
    expect(err.input).toBe("not valid gram ##!!")
  })

  it("rejects with GramParseError on unsupported null stringify values", async () => {
    const pattern = Pattern.point(
      Subject.fromId("alice").withProperty("nickname", Value.Null())
    )
    const err = await Gram.stringify([pattern]).catch(e => e as GramParseError)
    expect(err).toBeInstanceOf(GramParseError)
    expect(String(err.cause)).toContain("not representable")
  })
})
