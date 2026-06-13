import { describe, expect, it } from "vitest"
import { Gram } from "../src/gram.js"
import { GramParseError } from "../src/errors.js"

describe("Gram parity", () => {
  describe("parseWithHeader", () => {
    it("splits an optional header record from patterns", async () => {
      const { header, patterns } = await Gram.parseWithHeader("{version: 1} (a)-->(b)")
      expect(header).toEqual({ version: 1 })
      expect(patterns).toHaveLength(1)
    })

    it("returns undefined header when no header is present", async () => {
      const { header, patterns } = await Gram.parseWithHeader("(a)-->(b)")
      expect(header).toBeUndefined()
      expect(patterns).toHaveLength(1)
    })

    it("handles header-only input (no patterns)", async () => {
      const { header, patterns } = await Gram.parseWithHeader("{version: 1}")
      expect(header).toEqual({ version: 1 })
      expect(patterns).toHaveLength(0)
    })

    it("handles empty input", async () => {
      const { header, patterns } = await Gram.parseWithHeader("")
      expect(header).toBeUndefined()
      expect(patterns).toHaveLength(0)
    })

    it("fails with GramParseError on invalid input", async () => {
      await expect(Gram.parseWithHeader("not valid gram ##!!")).rejects.toBeInstanceOf(GramParseError)
      await expect(Gram.parseWithHeader("not valid gram ##!!")).rejects.toMatchObject({ input: "not valid gram ##!!" })
    })
  })

  describe("stringifyWithHeader", () => {
    it("includes header record in output", async () => {
      const patterns = await Gram.parse("(a)-->(b)")
      const gram = await Gram.stringifyWithHeader({ version: 1 }, patterns)
      expect(gram).toContain("{")
      expect(gram).toContain("version")
    })

    it("undefined header produces same output as plain stringify", async () => {
      const patterns = await Gram.parse("(a)-->(b)")
      const plain = await Gram.stringify(patterns)
      const withUndefined = await Gram.stringifyWithHeader(undefined, patterns)
      expect(withUndefined).toBe(plain)
    })

    it("handles empty patterns with a header", async () => {
      const gram = await Gram.stringifyWithHeader({ version: 1 }, [])
      expect(gram).toContain("version")
    })

    it("full round-trip: stringify_with_header → parse_with_header", async () => {
      const originalHeader = { version: 2, schema: "test" }
      const originalPatterns = await Gram.parse("(alice:Person)-[:KNOWS]->(bob:Person)")
      const gram = await Gram.stringifyWithHeader(originalHeader, originalPatterns)
      const { header: header2, patterns: patterns2 } = await Gram.parseWithHeader(gram)
      expect(header2).toEqual(originalHeader)
      expect(patterns2).toHaveLength(originalPatterns.length)
    })
  })

  describe("tagged-string round-trip (Node.js CJS path)", () => {
    it("preserves tagged-string values through parse → stringify", async () => {
      const input = "(a {code: h3`8f283082aa20c00`})"
      const patterns = await Gram.parse(input)
      expect(patterns).toHaveLength(1)
      const codeVal = patterns[0]!.value.properties["code"]
      expect(codeVal).toBeDefined()
      expect(codeVal?._tag).toBe("TaggedStringVal")
      const gram = await Gram.stringify(patterns)
      const reparsed = await Gram.parse(gram)
      expect(reparsed).toHaveLength(1)
      const reparsedCode = reparsed[0]!.value.properties["code"]
      expect(reparsedCode?._tag).toBe("TaggedStringVal")
      if (reparsedCode?._tag === "TaggedStringVal") {
        expect(reparsedCode.tag).toBe("h3")
        expect(reparsedCode.content).toBe("8f283082aa20c00")
      }
    })

    it("preserves tagged-string values through stringifyWithHeader round-trip", async () => {
      const input = "(a {code: h3`8f283082aa20c00`})"
      const patterns = await Gram.parse(input)
      const header = { version: 1 }
      const gram = await Gram.stringifyWithHeader(header, patterns)
      const { header: h2, patterns: reparsed } = await Gram.parseWithHeader(gram)
      expect(h2).toEqual(header)
      expect(reparsed).toHaveLength(1)
      const reparsedCode = reparsed[0]!.value.properties["code"]
      expect(reparsedCode?._tag).toBe("TaggedStringVal")
      if (reparsedCode?._tag === "TaggedStringVal") {
        expect(reparsedCode.tag).toBe("h3")
        expect(reparsedCode.content).toBe("8f283082aa20c00")
      }
    })
  })
})
