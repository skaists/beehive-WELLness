# dockets

Specs for this tree's crates and curriculum.

- **FAT_PROFILE_READER** — the `fat-profile` crate spec (`Measured<T>`, the fat taxonomy,
  chain-length mapping, `unresolved`, lauric-C12 config). Implemented in
  `crates/fat-profile`.
- **SOVEREIGN_HEALTH_VAULT** — delivered. `crates/health-vault` is the full FHIR R4 schema (Observation, DiagnosticReport, MedicationStatement, NutritionIntake, Consent, Provenance), consent-gated, no-PII, synthetic-only.
  only the guarantee-spine (pseudonymous subject, no PII field). The FHIR R4-shaped
  record types land when this spec arrives.
- **FAT_MASTERY / obesity-onboarding** — the curriculum + scanner surfaces (Modules 1–4,
  D-12). Static, zero external references; they clear for Arweave only after per-surface category review.
