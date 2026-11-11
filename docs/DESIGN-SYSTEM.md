# Design System: Telosgard (v3)

> **v3 (Wave 1.9, 2026-11-11):** the Learner's reference look. **Light-first**: a warm off-white ground, deep teal-green ink and an **ochre** accent; white cards with soft shadows; **pill** buttons (ochre primary, outlined secondary); **Newsreader** headlines ending in an *italic accent phrase* in teal ("Today, *one step at a time.*"); **Plus Jakarta Sans** for the interface; **small-caps eyebrows** in dark ochre; **tinted status chips** (green done, amber due, red needs action). Dark mode is built from the original greens with a gold accent. The workspace layout (§4.1), motion (§4.2) and Milestone moments carry over from v2.
>
> v3 reverses three v2 choices at the Learner's request: tracked capitals for eyebrows (v2: sentence case), pill buttons (v2: one 10px rectangle), light-first (v2: dark-first). Sections below that still describe v2 values are superseded by §0.

## 0. v3 tokens (source of truth: `crates/app/style/tailwind.css`)

| Token | Light (default) | Dark | Use |
|---|---|---|---|
| `bg` | `#F3F5F2` | `#0F1F17` | Ground, frames |
| `surface` | `#FFFFFF` | `#1E3A2E` | Cards, sheets, panels' inputs |
| `raised` | `#EEF2EC` | `#24443A` | Hover, quiet fills, neutral chip |
| `fill` / `on-fill` | `#1F4A45` / `#F4F5EF` | `#3E5A3F` / `#F4F5EF` | Featured block (Up next), selected segment, progress bars |
| `accent` / `on-accent` | `#D9A549` / `#1A2B27` | `#D4B062` / `#13221B` | The primary pill |
| `text` | `#1C3531` | `#C7CBB8` | Ink |
| `text-muted` | `#55675F` | `#9DAD98` | Secondary text |
| `eyebrow-ink` | `#875812` | `#D4B062` | Small-caps eyebrows |
| `accent-ink` | `#2C5A55` | `#9CC9B9` | The italic accent phrase |
| `ok-bg` / `ok-ink` | `#E2EFE5` / `#2E6A3D` | `#2E513A` / `#A6DBA1` | Chips: done, followed plan, Got it |
| `warn-bg` / `warn-ink` | `#F7EBD2` / `#7E530E` | `#3B4D36` / `#E6C987` | Chips: awaiting, Almost |
| `bad-bg` / `bad-ink` | `#F6E2DE` / `#9A3A2C` | `#3E4638` / `#F2A897` | Chips: broke plan, Not yet |
| `canvas` / `grid` | `#E8ECE6` / ink at 10% | `#0A1711` / text at 11% | Workspace dot grid |

**Measured contrast (WCAG 2.x), lowest per text token across bg / surface / raised / canvas:** light: text 10.96, muted 5.03, eyebrow 5.12, accent-ink 6.51, up 5.23, down 5.36, gold 5.22; on-accent on accent 6.65; on-fill on fill 9.02; chips 5.46 / 5.68 / 5.59. Dark: text 6.45, muted 4.52, eyebrow 5.19, accent-ink 5.84; on-accent on accent 8.01; chips 5.64 / 5.68 / 5.06.

**Type:** Newsreader for every `h1` and the Large Title / Title 1 sizes (regular 400, an `<em>` inside is the teal italic accent); Plus Jakarta Sans (400–700) everywhere else. Large Title 42/46, Title 1 34/40, Title 2 22/28 600, Headline 17/24 600, Body 17/29, Callout 16/25, Footnote 13/19, Eyebrow 12/16 600, +0.08em, uppercase.

**Shape:** cards and frames 18 px, rows and fields 12 px, buttons and chips full pills. Light cards: hairline ring + `0 1px 2px` + `0 10px 30px` soft shadow (ink at 4% / 7%); dark: hairline ring only.

## 1. Principles
1. **Content is the hero.** Generous whitespace, one idea per card, no decoration that doesn't carry meaning.
2. **One strict system.** One type pairing, one spacing scale, one radius family, one hairline, one button shape, one card recipe. Nothing is styled outside it.
3. **Edges, not shadows.** Cards and panels are separated by a 1px hairline (a ring drawn with `box-shadow`, so it never shifts layout). Light mode adds a whisper of shadow; dark mode none.
4. **Colour means something.** Green/red only for price direction and P/L. Gold only for **Milestones** (see CONTEXT). The interface itself stays neutral green-grey.
5. **Quiet motion that answers.** Every link, button and linked card responds to hover and press within 150 ms; bigger moments only for Milestones. All motion is off under `prefers-reduced-motion`.

### 1.1 What we borrowed from taste-skill (and what we didn't)
[taste-skill](https://github.com/leonxlnx/taste-skill) 2.5.1 is written for company marketing sites. It is **not installed**; these of its rules fit a learning app and are adopted:
- One system per page; spacing on a 4/8 scale; one radius family; one hairline at 12–15% of the ink.
- Nothing under 13 px. Text people read is 16 px or more in full ink colour; muted colour only for labels, meta and captions.
- Labels in sentence case at 13 px / 500, never tracked capitals.
- Hover and press numbers: hover in 150 ms, out 120 ms, press 90 ms; buttons step up 1 px and press 1 px down at scale .985; linked cards lift 2 px and their edge darkens; one ease-out curve `cubic-bezier(.2,.8,.2,1)`, no overshoot; the focus ring never animates; hover only for a fine pointer.
- Nothing reveals, staggers or moves on scroll.

Not adopted (marketing-site rules): a hero on every page, one section per screen, photographs, shader effects, "no caption under a visual" (our charts must carry their *Illustrative* label), and its finish-gate hook.

## 2. Colour tokens

### Dark (default)
| Token | Hex | Use | Contrast |
|---|---|---|---|
| `bg` | `#0F1F17` | Page background | — |
| `surface` | `#1E3A2E` | Cards, sheets | — |
| `fill` | `#3E5A3F` | Primary button fill, selected segment, progress track fill | — |
| `on-fill` | `#F4F5EF` | Text on `fill` | 6.99 on `fill` ✅ |
| `text` | `#C7CBB8` | Primary text | 10.31 on bg / 7.45 on surface ✅ |
| `text-muted` | `#9DAD98` | Secondary text, captions | 7.21 on bg / 5.21 on surface ✅ |
| `line` | `#6B7F67` | Borders, dividers, icons, inactive dots. **Never body text** (2.86 on surface ❌) | — |
| `locked` | `#3D4337` | Locked card background, disabled controls | — |
| `up` | `#7FC97A` | Bullish candle, profit, "adherent" | 6.20 on surface ✅ |
| `down` | `#E8826B` | Bearish candle, loss, "not adherent" | 4.61 on surface ✅ |
| `gold` | `#D4B062` | Achievements only: Gate passed, Live Readiness, logo mark | 5.99 on surface ✅ |

### Light
| Token | Hex | Use | Contrast |
|---|---|---|---|
| `bg` | `#E4E6DA` | Page background (a lighter tint of #C7CBB8) | — |
| `surface` | `#F4F5EF` | Cards, sheets | — |
| `fill` | `#3E5A3F` | Primary button fill | — |
| `on-fill` | `#F4F5EF` | Text on `fill` | 6.99 ✅ |
| `text` | `#0F1F17` | Primary text | 15.59 on surface / 13.53 on bg ✅ |
| `text-muted` | `#4E5F4B` | Secondary text | 6.25 on surface / 5.43 on bg ✅ |
| `line` | `#C7CBB8` | Borders, dividers, chip backgrounds | — |
| `locked` | `#3D4337` @ 12% alpha | Locked card tint | — |
| `up` | `#2A6E27` | Bullish / profit | 5.70 on surface / 4.95 on bg ✅ (darkened from #2F7A2C in Phase 8: that was 4.22 on bg) |
| `down` | `#9E4232` | Bearish / loss | 5.84 on surface ✅ |
| `gold` | `#7A5C14` (text) / `#D4B062` (fills, icons) | Achievements | 5.69 on surface ✅ |

### Rules
- **Locked cards use `text`, never `text-muted`:** muted measures 4.31:1 on `#3D4337`, below AA. Light-mode locked is `#DEE0D9` (12% of #3D4337 over surface).
- **Extra tokens added in Phase 3:** `raised` (hover, #24443A dark / #FFFFFF light), `gold-fill` (#D4B062 in both themes), `on-gold` (#0F1F17), `scrim` (sheet backdrop), `shadow-card` (none in dark), `shadow-sheet`.
- **Charts:** bullish candles are hollow and bearish filled, so direction never relies on colour alone.
- Up/down **always** come with a second signal: ▲/▼, "+"/"−", or a word ("Profit", "Loss"). Colour is never the only cue.
- Theme: follow `prefers-color-scheme` by default. A manual toggle sets `data-theme` on `<html>` and is saved in `profile.theme`, with `localStorage` as a fast first-paint hint.

### 2.1 Measured text pairings (v2)
| Pairing | Dark | Light |
|---|---|---|
| `text` on bg / surface / raised | 10.31 / 7.45 / 6.45 | 13.53 / 15.59 / 17.09 |
| `text-muted` on bg / surface / raised | 7.21 / 5.21 / **4.52** | 5.43 / 6.25 / 6.86 |
| `on-fill` on `fill` (buttons) | 6.99 | 6.99 |

Muted text on `raised` in dark is the tightest pairing (4.52): keep reading text off it. The hairline (`text` at 13% dark, `bg-ink` at 12% light; 1.34 / 1.27 against surface) is decorative; meaning never rests on it alone.

### 2.2 New v2 tokens
| Token | Dark | Light | Use |
|---|---|---|---|
| `hairline` | `rgb(199 203 184 / .13)` | `rgb(15 31 23 / .12)` | Card and panel edges, dividers |
| `grid` | `rgb(199 203 184 / .11)` | `rgb(15 31 23 / .10)` | Canvas dot grid (Phase 14) |
| `shadow-card` | 1px hairline ring | ring + `0 1px 2px` at 5% | Every card |
| `shadow-lift` | ring at 22% + soft 24px shadow | ring at 20% + soft shadow at 8% | A linked card on hover |

## 3. Typography
Two faces from Google Fonts (the only font host the artifact allows), with system fallbacks if they fail to load:
- **Inter** (400, 500, 600, 700) for the interface and body text. Feature `cv05` (lower-case l with a tail) so l, 1 and I never blur in prices.
- **Newsreader** (opsz 6–72; 400, 500, 600) for display: every `h1`, and the Large Title and Title 1 sizes wherever they appear; lesson step headings use it at Title 2 (`font-display`).

Numbers use `font-variant-numeric: tabular-nums` (prices, lots, P/L).

| Style | Face | Size / line height | Weight | Use |
|---|---|---|---|---|
| Large Title | Newsreader | 38 / 44, −0.015em | 500 | Screen titles ("Today") |
| Title 1 | Newsreader | 32 / 38, −0.01em | 500 | Lesson title, stage card title |
| Title 2 | Inter (Newsreader for step headings) | 22 / 28, −0.01em | 600 (500 serif) | Section headings |
| Headline | Inter | 17 / 24, −0.01em | 600 | Card headers, quiz question |
| Body | Inter | 17 / 28 | 400 | Lesson text (reading column max 640 px) |
| Callout | Inter | 16 / 24 | 400 | Most interface text, buttons (600) |
| Footnote | Inter | 13 / 18 | 400 | Captions, "Illustrative" labels, tab labels (500) |
| Label (`text-eyebrow`) | Inter | 13 / 18, sentence case | 500 | "Stage 1 · Lesson 2", "Up next" |

## 4. Shape, spacing, depth
- **Radius family ("soft"):** cards and frames `16px`, buttons and inputs `10px`, chips `999px`. One button shape: `rounded-btn`, 44 px high, 600 label.
- **Spacing:** multiples of 4 (4, 8, 12, 16, 24, 32, 48, 64). The page gutter is 16 on phones and 24 from 768 px up.
- **Touch targets:** at least 44 × 44.
- **Depth:** every card has the hairline ring (`shadow-card`). Linked cards use `lift`. Sheets add a soft shadow.
- **Glass:** the bars use `backdrop-filter: blur(20px) saturate(140%)` over `bg` at 72% alpha.

## 4.1 Workspace layout (Phase 14)
From **1024 px** wide the app is a workspace, like Figma's editor:
- **Left sidebar:** Today, Practice, Tools, Journal, then the course outline as a layers-style tree (every stage; the current stage open; ✓ passed, ● current, 🔒 locked). Replaces the bottom tab bar.
- **Canvas:** a faint dot grid (`grid` token, 16 px spacing, 1 px dots). The screen sits on one **frame** card with a small Figma-style frame label above it ("Lesson 2 · Step 2").
- **Right panel**, only where it adds context: in a lesson, progress, the Gate, Ask Claude (inline) and sources; in the Journal and on trade pages, Live Readiness at a glance and today's checklist. Today, Practice and Tools use the full canvas width.
- Panels are separated by hairlines. Keyboard order: sidebar → canvas → panel.

Below 1024 px: one column and the bottom tab bar; Ask Claude stays a sheet.

## 4.2 Motion
Tokens: `--m-in` 150 ms, `--m-out` 120 ms, `--m-press` 90 ms, `--m-slide` 240 ms, `--m-panel` 200 ms; `--ease` `cubic-bezier(.2,.8,.2,1)` for every UI step, `--ease-enter` `cubic-bezier(.22,1,.36,1)` for things settling. Transform, opacity and colour only.

| Part | Motion |
|---|---|
| Button (`press`) | hover (fine pointer): up 1 px; press: 1 px down, scale .985 |
| Linked card (`lift`) | hover: up 2 px, edge darkens (`shadow-lift`); press: scale .995 |
| Text link | the underline rests at 40% and darkens to full on hover |
| Focus | 2 px ring in `text`, 2 px off the control, never animated |
| Lesson step (Phase 15) | slides a few px as it changes |
| Progress bars, Gate checks (Phase 15) | fill when they change |
| **Milestone** moments (Phase 15) | lesson passed (on the Gate card), stage complete, Live Readiness: under a second, greens and gold. Never for a profitable trade |
| Count-ups (Phase 15) | stats count up to their value once when they first appear; the final figure is in the HTML |
| Screen transitions (Phase 15) | the canvas cross-fades between routes |

Under reduced motion every transition and animation is off and every final state shows at once.

## 5. Components

| Component | Description |
|---|---|
| **StageCard** | The home "Today" card. Label (Stage n), Title 1, progress bar ("5 of 9"), Continue pill. Locked: `locked` background, 🔒 glyph, the unlock condition in muted text. Completed: a gold check badge. |
| **LessonRow** | Inside an expanded StageCard: number, title, and state glyph (✓ in gold / ● current / 🔒). |
| **Reader** | A single 640 px column. Analogy blocks are a `surface` card with a 4px `fill` left bar and a 💡 eyebrow "ANALOGY"; example blocks use 🌍 "REAL LIFE"; Key takeaways is a check list. |
| **VideoCard** | A link-out card (the platform blocks YouTube embeds and thumbnails): ▶ glyph tile in `fill`, video title, channel · duration, a "Watch for:" line, start time, opening YouTube in a new tab. |
| **QuizCard** | One question at a time, options as full-width tappable rows, instant feedback with a "why" line, and a progress dot row. |
| **ExplainBackSheet** | Bottom sheet (phone) / side panel (desktop). Textarea, Submit pill, then a Grade chip (Got it = gold, Almost = text-muted outline, Not yet = line outline) and the feedback body. |
| **GateStatus** | Two pills, "Quiz ≥ 80%" and "Explain-Back: Got it", each ticking gold when met. The Next Lesson button activates once both are met. |
| **SegmentedControl** | iOS-style, used for Buy/Sell, theme, and journal filters. |
| **NumberField** | Large tabular numerals, unit suffix ("$", "%", "lots"), stepper on phones. |
| **CandleChart** | SVG candles in `up`/`down`, axis in `text-muted`, gridlines in `line` at 30%. A footnote below every chart: "Illustrative, not real market data." |
| **RingProgress** | Live Readiness rings (Apple-Fitness style): trades x/50, adherence %, days x/56. Gold when complete. |
| **Toast** | Top-centre glass pill for "Saved", "Grading…" and error messages. |

## 6. Tailwind v4 mapping
The source of truth is `crates/app/style/tailwind.css`: `@theme static` holds the dark tokens, fonts, type scale and radii; the two guarded light blocks redefine colours, hairline, grid and shadows; `:root` holds the motion tokens; `press`, `lift`, `gutter`, `glass` and `tabular` are utilities. Classes are then `bg-surface`, `text-text-muted`, `rounded-card`, `shadow-card`, `font-display` and so on.

## 7. Voice
A patient friend who trades: short sentences, "you" and "we", no jargon without an immediate definition, and no hype ("easy money", "guaranteed"). Mistakes are framed as normal: *"Almost. You've got the idea of the spread; the missing piece is when you pay it."*
