import { describe, expect, it } from "vitest";
import { lessonDocFromNumber } from "../src/lesson-ids";
import { deriveEvents } from "../src/events";

describe("stored lesson numbers", () => {
  it("map Core, Live, Indicators and Advanced lessons to their doc ids", () => {
    expect(lessonDocFromNumber(5)).toBe("lesson-05");
    expect(lessonDocFromNumber(44)).toBe("lesson-44");
    expect(lessonDocFromNumber(100)).toBe("lesson-ind");
    expect(lessonDocFromNumber(101)).toBe("lesson-a1");
    expect(lessonDocFromNumber(110)).toBe("lesson-a10");
    expect(lessonDocFromNumber(111)).toBeNull();
    expect(lessonDocFromNumber(0)).toBeNull();
  });
  it("tag an Advanced Explain-Back with its lesson", () => {
    const [e] = deriveEvents("attempt", "attempt-1", null, { lesson: 103, grade: "got_it", at: 5 }, 9);
    expect(e?.lesson).toBe("lesson-a3");
  });
});
