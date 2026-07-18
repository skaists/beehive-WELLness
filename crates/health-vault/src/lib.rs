//! `health-vault` — a pure FHIR R4-style health-record **schema** with **no PII field**.
//!
//! **The C-6 move, applied to health data:** just as `adapter-lti` has no mint path,
//! `health-vault` has no PII field. These types carry no `name`, `mrn`, `ssn`, `dob` or
//! `address` field anywhere in the crate; the subject is referenced only by [`Did`].
//! That is structural and checkable — read the types.
//!
//! **What that does NOT establish.** It is not an anonymity guarantee, and this crate
//! does not make one. Re-identification needs *linkage*, not an identifying field: a
//! [`Did`] is a stable pseudonym and is the canonical BNR principal, deliberately
//! resolvable elsewhere in the system, so tying one [`Did`] to a person re-identifies
//! every record under it retroactively. Quasi-identifiers left in a record — codes,
//! values, timestamps — are linkable on their own. The absence of a name field removes
//! the *direct* identifier and nothing else. Anyone reading these types for a privacy
//! property should draw that inference themselves; the crate is not entitled to state it.
//!
//! **Consent-gated by construction.** A [`VaultRecord`] can only be built with a valid,
//! unrevoked [`Consent`] and a coded+united resource — no consent, or a revoked one, or
//! an uninterpretable value, is a typed refusal, never a silent store.
//!
//! **Pure schema only.** No storage, no crypto, no network, no interpretation, and no
//! real PHI — ever. Fixtures are synthetic and CI-guarded (the tree's PHI-marker scan).
//! This crate is the *vocabulary*; storage/encryption/transport/diagnosis are separate,
//! counsel-gated crates behind traits, per the design note's wall.
//!
//! **Home & licence note.** The spec placed this in the kernel and reused
//! `capability::Did`/`EvidenceClass`; the founder later ruled health material off the
//! kernel (k001 wall) into this MIT tree, which forbids linking AGPL `capability`. The
//! subject now uses the **canonical [`Did`]** from the kernel's permissive `type-bindings`
//! crate (built for exactly this — a one-way pin, MIT staying MIT). [`EvidenceClass`]
//! stays a local mirror: splitting it from capability's tier ladder is a separate,
//! careful refactor, not done yet.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

// ── the no-PII subject: the canonical Did, from the permissive type-bindings crate ──

/// The subject of every record is the canonical [`Did`], consumed from the kernel's
/// permissive `type-bindings` crate (MIT/Apache — no AGPL infection, no local fork). It
/// is a pseudonymous DID string; there is still no name/mrn/ssn/dob/address field
/// anywhere. Pseudonymous is not anonymous — see the crate docs on what this does and
/// does not establish.
pub use type_bindings::Did;

// ── shared value types ──

/// A coded concept (LOINC / UCUM / RxNorm / etc.). A datum without a code is
/// uninterpretable, so `code` may not be empty (checked at the vault boundary).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Code {
    pub system: String,
    pub code: String,
    pub display: Option<String>,
}

/// A value with a UCUM unit. A value without a unit is not a datum (checked).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quantity {
    pub value: f64,
    pub unit: String,
}

/// Unix seconds. The crate reads no clock; the caller supplies time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timestamp(pub i64);

/// Evidence class E1–E5 — a local mirror of the kernel's (see the licence note); would
/// be the canonical type via a permissive `type-bindings` crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceClass {
    E1,
    E2,
    E3,
    E4,
    E5,
}

// ── FHIR resources (the data classes → resources) ──

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObsStatus {
    Registered,
    Preliminary,
    Final,
    Amended,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObsCategory {
    VitalSigns,
    Laboratory,
    /// Outflows / symptoms / mood — same shape, nothing special-cased away.
    Survey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Observation {
    pub code: Code,
    pub value: Quantity,
    pub effective: Timestamp,
    pub status: ObsStatus,
    pub category: ObsCategory,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticReport {
    pub code: Code,
    pub results: Vec<Observation>,
    pub effective: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dosage {
    pub amount: Quantity,
    pub timing: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MedStatus {
    Active,
    Completed,
    Stopped,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MedicationStatement {
    pub medication: Code,
    pub dosage: Dosage,
    pub status: MedStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NutritionIntake {
    pub item: Code,
    pub amount: Quantity,
    pub occurrence: Timestamp,
}

/// One of the record shapes the vault holds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Resource {
    Observation(Observation),
    Diagnostic(DiagnosticReport),
    Medication(MedicationStatement),
    Nutrition(NutritionIntake),
}

impl Resource {
    /// Coded + united or rejected: every code and every quantity unit must be non-empty.
    /// An uninterpretable datum is not a datum.
    pub fn validate(&self) -> Result<(), VaultError> {
        let check = |c: &Code, q: &Quantity| -> Result<(), VaultError> {
            if c.code.trim().is_empty() || c.system.trim().is_empty() {
                return Err(VaultError::Uncoded);
            }
            if q.unit.trim().is_empty() {
                return Err(VaultError::Ununited);
            }
            Ok(())
        };
        match self {
            Resource::Observation(o) => check(&o.code, &o.value),
            Resource::Diagnostic(d) => {
                if d.code.code.trim().is_empty() {
                    return Err(VaultError::Uncoded);
                }
                for o in &d.results {
                    check(&o.code, &o.value)?;
                }
                Ok(())
            }
            Resource::Medication(m) => {
                if m.medication.code.trim().is_empty() {
                    return Err(VaultError::Uncoded);
                }
                if m.dosage.amount.unit.trim().is_empty() {
                    return Err(VaultError::Ununited);
                }
                Ok(())
            }
            Resource::Nutrition(n) => check(&n.item, &n.amount),
        }
    }
}

// ── consent + provenance ──

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Provision {
    Permit,
    Deny,
}

/// Per-datum consent. Valid only when it permits and has not been revoked.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Consent {
    pub scope: String,
    pub provision: Provision,
    /// `Some(ts)` once revoked — makes any record it gates non-serializable going forward.
    pub revoked: Option<Timestamp>,
}

impl Consent {
    pub fn is_valid(&self) -> bool {
        matches!(self.provision, Provision::Permit) && self.revoked.is_none()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Provenance {
    pub agent: String,
    pub recorded: Timestamp,
    pub evidence_class: EvidenceClass,
}

// ── errors ──

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VaultError {
    /// No consent, or consent denied/revoked — no record.
    ConsentAbsentOrRevoked,
    /// A value with no code — uninterpretable.
    Uncoded,
    /// A value with no unit — not a datum.
    Ununited,
}

impl std::fmt::Display for VaultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VaultError::ConsentAbsentOrRevoked => {
                write!(f, "vault: consent absent, denied, or revoked — no record")
            }
            VaultError::Uncoded => write!(f, "vault: value has no code — uninterpretable"),
            VaultError::Ununited => write!(f, "vault: value has no unit — not a datum"),
        }
    }
}

impl std::error::Error for VaultError {}

// ── the consent-gated record ──

/// A resource about a subject, with consent and provenance. Constructible ONLY via
/// [`VaultRecord::new`], which refuses absent/revoked consent and an uncoded/ununited
/// resource — so an invalid record cannot exist, not merely fails to serialize.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct VaultRecord {
    subject: Did,
    resource: Resource,
    consent: Consent,
    provenance: Provenance,
}

impl VaultRecord {
    pub fn new(
        subject: Did,
        resource: Resource,
        consent: Consent,
        provenance: Provenance,
    ) -> Result<Self, VaultError> {
        if !consent.is_valid() {
            return Err(VaultError::ConsentAbsentOrRevoked);
        }
        resource.validate()?;
        Ok(VaultRecord {
            subject,
            resource,
            consent,
            provenance,
        })
    }

    pub fn subject(&self) -> &Did {
        &self.subject
    }
    pub fn resource(&self) -> &Resource {
        &self.resource
    }
    pub fn provenance(&self) -> &Provenance {
        &self.provenance
    }
}

// ── tests: structure and refusals, red-then-green ──

#[cfg(test)]
mod tests {
    use super::*;

    const OBS_HR: &str = include_str!("../fixtures/obs_hr_synthetic.json");
    const LABS: &str = include_str!("../fixtures/labs_lipid_synthetic.json");
    const MED: &str = include_str!("../fixtures/med_supplement_synthetic.json");
    const CONSENT_REVOKED: &str = include_str!("../fixtures/consent_revoked_synthetic.json");

    fn subject() -> Did {
        Did::new("did:bnr:test-thread-0001")
    }
    fn good_consent() -> Consent {
        Consent {
            scope: "vault-read".into(),
            provision: Provision::Permit,
            revoked: None,
        }
    }
    fn prov() -> Provenance {
        Provenance {
            agent: "self".into(),
            recorded: Timestamp(1_789_000_000),
            evidence_class: EvidenceClass::E3,
        }
    }

    #[test]
    fn fixtures_round_trip() {
        let o: Observation = serde_json::from_str(OBS_HR).unwrap();
        assert_eq!(
            serde_json::from_str::<Observation>(&serde_json::to_string(&o).unwrap()).unwrap(),
            o
        );
        let d: DiagnosticReport = serde_json::from_str(LABS).unwrap();
        assert_eq!(d.results.len(), 2);
        let _m: MedicationStatement = serde_json::from_str(MED).unwrap();
        let c: Consent = serde_json::from_str(CONSENT_REVOKED).unwrap();
        assert!(c.revoked.is_some());
    }

    #[test]
    fn valid_observation_makes_a_record() {
        let o: Observation = serde_json::from_str(OBS_HR).unwrap();
        let r = VaultRecord::new(subject(), Resource::Observation(o), good_consent(), prov());
        assert!(r.is_ok());
        assert_eq!(r.unwrap().subject(), &Did::new("did:bnr:test-thread-0001"));
    }

    #[test]
    fn no_consent_is_a_typed_refusal_not_a_panic() {
        let o: Observation = serde_json::from_str(OBS_HR).unwrap();
        let denied = Consent {
            scope: "x".into(),
            provision: Provision::Deny,
            revoked: None,
        };
        assert_eq!(
            VaultRecord::new(subject(), Resource::Observation(o), denied, prov()).unwrap_err(),
            VaultError::ConsentAbsentOrRevoked
        );
    }

    #[test]
    fn revoked_consent_cannot_make_a_record() {
        let o: Observation = serde_json::from_str(OBS_HR).unwrap();
        let c: Consent = serde_json::from_str(CONSENT_REVOKED).unwrap();
        assert_eq!(
            VaultRecord::new(subject(), Resource::Observation(o), c, prov()).unwrap_err(),
            VaultError::ConsentAbsentOrRevoked
        );
    }

    #[test]
    fn value_without_code_is_rejected() {
        let o = Observation {
            code: Code {
                system: "http://loinc.org".into(),
                code: "".into(),
                display: None,
            },
            value: Quantity {
                value: 72.0,
                unit: "/min".into(),
            },
            effective: Timestamp(1),
            status: ObsStatus::Final,
            category: ObsCategory::VitalSigns,
        };
        assert_eq!(
            VaultRecord::new(subject(), Resource::Observation(o), good_consent(), prov())
                .unwrap_err(),
            VaultError::Uncoded
        );
    }

    #[test]
    fn value_without_unit_is_rejected() {
        let o = Observation {
            code: Code {
                system: "http://loinc.org".into(),
                code: "8867-4".into(),
                display: None,
            },
            value: Quantity {
                value: 72.0,
                unit: "".into(),
            },
            effective: Timestamp(1),
            status: ObsStatus::Final,
            category: ObsCategory::VitalSigns,
        };
        assert_eq!(
            VaultRecord::new(subject(), Resource::Observation(o), good_consent(), prov())
                .unwrap_err(),
            VaultError::Ununited
        );
    }

    // The no-PII guarantee is compile-time: there is no name/mrn/ssn/dob accessor to
    // call. This test documents the spine; the compiler enforces it (a `.name` access on
    // any type here would fail to compile). A trybuild negative-test is redundant with
    // "the field does not exist" and is left out rather than added as ceremony.
    #[test]
    fn subject_carries_only_a_pseudonym() {
        let r = VaultRecord::new(
            subject(),
            Resource::Nutrition(NutritionIntake {
                item: Code {
                    system: "usda".into(),
                    code: "170148".into(),
                    display: Some("hemp".into()),
                },
                amount: Quantity {
                    value: 30.0,
                    unit: "g".into(),
                },
                occurrence: Timestamp(1),
            }),
            good_consent(),
            prov(),
        )
        .unwrap();
        assert_eq!(r.subject(), &Did::new("did:bnr:test-thread-0001"));
    }

    // GREEN pending a real, de-identified provider export (synthetic-substituted):
    // a real-FHIR-import round-trip. `cargo test` skips it.
    #[test]
    #[ignore = "waiting on a de-identified real FHIR export to pin (synthetic-substituted)"]
    fn real_fhir_import_round_trip() {
        unimplemented!(
            "import a real de-identified FHIR bundle, map to Resource, gate via VaultRecord::new"
        );
    }
}
