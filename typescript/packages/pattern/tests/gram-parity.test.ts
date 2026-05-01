import { Effect, Either, HashMap, Option } from "effect"
import { describe, expect, it } from "vitest"
import { Gram } from "../src/gram.js"
import { GramParseError } from "../src/errors.js"

describe("Gram parity", () => {
  describe("parseWithHeader", () => {
    it("splits an optional header record from patterns", async () => {
      const { header, patterns } = await Effect.runPromise(
        Gram.parseWithHeader("{version: 1} (a)-->(b)")
      )
      expect(header).toEqual({ version: 1 })
      expect(patterns).toHaveLength(1)
    })

    it("returns undefined header when no header is present", async () => {
      const { header, patterns } = await Effect.runPromise(
        Gram.parseWithHeader("(a)-->(b)")
      )
      expect(header).toBeUndefined()
      expect(patterns).toHaveLength(1)
    })

    it("handles header-only input (no patterns)", async () => {
      const { header, patterns } = await Effect.runPromise(
        Gram.parseWithHeader("{version: 1}")
      )
      expect(header).toEqual({ version: 1 })
      expect(patterns).toHaveLength(0)
    })

    it("handles empty input", async () => {
      const { header, patterns } = await Effect.runPromise(
        Gram.parseWithHeader("")
      )
      expect(header).toBeUndefined()
      expect(patterns).toHaveLength(0)
    })

    it("fails with GramParseError on invalid input", async () => {
      const result = await Effect.runPromise(
        Effect.either(Gram.parseWithHeader("not valid gram ##!!"))
      )
      expect(Either.isLeft(result)).toBe(true)
      if (Either.isLeft(result)) {
        expect(result.left).toBeInstanceOf(GramParseError)
        expect(result.left.input).toBe("not valid gram ##!!")
      }
    })
  })

  describe("stringifyWithHeader", () => {
    it("includes header record in output", async () => {
      const patterns = await Effect.runPromise(Gram.parse("(a)-->(b)"))
      const gram = await Effect.runPromise(
        Gram.stringifyWithHeader({ version: 1 }, patterns)
      )
      expect(gram).toContain("{")
      expect(gram).toContain("version")
    })

    it("undefined header produces same output as plain stringify", async () => {
      const patterns = await Effect.runPromise(Gram.parse("(a)-->(b)"))
      const plain = await Effect.runPromise(Gram.stringify(patterns))
      const withUndefined = await Effect.runPromise(
        Gram.stringifyWithHeader(undefined, patterns)
      )
      expect(withUndefined).toBe(plain)
    })

    it("handles empty patterns with a header", async () => {
      const gram = await Effect.runPromise(
        Gram.stringifyWithHeader({ version: 1 }, [])
      )
      expect(gram).toContain("version")
    })

    it("full round-trip: stringify_with_header → parse_with_header", async () => {
      const originalHeader = { version: 2, schema: "test" }
      const originalPatterns = await Effect.runPromise(
        Gram.parse("(alice:Person)-[:KNOWS]->(bob:Person)")
      )

      const gram = await Effect.runPromise(
        Gram.stringifyWithHeader(originalHeader, originalPatterns)
      )
      const { header: header2, patterns: patterns2 } = await Effect.runPromise(
        Gram.parseWithHeader(gram)
      )

      expect(header2).toEqual(originalHeader)
      expect(patterns2).toHaveLength(originalPatterns.length)
    })
  })

  describe("tagged-string round-trip (Node.js CJS path)", () => {
    it("preserves tagged-string values through parse → stringify", async () => {
      // Tagged strings use tag`content` notation in gram.
      // This test guards against the Node.js CJS WASM bridge corrupting
      // { type: "tagged", tag, content } property objects on stringify.
      const input = "(a {code: h3`8f283082aa20c00`})"
      const patterns = await Effect.runPromise(Gram.parse(input))

      expect(patterns).toHaveLength(1)
      const codeVal = Option.getOrUndefined(
        HashMap.get(patterns[0]!.value.properties, "code")
      )
      expect(codeVal).toBeDefined()
      expect(codeVal?._tag).toBe("TaggedStringVal")

      const gram = await Effect.runPromise(Gram.stringify(patterns))

      // The stringified output must re-parse to the same tagged-string value.
      const reparsed = await Effect.runPromise(Gram.parse(gram))
      expect(reparsed).toHaveLength(1)
      const reparsedCode = Option.getOrUndefined(
        HashMap.get(reparsed[0]!.value.properties, "code")
      )
      expect(reparsedCode).toBeDefined()
      expect(reparsedCode?._tag).toBe("TaggedStringVal")
      if (reparsedCode?._tag === "TaggedStringVal") {
        expect(reparsedCode.tag).toBe("h3")
        expect(reparsedCode.content).toBe("8f283082aa20c00")
      }
    })

    it("preserves tagged-string values through stringifyWithHeader round-trip", async () => {
      const input = "(a {code: h3`8f283082aa20c00`})"
      const patterns = await Effect.runPromise(Gram.parse(input))
      const header = { version: 1 }

      const gram = await Effect.runPromise(
        Gram.stringifyWithHeader(header, patterns)
      )
      const { header: h2, patterns: reparsed } = await Effect.runPromise(
        Gram.parseWithHeader(gram)
      )

      expect(h2).toEqual(header)
      expect(reparsed).toHaveLength(1)
      const reparsedCode = Option.getOrUndefined(
        HashMap.get(reparsed[0]!.value.properties, "code")
      )
      expect(reparsedCode).toBeDefined()
      expect(reparsedCode?._tag).toBe("TaggedStringVal")
      if (reparsedCode?._tag === "TaggedStringVal") {
        expect(reparsedCode.tag).toBe("h3")
        expect(reparsedCode.content).toBe("8f283082aa20c00")
      }
    })
  })
})
