import { Data } from "effect"
import { describe, expect, it } from "vitest"
import { values } from "../src/ops.js"
import { Pattern } from "../src/pattern.js"
import { Subject } from "../src/subject.js"

describe("Pattern ops", () => {
  it("values returns subjects in pre-order", () => {
    const pattern = new Pattern({
      value: Subject.fromId("root"),
      elements: Data.array([
        Pattern.point(Subject.fromId("left")),
        new Pattern({
          value: Subject.fromId("right"),
          elements: Data.array([Pattern.point(Subject.fromId("leaf"))]),
        }),
      ]),
    })

    expect(values(pattern).map((subject) => subject.identity)).toEqual([
      "root",
      "left",
      "right",
      "leaf",
    ])
  })
})
