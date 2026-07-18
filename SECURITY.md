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

## Reporting

Report a vulnerability or a PHI-handling concern privately to the maintainers before
opening a public issue. A finding that the structural properties above can be bypassed is
the highest-priority class — those properties are the reason this tree can exist.

> **No private reporting channel is published yet.** This section names a policy without
> naming a route, which means it does not currently work. Choosing the route — GitHub
> private vulnerability reporting, or a published address — is a maintainer decision and
> is open. Until it is made, treat this policy as stated intent, not a working process.
