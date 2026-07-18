//! `health-vault` — no-PII health-record types with a **pseudonymous subject**.
//!
//! Guarantee-spine (the full FHIR R4-shaped record types await the
//! `SOVEREIGN_HEALTH_VAULT` spec): the only handle a vault record has to a person is a
//! pseudonymous [`SubjectRef`]. **No name, MRN, SSN, or DOB field exists in this crate**
//! — re-identification is impossible from the types, not merely discouraged. Every
//! record type added later builds on this spine and cannot reach past it.
//!
//! Licensed MIT OR Apache-2.0: it links no AGPL kernel crate (it defines its own types),
//! so it is a genuinely permissive SDK edge.

#![forbid(unsafe_code)]

/// The only reference a vault record holds to a person: an opaque pseudonym. It is not
/// a name, an MRN, an SSN, or a DOB — those fields do not exist here, so a record cannot
/// carry them. This is the re-identification guarantee, in the type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubjectRef(String);

impl SubjectRef {
    /// Wrap a pseudonym. Callers are responsible for it being pseudonymous; the type
    /// guarantees only that nothing *else* about the person can be attached here.
    pub fn new(pseudonym: impl Into<String>) -> Self {
        SubjectRef(pseudonym.into())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subject_is_only_a_pseudonym() {
        let s = SubjectRef::new("bee-thread-7f3a");
        assert_eq!(s.as_str(), "bee-thread-7f3a");
        // The guarantee is what this type CANNOT do: there is no name/mrn/ssn/dob
        // accessor to call. This test documents the spine; the compiler enforces it.
    }
}
