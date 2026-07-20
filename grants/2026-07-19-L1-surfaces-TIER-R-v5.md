# GRANT · Level 1 · the eight surfaces — **TIER R (reproducible)**

**Supersedes** [`2026-07-19-L1-surfaces-TIER-R-v4.md`](2026-07-19-L1-surfaces-TIER-R-v4.md).
Issued because **the graded bytes changed** — the eight surfaces were remediated to WCAG 2.1
AA under order RELAY-10 — so every digest v4 pinned is now stale. A grant is bound to specific
bytes; new bytes need a new grant, never an edit to the old one (ladder §5.1a). All earlier
records stay beneath their banners, because the history is the argument.

**This supersession also lifts a limitation v4 recorded.** v4 held the grant while stating in
plain text that `d12` (and, it turns out, seven others) failed AA contrast. That sentence was
the honest thing to do at the time — *a grant is not an accessibility audit* — and it is the
sentence this record retires. The surfaces now pass AA, measured, and a guard is in CI to keep
them there.

---

> ## Do not take our word for it. Here is the command.
>
> ```sh
> git clone https://github.com/skaists/beehive-WELLness
> cd beehive-WELLness
> sh verify/verify-l1.sh                 # A1/A1b/D1 — self-contained bytes, served digests, disclosure
> node verify/check-contrast.mjs         # RELAY-10 — every surface's text tokens clear 4.5:1
> node verify/check-contrast.mjs selftest   # watch the contrast guard FAIL on a bad token first
> ```
>
> And the rendered authority, which no static script can stand in for: open each surface in a
> browser and paste [`verify/contrast-probe.js`](../verify/contrast-probe.js) into the console.
> It composites the real backgrounds (including `color-mix()`), runs its own positive controls
> first, and reports every text element below AA. It reports **zero** on all eight.
>
> **If your result differs from ours, ours is wrong.** Tell us at the address in
> [`SECURITY.md`](../SECURITY.md).

---

## The grant

| Field | Value |
|---|---|
| **Level** | **L1** |
| **Tier** | **R — reproducible** (ladder §5.1b) |
| Axis A · Artifact | 1 |
| Axis D · Delivery | 1 |
| Axis O · Operations | 0 — no requirement at L1 |
| **Accessibility** | **WCAG 2.1 AA contrast: PASS on all eight** (rendered probe = 0 failures; token guard in CI) |
| Graded commit | `45e56f4118c6a1750760d8cea85d6d7303876cd3` |
| Verifier | `verify/verify-l1.sh` + `verify/check-contrast.mjs` at the same commit |
| Date | 2026-07-19 |
| Granting party | **any reader.** That is the point. |

## The expected output, verbatim

`verify/verify-l1.sh`:

```
artifacts_enumerated=8
A1a_external_references=0 (over 8 files)
A1b_fetched=8 served_matches_recorded=8
D1_disclosure_in_artifacts=8/8
RESULT=PASS
```

`node verify/check-contrast.mjs`:

```
surfaces_checked=8 failed=0
```

## The graded digests (post-remediation bytes)

| surface | sha256 |
|---|---|
| `beehive_wellness_index.html` | `95ae08c07efc9884dd8133d10566afb51e2dd3460ba30ce4489debbbd8082508` <!-- PUBLIC-CONSTANT: graded digest --> |
| `blOVErai_companion_surface.html` | `d6bb318ebd4eb405bb222ad9e99d4843acdee452a49506224c784a074f54273d` <!-- PUBLIC-CONSTANT: graded digest --> |
| `bqueenbee_analytics.html` | `60ca37aac70b14ceac07df2ede563e7a600ef65e8be8cafa9c77984a44bf46ce` <!-- PUBLIC-CONSTANT: graded digest --> |
| `d12_fat_scan_result.html` | `3f17abd25724784a54cc857e1735d02a50d9bd231c5fc4d9d3911ffa061b543c` <!-- PUBLIC-CONSTANT: graded digest --> |
| `module1_hemp_hearts_label_reader.html` | `cfe32d59e5b16d06af172cf7328a5d43eabd392c267419892cf3dbcb8a2167de` <!-- PUBLIC-CONSTANT: graded digest --> |
| `module2_fats_label_wont_show.html` | `06814e3e77264c44cc422b0a609315d91db11c8ec55875fb7c6e0589d8d98cb2` <!-- PUBLIC-CONSTANT: graded digest --> |
| `module3_portion_reality.html` | `8ba50be3b03f24dfd6e05a4efd8bfce3727f67e9f70462e89b0bda5100c301ae` <!-- PUBLIC-CONSTANT: graded digest --> |
| `module4_decide_for_yourself.html` | `c3d81b27aa894823142aeec0f1e7375fde07e3c5d711edac5eaab42cb7f75a3d` <!-- PUBLIC-CONSTANT: graded digest --> |

## What the remediation did, and the discipline it followed

**Governing principle (RELAY-10):** *a colour that carries meaning may not be repainted to fix
a contrast defect; a colour that carries only emphasis may.* Applied per instance:

- **Emphasis text** (dim labels, accents, the `bLOVErAi` wordmark) — darkened in place, hue
  kept: `--ink-dim`, `--ai` to `#0B7A89`, `--info` to `#2A73AE`. On the one dark surface the
  dim caption is **lightened** (`--dim` to `#6A847A`) — the same principle, opposite direction.
- **Datum colours** — the fat-category and family hues (`--biomass`, `--o3`, `--o6`, `--sfa`,
  `--mufa`, `--nm`) are the data and are **not repainted**. Where a data hue appeared as text,
  a darkened *in-ramp* derivative carries the text (`--biomass-ink`, `--sfa-ink`) while the
  datum token keeps its value for the bar fills. Large stat numbers that already clear the 3:1
  large-text bar are left as the datum they are.
- **Honey** (`--b-value`) — the colour of `b`, on every surface by D-14. An unreadable balance
  gauge is unreadable everywhere, so "documented limitation" was refused. Honey is not
  repainted; it is given a **dark chip** on which gold clears 4.5:1 (about 8.5:1 on `--ink`).

**The audit that drove this was itself corrected, by the positive control this order mandated.**
The prior "161 failures, dark-on-dark across the modules, `--guard` failing" was inflated by a
probe that mis-read `color(srgb ...)` computed backgrounds — the founder's required positive
control caught it, the corrected probe found the true set (135), and the remediation took it to
0. No level is claimed here beyond the axes above; the accessibility line is a **measured pass**,
not a badge.
