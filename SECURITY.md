# Security & PHI policy

## No real personal health data. Ever. In the default build.

This tree teaches nutrition literacy and models health-record *structure*. It does not
collect, store, or process real personal health information in any default build.

- **Fixtures are synthetic only.** The PHI-marker guard (`scripts/phi-scan.sh`) fails
  the build on an email, SSN shape, or DOB shape in tracked files.
- **`health-vault` carries no direct identifier** — the types hold no name / MRN / SSN /
  DOB / address field anywhere, and the subject is referenced only by a `Did`. That is
  structural and checkable by reading the types. **It is not an anonymity claim.**
  Re-identification needs linkage, not an identifying field: a `Did` is a stable
  pseudonym and is the canonical BNR principal, resolvable elsewhere in the system by
  design, so linking one `Did` to a person re-identifies every record under it. Removing
  the direct identifier removes the direct identifier — nothing more is asserted here.
- **Real PHI, provider (FHIR) import, or interpretation-that-diagnoses** is counsel-gated,
  off by default, and never in a default build.

## Reporting a vulnerability

**Email: `loviswater44@yahoo.com`** <!-- PUBLIC-CONTACT: maintainer disclosure address, published by ruling R-5 -->

Report a vulnerability or a PHI-handling concern privately to that address **before**
opening a public issue. A finding that the structural properties above can be bypassed is
the highest-priority class — those properties are the reason this tree can exist.

Useful things to include: what you did, what you expected, what happened, and the commit
or digest you were looking at. A rough report sent early beats a polished one sent late.

### What to expect

- **Acknowledgement within 7 days.** If you have not heard back in that window, assume the
  mail was lost or filtered and send it again — a silent report is worse for this project
  than a duplicate one.
- **No bounty programme.** There is no payment, and this is stated up front so nobody
  spends effort on the expectation of one. Credit in the fix commit is offered by default
  and declined on request.
- **No fixed remediation clock.** This is a small project and promising a repair deadline
  it cannot keep would be the same overclaim the rest of this repository exists to avoid.
  What is promised is the acknowledgement above and an honest status when asked.

### About this address

Two things stated once, because a reader deserves them and repeating them adds nothing.

**A published address in a public repository will be scraped.** That is accepted
deliberately as the cost of having a route at all — a disclosure policy with no route is
not a policy, which is what this file was until now. If you would rather not email,
**GitHub private vulnerability reporting** on this repository is the alternative path when
enabled; it keeps the report inside GitHub and exposes no address. The two are meant to run
alongside each other, not instead of each other.

**This address is not the only public contact attributable to this project's maintainer.**
A different address appears on the FDA docket submission `mro-jl22-ajin`. That is not a
conflict and neither supersedes the other — it is recorded here so the difference reads as
deliberate rather than accidental to anyone who encounters both.

## Scope

In scope: this repository — the crates, the guard scripts, the CI workflows, and the
static surfaces under `surfaces/`.

Out of scope: the delivery layer. These surfaces are served by GitHub Pages, which can
observe every reader and withdraw the files at will; that is a disclosed limitation of how
they are published rather than a defect to report. See the top of
[`surfaces/README.md`](surfaces/README.md).
