# Test Critique — Sprint 0

Final pass: the third read-only critic run, against tested head `50dc351` and
the result files, report and INT-0001 evidence line as committed with this
file. Each concern carries the response taken.

## Concerns

### C-001: The guard fixes the default-`legendary` problem, but a skipped run still shows as passed
- **Where:** `crates/panaseeya-cli/tests/cli.rs` `legendary_on_path()`; `e2e-tests.md` row `cli_live_import_with_default_program_absent_names_legendary`
- **Quote:** "if legendary_on_path() { eprintln!(\"skipped: ...\"); return; }"
- **Failure mode:** evidence-drift
- **Why it matters:** An early return counts as `ok`, not `ignored`, so on a machine with Legendary installed the pass count would include a test that checked nothing. The critic confirmed the recorded run did not skip.
- **Suggested response:** defer-with-rationale
- **Response (deferred):** documented in `e2e-tests.md` and in `test-report.md` Coverage Observations, with the instruction to check standard error for "skipped" before copying a pass count.

### C-002: The test report says CI is "not configured", but the workflow exists
- **Where:** `test-report.md` Summary and CI Confirmation
- **Quote:** "CI status: not-configured (the workflow exists but has never run ...)"
- **Failure mode:** evidence-drift
- **Why it matters:** Contradicts E-002.1's evidence; the true state is configured, never run.
- **Suggested response:** tighten-assertion
- **Response (done):** both places now say "configured, never run".

### C-003: The result files at the recorded head are empty; the evidence exists only in the working tree
- **Where:** result files, `test-report.md`, `critique.md`, INT-0001 Test evidence
- **Quote:** "`git diff` shows `index e69de29..` (the empty blob) for all three results files at `50dc351`"
- **Failure mode:** evidence-drift
- **Why it matters:** The recorded head carries no evidence until the docs are committed.
- **Suggested response:** add-test (process step)
- **Response (done):** all evidence files and the INT-0001 line are committed together in a docs-only commit after `50dc351`; the report notes that the tested code is exactly `50dc351`.

### C-004: Criteria 3 and 8 are tested, not observed; criterion 1's host run and the Linux `sh` branch were never run
- **Where:** `test-report.md` rows for criteria 1, 3, 8; INT-0001 Test evidence; `e2e-tests.md` "Not verified on Linux"
- **Quote:** "criteria 3 and 8 tested, not observed against real Legendary; CI not yet run"
- **Failure mode:** intent-coverage
- **Why it matters:** Real-tool and host-run evidence are missing; recorded correctly with named unlocks.
- **Suggested response:** defer-with-rationale
- **Response (deferred):** INT-0001 stays `active`; the Loop phase carries the real Legendary run and the first CI run as backlog items.

### C-005: The resolver test's oracle still copies the implementation (carried over, accepted)
- **Where:** `db_path.rs` `resolve_db_path_defaults_to_user_data_dir` (E-010.6)
- **Quote:** "Its main oracle remains what `directories` reports, as the locked plan defines"
- **Failure mode:** stub-leak
- **Why it matters:** A wrong qualifier or organisation argument would still pass; small risk.
- **Suggested response:** defer-with-rationale
- **Response (accepted as plan-conformant):** the locked plan defines this oracle; the `Panaseeya`-component check is the independent part.

## Earlier passes

- **Pass 1** (head `49af771`, verdict `block`, nine concerns). Fixed in `3fede0c`: case-insensitive title ordering (E-007.1), origin tie-break through `list()` (E-007.2), default `legendary` program exercised (E-011.4), stored-row comparison for live vs file import (E-011.3), hand-written integration oracle (E-008.5), rendered newer-schema message (E-004.4).
- **Pass 2** (head `3fede0c`, verdict `proceed-with-caveats`). Its C-001 (the default-program test would run a real installed Legendary on Windows) was fixed in `50dc351` with a `PATH` guard; its ETXTBSY remedy was corrected to fit the runner's single-program API.

## Confidence
proceed-with-caveats
