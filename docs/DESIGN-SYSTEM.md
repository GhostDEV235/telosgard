# Design System: Gold School

> Apple-style, calm and content-first. Palette from the Learner's reference image (#0F1F17 · #1E3A2E · #3E5A3F · #6B7F67 · #C7CBB8 · #3D4337).
> All contrast ratios below were measured (WCAG 2.x formula). AA body text needs ≥ 4.5:1.

## 1. Principles
1. **Content is the hero.** Generous whitespace, one idea per card, no decoration that doesn't carry meaning.
2. **Depth through layers, not lines.** Background → surface card → raised sheet. Soft shadows in light mode, tonal steps in dark mode.
3. **Colour means something.** Green/red only for price direction and P/L. Gold only for achievement. Everything else comes from the palette.
4. **Calm motion.** Short spring-like transitions (200–350 ms). All motion is disabled under `prefers-reduced-motion`.

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
| `up` | `#2F7A2C` | Bullish / profit | 4.86 on surface ✅ (use on surface, not bg) |
| `down` | `#9E4232` | Bearish / loss | 5.84 on surface ✅ |
| `gold` | `#7A5C14` (text) / `#D4B062` (fills, icons) | Achievements | 5.69 on surface ✅ |

### Rules
- **Locked cards use `text`, never `text-muted`:** muted measures 4.31:1 on `#3D4337`, below AA. Light-mode locked is `#DEE0D9` (12% of #3D4337 over surface).
- **Extra tokens added in Phase 3:** `raised` (hover, #24443A dark / #FFFFFF light), `gold-fill` (#D4B062 in both themes), `on-gold` (#0F1F17), `scrim` (sheet backdrop), `shadow-card` (none in dark), `shadow-sheet`.
- **Charts:** bullish candles are hollow and bearish filled, so direction never relies on colour alone.
- Up/down **always** come with a second signal: ▲/▼, "+"/"−", or a word ("Profit", "Loss"). Colour is never the only cue.
- Theme: follow `prefers-color-scheme` by default. A manual toggle sets `data-theme` on `<html>` and is saved in `profile.theme`, with `localStorage` as a fast first-paint hint.

## 3. Typography
System stack: `-apple-system, BlinkMacSystemFont, "SF Pro Text", "SF Pro Display", "Helvetica Neue", system-ui, sans-serif`. Numbers use `font-variant-numeric: tabular-nums` (prices, lots, P/L).

| Style | Size / line height | Weight | Use |
|---|---|---|---|
| Large Title | 34 / 41 | 700 | Screen titles ("Today") |
| Title 1 | 28 / 34 | 700 | Stage card title, lesson title |
| Title 2 | 22 / 28 | 600 | Section headings in lessons |
| Headline | 17 / 22 | 600 | Card headers, quiz question |
| Body | 17 / 26 | 400 | Lesson text (reading column max 640 px) |
| Callout | 16 / 22 | 400 | Analogy and example call-outs |
| Footnote | 13 / 18 | 400 | Captions, "Illustrative data" labels |
| Eyebrow | 12 / 16, tracking +0.06em, uppercase | 600 | "STAGE 1", "LESSON 5" |

## 4. Shape, spacing, depth
- Radius: cards `22px`, buttons `14px` (full pill for primary CTAs), chips `999px`, inputs `12px`.
- Spacing scale (px): 4, 8, 12, 16, 20, 24, 32, 44, 64. The page gutter is 16 on phones and 24 from 768 px up.
- Touch targets: at least 44 × 44.
- Depth: dark mode uses no shadows. Raised = `surface` + 1px `line` at 40% alpha. Light mode cards use `0 1px 2px rgb(15 31 23 / .06), 0 8px 24px rgb(15 31 23 / .06)`.
- Glass: the sticky top bar uses `backdrop-filter: blur(20px) saturate(140%)` over `bg` at 72% alpha.

## 5. Components

| Component | Description |
|---|---|
| **StageCard** | The home "Today" card. Eyebrow (STAGE n), Title 1, progress bar ("5 of 9"), Continue pill. Locked: `locked` background, 🔒 glyph, the unlock condition in muted text. Completed: a gold check badge. |
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
```css
@import "tailwindcss";

@theme {
  --color-bg: #0F1F17;      --color-surface: #1E3A2E;  --color-fill: #3E5A3F;
  --color-on-fill: #F4F5EF; --color-text: #C7CBB8;     --color-text-muted: #9DAD98;
  --color-line: #6B7F67;    --color-locked: #3D4337;
  --color-up: #7FC97A;      --color-down: #E8826B;     --color-gold: #D4B062;
  --radius-card: 22px;
  --font-sans: -apple-system, BlinkMacSystemFont, "SF Pro Text", "Helvetica Neue", system-ui, sans-serif;
}

@media (prefers-color-scheme: light) {
  :root:not([data-theme="dark"]) { --color-bg:#E4E6DA; --color-surface:#F4F5EF; --color-text:#0F1F17;
    --color-text-muted:#4E5F4B; --color-line:#C7CBB8; --color-up:#2F7A2C; --color-down:#9E4232; --color-gold:#7A5C14; }
}
:root[data-theme="light"] { /* same overrides as above */ }

body { background: var(--color-bg); color: var(--color-text); }
```
The classes are then `bg-surface`, `text-text-muted`, `rounded-card`, and so on.

## 7. Voice
A patient friend who trades: short sentences, "you" and "we", no jargon without an immediate definition, and no hype ("easy money", "guaranteed"). Mistakes are framed as normal: *"Almost. You've got the idea of the spread; the missing piece is when you pay it."*
