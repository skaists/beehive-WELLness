//! `fat-profile` — report a food's fatty-acid profile from an authoritative database,
//! and **mark what the source didn't measure**. Pure data translation: no health logic,
//! no interpretation, no intake math — vocabulary and lookup only.
//!
//! **Honesty is the type.** The load-bearing primitive is [`Measured<T>`]: every
//! sub-fraction is `Value(T)` or `NotMeasured`, and **no code path fills a
//! `NotMeasured` with an interpolation, an average, or a guess.** A gap in USDA is a
//! `NotMeasured` all the way to the UI's hatched chip. `unresolved` (total − Σ resolved)
//! is surfaced, never redistributed. The one modeling call — lauric C12 — is documented
//! and configurable, never silent.
//!
//! **No health fields.** There is no "recommended intake," "deficiency risk," or "good
//! for you" field. Those belong to a separate, bounded interpretation layer, not here.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Report what the source measured, or mark that it didn't. There is deliberately **no**
/// constructor that turns `NotMeasured` into a number.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Measured<T> {
    Value(T),
    NotMeasured,
}

impl<T: Copy> Measured<T> {
    pub fn value(self) -> Option<T> {
        match self {
            Measured::Value(v) => Some(v),
            Measured::NotMeasured => None,
        }
    }
    pub fn is_measured(&self) -> bool {
        matches!(self, Measured::Value(_))
    }
}

/// Grams of a fatty-acid fraction. A plain quantity — no meaning attached.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Grams(pub f64);

// ── the fat taxonomy as data ──

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Basis {
    Per100g,
    PerServing { grams: f64 },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Source {
    UsdaFdc { fdc_id: u32 },
    HealthCanadaCnf { code: String },
    BrandPanel,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SaturatedBreakdown {
    pub total: Measured<Grams>,
    pub scfa: Measured<Grams>,
    pub mcfa_mct: Measured<Grams>,
    pub lcfa: Measured<Grams>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Omega6 {
    pub total: Measured<Grams>,
    pub la: Measured<Grams>,
    pub gla: Measured<Grams>,
    pub aa: Measured<Grams>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Omega3 {
    pub total: Measured<Grams>,
    pub ala: Measured<Grams>,
    pub epa: Measured<Grams>,
    pub dha: Measured<Grams>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FatProfile {
    pub basis: Basis,
    pub source: Source,
    pub total_fat: Measured<Grams>,
    pub saturated: SaturatedBreakdown,
    pub monounsaturated: Measured<Grams>,
    pub omega6: Omega6,
    pub omega3: Omega3,
    pub trans: Measured<Grams>,
    /// total − Σ(resolved fractions), surfaced. Never hidden, never redistributed.
    pub unresolved: Grams,
}

// ── chain-length mapping (the one modeling call is explicit) ──

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaClass {
    Scfa,
    Mcfa,
    Lcfa,
}

/// How to classify lauric acid (C12) — it is conventionally an MCFA but metabolizes
/// partly like an LCFA. Default MCFA; the choice is the caller's and is never silent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LauricPolicy {
    Mcfa,
    Lcfa,
}

impl Default for LauricPolicy {
    fn default() -> Self {
        LauricPolicy::Mcfa
    }
}

/// Map a saturated FA's carbon count to a chain class. Boundaries stated, not implied:
/// C≤6 → SCFA, C7–C11 → MCFA, C12 → `lauric` (default MCFA), C≥13 → LCFA.
pub fn classify_chain(carbons: u8, lauric: LauricPolicy) -> FaClass {
    match carbons {
        0..=6 => FaClass::Scfa,
        7..=11 => FaClass::Mcfa,
        12 => match lauric {
            LauricPolicy::Mcfa => FaClass::Mcfa,
            LauricPolicy::Lcfa => FaClass::Lcfa,
        },
        _ => FaClass::Lcfa,
    }
}

/// Sum individual saturated FAs (carbon count, grams) into a breakdown by chain class.
/// A class with no contributing FA is `NotMeasured` — never a fabricated 0. `total` is
/// carried through as given; the caller owns whether it was measured.
pub fn saturated_from_fas(
    total: Measured<Grams>,
    fas: &[(u8, Grams)],
    lauric: LauricPolicy,
) -> SaturatedBreakdown {
    let mut scfa: Option<f64> = None;
    let mut mcfa: Option<f64> = None;
    let mut lcfa: Option<f64> = None;
    for &(c, Grams(g)) in fas {
        let slot = match classify_chain(c, lauric) {
            FaClass::Scfa => &mut scfa,
            FaClass::Mcfa => &mut mcfa,
            FaClass::Lcfa => &mut lcfa,
        };
        *slot = Some(slot.unwrap_or(0.0) + g);
    }
    let m = |o: Option<f64>| {
        o.map(|g| Measured::Value(Grams(g)))
            .unwrap_or(Measured::NotMeasured)
    };
    SaturatedBreakdown {
        total,
        scfa: m(scfa),
        mcfa_mct: m(mcfa),
        lcfa: m(lcfa),
    }
}

/// `unresolved = total − Σ(resolved parts)`, floored at 0 for float noise. Surfaced so
/// an incomplete USDA measurement is visible, not silently dropped or redistributed.
/// Only meaningful when `total` is `Value`; returns `Grams(0.0)` otherwise.
pub fn unresolved(total: Measured<Grams>, resolved_parts: &[Measured<Grams>]) -> Grams {
    match total {
        Measured::NotMeasured => Grams(0.0),
        Measured::Value(Grams(t)) => {
            let sum: f64 = resolved_parts
                .iter()
                .filter_map(|m| m.value().map(|Grams(g)| g))
                .sum();
            Grams((t - sum).max(0.0))
        }
    }
}

/// Scale a per-100g profile to a serving. A clearly-separate pure helper, so
/// composition data and dietary interpretation never mix in one type. `NotMeasured`
/// stays `NotMeasured` — scaling a gap does not invent a value.
pub fn per_serving(total_fat: Measured<Grams>, serving_g: f64) -> Measured<Grams> {
    match total_fat {
        Measured::NotMeasured => Measured::NotMeasured,
        Measured::Value(Grams(g)) => Measured::Value(Grams(g * serving_g / 100.0)),
    }
}

// ── the source seam (real HTTP gates on nothing secret; mock ships now) ──

#[derive(Debug, Clone, PartialEq)]
pub enum ReaderError {
    NotFound(u32),
    Parse(&'static str),
}

/// Look up a food's fat profile. The real impl fetches USDA FDC / Health Canada CNF over
/// HTTP; v1 ships [`MockFatSource`] reading pinned fixtures — no network in tests.
pub trait FatSource {
    fn profile(&self, fdc_id: u32) -> Result<FatProfile, ReaderError>;
}

/// Reads a pinned USDA-shaped JSON fixture. The real reader parses the live FDC response
/// through the same `FatProfile` shape.
pub struct MockFatSource {
    fixtures: std::collections::BTreeMap<u32, FatProfile>,
}

impl MockFatSource {
    pub fn new() -> Self {
        MockFatSource {
            fixtures: std::collections::BTreeMap::new(),
        }
    }
    /// Load a fixture from `FatProfile` JSON.
    pub fn load(&mut self, fdc_id: u32, json: &str) -> Result<(), ReaderError> {
        let p: FatProfile =
            serde_json::from_str(json).map_err(|_| ReaderError::Parse("not a FatProfile"))?;
        self.fixtures.insert(fdc_id, p);
        Ok(())
    }
}

impl Default for MockFatSource {
    fn default() -> Self {
        Self::new()
    }
}

impl FatSource for MockFatSource {
    fn profile(&self, fdc_id: u32) -> Result<FatProfile, ReaderError> {
        self.fixtures
            .get(&fdc_id)
            .cloned()
            .ok_or(ReaderError::NotFound(fdc_id))
    }
}

// ── tests: structure, not magic numbers ──

#[cfg(test)]
mod tests {
    use super::*;

    const HEMP: &str = include_str!("../fixtures/usda_hemp_hearts_170148.json");
    const COCONUT: &str = include_str!("../fixtures/usda_coconut_oil.json");
    const SALMON: &str = include_str!("../fixtures/usda_salmon.json");
    const PARTIAL: &str = include_str!("../fixtures/partial_measurement.json");

    #[test]
    fn chain_mapping_boundaries() {
        let d = LauricPolicy::default();
        assert_eq!(classify_chain(4, d), FaClass::Scfa); // butyric
        assert_eq!(classify_chain(8, d), FaClass::Mcfa); // caprylic
        assert_eq!(classify_chain(16, d), FaClass::Lcfa); // palmitic
    }

    #[test]
    fn lauric_flips_mcfa_lcfa_and_only_that() {
        assert_eq!(classify_chain(12, LauricPolicy::Mcfa), FaClass::Mcfa);
        assert_eq!(classify_chain(12, LauricPolicy::Lcfa), FaClass::Lcfa);
        // neighbours are unaffected by the policy
        assert_eq!(classify_chain(10, LauricPolicy::Lcfa), FaClass::Mcfa);
        assert_eq!(classify_chain(14, LauricPolicy::Mcfa), FaClass::Lcfa);
    }

    #[test]
    fn not_measured_never_becomes_a_number() {
        let sb = saturated_from_fas(Measured::NotMeasured, &[], LauricPolicy::default());
        assert_eq!(
            sb.scfa,
            Measured::NotMeasured,
            "no FA in a class stays NotMeasured, not 0"
        );
        assert_eq!(
            per_serving(Measured::NotMeasured, 30.0),
            Measured::NotMeasured
        );
    }

    #[test]
    fn unresolved_is_total_minus_resolved_surfaced() {
        // total 10, resolved 3 + 4 -> unresolved 3, never redistributed into the parts.
        let u = unresolved(
            Measured::Value(Grams(10.0)),
            &[
                Measured::Value(Grams(3.0)),
                Measured::Value(Grams(4.0)),
                Measured::NotMeasured,
            ],
        );
        assert_eq!(u, Grams(3.0));
    }

    #[test]
    fn total_preserving_sum_plus_unresolved_equals_total() {
        let fas = [(4u8, Grams(1.0)), (8, Grams(2.0)), (16, Grams(5.0))];
        let sb = saturated_from_fas(Measured::Value(Grams(10.0)), &fas, LauricPolicy::default());
        let parts = [sb.scfa, sb.mcfa_mct, sb.lcfa];
        let u = unresolved(sb.total, &parts);
        let resolved: f64 = parts
            .iter()
            .filter_map(|m| m.value().map(|Grams(g)| g))
            .sum();
        assert!(
            (resolved + u.0 - 10.0).abs() < 1e-9,
            "Σ resolved + unresolved == total"
        );
    }

    #[test]
    fn hemp_is_all_ala_no_epa_dha_gla_not_measured() {
        let mut src = MockFatSource::new();
        src.load(170148, HEMP).unwrap();
        let p = src.profile(170148).unwrap();
        assert!(p.omega3.ala.is_measured());
        assert_eq!(
            p.omega3.epa,
            Measured::Value(Grams(0.0)),
            "EPA measured and zero"
        );
        assert_eq!(
            p.omega3.dha,
            Measured::Value(Grams(0.0)),
            "DHA measured and zero"
        );
        assert_eq!(
            p.omega6.gla,
            Measured::NotMeasured,
            "GLA not measured — a gap, not a 0"
        );
    }

    #[test]
    fn salmon_has_epa_dha() {
        let mut src = MockFatSource::new();
        src.load(175168, SALMON).unwrap();
        let p = src.profile(175168).unwrap();
        assert!(p.omega3.epa.is_measured() && p.omega3.dha.is_measured());
    }

    #[test]
    fn coconut_and_partial_parse() {
        let mut src = MockFatSource::new();
        src.load(1, COCONUT).unwrap();
        src.load(2, PARTIAL).unwrap();
        assert!(
            src.profile(1).unwrap().saturated.mcfa_mct.is_measured(),
            "coconut has MCTs"
        );
        // partial: sub-fractions incomplete -> unresolved > 0
        let p = src.profile(2).unwrap();
        assert!(
            p.unresolved.0 > 0.0,
            "an incomplete measurement surfaces unresolved"
        );
    }

    // GREEN pending a pinned real FDC response: live USDA fetch through the same shape.
    #[test]
    #[ignore = "waiting on a captured real USDA FDC API response to pin"]
    fn live_usda_fetch() {
        unimplemented!("real FatSource over HTTP — parse the live FDC response into FatProfile");
    }
}
