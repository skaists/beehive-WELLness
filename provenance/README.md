# provenance — the eleven-year lineage

This tree did not start in 2026. `health-vault` — a personal, patient-controlled health
record with no custodian — was written down as a project proposal on **14 January 2015**,
by the founder, then a practicing Physician Assistant. What we specced this year as a
design is, in substance, that document.

## The original statement (founder's own words, 2015)

From `My Health Safe project proposal.docx`, 14 January 2015:

> "Project SAFE … has inspired me to build personal controlled electronic health record
> system. I am a practicing Physician Assistant … who sees both the amazing potential of
> electronic health record system, but also the destructive potential. Unfortunately with
> the continued progression of big business corporate development the latter option is
> being realized. With the creation of a true decentralized network it is now possible for
> health care providers and patients to take back ownership of their health information."

Quoted with the author's consent. **The source documents are not mirrored into this
repository** — the lineage is recorded, the private material stays private. What is public
here is the founder's own words and nothing else's.

## The derivation chain

Three lines of 2014–2015 research resolve into properties this tree now enforces in types:

| 2015 source | What it became | Where it lives now |
|---|---|---|
| **MaidSafe self-encryption** — data encrypted by its own content, no server holding plaintext | zero-plaintext, **no-custodian** storage; a vault whose operator cannot read it | the storage seam behind `health-vault` (counsel-gated, off by default) |
| **Nymi + Quantum-Secure Authentication** — identity as a live, continuous signal rather than a stored credential | **PoUL as a living thread**, not an account — the thing that cannot be farmed by making more accounts | `Did` / PoUL thread; the distinct-counterparty edge rule |
| **VistA corpus** — `VistA_Monograph.pdf`, the 2014 evolution program plan, the Terry Cullen deck | provider-side **interoperability** (FHIR / Lighthouse shape) | `health-vault`'s FHIR R4 resource mapping |

The third line is the one worth stating plainly: **FHIR/Lighthouse interoperability was
filed in our specs as a *stretch* goal. The 2015 record shows it was the origin.** We
rediscovered the starting point and mistook it for an extension.

## Category review (Code seat, not inherited)

The quotation was reviewed for category before commit rather than accepted on the design
seat's read. **It is not a health claim.** It asserts nothing about any body, treatment,
nutrient, or outcome; its entire subject is *records custody and system architecture* —
who holds health information and on whose terms. That is a governance/ownership problem
statement that happens to be about health data, which is why it lives in this tree and not
in the kernel.

Two nuances recorded rather than smoothed over:

- **The clinician credential is biography, not warrant.** "Practicing Physician Assistant"
  establishes why the author was in the room in 2015. It is not offered — and must never be
  used — as clinical authority for any claim this tree makes. Nothing here inherits medical
  weight from it.
- **It names no actor.** "Big business corporate development" is a description of a market
  vector, not an allegation against a party. That is the same discipline the wall runs on:
  design against the capture vector, never allege an actor.

## Why this is recorded at all

Not sentiment. A design whose properties are eleven years old, written before there was
anything to sell, is evidence the properties are load-bearing rather than fashionable —
and the record of *when* a claim to originality was made is itself the honest kind of
provenance. It also fixes an attribution error: we would otherwise have credited 2026 for
a 2015 thesis.
