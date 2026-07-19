# C-5 part two · WCAG 2.1 AA audit of the eight surfaces

Rendered-contrast audit of all eight WELLness surfaces, measured in a browser against the
**real composited backgrounds** (alpha-blended, not estimated from tokens), each page
title-asserted before any reading was trusted (Law 1d — before asserting a property of a
thing, assert you are looking at the thing).

**Result: 161 text elements below WCAG 2.1 AA across the eight surfaces.** This is far larger
than the single `d12` `--info` failure that was confirmed earlier, and it splits into three
causes — one mechanical, two that are design decisions on granted bytes.

## Per surface

| surface | elements | failures | worst |
|---|---|---|---|
| `d12_fat_scan_result` | 129 | **36** | honey `--b-value` 1.88:1 |
| `blOVErai_companion` | 48 | **26** | dark ink on dark section 1.26:1 |
| `module2_fats` | 76 | **22** | black on bar 1.01:1 |
| `module1_hemp_hearts` | 87 | **20** | black on bar 1.01:1 |
| `beehive_wellness_index` | 48 | **18** | biomass green 3.02:1 |
| `module3_portion_reality` | 54 | **16** | black on bar 1.01:1 |
| `module4_decide` | 52 | **14** | black on bar 1.01:1 |
| `bqueenbee_analytics` (dark) | 42 | **9** | dark ink-dim 4.1:1 |

## Cause 1 — token failures (mechanical, computed fix)

The same decorative-grey-as-text defect the C-2 explainer had, plus the semantic hues used as
text. All fixable by darkening the inlined token to a computed AA-passing value that preserves
identity — exactly the fix already ratified for `bnr-design` and the explainer.

| token | value | ratio | where |
|---|---|---|---|
| `--ink-dim` | `#93A096` / `#8AA096` | 2.52–2.72 | section labels, captions, footers — the biggest single source |
| `--biomass` | `#5FA544` | 2.79–3.02 | module tags, "open →" links, seals |
| `--ai` | `#0E96A8` | 3.53 | bLOVErAi accents |
| `--info` | `#2E7FC0` | 3.65–3.95 | the confirmed `d12` spread chip, and more |
| `--guard` | `#7D5FB0` | 4.11–4.34 | flags |
| `--ink-mut` (dark) | `#5E7A6F` | ~4.1 | bqueenbee dark face |

**These I can fix confidently** — computed against each surface's real background, no taste
involved, the move this project has already ratified twice.

## Cause 2 — structural dark-on-dark (DESIGN DECISION)

Text at **1.01–1.31:1** — effectively invisible. Not a token: text inheriting near-black ink
that sits on a saturated or dark background.

- **`module1`–`module4`**: pure black at **1.01:1**, on the coloured stacked composition-bar
  segments and/or dark reward cards. 8 / 3 / 2 / 1 instances.
- **`blOVErai`**: dark ink `#16211F` at **1.26:1** in the teal AI-seat's dark sections, 5×.
- **`module3`**: dark ink `#1E2320` at 1.31:1, 1×.

The fix is not "darken a colour" — it is deciding whether that text goes **light on the dark
background**, or the **background lightens**. That is a visual-design call on granted surfaces,
and it changes how the piece looks, so it is yours rather than mine.

## Cause 3 — the honey `--b-value` (DESIGN DECISION · brand vs AA)

`#E8B54B` — the honey-gold `+3 b` reward number — measures **1.88:1** on light backgrounds,
and it appears on almost every surface (17 instances total). **Honey that clears 4.5:1 stops
being honey; it goes brown.** Options, none free:

1. Darken to a deep amber (loses the honey read).
2. Put the `+3 b` on a dark chip, where gold-on-dark passes (changes the layout).
3. Make it large-and-bold to qualify for the 3.0 large-text threshold — but 1.88 fails 3.0
   too, so it would still need darkening.

`honey = b` is a brand constant in this project. Reconciling it with AA is a founder call, not
a unilateral rebrand across granted surfaces.

## What this does to the grant

Every surface fix changes bytes, which **voids the L1 Tier R grant** for each changed surface
and requires the grant superseded (never amended, §5.1a). The grant record already states *a
grant is not an accessibility audit* — this audit is the substance behind that sentence.

## Recommendation

1. **I fix Cause 1 now** on your word — mechanical, computed, ratified-shape, no taste.
2. **Cause 2 and Cause 3 need one ruling each** — light-on-dark vs lighten-the-background for
   the bar/card text, and which of the three honey options. Small rulings; then I execute all
   three in one byte pass per surface, re-verify each in the browser, regenerate digests, and
   supersede the grant once.

Doing all three unilaterally across eight granted surfaces — the day a "are we still aligned"
reassessment is underway — would be the moving-fast-past-things pattern the reassessment
exists to catch. So: audit delivered, two decisions surfaced, fix held for your word.
