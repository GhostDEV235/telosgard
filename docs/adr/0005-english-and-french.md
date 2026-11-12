# English and French, with "tu"

*Decided 2026-11-12.*

Telosgard is available in **English and French**. The whole course is translated: interface, the written lessons (quizzes, Recall, Explain-Back rubrics and fixtures), drills, Hard Rule messages, and Professor Aurel's replies. The French addresses the Learner as **tu**, matching the course's "friend who trades" voice. The app starts in English; a **Language** switch in Appearance (and on the sign-in and join screens) changes it. The choice is saved on the device and in the profile, so it follows the account.

## How it works

- **Lessons:** each `lessons/NN-*.md` has a French twin in `lessons/fr/`. The build checks every twin against its English lesson (stage, steps and widgets, quiz and Recall ids, option counts and answers, Explain-Back shape, video ids and source URLs, drills), so progress and Question Records are shared across languages. French may run about a fifth longer (75 words per step, 6-minute main path).
- **Interface:** strings are English–French pairs at the point of use, `t("Today", "Aujourd’hui")`. A language change re-renders the whole shell, so text needn't be reactive.
- **Professor Aurel:** prompts stay in English, where they were tuned; in French a fixed instruction is appended asking for natural French with "tu", leaving JSON keys and grade names in English. Rubrics come from the French lesson.
- **Numbers:** French formatting (`4 100,00 $`, `0,20 lot`, `1 %`); input accepts either style.
- **Videos** stay in English, labelled « en anglais »; sources are cited as published.
- Terms and typography: [`docs/FRENCH.md`](../FRENCH.md).

## Considered Options

- **A translation catalogue (Fluent or gettext files).** Rejected for now: with two languages, pairs in place are easier to keep in step and impossible to leave half-done unnoticed.
- **Translating Professor Aurel's prompts.** Rejected: the English prompts are tuned against the grader lab; one appended instruction keeps a single prompt to maintain.
- **Following the device language.** Rejected by the Learner: English by default, with a switch.

## Consequences

- Every new lesson needs its French twin; the content test `every_lesson_has_a_french_twin` fails until it exists.
- New interface text must be written as a pair.
- The French Explain-Back fixtures should be run through the grader lab in French before relying on French grading.
