# Contributing to beehive-WELLness

## The three rules that make this tree safe to exist

1. **Per-crate license, against the real dependency graph.** A crate that links no
   kernel crate ships MIT OR Apache-2.0 and pins nothing from the AGPL kernel. A crate
   that must link kernel internals is AGPL-3.0 and says so. Never label a crate
   permissive while it Cargo-depends on the AGPL kernel — that is not a permissive
   artifact, and the label would be false. (See README · Licensing.)
2. **Category gate, not keyword gate.** A surface is cleared for publish by human
   category-review. "grep found nothing" is not "category-clean" — that lesson is why
   the gate is a person, not a regex.
3. **No health *claim* in code or copy.** Established biochemistry may be stated; any
   physiological or longevity *outcome* claim must be graded (proved / hypothesized)
   and sourced (USDA / CNF / citation), never asserted.

## Enforced on every commit and in CI

- `scripts/secret-scan.sh` — secret-shaped filenames, 48+ hex runs, PEM blocks.
- `scripts/phi-scan.sh` — emails, SSN shapes, DOB shapes in tracked text. A deliberate
  synthetic value carries a same-line `SYNTHETIC-OK` marker. This is a **backstop** to
  the structural guarantee (no PII field exists), not a substitute for it.

Enable the hooks: `git config core.hooksPath .githooks`.

## Structural guarantees are not optional

New crates/types must not add: an invented-number path (use `Measured<T>`), a PII field
(name / MRN / SSN / DOB), or a goal-weight / deficit field. ED-safety is a type-level
gate the interpretation layer cannot route around. A reviewer must be able to
*demonstrate* each guarantee from the types, not read it in a comment.

## Commits

Follows the kernel's receipts: an author of record, machine seats credited via
`Co-authored-by`, no machine self-sign-off. External contributions use DCO
(`git commit -s`). Real command + real output for any "it builds / it passes" claim.
