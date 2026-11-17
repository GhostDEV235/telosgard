// Lesson numbers as the app stores them (`LessonId::content_id` in telosgard-core):
// L1–44 as themselves, the Indicators side lesson as 100, A1–A10 as 101–110.

/** Doc id ("lesson-05", "lesson-ind", "lesson-a1") for a stored lesson number, or null. */
export function lessonDocFromNumber(n: number): string | null {
  if (!Number.isInteger(n) || n < 1) return null;
  if (n === 100) return "lesson-ind";
  if (n > 100 && n <= 110) return `lesson-a${n - 100}`;
  if (n < 100) return `lesson-${String(n).padStart(2, "0")}`;
  return null;
}
