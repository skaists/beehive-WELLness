# AUDIT — beehive-WELLness at `209a2b6`

> **Status since publication.** The founder accepted all findings and owned the lineage of
> the top three. Defects 1–3 are fixed (`33a84a8`); the kernel's copy of the fail-open
> guard is fixed in its own tree (`1c84d8f`); and the L1 badge has been removed from all
> eight footers, voiding every prior clearance — see `surfaces/README.md` § L-1.
> **The digests recorded below are superseded.** `surfaces/README.md` holds the current
> table; this file is a record of what was true at `209a2b6` and is not updated to track
> the tree. Findings 4–8 remain open, and the L1 stamp remains withheld until a ladder is
> ratified.

End-to-end audit ordered as C-G. Five checks, each adversarially re-run by three
independent skeptics briefed to refute rather than confirm, plus a completeness critic
asked what the audit failed to look at. 21 reviewers, all read-only.

**Audited tree:** `209a2b6606b77b744b3c61051f66396483149743`, working tree clean.
This file is the next commit and is not itself part of the audited bytes.

---

## The stamp is WITHHELD

C-G ordered this audit "stamped **L1 achieved**." It is not stamped, and the reason is
the first finding:

> **`grep -rniE "L1 (means|=|is )|layer 1|\"L1\"" --include=*.md .` → no match.**
> The token **L1 appears 20 times** — `README.md:49`, `dockets/README.md:12`,
> `surfaces/README.md:3`, and in all 8 surface files (header comment + footer) — and
> **nothing in this repository says what L1 is, what a surface must satisfy to be L1,
> or who judges it.**

A level with no written criteria cannot be achieved, because there is nothing to
achieve. Stamping it would be the precise failure this project's language law exists to
prevent: displaying a grade whose rubric does not exist. The standing rule is *display
the level, never claim one* — and there is no level here to display, only a badge that
has been repeated until it reads like one.

Two further reasons, either sufficient on its own:

- **Check 3 (guards) did not survive.** 2 of its 3 skeptics refuted it, and an
  independent defect was found in the guard scripts themselves (below).
- **Fail-closed.** Indeterminate is blocked, never "proceed anyway."

**What the audit does establish:** the eight surfaces are byte-intact against their
recorded digests, carry no network dependency, resolve every internal link, and the
licence edge to the AGPL kernel runs one way. Those are real, and each is receipted
below. They are simply not the same thing as a level.

Nothing here crosses to Arweave. No L2 write is proposed, recommended, or prepared.

---

## Verdicts

| # | Check | Verdict | Refuted by | Survives |
|---|---|---|---|---|
| 1 | Served bytes hash to cleared bytes | PASS (with notes) | 0 / 3 | yes |
| 2 | Links live · zero external · CONCEPT tag | PASS (with notes) | 1 / 3 | yes |
| 3 | Guards green · exemption still bites | **DID NOT SURVIVE** | **2 / 3** | **no** |
| 4 | The cargo/licence edge is one-way | PASS (with notes) | 1 / 3 | yes |
| 5 | Clearance chain reconciled | PASS (with notes) | 1 / 3 | yes |

Every verdict is `PASS_WITH_NOTES`, not `PASS`. No check came back clean, and that is
the honest shape of a first audit.

### Check 1 — digests

8/8 surfaces hash exactly to the table in `surfaces/README.md`. Counts reconcile: 8
files, 8 `PUBLIC-CONSTANT` markers, 8 rows, no orphan either way. The two C-A
re-cleared rows carry new digests; hashing the pre-C-A blobs out of commit `d1a9d35`
reproduces the two superseded digests exactly, which confirms the re-clear was a real
byte change and not a relabel. Neither superseded digest appears in the served tree.

This document deliberately **does not restate the eight digests.** A second copy is a
second source of truth, and the drift it invites is the failure this tree already
avoided by linking `tokens.css` instead of inlining it. The table in
`surfaces/README.md` is the record; this check says it is accurate.

*Note:* the check proves digest-to-bytes correspondence only. A tampered file plus a
tampered row would pass it identically. Binding the table to an out-of-band clearance
record is not done and is not claimed.

### Check 2 — zero network

No `http(s)://`, CDN, `@import`, remote font or remote `src`/`href` in any of the 8
surfaces. All 7 index hrefs resolve to real sibling files. CONCEPT tag present in 8/8.

### Check 3 — guards *(did not survive)*

What holds, with receipts: both scanners are green on the real tree over 37 tracked
files, and both genuinely run in CI (`guards.yml` steps `secret-scan (tree)` and
`phi-scan (tree)`, observed passing at this commit — a guard CI does not run is
decoration, and these are not that).

The per-line exemption was demonstrated to still bite, on a copy, never the repo:

```
baseline (markers intact)          → secret-scan: clean, exit 0, 37 files enumerated
strip PUBLIC-CONSTANT/TESTNET-ONLY → exit 1, 8 lines flagged
```

Eight flagged lines for eight digest rows: the exemption is per-line, not per-file.

**The defect — the guards fail open.** Both scanners enumerate via `git ls-files`
(`secret-scan.sh:15`). Run where git yields nothing, the loop never executes, `fail`
stays 0, and the script prints `secret-scan: clean` and exits 0 **having inspected zero
files**:

```
$ cd /a/directory/that/is/not/a/git/repo
$ sh secret-scan.sh tree
secret-scan: clean          ← scanned nothing
exit 0
```

A guard that reports clean without looking is a fail-open, and this project's rule is
fail-closed. It is not exploitable inside CI, which checks out a real repo — the risk is
a standalone or misdirected invocation reading as a pass. **The kernel's
`scripts/secret-scan.sh` shares the defect** (same `git ls-files` enumeration,
`:53`), so the fix belongs in both trees.

This was found by the audit's own first attempt *failing this way* — a marker-strip test
run against a non-git copy returned "clean," which would have been reported as the
exemption having no teeth. The test was wrong, not the exemption; re-run against a real
git copy it bit correctly. A check that passes vacuously and a check that fails
vacuously are the same bug wearing different clothes.

### Check 4 — the one-way edge

The strongest-receipted check. All three crates declare `MIT OR Apache-2.0`.
`health-vault` pins the kernel's `type-bindings` at rev `270490f`, which is itself
`MIT OR Apache-2.0`, so no AGPL is linked. The fully resolved graph:

```
fat-profile, health-vault, itoa, proc-macro2, quote, serde, serde_core,
serde_derive, serde_json, syn, type-bindings   = MIT OR Apache-2.0
memchr = Unlicense OR MIT     zmij = MIT
unicode-ident = (MIT OR Apache-2.0) AND Unicode-3.0
```

**14 packages, zero AGPL.** The kernel contains no dependency edge back into this tree;
its only mention of WELLness is a prose `description` string.

### Check 5 — clearance chain

The chain is honest about its own weakest point: `surfaces/README.md` records that C-A
voided the founder's clearance for two modules, states the new digests, and does **not**
claim the founder re-eyeballed the rendered result. Clearance is therefore not uniform
across the eight — six carry a founder eyeball, two carry a Code-seat re-clearance —
and the document says so rather than smoothing it.

---

## Defects found

Ranked by consequence. None were fixed during the audit: **changing an artifact while
it is being audited makes the audit worthless**, so every item below is recorded against
`209a2b6` and repaired afterwards, as separate work.

**1 · Privacy overclaim in the health crate.** `crates/health-vault/src/lib.rs:6` and
`:35`, and `SECURITY.md:12`:

> "Re-identification is impossible from these types"

The structural fact underneath is true and verified: no `name`/`mrn`/`ssn`/`dob`/
`address` field exists, and the subject is a pseudonymous `Did`. The claim built on it is
not. Re-identification does not require an identifying field; it requires linkage. `Did`
is the canonical BNR principal, **deliberately resolvable elsewhere in the system**, and a
stable pseudonym plus quasi-identifiers — LOINC codes, values, timestamps — is linkable.
The moment one `Did` is tied to a person, every record under it is retroactively
re-identified. That is what the identifier is *for*.

Three violations at once: a banned absolute, a comment claiming a property the code lacks
(k001), and a privacy overclaim in a public health-data repository — the single sentence
most likely to be quoted back by a regulator or a plaintiff. The defensible form is what
the code actually earns: *these types carry no direct identifier; that is not an
anonymity guarantee.*

**2 · The README's licence rationale is false.** `README.md:35` says `health-vault`
"does not link any kernel crate and pins nothing from the kernel"; `README.md:45` says
the workspace "pins **NO** kernel crate today." Both are untrue — `health-vault`
pins `type-bindings` from the kernel repo. The licence *conclusion* survives, because
`type-bindings` is permissive, but the stated *reason* is false, and it is false in the
most-read file in the repository. The README describes the world before `type-bindings`
existed and was never updated when the pin landed.

**3 · Guards fail open.** Both trees. Detailed under Check 3.

**4 · Banned word inside founder-cleared bytes.** `bqueenbee_analytics.html:63,99`
("comprehensions proven", "proven understanding") and
`module4_decide_for_yourself.html:100` ("COMPREHENSION PROVEN"). **Founder ruling
required — not touched.** Clearance attaches to specific bytes; editing these would void
their clearance and substitute the Code seat's judgement for the founder's. There is also
a live question of whether the educational sense of "proven" is what the language law
aims at, which is a ruling, not a call this seat makes.

**5 · Documentation accuracy.** `surfaces/README.md` says the superseded digests are
"recorded nowhere." They are recoverable from git history in one command
(`git log -p --all -- surfaces/README.md`, commit `d1a9d35`). Absent from the *served
tree* is the true and still-sufficient claim.

**6 · Unenforced claims.** CC-BY-4.0 is asserted for curriculum and docs with no CC
licence file and no surface carrying the marker. No CI guard enforces the digest table,
so the clearance chain is documented but not mechanically defended. `dockets/README.md`
is garbled and indexes docket files that do not exist. `SECURITY.md` states a disclosure
posture but gives no reporting channel.

**7 · CI hardening.** No `permissions:` block; actions pinned to a tag rather than a
SHA; no `--locked` (the kernel uses it, so this is below kernel parity); no clippy.
`Cargo.lock` is untracked, which is a reproducibility gap — though **not** a CI failure,
since this workflow does not pass `--locked`.

**8 · Coverage gaps.** `provenance/README.md` was never swept for overclaims by any
check. The two Rust crates (~801 lines) were barely read beyond their manifests. The
JSON fixtures were pattern-scanned but not content-reviewed against the "synthetic only"
claim. No check verified the surfaces actually render in a browser.

---

## Corrections to this audit

Four claims made by the reviewers were checked and found **wrong**. They are recorded
because an audit that hides its own errors is worth less than no audit:

- *"CI has never been observed passing."* False. Run `29634732818` at `209a2b6`:
  job `scans` success (both scanners), job `cargo` success (build, test, fmt).
- *"`cargo build --locked` fails on a fresh clone with no lockfile."* False. This
  workflow does not pass `--locked`.
- *"Not independently reproducible from the public tree"* — filed as the audit's only
  **blocking** finding. **Refuted.** A clean `ubuntu-latest` runner checking out only the
  public repository fetched kernel rev `270490f` and built successfully, which
  establishes both that the pin is publicly resolvable and that the licence census
  reproduces from public bytes alone.
- *"No GitHub Pages configuration of any kind."* Misleading. `pages-build-deployment`
  runs on this commit; Pages is configured through repository settings, not a workflow
  file, so looking for a file found nothing and concluded too much.

---

## What this audit is not

It is not a claim that this tree is safe, private, or finished. It is a record of what
was run, what the commands returned, and what that does and does not support. Twenty
reviewers looking for two hours find what they find; the defects nobody looked for are
still here, and section 8 names the places they are most likely hiding.

**Audited ≠ proven. Sound by construction ≠ unbreakable.** The strongest statement these
receipts support is that at `209a2b6` the surfaces are byte-intact, network-free, and
one-way licensed — and that the guards, the README's licence rationale, and one privacy
claim need work before anything here is described as a level.
