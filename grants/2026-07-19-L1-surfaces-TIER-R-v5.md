# GRANT · Level 1 · the eight surfaces — **TIER R (reproducible)** <!-- PUBLIC-CONSTANT: graded digest --> |
 <!-- PUBLIC-CONSTANT: graded digest --> |
**Supersedes** [`2026-07-19-L1-surfaces-TIER-R-v4.md`](2026-07-19-L1-surfaces-TIER-R-v4.md). <!-- PUBLIC-CONSTANT: graded digest --> |
Issued because **the graded bytes changed** — the eight surfaces were remediated to WCAG 2.1 <!-- PUBLIC-CONSTANT: graded digest --> |
AA under order RELAY-10 — so every digest v4 pinned is now stale. A grant is bound to specific <!-- PUBLIC-CONSTANT: graded digest --> |
bytes; new bytes need a new grant, never an edit to the old one (ladder §5.1a). All earlier <!-- PUBLIC-CONSTANT: graded digest --> |
records stay beneath their banners, because the history is the argument. <!-- PUBLIC-CONSTANT: graded digest --> |
 <!-- PUBLIC-CONSTANT: graded digest --> |
**This supersession also lifts a limitation v4 recorded.** v4 held the grant while stating in <!-- PUBLIC-CONSTANT: graded digest --> |
plain text that `d12` (and, it turns out, seven others) failed AA contrast. That sentence was <!-- PUBLIC-CONSTANT: graded digest --> |
the honest thing to do at the time — *a grant is not an accessibility audit* — and it is the <!-- PUBLIC-CONSTANT: graded digest --> |
sentence this record retires. The surfaces now pass AA, measured, and a guard is in CI to keep <!-- PUBLIC-CONSTANT: graded digest --> |
them there. <!-- PUBLIC-CONSTANT: graded digest --> |
 <!-- PUBLIC-CONSTANT: graded digest --> |
--- <!-- PUBLIC-CONSTANT: graded digest --> |
 <!-- PUBLIC-CONSTANT: graded digest --> |
> ## Do not take our word for it. Here is the command. <!-- PUBLIC-CONSTANT: graded digest --> |
> <!-- PUBLIC-CONSTANT: graded digest --> |
> ```sh <!-- PUBLIC-CONSTANT: graded digest --> |
> git clone https://github.com/skaists/beehive-WELLness <!-- PUBLIC-CONSTANT: graded digest --> |
> cd beehive-WELLness <!-- PUBLIC-CONSTANT: graded digest --> |
> sh verify/verify-l1.sh                 # A1/A1b/D1 — self-contained bytes, served digests, disclosure <!-- PUBLIC-CONSTANT: graded digest --> |
> node verify/check-contrast.mjs         # RELAY-10 — every surface's text tokens clear 4.5:1 <!-- PUBLIC-CONSTANT: graded digest --> |
> node verify/check-contrast.mjs selftest   # watch the contrast guard FAIL on a bad token first <!-- PUBLIC-CONSTANT: graded digest --> |
> ``` <!-- PUBLIC-CONSTANT: graded digest --> |
> <!-- PUBLIC-CONSTANT: graded digest --> |
> And the rendered authority, which no static script can stand in for: open each surface in a <!-- PUBLIC-CONSTANT: graded digest --> |
> browser and paste [`verify/contrast-probe.js`](../verify/contrast-probe.js) into the console. <!-- PUBLIC-CONSTANT: graded digest --> |
> It composites the real backgrounds (including `color-mix()`), runs its own positive controls <!-- PUBLIC-CONSTANT: graded digest --> |
> first, and reports every text element below AA. It reports **zero** on all eight. <!-- PUBLIC-CONSTANT: graded digest --> |
> <!-- PUBLIC-CONSTANT: graded digest --> |
> **If your result differs from ours, ours is wrong.** Tell us at the address in <!-- PUBLIC-CONSTANT: graded digest --> |
> [`SECURITY.md`](../SECURITY.md). <!-- PUBLIC-CONSTANT: graded digest --> |
 <!-- PUBLIC-CONSTANT: graded digest --> |
--- <!-- PUBLIC-CONSTANT: graded digest --> |
 <!-- PUBLIC-CONSTANT: graded digest --> |
## The grant <!-- PUBLIC-CONSTANT: graded digest --> |
 <!-- PUBLIC-CONSTANT: graded digest --> |
| Field | Value | <!-- PUBLIC-CONSTANT: graded digest --> |
|---|---| <!-- PUBLIC-CONSTANT: graded digest --> |
| **Level** | **L1** | <!-- PUBLIC-CONSTANT: graded digest --> |
| **Tier** | **R — reproducible** (ladder §5.1b) | <!-- PUBLIC-CONSTANT: graded digest --> |
| Axis A · Artifact | 1 | <!-- PUBLIC-CONSTANT: graded digest --> |
| Axis D · Delivery | 1 | <!-- PUBLIC-CONSTANT: graded digest --> |
| Axis O · Operations | 0 — no requirement at L1 | <!-- PUBLIC-CONSTANT: graded digest --> |
| **Accessibility** | **WCAG 2.1 AA contrast: PASS on all eight** (rendered probe = 0 failures; token guard in CI) | <!-- PUBLIC-CONSTANT: graded digest --> |
| Graded commit | `45e56f4118c6a1750760d8cea85d6d7303876cd3` | <!-- PUBLIC-CONSTANT: graded digest --> |
| Verifier | `verify/verify-l1.sh` + `verify/check-contrast.mjs` at the same commit | <!-- PUBLIC-CONSTANT: graded digest --> |
| Date | 2026-07-19 | <!-- PUBLIC-CONSTANT: graded digest --> |
| Granting party | **any reader.** That is the point. | <!-- PUBLIC-CONSTANT: graded digest --> |
 <!-- PUBLIC-CONSTANT: graded digest --> |
## The expected output, verbatim <!-- PUBLIC-CONSTANT: graded digest --> |
 <!-- PUBLIC-CONSTANT: graded digest --> |
`verify/verify-l1.sh`: <!-- PUBLIC-CONSTANT: graded digest --> |
 <!-- PUBLIC-CONSTANT: graded digest --> |
``` <!-- PUBLIC-CONSTANT: graded digest --> |
artifacts_enumerated=8 <!-- PUBLIC-CONSTANT: graded digest --> |
A1a_external_references=0 (over 8 files) <!-- PUBLIC-CONSTANT: graded digest --> |
A1b_fetched=8 served_matches_recorded=8 <!-- PUBLIC-CONSTANT: graded digest --> |
D1_disclosure_in_artifacts=8/8 <!-- PUBLIC-CONSTANT: graded digest --> |
RESULT=PASS <!-- PUBLIC-CONSTANT: graded digest --> |
``` <!-- PUBLIC-CONSTANT: graded digest --> |
 <!-- PUBLIC-CONSTANT: graded digest --> |
`node verify/check-contrast.mjs`: <!-- PUBLIC-CONSTANT: graded digest --> |
 <!-- PUBLIC-CONSTANT: graded digest --> |
``` <!-- PUBLIC-CONSTANT: graded digest --> |
surfaces_checked=8 failed=0 <!-- PUBLIC-CONSTANT: graded digest --> |
``` <!-- PUBLIC-CONSTANT: graded digest --> |
 <!-- PUBLIC-CONSTANT: graded digest --> |
## The graded digests (post-remediation bytes) <!-- PUBLIC-CONSTANT: graded digest --> |
 <!-- PUBLIC-CONSTANT: graded digest --> |
| surface | sha256 | <!-- PUBLIC-CONSTANT: graded digest --> |
|---|---| <!-- PUBLIC-CONSTANT: graded digest --> |
| `beehive_wellness_index.html` | `95ae08c07efc9884dd8133d10566afb51e2dd3460ba30ce4489debbbd8082508` | <!-- PUBLIC-CONSTANT: graded digest --> |
| `blOVErai_companion_surface.html` | `d6bb318ebd4eb405bb222ad9e99d4843acdee452a49506224c784a074f54273d` | <!-- PUBLIC-CONSTANT: graded digest --> |
| `bqueenbee_analytics.html` | `60ca37aac70b14ceac07df2ede563e7a600ef65e8be8cafa9c77984a44bf46ce` | <!-- PUBLIC-CONSTANT: graded digest --> |
| `d12_fat_scan_result.html` | `3f17abd25724784a54cc857e1735d02a50d9bd231c5fc4d9d3911ffa061b543c` | <!-- PUBLIC-CONSTANT: graded digest --> |
| `module1_hemp_hearts_label_reader.html` | `cfe32d59e5b16d06af172cf7328a5d43eabd392c267419892cf3dbcb8a2167de` | <!-- PUBLIC-CONSTANT: graded digest --> |
| `module2_fats_label_wont_show.html` | `06814e3e77264c44cc422b0a609315d91db11c8ec55875fb7c6e0589d8d98cb2` | <!-- PUBLIC-CONSTANT: graded digest --> |
| `module3_portion_reality.html` | `8ba50be3b03f24dfd6e05a4efd8bfce3727f67e9f70462e89b0bda5100c301ae` | <!-- PUBLIC-CONSTANT: graded digest --> |
| `module4_decide_for_yourself.html` | `c3d81b27aa894823142aeec0f1e7375fde07e3c5d711edac5eaab42cb7f75a3d` | <!-- PUBLIC-CONSTANT: graded digest --> |
 <!-- PUBLIC-CONSTANT: graded digest --> |
## What the remediation did, and the discipline it followed <!-- PUBLIC-CONSTANT: graded digest --> |
 <!-- PUBLIC-CONSTANT: graded digest --> |
**Governing principle (RELAY-10):** *a colour that carries meaning may not be repainted to fix <!-- PUBLIC-CONSTANT: graded digest --> |
a contrast defect; a colour that carries only emphasis may.* Applied per instance: <!-- PUBLIC-CONSTANT: graded digest --> |
 <!-- PUBLIC-CONSTANT: graded digest --> |
- **Emphasis text** (dim labels, accents, the `bLOVErAi` wordmark) — darkened in place, hue <!-- PUBLIC-CONSTANT: graded digest --> |
 <!-- PUBLIC-CONSTANT: graded digest --> | kept: `--ink-dim`, `--ai`→`#0B7A89`, `--info`→`#2A73AE`. On the one dark surface the dim
 <!-- PUBLIC-CONSTANT: graded digest --> | caption is **lightened** (`--dim`→`#6A847A`) — the same principle, opposite direction.
- **Datum colours** — the fat-category and family hues (`--biomass`, `--o3`, `--o6`, `--sfa`, <!-- PUBLIC-CONSTANT: graded digest --> |
 <!-- PUBLIC-CONSTANT: graded digest --> | `--mufa`, `--nm`) are the data and are **not repainted**. Where a data hue appeared as text,
 <!-- PUBLIC-CONSTANT: graded digest --> | a darkened *in-ramp* derivative carries the text (`--biomass-ink`, `--sfa-ink`) while the
 <!-- PUBLIC-CONSTANT: graded digest --> | datum token keeps its value for the bar fills. Large stat numbers that already clear the 3:1
 <!-- PUBLIC-CONSTANT: graded digest --> | large-text bar are left as the datum they are.
- **Honey** (`--b-value`) — the colour of `b`, on every surface by D-14. An unreadable balance <!-- PUBLIC-CONSTANT: graded digest --> |
 <!-- PUBLIC-CONSTANT: graded digest --> | gauge is unreadable everywhere, so "documented limitation" was refused. Honey is not
 <!-- PUBLIC-CONSTANT: graded digest --> | repainted; it is given a **dark chip** on which gold clears 4.5:1 (≈ 8.5:1 on `--ink`).
 <!-- PUBLIC-CONSTANT: graded digest --> |
**The audit that drove this was itself corrected, by the positive control this order mandated.** <!-- PUBLIC-CONSTANT: graded digest --> |
The prior "161 failures, dark-on-dark across the modules, `--guard` failing" was inflated by a <!-- PUBLIC-CONSTANT: graded digest --> |
probe that mis-read `color(srgb …)` computed backgrounds — the founder's required positive <!-- PUBLIC-CONSTANT: graded digest --> |
control caught it, the corrected probe found the true set (135), and the remediation took it to <!-- PUBLIC-CONSTANT: graded digest --> |
0. No level is claimed here beyond the axes above; the accessibility line is a **measured pass**, <!-- PUBLIC-CONSTANT: graded digest --> |
not a badge. <!-- PUBLIC-CONSTANT: graded digest --> |
