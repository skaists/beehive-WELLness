<p align="center"><img src="assets/beehive-wellness-logo.svg" alt="beehive-WELLness mandala — lime rim through teal to a purple core" width="200"></p>

# beehive-WELLness

*The nutrition-literacy tree of the Beehive Nature Reserve. It teaches people to read food, and it holds nothing it shouldn't.*

> **CONCEPT · simulated data · not medical or nutritional advice.** This tree teaches how to evaluate nutrition claims and delivers none. Anything touching real personal health data is counsel-gated and off by default.

## Why this is its own tree

The BNR kernel ([`beehive-nature`](https://github.com/beehive-nature/beehive-nature)) carries **governance and economics only** — the k001 wall. Nutrition literacy, food-composition data, and health-data infrastructure are neither, so they live here, downstream of the kernel like [`skaists/LOVErnment-DAO`](https://github.com/skaists/LOVErnment-DAO).

This isn't fastidiousness — it's **compartmentalized liability**. A legal or regulatory challenge to nutrition/health material structurally cannot reach the kernel, because the thing to challenge isn't in it. The wall is legal armor; this tree boundary is where the blast radius stops. (It is designed against a capture vector — it alleges no one.)

## The three structural guarantees (verifiable, not promised)

Every crate encodes its safety in the type system, so a reviewer can *demonstrate* it, not just read a disclaimer:

- **`fat-profile` can't invent a number** — `Measured<T> { Value | NotMeasured }`; no code path fills a gap with a guess.
- **`health-vault` can't re-identify** — the subject is a pseudonymous id; no name / MRN / SSN / DOB field exists in the crate.
- **The interpretation layer can't prescribe or harm** — no goal-weight or deficit field exists; ED-safety is a type-level gate the interpretation layer cannot route around, not a warning it chooses to show. The advice ceiling is *"consider a clinician."*

A claim never made can't ground a public-risk case; a field that doesn't exist can't be misused.

## Dependency direction — one-way, downward

`beehive-WELLness → beehive-nature`, **never the reverse.** The kernel does not know this tree exists; nothing here can be imported upward. That one-way edge is the wall in the build graph, enforced by `cargo`, not by memory.

Today no crate here *needs* kernel internals — `fat-profile` is `serde`-only, `health-vault` defines its own no-PII types — so the tree currently links nothing from the kernel at all (see **Licensing**, which is why it can).

## Licensing — decided per crate, against the real dependency graph

The kernel is **AGPL-3.0-only** by design (anti-capture). AGPL is copyleft: *anything that statically links an AGPL crate becomes AGPL* — an MIT label on a crate that Cargo-depends on a kernel crate is fiction. So the rule here is per-crate, not blanket:

- A crate that **does not link** any kernel crate ships **MIT OR Apache-2.0** and pins nothing from the kernel. `fat-profile` (serde-only) and `health-vault` (own types) are both here — cleanly permissive.
- A crate that needs to **talk to** the kernel network (wire boundary — a client/SDK that serializes to it) stays a separate work and can be MIT OR Apache-2.0. This is the kernel's own `LICENSING.md` "permissive SDK edges" intent.
- A crate that must **link kernel internals** is **AGPL-3.0**, labelled honestly, and pins the kernel by rev. None exist yet.

Curriculum and docs: **CC-BY-4.0**. The kernel it consumes stays **AGPL-3.0-only**.

## Layout

```
beehive-WELLness/
├─ Cargo.toml            # workspace — pins NO kernel crate today (nothing here needs one)
├─ crates/
│  ├─ fat-profile/       # USDA/CNF fatty-acid reader · Measured<T> · mock-first · MIT/Apache
│  └─ health-vault/      # no-PII health record types · pseudonymous subject · MIT/Apache
├─ surfaces/             # static, L1, zero-network concept surfaces (the modules, the scanner)
├─ dockets/              # crate + curriculum specs
├─ fixtures/             # synthetic ONLY — the PHI-marker guard fails the build on real PHI
├─ .githooks/            # secret-scan + PHI-marker scan
└─ CONTRIBUTING.md  SECURITY.md  LICENSE-MIT  LICENSE-APACHE
```

## Wall rules (CI-enforced)

- **PHI-marker guard** — fixtures/tests fail the build on an email, SSN-shape, or DOB-shape. A backstop to the structural "no-PII-field" guarantee, not a substitute (names and dates aren't as pattern-regular as keys; the field that doesn't exist is what carries the weight).
- **Category gate, not keyword gate** — a surface clears for publish by human category-review. "grep found no bad words" ≠ "this is category-clean."
- **No health *claim* in code or copy** — established biochemistry may be stated; any physiological or longevity *outcome* claim must be graded and sourced (USDA/CNF/citation), never asserted.
- **Real PHI, provider import, or interpretation-that-diagnoses** — counsel-gated, off by default, never in a default build.
