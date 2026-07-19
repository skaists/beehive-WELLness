# GRANT · Level 1 · the eight surfaces — **TIER R (reproducible)**

**Supersedes** [`2026-07-18-L1-surfaces-TIER-R.md`](2026-07-18-L1-surfaces-TIER-R.md),
which superseded the provisional grant. Superseding, never amending — ladder §5.1a. Both
earlier records stay in place beneath their banners, because **the history is the argument**.

**Two supersessions in one day is not embarrassing — it is the record working.** The first
grant was honest about its weight and obsolete by evening; the second was obsolete by the
next order; both are still readable.

---

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

## The grant

| Field | Value |
|---|---|
| **Level** | **L1** |
| **Tier** | **R — reproducible** (ladder §5.1b) |
| Axis A · Artifact | 1 |
| Axis D · Delivery | 1 |
| Axis O · Operations | 0 — no requirement at L1 |
| Graded commit | `9d271e8be3e69ed5fa500159f694f9a197299614` |
| Verifier | `verify/verify-l1.sh` at the same commit — the tool is pinned as tightly as the artifacts |
| Date | 2026-07-18 |
| Granting party | **any reader.** That is the point. |

## The expected output, verbatim

Run the command above and you should see exactly this. Diff it character for character.

```
artifacts_enumerated=8
A1a_external_references=0 (over 8 files)
A1b_fetched=8 served_matches_recorded=8
D1_disclosure_in_artifacts=8/8
RESULT=PASS
```

Exit code `0`. **Exit `2` is a refusal, not a pass** — no verdict could be formed.

## What changed since the superseded grant

**D1 is now satisfied in each artifact, not at directory level.** All eight footers carry a
byte-identical disclosure:

> Served by one operator who can log every reader and withdraw this page at will. Not
> censorship-resistant, not unobserved.

A reader arriving by direct link — the normal case — now receives the disclosure. Previously
they received none, because it lived only in a directory README they never saw. D1 was **not**
amended to accept the weaker reading; the artifacts were changed to meet the ratified rule.

The output line reads `D1_disclosure_in_artifacts=8/8`. The old `directory-level` label is
gone — a label describing the superseded reading would be a stale claim.

## The graded digests

| Surface | sha256 |
|---|---|
| `beehive_wellness_index.html` | `33657fe5822b4bc47f3895afff2a702bf3bbacfcf2ca5635c8f0e47dbe9df834` <!-- PUBLIC-CONSTANT: graded digest --> |
| `blOVErai_companion_surface.html` | `768e5a1ab75b823804b3939a5726ab7d1d3e37bbee6c7d483893bf98ff32d4b7` <!-- PUBLIC-CONSTANT: graded digest --> |
| `bqueenbee_analytics.html` | `d5a16072e139902e0581f97656891d534b8990dd8e2dd6d7d12527dd957e84fb` <!-- PUBLIC-CONSTANT: graded digest --> |
| `d12_fat_scan_result.html` | `1dca9a215ec0f7c5fb57476053449d923ae76856cd1404cd07e9b2b8f1ac092c` <!-- PUBLIC-CONSTANT: graded digest --> |
| `module1_hemp_hearts_label_reader.html` | `dd1d28cd10a30ff2ceb90839784ed7bb5fff1cf16c7c2e3863d1415bfb6c8dc4` <!-- PUBLIC-CONSTANT: graded digest --> |
| `module2_fats_label_wont_show.html` | `1f6a0473d2f7010ecf76e6915fc61e14934c6120ad914b0b140a4fa3345b5171` <!-- PUBLIC-CONSTANT: graded digest --> |
| `module3_portion_reality.html` | `ee21fe30dd0b8bfbea90e7b727d4eb8349837b8297f983b1ed35e3be80a9fbe0` <!-- PUBLIC-CONSTANT: graded digest --> |
| `module4_decide_for_yourself.html` | `55ed72c7d21b44ff6009cad3e95cff8670c57b12baecc9ec47152027515e8f4d` <!-- PUBLIC-CONSTANT: graded digest --> |

A byte change voids this grant for the changed file. That is why the digests are here.

## The verifier was watched biting (Law 2)

`sh verify/verify-l1.sh selftest` runs **14 fixtures on every CI build**. Run it yourself.

The three added for this grant test D1 per-artifact, and the first is the one that carries
the argument — the disclosure removed from a **single** artifact while the README stays
intact:

```
OLD check (directory-level)  ->  D1 PASS   <- would have shipped
NEW check (per-artifact)     ->  D1 FAIL   <- bites
```

Same bytes, opposite verdicts. A refined check nobody watched change behaviour is a
refactor wearing a fix's clothes.

| Fixture | Expected |
|---|---|
| intact copy | pass |
| `<script src>`, `<link href>`, `@import`, `url()`, `fetch()`, protocol-relative `//` | fail (six fixtures) |
| inert URL in text | **pass** — text is not a reference |
| one byte flipped, size and refs unchanged | fail (digest alone) |
| **disclosure removed from one artifact, README intact** | **fail** |
| disclosure **altered** in one artifact | fail — identical wording is the property |
| disclosure in README only, absent from all eight | fail |
| **empty file set** | **refuse, exit 2** |
| wrong-origin serve | fail |

---

## What this does not buy — carried forward unsoftened

**A reproducible L1 is a strong L1. It is not a higher level.** The three standing
limitations do not disappear because the tier went up or because D1 got stricter; a stronger
grant that quietly drops the weaker one's caveats is a worse grant.

1. **Digest provenance is trust-on-first-use.** This verifies local == recorded == served.
   It does **not** establish the digests were recorded *before* publication rather than
   back-filled. That rests on the git history, which the verifier does not audit.
2. **Serving is point-in-time.** The bytes matched when you ran it. The disclosure now
   printed on every page concedes the operator can withdraw them at will.
3. **No egress-blocked render was observed.** Zero external references implies it by
   construction — there is nothing to fetch — but nobody watched a browser do it.

These surfaces are **not** content-addressed, **not** retrievable from independent gateways,
and **not** censorship-resistant. Those are L2 and L3 properties and this tree does not hold
them. One operator serves these and can withdraw them — which is now stated on every page
rather than in a README most readers never open.

## Clearance status, stated plainly

**All eight founder clearances are void** as of this byte pass — the second reset in one
day. A grant is not a clearance: this record certifies that the served bytes match what was
recorded and carry no fetchable external references and carry the disclosure. It does **not**
certify that a human has read them for category. That eyeball is owed.

---

**Audited, not proven.** The strongest claim here is *sound by construction*, for item 3
only. A1 and D1 are met on evidence any reader can regenerate.

---

## Line-ending durability — checked after the fact, and it holds

An R-tier grant promises *"clone it and run the command yourself."* That promise is only
worth what it survives on **someone else's machine**, and a clean-clone check run under the
author's own git configuration establishes that it works *there* — not that it works.

Git's `core.autocrlf` rewrites line endings on checkout. Where it applies, a digest-pinned
artifact hashes differently for the reader than for the author, and **the reader's honest
conclusion is that the file was tampered with.** A digest that fails on clone is worse than
no digest at all.

This was checked properly rather than assumed. `.gitattributes` in this repository carries
`* text=auto eol=lf`, which forces LF on checkout regardless of the reader's setting, and
the surfaces were re-cloned under **every** relevant configuration:

| clone config | digests matching |
|---|---|
| `core.autocrlf=true` (Windows default) | **8/8** |
| `core.autocrlf=input` | **8/8** |
| `core.autocrlf=false` | **8/8** |

The verifier was then run end-to-end from a clone made under `core.autocrlf=true` — the
hostile case — and reproduced L1 in full.

**Why this is recorded rather than merely done:** the sibling repository
`beehive-biomass/bNATURE.bio` pinned a fixture digest *without* this protection and the
digest did not survive a clone. It was caught, fixed with `fixtures/** -text`, and
re-verified by re-cloning rather than by re-hashing a working copy. The same exposure would
have made this grant nominal in exactly the case it exists to serve. It does not, and now
there is a record saying so with the configurations named.
