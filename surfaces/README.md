# surfaces

Static, L1, zero-network concept surfaces. **Rehearsed on GitHub Pages; a surface crosses
to Arweave (L2) only after a per-surface category clearance** — there is no unpublish.

## Layout — flat, deliberately

The eight files sit **flat, as siblings**, not one-folder-each. That is a decision, not
laziness: the index links to flat siblings (`href="module1_hemp_hearts_label_reader.html"`),
so foldering them would require rewriting the index's seven hrefs — i.e. **modifying an
artifact that was already cleared**. Clearance attaches to specific bytes; a rewritten
index is a different artifact wearing an approval it never earned. Flat keeps every file
byte-identical to what was cleared *and* every link resolving (verified, not assumed).

## Clearance chain

Cleared by the founder (eyeballed) → homed and **category-reviewed** by the Code seat
before publish. Category review means reading each surface and judging *category*, not
grepping for words — "grep found nothing" is not a clearance.

| surface | sha256 (cleared bytes) |
|---|---|
| `beehive_wellness_index.html` | `44772611597a701dda95c1bbf9cb3e8b694fb6253f2d7d739ba092f973fa7a49` <!-- PUBLIC-CONSTANT: content digest, re-cleared L-1 --> |
| `module1_hemp_hearts_label_reader.html` | `909917a1cb5049f35fe6bc78993a0f9e66add8b3cfdb1202021d05b0641317f2` <!-- PUBLIC-CONSTANT: content digest, re-cleared L-1 --> |
| `module2_fats_label_wont_show.html` | `948e31ca56698cc65e4722651237ff095b7a676643ffb3a1770f4cd45f706bf9` <!-- PUBLIC-CONSTANT: content digest, re-cleared C-A then L-1 --> |
| `module3_portion_reality.html` | `99bc8829b2bb9e6bb3d2e9e28c9ef33df8d0b650fa01f7e290e9458f8cd8ba24` <!-- PUBLIC-CONSTANT: content digest, re-cleared C-A then L-1 --> |
| `module4_decide_for_yourself.html` | `b43a158bc20c84de6597aafc21ff22a183d5f5ed84761f162bbf489ee60758fa` <!-- PUBLIC-CONSTANT: content digest, re-cleared L-1 --> |
| `d12_fat_scan_result.html` | `70254ae128d61e75d1898627b1274126c8bfbe42ce5fb06b8c37e97f8e8254c4` <!-- PUBLIC-CONSTANT: content digest, re-cleared C-E --> |
| `blOVErai_companion_surface.html` | `74e41657b813fdeaed3524c18b0fed37e1d91cd0319ca02f7cf20c77917be056` <!-- PUBLIC-CONSTANT: content digest, re-cleared L-1 --> |
| `bqueenbee_analytics.html` | `d7bd6bf5ab92d327f95addab5b512e20f138e0b8265f38ff30704c18fe44c945` <!-- PUBLIC-CONSTANT: content digest, re-cleared L-1 --> |

## Mechanical verification (all eight)

- **Zero network** — no `http(s)://`, no CDN, no `@import`, no remote fonts, anywhere.
- **Links** — the index's 7 hrefs all resolve as siblings.
- **CONCEPT-tagged** — every surface carries it.
- **Gates** — secret-scan and PHI-marker scan clean.

## Category review — findings

All eight clear. Notes worth keeping:

- **Module 2** grades its own claims: an explicit *Proved* / *Still argued — teach as
  claims* split, with every disease-outcome claim (omega-6:3 ratio, saturated fat and
  heart-disease risk, chronic-disease outcomes, MCT benefits beyond metabolism) parked in
  the argued column rather than asserted. That is the tree's grading rule, self-applied.
- **Module 4** contains `"eat X to cure Y"` and `"hemp cures inflammation"` — both as the
  thing being *refused*. The capstone teaches readers to source-check exactly the claim
  class this tree will not publish.
- **bLOVErAi** lists `diagnose, prescribe, set a target/deficit/goal weight` in its
  **"it will not"** column, with the ceiling stated: *consider discussing this with a
  clinician*. Its nearest edge — "flag *this is outside the usual range* and teach you to
  read it" — stays observational, paired with literacy, and never names a condition.
- **Module 3** carries the ED-safety note with a **live** helpline (National Alliance for
  Eating Disorders — not the discontinued NEDA line), no goal weight, no deficit.
- **bQueenBee** draws both walls at once: de-identified aggregate only ("counted, never
  named") and participation/evidence metrics, explicitly *not* health outcomes.

## C-E · D-12 v2 — per-value provenance, and the disagreements kept

`d12_fat_scan_result.html` gained a provenance layer: every value carries its **origin and
its basis**, and an independence chip driven by a count that is *derived from provenance,
never assigned* (the `food-composition` crate, order C-F). Bytes changed, so **clearance
did not travel** — the digest above is new and the L-1 one is void.

**Category review (Code seat, read not grepped).** Still Axis A only: nutrient numbers and
where they came from. No directive to the reader, no condition named, no dose, no intake
advice; the scope line is unchanged. The new material is *about the data's provenance*,
which is a literacy subject rather than a health subject — it teaches the reader that a
number has a source and a basis, and that copies are not corroboration.

**The wording is the risk here, and it is where this surface is most likely to fail.** A
provenance display fails by asserting a property the data lacks — the same defect as
"re-identification is impossible", one domain over. So: one source reads **1
determination**. Not "unverified" (a judgement), not "verified" (a claim). Verified,
confirmed, corroborated, proven, guaranteed and impossible appear **zero times in the
rendered body** — the two occurrences of "verified" in the file are the CSS token name
`--verified` inside the header comment stating the colour law. Each "never" was read in
context and describes *what this page does* (never averaged, never estimated), never a
property of the data.

**Colour law verified against computed styles, not intent.** No provenance element resolves
to a green token: badges and chips compute to `--ink-mut`/`--ink-dim` (neutral), spread to
`--info`, the plausibility flag to `--guard`. Green stays unbound from the count, because
colouring agreement as success would smuggle the banned claim back in through hue.

**The three divergences are rendered and none is resolved** — hemp protein 42% vs 31.6 g as
a *basis* difference with no spread reported (two foods, not two opinions); hemp iron
0.28 vs 7.95 mg as 2 independent determinations with the spread shown and no winner picked;
beef potassium 252 g flagged `exceeds the whole food` and left exactly as published.

**RTL (E-5):** 33 machine strings carry both halves of the rule and were verified by range
geometry to stay LTR-ordered under `dir="rtl"`. A wrapping defect was found and fixed in
passing — two quantities split across lines, which also made the geometric test meaningless
until `white-space:nowrap` put them back on one line.

## L-1 · the L1 badge removed from all eight footers — clearance void across the board

The C-G audit found that **`L1` is defined nowhere in this repository.** The token appeared
20 times — here, in the root README, in `dockets/`, and in all eight surfaces — with no
criteria, no rubric, and no stated judge. The founder ruled the badge comes off the
footers pending a ratified ladder: *an undefined level displayed is a claim.*

**What was changed:** exactly one string, ` · L1 · ` → ` · `, in the footer line of each of
the eight surfaces. Nothing else. The diff is 8 lines, one per file.

**What it cost, stated plainly.** Every surface changed bytes, so **all eight digests are
new and every prior clearance is void — including the six that still carried the founder's
own eyeball.** Before this edit the chain was six founder-eyeballed plus two Code-seat
re-clearances; after it, **zero of the eight carry a founder eyeball.** That is a real
downgrade of the clearance chain, accepted deliberately because a live undefined claim on
a published surface outranks clearance bookkeeping. It is recorded rather than absorbed
quietly, and it means **all eight need a founder re-eyeball**, not just the two from C-A.

**Deliberately not changed:** the `L1 zero-network` note in each file's HTML header
comment. The ruling addressed the footers, and the objection was to a *displayed*
undefined level; a header comment is a design note explaining why tokens are inlined, not
a badge shown to a reader. Whether those 8 comments and the 3 remaining markdown mentions
also go is open, and is a second question rather than an assumed extension of the first.

**Still open on a founder ruling:** the word "proven" in `bqueenbee_analytics.html` and
`module4_decide_for_yourself.html`. Those two files will therefore take a *second* edit if
the ruling removes it — two clearance events where one would have done. That sequencing
cost is the direct consequence of ruling the badge before ruling the wording, and is noted
so the next batch can be ordered the other way round.

## C-A · the one open item, now closed — and the two modules re-cleared

The open item was Module 3's *"most people are over-fed and under-nourished"* — an
ungraded population assertion sitting two clicks from Module 2's own grading rule. The
founder resolved it with sourced, founder-ruled replacement copy (WHO/FAO TRS 916, Joint
WHO/FAO Expert Consultation, Geneva 2003).

- **Module 3** — the ungraded assertion is gone (0 occurrences). In its place, a sourced
  block in the same visual register as Module 2's Proved split: WHO/FAO grades high intake
  of energy-dense, micronutrient-poor foods as *convincing* for weight gain (TRS 916,
  Table 7), and notes obesity and chronic undernutrition coexisting in populations in
  economic transition. The trailing scope line — **"Population-level, not a statement
  about you"** — is retained as load-bearing ED-safety copy. The completeness-not-
  restriction pedagogy and the ED-safety note (live helpline) are unchanged.
- **Module 2** — a sourced row added to the *Proved* side: the TRS 916 §5.1 population
  nutrient goals, explicitly labelled *population goals from a named consultation, not
  individual targets*. They are categorically different from the ratio arguments the
  module correctly parks in "still argued."

**Clearance did not travel.** Both files changed bytes, so the founder's original
clearance is void for them. Both were re-verified mechanically (zero network, links,
CONCEPT tag) and re-read for category.

> **Two sentences in this section are superseded — left in place, corrected here, rather
> than quietly rewritten, because this file is the clearance record and editing away its
> own history is the failure it exists to prevent.**
> 1. *"the old ones are recorded nowhere and must not be reused"* — inaccurate as written.
>    The superseded digests are recoverable from git history in one command
>    (`git log -p --all -- surfaces/README.md`). **Absent from the served tree** is the true
>    and sufficient claim. Flagged by the C-G audit.
> 2. *"The other six are byte-unchanged and keep their original clearance"* — true when
>    written, **false since L-1**, which changed all eight footers. No surface now holds its
>    original founder clearance. See the L-1 section above.

Provenance of this change, stated plainly: **the replacement copy is founder-authored and
founder-ruled**; the Code seat integrated it, escaped it for HTML, matched the register,
and re-reviewed category. The rendered result has not been re-eyeballed by the founder —
if that matters for these two, it is a re-eyeball, not a rebuild.

**Nearest edge in the new copy:** Module 2's row now carries numeric goals (`saturated
<10%` and the rest). Numbers in a nutrition course sit close to advice; what keeps them on
the right side is the same-line scope label — population goals, not individual targets —
and their placement as a *sourced fact about what WHO published*, not a recommendation to
the reader. If that label is ever dropped for length, the row changes category.
