# GRANT · Level 1 · the eight surfaces — **TIER R · SUPERSEDED**

> ## SUPERSEDED by [`2026-07-18-L1-surfaces-TIER-R-v2.md`](2026-07-18-L1-surfaces-TIER-R-v2.md)
>
> Kept in place and unedited below. Superseding replaces a record and leaves the original
> legible; amending would let a grant quietly acquire a strength it was never given (§5.1a).
> Its digests are from commit `f52bc6f` and its D1 check was directory-level. Both are
> superseded — the disclosure now lives in every artifact.

**Supersedes** `2026-07-18-L1-surfaces.md` (Tier P, provisional). Superseding, never
amending — ladder §5.1a. The provisional record stays in place, marked superseded, because
the history is the argument.

> ## Do not take our word for it. Here is the command.
>
> ```sh
> git clone https://github.com/skaists/beehive-WELLness
> cd beehive-WELLness
> sh verify/verify-l1.sh
> ```
>
> No account, no token, no membership, no permission from us. Public repository, public
> pages, stock `curl` and `sha256sum`.
>
> **If your result differs from ours, ours is wrong.** Tell us at the address in
> [`SECURITY.md`](../SECURITY.md).

---

## Why this is Tier R and not Tier X

The obvious route to a stronger grant is a more independent auditor. That route is wrong
for this project, and the reason is the project's own thesis.

**A grant from a trusted stranger relocates the trust; it does not remove it.** BNR exists
so a person's own sensing is verifiable *by them*. So the strongest grant is not *"a
reputable party checked this"* — it is *"here is the procedure, run it yourself."*

A grant anyone can re-derive **dominates** a grant from a party you happen to trust,
because it survives that party being wrong, compromised, bored, or gone in ten years.
Tier X rests on a stranger's continued care and existence. Tier R rests on nothing.

---

## The grant

| Field | Value |
|---|---|
| **Level** | **L1** |
| **Tier** | **R — reproducible** (ladder §5.1b) |
| Axis A · Artifact | 1 |
| Axis D · Delivery | 1 |
| Axis O · Operations | 0 — no requirement at L1 |
| Graded commit | `f52bc6f2202f4f268322944a1088d33891b61d09` |
| Verifier | `verify/verify-l1.sh` at the same commit — the tool is pinned as tightly as the artifacts |
| Date | 2026-07-18 |
| Granting party | **any reader.** That is the point. |

## The expected output, verbatim

Run the command above and you should see exactly this. Diff it character for character.

```
artifacts_enumerated=8
A1a_external_references=0 (over 8 files)
A1b_fetched=8 served_matches_recorded=8
D1_disclosure=present (directory-level, in surfaces/README.md)
RESULT=PASS
```

Exit code `0`. **Exit `2` is a refusal, not a pass** — it means no verdict could be formed
and something is wrong with the inputs or your environment.

## The graded digests

| Surface | sha256 |
|---|---|
| `beehive_wellness_index.html` | `77c7c05bcc5343f2c4dd7824606036e1cad3b9a5ae92cc34cbca4da6db0ca600` <!-- PUBLIC-CONSTANT: graded digest --> |
| `module1_hemp_hearts_label_reader.html` | `b30e55a38f4fcb4f99d867e4ce16633b5530236a827a6fb426d71d1be815aa7f` <!-- PUBLIC-CONSTANT: graded digest --> |
| `module2_fats_label_wont_show.html` | `7c7755c4553962ea0487a422c9cef3e25bffe873746f9ec8aa2266131d9ad5dd` <!-- PUBLIC-CONSTANT: graded digest --> |
| `module3_portion_reality.html` | `999d0961f7471d9c3fd0423d3026542823e3c34ed3d4d3292d53df7e4d450344` <!-- PUBLIC-CONSTANT: graded digest --> |
| `module4_decide_for_yourself.html` | `e89c0b9bb84496b8c8cf6d2973da63f178aaa189066c333e78e2f73f309aabeb` <!-- PUBLIC-CONSTANT: graded digest --> |
| `d12_fat_scan_result.html` | `70254ae128d61e75d1898627b1274126c8bfbe42ce5fb06b8c37e97f8e8254c4` <!-- PUBLIC-CONSTANT: graded digest --> |
| `blOVErai_companion_surface.html` | `954e9980737eb9e07c858b8f650f798f7ed358abe673ae8662f40d06c5a5ae39` <!-- PUBLIC-CONSTANT: graded digest --> |
| `bqueenbee_analytics.html` | `57ab6921efab5c53b5a6b5706aaada5f7ab0e76a24a9e4c327323abf2ae9ceb3` <!-- PUBLIC-CONSTANT: graded digest --> |

A byte change voids this grant for the changed file. That is not a loophole — it is why the
digests are here.

## The verifier was watched biting (Law 2)

A tool that has only ever been observed passing is a decoration. This project has shipped
two such gates already — scanners that reported `clean` over zero files, and a `grep -v`
basic-regex bug that silently disabled an exemption and was caught only because a *positive*
control failed.

So `sh verify/verify-l1.sh selftest` runs **twelve fixtures on every CI build**:

| Fixture | Expected | Result |
|---|---|---|
| intact copy | pass | ok |
| `<script src="https://…">` | fail | ok |
| `<link href="https://…">` | fail | ok |
| CSS `@import url(https://…)` | fail | ok |
| CSS `background:url(https://…)` | fail | ok |
| runtime `fetch("https://…")` | fail | ok |
| protocol-relative `src="//…"` | fail | ok |
| **inert URL in text** | **pass** | ok |
| one byte flipped, size and refs unchanged | fail (digest alone) | ok |
| disclosure removed | fail (D1) | ok |
| **empty file set** | **refuse, exit 2** | ok |
| wrong-origin serve | fail | ok |

Run it yourself: `sh verify/verify-l1.sh selftest`.

---

## What this does not buy — carried forward unsoftened

**A reproducible L1 is a strong L1. It is not a higher level**, and this record must not
read as though it were. The three limitations from the provisional grant do not disappear
because the tier went up; a stronger grant that quietly drops the weaker one's caveats is a
worse grant.

1. **Digest provenance is trust-on-first-use.** This verifies local == recorded == served.
   It does **not** establish that the digests were recorded *before* publication rather than
   back-filled to match. That rests on the git history, which the verifier does not audit.
2. **Serving is point-in-time.** The bytes matched when you ran it. The disclosure itself
   concedes the operator can change or withdraw them at any time. Nothing in A1 binds future
   delivery.
3. **No egress-blocked render was observed.** Zero external references implies it *by
   construction* — there is nothing to fetch — but nobody has watched a browser render these
   with the network unplugged.

And what L1 is not: these surfaces are **not** content-addressed, **not** retrievable from
independent gateways, and **not** censorship-resistant. Those are L2 and L3 properties and
this tree does not hold them. One operator serves these and can withdraw them.

## One divergence from the order, recorded

Order E-2 specified that D1 be confirmed *"present in each artifact."* The delivery
disclosure is **directory-level** — in `surfaces/README.md`, covering the published set —
and appears in **zero of the eight** HTML files. The verifier therefore checks it where it
actually is and labels the output `directory-level` rather than asserting something untrue
about the artifacts.

The alternative was pushing the disclosure into all eight files, which would void eight
clearances and eight grants to satisfy a wording. **Open for founder ruling**; the verifier
states which reading it implements so no reader has to guess.

---

**Audited, not proven.** The strongest claim here is *sound by construction*, for item 3
only. A1 and D1 are met on evidence any reader can regenerate.
