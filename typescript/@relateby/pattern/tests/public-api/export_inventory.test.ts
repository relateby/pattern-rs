import { describe, expect, it } from "vitest"
import * as PublicApi from "../../src/index.js"

const expectedTopLevelExports = [
  "Gram",
  "GramParseError",
  "Pattern",
  "StandardGraph",
  "Subject",
  "Value",
  "findFirst",
  "fold",
  "toGraphView",
] as const

describe("@relateby/pattern public export inventory", () => {
  it("exposes the documented native top-level symbols", () => {
    for (const symbol of expectedTopLevelExports) {
      expect(PublicApi, `missing export: ${symbol}`).toHaveProperty(symbol)
    }
  })

  it("keeps the package-level Gram facade callable", () => {
    expect(PublicApi.Gram).toBeDefined()
    expect(typeof PublicApi.Gram.parse).toBe("function")
    expect(typeof PublicApi.Gram.stringify).toBe("function")
    expect(typeof PublicApi.Gram.validate).toBe("function")
  })
})
