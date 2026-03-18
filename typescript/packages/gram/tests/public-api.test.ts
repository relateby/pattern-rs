import { describe, expect, it } from "vitest";

import { Gram, init } from "../src/index.js";

describe("@relateby/gram public API", () => {
  it("exports Gram and init", () => {
    expect(Gram).toBeDefined();
    expect(typeof Gram.parse).toBe("function");
    expect(typeof Gram.stringify).toBe("function");
    expect(typeof init).toBe("function");
  });
});
