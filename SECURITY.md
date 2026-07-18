# Security & PHI policy

## No real personal health data. Ever. In the default build.

This tree teaches nutrition literacy and models health-record *structure*. It does not
collect, store, or process real personal health information in any default build.

- **Fixtures are synthetic only.** The PHI-marker guard (`scripts/phi-scan.sh`) fails
  the build on an email, SSN shape, or DOB shape in tracked files.
- **`health-vault` cannot re-identify by construction** — the subject is a pseudonymous
  id; no name / MRN / SSN / DOB field exists in the crate. Re-identification is not
  "discouraged," it is impossible from the types.
- **Real PHI, provider (FHIR) import, or interpretation-that-diagnoses** is counsel-gated,
  off by default, and never in a default build.

## Reporting

Report a vulnerability or a PHI-handling concern privately to the maintainers before
opening a public issue. A finding that the structural guarantees can be bypassed is the
highest-priority class — those guarantees are the reason this tree can exist.
