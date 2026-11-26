# Sprint 0 Test Report

## Intent Verification
| Intent | Acceptance criterion | EARS / tests | Result | Intent evidence update |
|--------|----------------------|---------------|--------|------------------------|
| [INT-0001](../../../intents/INT-0001-unified-catalog-core.md) | 1 — builds; fmt, clippy, test pass, and the host runs them | T-001 / E-001.1–3 command checks; T-002 / E-002.1–2 file checks | pass locally on Windows; **host run not observed** (no CI run, `dev` not pushed) | Test evidence links this report |
| [INT-0001](../../../intents/INT-0001-unified-catalog-core.md) | 2 — single SQLite file, versioned schema | T-004 / E-004.1–5; T-010 / E-010.5–7 | pass (default per-user location checked as a resolved path only) | Test evidence links this report |
| [INT-0001](../../../intents/INT-0001-unified-catalog-core.md) | 3 — one `epic` ownership per base game, none for DLC | T-008 / E-008.1–5; T-011 / E-011.1, E-011.6 | **tested, not observed**: pass on the hand-built fixture; `manual_real_legendary_import` not verified | Test evidence links this report; not eligible for `realized` until the real run is recorded |
| [INT-0001](../../../intents/INT-0001-unified-catalog-core.md) | 4 — importing twice changes nothing | T-005 / E-005.1, E-005.4, E-005.5; T-011 / E-011.2 | pass | Test evidence links this report |
| [INT-0001](../../../intents/INT-0001-unified-catalog-core.md) | 5 — manual entry with label and medium | T-006 / E-006.1–4; T-010 / E-010.1–2 | pass | Test evidence links this report |
| [INT-0001](../../../intents/INT-0001-unified-catalog-core.md) | 6 — equivalent titles group, different titles do not | T-003 / E-003.1–3; T-005 / E-005.2–3 | pass | Test evidence links this report |
| [INT-0001](../../../intents/INT-0001-unified-catalog-core.md) | 7 — `list` shows each title once with all sources, also as JSON | T-007 / E-007.1–2; T-010 / E-010.3–4; T-011 / E-011.7 | pass | Test evidence links this report |
| [INT-0001](../../../intents/INT-0001-unified-catalog-core.md) | 8 — live import runs Legendary; absent tool fails cleanly | T-009 / E-009.1–3; T-011 / E-011.3–5 | **tested, not observed**: pass with a Windows stand-in and with Legendary absent; real Legendary not run; `sh` stand-in branch not run | Test evidence links this report; not eligible for `realized` until the real run is recorded |

Every EARS clause E-001.1 to E-011.7 maps to at least one executed check or
test; see [unit-tests.md](unit-tests.md), [integration-tests.md](integration-tests.md)
and [e2e-tests.md](e2e-tests.md).

## Summary
- Unit tests: 27 passed / 0 failed / 27 total (26 core, 1 CLI), plus 5 project checks passed
- Integration tests: 3 passed / 0 failed / 3 total
- E2E tests: 14 passed / 0 failed / 14 total; 1 manual test (`manual_real_legendary_import`) not verified
- CI status: configured, never run (`.github/workflows/sprint-loops-ci.yml` exists; `dev` is not pushed)

## CI Confirmation
- **Head SHA:** `50dc3511dfe497afe6994c64d69e8e201d2a0e60`
- **CI run:** none
- **Conclusion:** none
- **Confirmations:** CI configured but never run — local confirmations only. On the head
  above, Windows 11 with `rustc 1.99.1`: `cargo build --workspace`,
  `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`, and
  `cargo test --workspace` each exited 0; `git status --porcelain` showed
  nothing under `target/`. Unlock: the first push of `dev` runs
  `.github/workflows/sprint-loops-ci.yml` on `windows-latest` and
  `ubuntu-latest`.

## Failures
None. Changes made during this phase were to tests only (commits `49af771`,
`3fede0c`, `50dc351`), to tighten assertions after the test critique; no
product code changed.

## Technical Debt Identified
- [INT-0001](../../../intents/INT-0001-unified-catalog-core.md) — `manual_real_legendary_import`: run a real, signed-in Legendary (saved document and live) and record the result in INT-0001's Transition history. Needs the owner's Epic account.
- [INT-0001](../../../intents/INT-0001-unified-catalog-core.md) — first CI run on `dev`: observes criterion 1 on the host and the `sh` stand-in branch; may flake with ETXTBSY on Linux (remedy recorded in e2e-tests.md).
- [INT-0001](../../../intents/INT-0001-unified-catalog-core.md) — the default per-user database location is checked only as a resolved path; no test writes to the real profile.

## Coverage Observations
- The Legendary fixture is hand-built from Legendary's source, not captured from a real run, so extra keys or a different DLC layout in real output are untested.
- The live-import tests use stand-in scripts; `cli_live_import_with_default_program_absent_names_legendary` skips itself on any machine where `legendary` is on `PATH`. A skip still counts as `ok` in `cargo test`, so on such a machine check its standard error for "skipped" before copying the pass count. On the recorded run it did not skip (`where legendary` finds nothing).
- Result files were committed after the tested head in a docs-only commit; the tested code is exactly `50dc351`.
- Title grouping follows the word-sequence rule exactly; titles that differ only by edition words ("GOTY", "Definitive Edition") stay separate by design.
- Critique: [critique.md](critique.md), final verdict `proceed-with-caveats`.
