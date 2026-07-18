//! `food-composition` — the multi-source backbone (order C-F).
//!
//! **The governing constraint, above every other line in this crate:**
//! *display what was measured; never a conclusion the measurement does not support.*
//! A value from one source is **1 determination**. Not "unverified" — that is a
//! judgement. Not "verified" — that is a claim. **1.**
//!
//! # Why this crate exists
//!
//! National food-composition tables borrow heavily from one another. A number that
//! originated in one USDA laboratory determination and was then reprinted by twelve
//! national tables is **still one measurement**. Counting those twelve as twelve
//! agreements would manufacture confidence out of copying — and would do it in the exact
//! shape of the defect this project has already had to repair once: a sentence asserting
//! a property the data does not have.
//!
//! So independence is not a number anyone types in. [`NutrientRecord::independence`]
//! **derives** it by following [`Origin::borrowed_from`] transitively and counting
//! *distinct roots*. There is no setter, no constructor argument, and no arithmetic path
//! that raises a count from a borrowed value. **More countries is not more confidence,**
//! and in these types it cannot be made to look like it.
//!
//! # What this crate refuses to do
//!
//! - **No imputation.** [`Measured`] has exactly two states. There is no "estimated from
//!   a similar food", no gap-filling, no interpolation. If nobody measured it, the type
//!   says so and every surface built on it says so.
//! - **No resolution of disagreement.** Where sources differ, [`Comparison`] reports the
//!   spread. It does not pick a winner, does not average into a false middle, and has no
//!   "best value" field for anyone to reach for.
//! - **No claims.** There is no type here for a study, a trial, a patent, a mechanism or
//!   an outcome — see the wall note below. This crate holds nutrient numbers and where
//!   they came from. That is the whole surface area.
//!
//! # The two-axis wall (F-3)
//!
//! **Axis A** is food composition: nutrient numbers, claim-free — this crate.
//! **Axis B** is cannabis research and IP: trials, patents, mechanism, GRADE-graded,
//! held off-repo and separately.
//!
//! Some countries appear on both axes for entirely unrelated reasons. The seam to guard
//! is a single source record spanning them, because that is how Axis B material would
//! enter a claim-free crate wearing Axis A clothes. The guard here is an **absence**:
//! [`SourceKind`] enumerates food-composition table kinds and nothing else. There is no
//! variant for a trial or a patent, so an Axis B item cannot be constructed as a source
//! in this crate at all. Nothing to police at review time, because there is no type to
//! misuse — the same move as having no split path to guard.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

// ── identity ──

/// Stable identifier for a data source, e.g. `usda-fdc`, `canada-cnf`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SourceId(pub String);

impl SourceId {
    pub fn new(s: impl Into<String>) -> Self {
        SourceId(s.into())
    }
}

/// An INFOODS tagname — the harmonised nutrient identity (`PROCNT`, `FE`, `K`, …).
///
/// Nutrients are matched on tagname, never on the label a table happens to print, because
/// "protein" in two tables can mean two different analytes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Tagname(pub String);

impl Tagname {
    pub fn new(s: impl Into<String>) -> Self {
        Tagname(s.into())
    }
}

/// What kind of thing a source is.
///
/// **This enum is a wall (F-3).** Every variant is a food-composition data source. There
/// is deliberately no variant for a clinical trial, patent, mechanism study or outcome
/// dataset, so Axis B material cannot be represented as a source here. Adding such a
/// variant would breach the wall by construction — that is the point of enumerating.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceKind {
    /// A national or regional food-composition table.
    NationalTable,
    /// An international federation/aggregation of national tables (e.g. FAO/INFOODS).
    Federation,
    /// A laboratory analysis report.
    LaboratoryReport,
    /// A manufacturer's declared composition for a specific product.
    ManufacturerDeclaration,
}

/// A registered source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Source {
    pub id: SourceId,
    pub kind: SourceKind,
    /// Display name, in the source's own primary language of publication (F-4).
    pub name: String,
    /// BCP-47 tag for `name` and for values published by this source.
    pub language: String,
}

// ── basis: what makes two numbers comparable, or not ──

/// The denominator a value is expressed against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReferenceQuantity {
    /// Per 100 g of the food as described.
    Per100gAsDescribed,
    /// Per 100 g of dry matter.
    Per100gDryMatter,
    /// Per stated serving.
    PerServing,
}

/// **The basis of a number: which food, expressed against what.**
///
/// Basis is what makes two values comparable or not, and getting it wrong is how a
/// spreadsheet ends up "disagreeing" with USDA about hemp protein when in fact the two
/// numbers describe two different foods — hulled hearts and a protein concentrate — and
/// are both correct. Two values with different bases are **not** a disagreement, and this
/// crate will not report them as one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Basis {
    /// The food form as published: "hulled hearts", "protein concentrate", "raw, lean".
    pub food_form: String,
    pub per: ReferenceQuantity,
}

impl Basis {
    pub fn new(food_form: impl Into<String>, per: ReferenceQuantity) -> Self {
        Basis {
            food_form: food_form.into(),
            per,
        }
    }

    /// Two values may be compared only when they describe the same food form against the
    /// same denominator. Anything else is two facts, not two opinions.
    pub fn comparable_with(&self, other: &Basis) -> bool {
        self.food_form == other.food_form && self.per == other.per
    }
}

// ── provenance ──

/// Where a single value came from.
///
/// `borrowed_from` is the load-bearing field. `None` means this source performed the
/// determination itself; `Some(id)` means it reprinted or derived the number from `id`,
/// and it therefore contributes **no** additional independence.
///
/// A **unit or basis conversion is a borrow**, not a new determination — converting
/// USDA's number into different units does not measure anything, so a converted value
/// carries `borrowed_from: Some(original)` and cannot inflate a count (F-4).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Origin {
    pub source_id: SourceId,
    pub basis: Basis,
    pub borrowed_from: Option<SourceId>,
}

impl Origin {
    /// An original determination by `source_id`.
    pub fn determined_by(source_id: SourceId, basis: Basis) -> Self {
        Origin {
            source_id,
            basis,
            borrowed_from: None,
        }
    }

    /// A value reprinted or derived from another source. Contributes no independence.
    pub fn borrowed(source_id: SourceId, basis: Basis, from: SourceId) -> Self {
        Origin {
            source_id,
            basis,
            borrowed_from: Some(from),
        }
    }
}

// ── measurement ──

/// A quantity as published, in the source's own units. Never silently converted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quantity {
    pub amount: f64,
    /// Unit as published: `g`, `mg`, `%`, `IU`.
    pub unit: String,
}

impl Quantity {
    pub fn new(amount: f64, unit: impl Into<String>) -> Self {
        Quantity {
            amount,
            unit: unit.into(),
        }
    }
}

/// Measured, or not. There is no third state and no imputation path (F-5).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Measured<T> {
    Value(T),
    NotMeasured,
}

impl<T> Measured<T> {
    pub fn as_value(&self) -> Option<&T> {
        match self {
            Measured::Value(v) => Some(v),
            Measured::NotMeasured => None,
        }
    }
    pub fn is_measured(&self) -> bool {
        matches!(self, Measured::Value(_))
    }
}

/// One source's statement about one nutrient in one food.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Determination {
    pub origin: Origin,
    pub value: Measured<Quantity>,
}

impl Determination {
    pub fn new(origin: Origin, value: Measured<Quantity>) -> Self {
        Determination { origin, value }
    }
}

// ── independence: derived, never assigned ──

/// Why an independence count could not be produced.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Indeterminate {
    /// `borrowed_from` chains form a cycle; no root exists.
    BorrowCycle { involving: Vec<SourceId> },
    /// A value cites a source that is not present among the determinations.
    DanglingBorrow { from: SourceId },
}

/// The result of resolving provenance. **Constructible only by
/// [`NutrientRecord::independence`]** — there is no public constructor, no setter, and no
/// arithmetic on it, so a count can only ever be something this crate derived.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Independence {
    /// No source measured this. Never rendered as blank, zero, or inferred.
    NotMeasured,
    /// `n` distinct roots after resolving every borrow. `n >= 1` by construction.
    Determinations(usize),
    /// The provenance graph is broken. **Fail closed:** a broken graph yields no number.
    /// Surfaces must show this state rather than guess a count.
    Indeterminate(Indeterminate),
}

impl Independence {
    /// The count, when there is one. `None` for `NotMeasured` and `Indeterminate` — so a
    /// caller cannot accidentally treat "we could not tell" as zero.
    pub fn count(&self) -> Option<usize> {
        match self {
            Independence::Determinations(n) => Some(*n),
            _ => None,
        }
    }
}

// ── unit plausibility: flag, never correct ──

/// A structural implausibility in a published value.
///
/// Reported so a reader can see it. **Never auto-corrected and never silently dropped** —
/// this crate does not know which of the amount, the unit or the column was wrong, and
/// guessing would be exactly the invented conclusion the governing constraint forbids.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnitPlausibility {
    Plausible,
    /// A mass fraction exceeding the whole: more than 100 g of a nutrient per 100 g of
    /// food. Commonly a column/unit slip (a mg figure printed in a grams column).
    ExceedsWhole {
        note: String,
    },
}

/// Check a published value for structural impossibility against its basis.
pub fn unit_plausibility(q: &Quantity, basis: &Basis) -> UnitPlausibility {
    let per_100g = matches!(
        basis.per,
        ReferenceQuantity::Per100gAsDescribed | ReferenceQuantity::Per100gDryMatter
    );
    if per_100g && q.unit == "g" && q.amount > 100.0 {
        return UnitPlausibility::ExceedsWhole {
            note: format!(
                "{} g per 100 g exceeds the whole food; often a mg value printed in a g column. \
                 Shown as published — not corrected.",
                q.amount
            ),
        };
    }
    UnitPlausibility::Plausible
}

// ── the record ──

/// Every determination of one nutrient for one food, across all sources.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NutrientRecord {
    pub tagname: Tagname,
    pub determinations: Vec<Determination>,
}

impl NutrientRecord {
    pub fn new(tagname: Tagname, determinations: Vec<Determination>) -> Self {
        NutrientRecord {
            tagname,
            determinations,
        }
    }

    /// **Derive** independence over the determinations sharing `basis`.
    ///
    /// Follows `borrowed_from` transitively and counts distinct roots. Twelve tables
    /// reprinting one USDA determination resolve to one root and therefore to
    /// `Determinations(1)`.
    ///
    /// Fails closed: a cycle or a dangling citation yields [`Independence::Indeterminate`]
    /// rather than a number.
    pub fn independence(&self, basis: &Basis) -> Independence {
        let in_scope: Vec<&Determination> = self
            .determinations
            .iter()
            .filter(|d| d.origin.basis.comparable_with(basis) && d.value.is_measured())
            .collect();

        if in_scope.is_empty() {
            return Independence::NotMeasured;
        }

        // borrow edges declared by the in-scope determinations
        let edges: BTreeMap<&SourceId, Option<&SourceId>> = in_scope
            .iter()
            .map(|d| (&d.origin.source_id, d.origin.borrowed_from.as_ref()))
            .collect();

        let mut roots: BTreeSet<&SourceId> = BTreeSet::new();
        for d in &in_scope {
            match resolve_root(&d.origin.source_id, &edges) {
                Ok(root) => {
                    roots.insert(root);
                }
                Err(e) => return Independence::Indeterminate(e),
            }
        }
        Independence::Determinations(roots.len())
    }

    /// Compare the values sharing `basis`. Reports the spread; resolves nothing.
    pub fn compare(&self, basis: &Basis) -> Comparison {
        let vals: Vec<(&SourceId, &Quantity)> = self
            .determinations
            .iter()
            .filter(|d| d.origin.basis.comparable_with(basis))
            .filter_map(|d| d.value.as_value().map(|q| (&d.origin.source_id, q)))
            .collect();

        let independence = self.independence(basis);

        if vals.is_empty() {
            return Comparison {
                independence,
                spread: None,
                values: Vec::new(),
            };
        }

        // Spread is only meaningful within one unit. Mixed units are reported as values
        // without a spread rather than converted — a conversion is a derived value and
        // would need its own origin (F-4).
        let first_unit = vals[0].1.unit.clone();
        let uniform = vals.iter().all(|(_, q)| q.unit == first_unit);
        let spread = if uniform && vals.len() > 1 {
            let amounts: Vec<f64> = vals.iter().map(|(_, q)| q.amount).collect();
            let min = amounts.iter().cloned().fold(f64::INFINITY, f64::min);
            let max = amounts.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            Some(Spread {
                min,
                max,
                unit: first_unit,
            })
        } else {
            None
        };

        Comparison {
            independence,
            spread,
            values: vals
                .into_iter()
                .map(|(id, q)| (id.clone(), q.clone()))
                .collect(),
        }
    }

    /// Every distinct basis present, so a caller can group before comparing rather than
    /// comparing across incomparable numbers.
    pub fn bases(&self) -> Vec<Basis> {
        let mut out: Vec<Basis> = Vec::new();
        for d in &self.determinations {
            if !out.iter().any(|b| b.comparable_with(&d.origin.basis)) {
                out.push(d.origin.basis.clone());
            }
        }
        out
    }
}

/// The observed range across sources. **There is no "best value" field** — nothing here
/// resolves a disagreement, because resolving one would state a conclusion the
/// measurements do not support.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spread {
    pub min: f64,
    pub max: f64,
    pub unit: String,
}

impl Spread {
    pub fn differs(&self) -> bool {
        self.min != self.max
    }
}

/// What can honestly be said about one nutrient at one basis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comparison {
    pub independence: Independence,
    /// `None` when there is one value, or when units are mixed and no conversion was made.
    pub spread: Option<Spread>,
    pub values: Vec<(SourceId, Quantity)>,
}

fn resolve_root<'a>(
    start: &'a SourceId,
    edges: &BTreeMap<&'a SourceId, Option<&'a SourceId>>,
) -> Result<&'a SourceId, Indeterminate> {
    let mut seen: Vec<&SourceId> = vec![start];
    let mut cur = start;
    loop {
        match edges.get(cur) {
            // performed its own determination — this is a root
            Some(None) => return Ok(cur),
            Some(Some(next)) => {
                if seen.contains(next) {
                    let mut involving: Vec<SourceId> = seen.iter().map(|s| (*s).clone()).collect();
                    involving.push((*next).clone());
                    return Err(Indeterminate::BorrowCycle { involving });
                }
                seen.push(next);
                cur = next;
            }
            // cites a source not present among the determinations
            None => return Err(Indeterminate::DanglingBorrow { from: cur.clone() }),
        }
    }
}

// ── the registry (F-2) ──

/// Root source: USDA FoodData Central. Anchor record FDC 170148 (hemp seeds, hulled).
pub const USDA_FDC: &str = "usda-fdc";
/// Federation layer: FAO/INFOODS.
pub const FAO_INFOODS: &str = "fao-infoods";

/// National tables carried per founder direction (F-2).
pub const NATIONAL_TABLES: &[(&str, &str, &str)] = &[
    ("canada-cnf", "Canadian Nutrient File", "en"),
    (
        "mexico-smae",
        "Sistema Mexicano de Alimentos Equivalentes",
        "es",
    ),
    (
        "spain-bedca",
        "Base de Datos Española de Composición de Alimentos",
        "es",
    ),
    (
        "latvia-fct",
        "Latvijas pārtikas produktu sastāva tabulas",
        "lv",
    ),
    ("albania-fct", "Tabelat e përbërjes së ushqimeve", "sq"),
    ("thailand-fct", "ตารางแสดงคุณค่าทางโภชนาการของอาหารไทย", "th"),
    ("korea-north-fct", "조선민주주의인민공화국 식품성분표", "ko"),
    ("korea-south-fct", "국가표준식품성분표", "ko"),
    (
        "ukraine-fct",
        "Таблиці складу харчових продуктів України",
        "uk",
    ),
    ("jamaica-fct", "Jamaican Food Composition Table", "en"),
    (
        "russia-fct",
        "Химический состав российских пищевых продуктов",
        "ru",
    ),
    ("china-fct", "中国食物成分表", "zh"),
    ("india-ifct", "Indian Food Composition Tables", "en"),
    ("israel-fct", "מאגר הרכב המזון הישראלי", "he"),
];

/// Build the registered source set.
///
/// Note `israel-fct` is an Axis A food-composition table and nothing else. Israel also
/// appears on Axis B for unrelated reasons; that material is off-repo and cannot be
/// represented here, because [`SourceKind`] has no variant able to hold it (F-3).
pub fn registry() -> Vec<Source> {
    let mut v = vec![
        Source {
            id: SourceId::new(USDA_FDC),
            kind: SourceKind::NationalTable,
            name: "USDA FoodData Central, SR Legacy".into(),
            language: "en".into(),
        },
        Source {
            id: SourceId::new(FAO_INFOODS),
            kind: SourceKind::Federation,
            name: "FAO/INFOODS".into(),
            language: "en".into(),
        },
    ];
    for (id, name, lang) in NATIONAL_TABLES {
        v.push(Source {
            id: SourceId::new(*id),
            kind: SourceKind::NationalTable,
            name: (*name).into(),
            language: (*lang).into(),
        });
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hearts() -> Basis {
        Basis::new("hulled hearts", ReferenceQuantity::Per100gAsDescribed)
    }
    fn concentrate() -> Basis {
        Basis::new("protein concentrate", ReferenceQuantity::Per100gAsDescribed)
    }
    fn g(a: f64) -> Measured<Quantity> {
        Measured::Value(Quantity::new(a, "g"))
    }
    fn mg(a: f64) -> Measured<Quantity> {
        Measured::Value(Quantity::new(a, "mg"))
    }

    /// **The invariant.** Twelve tables reprinting one USDA number is one determination.
    #[test]
    fn twelve_tables_copying_usda_is_one_determination() {
        let mut dets = vec![Determination::new(
            Origin::determined_by(SourceId::new(USDA_FDC), hearts()),
            g(31.6),
        )];
        for (id, _, _) in NATIONAL_TABLES.iter().take(12) {
            dets.push(Determination::new(
                Origin::borrowed(SourceId::new(*id), hearts(), SourceId::new(USDA_FDC)),
                g(31.6),
            ));
        }
        let rec = NutrientRecord::new(Tagname::new("PROCNT"), dets);
        assert_eq!(rec.independence(&hearts()), Independence::Determinations(1));
    }

    /// Borrowing is followed *transitively*: a chain of reprints collapses to its root.
    #[test]
    fn transitive_borrow_chain_collapses_to_one_root() {
        let rec = NutrientRecord::new(
            Tagname::new("FE"),
            vec![
                Determination::new(
                    Origin::determined_by(SourceId::new(USDA_FDC), hearts()),
                    mg(8.0),
                ),
                Determination::new(
                    Origin::borrowed(
                        SourceId::new("canada-cnf"),
                        hearts(),
                        SourceId::new(USDA_FDC),
                    ),
                    mg(8.0),
                ),
                // borrows from Canada, which borrowed from USDA
                Determination::new(
                    Origin::borrowed(
                        SourceId::new("jamaica-fct"),
                        hearts(),
                        SourceId::new("canada-cnf"),
                    ),
                    mg(8.0),
                ),
            ],
        );
        assert_eq!(rec.independence(&hearts()), Independence::Determinations(1));
    }

    /// Genuinely separate laboratory determinations do count separately.
    #[test]
    fn two_independent_labs_count_two() {
        let rec = NutrientRecord::new(
            Tagname::new("FE"),
            vec![
                Determination::new(
                    Origin::determined_by(SourceId::new(USDA_FDC), hearts()),
                    mg(8.0),
                ),
                Determination::new(
                    Origin::determined_by(SourceId::new("india-ifct"), hearts()),
                    mg(7.2),
                ),
            ],
        );
        assert_eq!(rec.independence(&hearts()), Independence::Determinations(2));
    }

    /// There is no way to raise a count from a borrowed value — the only path to a count
    /// is `independence()`, and it reads `borrowed_from`. This test documents the
    /// absence: adding borrowed sources never moves the number.
    #[test]
    fn adding_borrowed_sources_never_raises_the_count() {
        let base = vec![Determination::new(
            Origin::determined_by(SourceId::new(USDA_FDC), hearts()),
            g(31.6),
        )];
        let mut dets = base.clone();
        let mut last = Independence::Determinations(1);
        for (id, _, _) in NATIONAL_TABLES {
            dets.push(Determination::new(
                Origin::borrowed(SourceId::new(*id), hearts(), SourceId::new(USDA_FDC)),
                g(31.6),
            ));
            let n =
                NutrientRecord::new(Tagname::new("PROCNT"), dets.clone()).independence(&hearts());
            assert_eq!(n, last, "a borrowed source changed the count");
            last = n;
        }
        assert_eq!(last, Independence::Determinations(1));
    }

    /// Fail closed on a broken graph: a cycle yields no number at all.
    #[test]
    fn borrow_cycle_is_indeterminate_not_a_count() {
        let rec = NutrientRecord::new(
            Tagname::new("K"),
            vec![
                Determination::new(
                    Origin::borrowed(SourceId::new("a"), hearts(), SourceId::new("b")),
                    mg(100.0),
                ),
                Determination::new(
                    Origin::borrowed(SourceId::new("b"), hearts(), SourceId::new("a")),
                    mg(100.0),
                ),
            ],
        );
        match rec.independence(&hearts()) {
            Independence::Indeterminate(Indeterminate::BorrowCycle { .. }) => {}
            other => panic!("expected BorrowCycle, got {other:?}"),
        }
    }

    /// Fail closed on a citation to a source that is not present.
    #[test]
    fn dangling_borrow_is_indeterminate_not_a_count() {
        let rec = NutrientRecord::new(
            Tagname::new("K"),
            vec![Determination::new(
                Origin::borrowed(
                    SourceId::new("canada-cnf"),
                    hearts(),
                    SourceId::new("a-table-not-in-this-record"),
                ),
                mg(100.0),
            )],
        );
        match rec.independence(&hearts()) {
            Independence::Indeterminate(Indeterminate::DanglingBorrow { .. }) => {}
            other => panic!("expected DanglingBorrow, got {other:?}"),
        }
    }

    /// An indeterminate result must not be readable as zero.
    #[test]
    fn indeterminate_and_notmeasured_have_no_count() {
        assert_eq!(Independence::NotMeasured.count(), None);
        assert_eq!(
            Independence::Indeterminate(Indeterminate::DanglingBorrow {
                from: SourceId::new("x")
            })
            .count(),
            None
        );
        assert_eq!(Independence::Determinations(3).count(), Some(3));
    }

    /// Nobody measured it → NotMeasured. Never blank, never zero, never inferred.
    #[test]
    fn unmeasured_is_notmeasured_not_zero() {
        let rec = NutrientRecord::new(
            Tagname::new("F18D3CN3"),
            vec![Determination::new(
                Origin::determined_by(SourceId::new(USDA_FDC), hearts()),
                Measured::NotMeasured,
            )],
        );
        assert_eq!(rec.independence(&hearts()), Independence::NotMeasured);
        assert_eq!(rec.independence(&hearts()).count(), None);
    }

    // ── E-4 · the founder's own dataset is the acceptance test ──
    // All three divergences must survive. If any disappears, the feature is wrong.

    /// **Acceptance 1 — hemp protein 42% vs USDA 31.6 g is a BASIS difference.**
    /// Two correct numbers about two different foods: they must not be compared, must not
    /// be averaged, and must not be reported as a disagreement.
    #[test]
    fn acceptance_hemp_protein_basis_difference_is_not_a_disagreement() {
        let rec = NutrientRecord::new(
            Tagname::new("PROCNT"),
            vec![
                Determination::new(
                    Origin::determined_by(SourceId::new(USDA_FDC), hearts()),
                    g(31.6),
                ),
                Determination::new(
                    Origin::determined_by(SourceId::new("founder-ods"), concentrate()),
                    Measured::Value(Quantity::new(42.0, "%")),
                ),
            ],
        );

        // two distinct bases are surfaced, not collapsed
        assert_eq!(rec.bases().len(), 2);

        // each basis is one determination; neither is contaminated by the other
        assert_eq!(rec.independence(&hearts()), Independence::Determinations(1));
        assert_eq!(
            rec.independence(&concentrate()),
            Independence::Determinations(1)
        );

        // and crucially: no spread, because there is no disagreement to report
        assert!(rec.compare(&hearts()).spread.is_none());
        assert_eq!(rec.compare(&hearts()).values.len(), 1);
        assert_eq!(rec.compare(&concentrate()).values.len(), 1);
    }

    /// **Acceptance 2 — hemp iron 0.28 vs ~8 is a real unresolved divergence.**
    /// Both values shown, spread reported, nothing resolved.
    #[test]
    fn acceptance_hemp_iron_divergence_is_shown_not_resolved() {
        let rec = NutrientRecord::new(
            Tagname::new("FE"),
            vec![
                Determination::new(
                    Origin::determined_by(SourceId::new(USDA_FDC), hearts()),
                    mg(7.95),
                ),
                Determination::new(
                    Origin::determined_by(SourceId::new("founder-ods"), hearts()),
                    mg(0.28),
                ),
            ],
        );
        let c = rec.compare(&hearts());
        assert_eq!(c.independence, Independence::Determinations(2));

        let s = c.spread.expect("a real divergence must report a spread");
        assert!(s.differs(), "the disagreement must survive");
        assert_eq!(s.min, 0.28);
        assert_eq!(s.max, 7.95);

        // both values still present — neither dropped, no average anywhere
        assert_eq!(c.values.len(), 2);
    }

    /// **Acceptance 3 — beef potassium 252 in a grams column is flagged, not corrected.**
    #[test]
    fn acceptance_beef_potassium_unit_slip_is_flagged_not_corrected() {
        let basis = Basis::new("raw, lean", ReferenceQuantity::Per100gAsDescribed);
        let published = Quantity::new(252.0, "g");

        match unit_plausibility(&published, &basis) {
            UnitPlausibility::ExceedsWhole { .. } => {}
            other => panic!("252 g per 100 g must be flagged, got {other:?}"),
        }

        // the value is preserved exactly as published — not corrected to 252 mg,
        // not dropped, not clamped
        let rec = NutrientRecord::new(
            Tagname::new("K"),
            vec![Determination::new(
                Origin::determined_by(SourceId::new("founder-ods"), basis.clone()),
                Measured::Value(published.clone()),
            )],
        );
        let c = rec.compare(&basis);
        assert_eq!(c.values[0].1, published);
        assert_eq!(c.values[0].1.amount, 252.0);
        assert_eq!(c.values[0].1.unit, "g");
    }

    /// A plausible value is not flagged — the negative control, so the check is known to
    /// discriminate rather than to pass everything.
    #[test]
    fn plausible_values_are_not_flagged() {
        let basis = Basis::new("raw, lean", ReferenceQuantity::Per100gAsDescribed);
        assert_eq!(
            unit_plausibility(&Quantity::new(252.0, "mg"), &basis),
            UnitPlausibility::Plausible
        );
        assert_eq!(
            unit_plausibility(&Quantity::new(31.6, "g"), &basis),
            UnitPlausibility::Plausible
        );
    }

    /// Mixed units are not silently converted; a conversion would be a derived value
    /// needing its own origin (F-4).
    #[test]
    fn mixed_units_report_values_without_a_fabricated_spread() {
        let rec = NutrientRecord::new(
            Tagname::new("FE"),
            vec![
                Determination::new(
                    Origin::determined_by(SourceId::new(USDA_FDC), hearts()),
                    mg(8.0),
                ),
                Determination::new(
                    Origin::determined_by(SourceId::new("india-ifct"), hearts()),
                    g(0.008),
                ),
            ],
        );
        let c = rec.compare(&hearts());
        assert!(c.spread.is_none(), "must not compare across units");
        assert_eq!(c.values.len(), 2, "but both values are still shown");
    }

    /// A converted value is a borrow, so conversion cannot manufacture independence.
    #[test]
    fn a_unit_conversion_is_a_borrow_not_a_determination() {
        let rec = NutrientRecord::new(
            Tagname::new("FE"),
            vec![
                Determination::new(
                    Origin::determined_by(SourceId::new(USDA_FDC), hearts()),
                    mg(8.0),
                ),
                Determination::new(
                    Origin::borrowed(
                        SourceId::new("usda-fdc-converted"),
                        hearts(),
                        SourceId::new(USDA_FDC),
                    ),
                    g(0.008),
                ),
            ],
        );
        assert_eq!(rec.independence(&hearts()), Independence::Determinations(1));
    }

    #[test]
    fn registry_carries_the_directed_country_set() {
        let r = registry();
        assert_eq!(r.len(), 2 + NATIONAL_TABLES.len());
        assert_eq!(NATIONAL_TABLES.len(), 14);
        for id in [
            "israel-fct",
            "latvia-fct",
            "korea-north-fct",
            "korea-south-fct",
        ] {
            assert!(r.iter().any(|s| s.id == SourceId::new(id)), "missing {id}");
        }
    }

    /// The wall (F-3): every source kind is a food-composition kind. This test exists so
    /// that adding a trial/patent variant fails a test that says why.
    #[test]
    fn source_kinds_are_axis_a_only() {
        for s in registry() {
            match s.kind {
                SourceKind::NationalTable
                | SourceKind::Federation
                | SourceKind::LaboratoryReport
                | SourceKind::ManufacturerDeclaration => {}
            }
        }
    }

    #[test]
    fn fixtures_round_trip() {
        let rec = NutrientRecord::new(
            Tagname::new("PROCNT"),
            vec![Determination::new(
                Origin::determined_by(SourceId::new(USDA_FDC), hearts()),
                g(31.6),
            )],
        );
        let j = serde_json::to_string(&rec).unwrap();
        let back: NutrientRecord = serde_json::from_str(&j).unwrap();
        assert_eq!(rec, back);
    }
}
