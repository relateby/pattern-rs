import { describe, expect, it } from "vitest";

import { GramParseError } from "@relateby/pattern";
import { Gram, init } from "../src/index.js";

describe("@relateby/gram public API", () => {
  it("exports Gram and init", () => {
    expect(Gram).toBeDefined();
    expect(typeof Gram.parse).toBe("function");
    expect(typeof Gram.stringify).toBe("function");
    expect(typeof init).toBe("function");
  });

  describe("Gram.parse", () => {
    it("parses (a) into a single-element pattern array", async () => {
      const patterns = await Gram.parse("(a)");
      expect(patterns).toHaveLength(1);
    });

    it("parses (a)-->(b) into a pattern with two elements", async () => {
      const patterns = await Gram.parse("(a)-->(b)");
      expect(patterns).toHaveLength(1);
      expect(patterns[0]?.length).toBe(2);
    });
  });

  describe("Gram.stringify", () => {
    it("stringifies a parsed (a:Person) pattern to a non-empty string", async () => {
      const patterns = await Gram.parse("(a:Person)");
      const out = await Gram.stringify(patterns);
      expect(typeof out).toBe("string");
      expect(out.length).toBeGreaterThan(0);
    });
  });

  describe("Gram.validate", () => {
    it("succeeds for valid gram", async () => {
      await Gram.validate("(a)");
    });

    it("fails for invalid gram with GramParseError", async () => {
      await expect(Gram.validate("(unclosed")).rejects.toBeInstanceOf(GramParseError);
    });
  });

  describe("init()", () => {
    it("resolves without error", async () => {
      await expect(init()).resolves.toBeUndefined();
    });

    it("resolves without error when called a second time", async () => {
      await init();
      await expect(init()).resolves.toBeUndefined();
    });
  });
});
