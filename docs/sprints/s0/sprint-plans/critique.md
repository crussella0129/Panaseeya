# Plan Critique — Sprint 0

This is the second and final critic pass, run by a read-only reviewer against
the plans as rewritten after the first pass. The first pass and what it changed
are summarised under "Earlier pass". Each concern below carries the response
that was applied before the plans were locked.

## Concerns

### C-001: A T-010 test needs the import command that T-011 builds
- **Where:** `test-plan.md` End-to-End Tests, `cli_list_prints_one_line_per_title` (E-010.3) / `build-plan.md` T-010, T-011
- **Quote:** "`cli_list_prints_one_line_per_title` (E-010.3): after a fixture import and a manual add of an already-imported game"
- **Failure mode:** hidden-dep
- **Why it matters:** E-010.3 belongs to T-010, but its only test populated the catalog through `import legendary`, which does not exist until T-011. T-010 could not be shown green in the strict chain.
- **Suggested response:** fix-in-plan
- **Response applied:** fixed. The T-010 test now seeds through three `add` calls. A new clause E-011.7 and test `cli_list_after_import_shows_store_and_manual_on_one_line` cover the mixed store-and-manual line in T-011.

### C-002: E-003.1 is contradicted by the E-003.2 test table and by the stated rule
- **Where:** `build-plan.md` T-003 E-003.1 and Notes / `test-plan.md` `normalize_keeps_different_words_apart` / INT-0001 criterion 6 vs Consequences
- **Quote:** "two titles that differ only by letter case, punctuation, ... **SHALL** return the same key for both"
- **Failure mode:** EARS-vague
- **Why it matters:** The rule replaces punctuation with a space, so titles differing only by punctuation inside a word ("Meier's"/"Meiers") get different keys. E-003.1 as written was false for the prescribed implementation, and INT-0001 criterion 6 had the same conflict with its own Consequences.
- **Suggested response:** fix-in-plan
- **Response applied:** fixed. E-003.1 and E-003.2 now state the word-sequence rule. INT-0001 criterion 6 was reworded to the same rule, with the in-word punctuation case stated beside it and the change recorded in its Transition history.

### C-003: T-005 needs the medium type that T-006 is titled to create
- **Where:** `build-plan.md` T-005 / T-006
- **Quote:** "### T-006: Implement manual entries and the medium type"
- **Failure mode:** hidden-dep
- **Why it matters:** Every ownership recorded in T-005 must carry a medium and an origin to satisfy the T-004 schema, so the types must exist one task earlier than claimed.
- **Suggested response:** fix-in-plan
- **Response applied:** fixed. T-005 defines the `Medium` and `Origin` types; T-006 is retitled and narrowed to manual entries and parsing a medium from text.

### C-004: No task or clause owns turning parsed Legendary games into `epic` store ownerships in the core
- **Where:** `build-plan.md` T-008, T-011 E-011.1 / `test-plan.md` Catalog core integration
- **Quote:** "carrying that element's `app_name` as the source identifier and `app_title` as the title"
- **Failure mode:** plan-test-mismatch
- **Why it matters:** The `epic`/`digital`/`store` mapping first appeared as a clause in T-011, which touches only the CLI crate, while the T-008 integration tests already needed it in the core.
- **Suggested response:** fix-in-plan
- **Response applied:** fixed. New clause E-008.5 defines `import_legendary_document` in the core, with test `import_document_records_epic_digital_store_ownerships`; the integration tests cite it and E-011.1 calls it.

### C-005: The default database location is never created or exercised by the binary
- **Where:** `build-plan.md` T-010 E-010.5, E-010.6 / `test-plan.md` `resolve_db_path_defaults_to_user_data_dir`
- **Quote:** "no automated test runs the binary without `--db` or `PANASEEYA_DATA_DIR`"
- **Failure mode:** plan-test-mismatch
- **Why it matters:** No clause required the per-user directory, or a missing `--db` parent, to be created before SQLite opens the file.
- **Suggested response:** fix-in-plan
- **Response applied:** fixed. New clause E-010.7 requires the parent directory of any resolved path to be created, with test `cli_creates_missing_parent_directories`. Caveat kept: the default per-user location is still verified only as a resolved path, because running the binary there would write into the owner's real profile.

### C-006: An import batch rejected by the catalog has no CLI clause
- **Where:** `build-plan.md` T-011 E-011.6; T-008 E-008.4; T-005 E-005.5
- **Quote:** "**WHEN** `--file` names a file that does not exist or whose content the parser rejects"
- **Failure mode:** plan-test-mismatch
- **Why it matters:** A document with a blank `app_title` passed the parser and would be rejected by the catalog only after the database had been opened or created.
- **Suggested response:** fix-in-plan
- **Response applied:** fixed. E-008.4 now makes the parser reject a missing, non-string, or blank `app_name` or `app_title`, and its test covers both blank cases, so E-011.6 covers this path before the database is opened.

### C-007: The CI operating system is unspecified while the runner tests have per-OS branches
- **Where:** `build-plan.md` T-002 E-002.1; T-009 Notes / `test-plan.md` T-009 Stubs
- **Quote:** "stand-in script written to a temporary directory (`.cmd` on Windows, `sh` elsewhere)"
- **Failure mode:** missing-risk
- **Why it matters:** The build machine is Windows, so only the `.cmd` branch is observed during the sprint.
- **Suggested response:** fix-in-plan or defer-with-rationale
- **Response applied:** both. E-002.1 now requires a `windows-latest` and `ubuntu-latest` matrix. The test plan records that the non-Windows branch stays unverified until the first CI run, which needs a push the owner has not yet requested.

### C-008: The real-Legendary verification has no named sprint or tracked place to land
- **Where:** `test-plan.md` End-to-End Tests, `manual_real_legendary_import` / `build-plan.md` Intents
- **Quote:** "Unlocked by: the project owner running it after installing and signing in to Legendary"
- **Failure mode:** e2e-drift
- **Why it matters:** Nothing in the Book scheduled the run or said where its result is recorded.
- **Suggested response:** defer-with-rationale
- **Response applied:** deferred with rationale. The run needs the owner's Epic account and cannot be scheduled into a sprint by the agent. The test plan now states that Sprint 0 completion evidence lists criteria 3 and 8 as "tested, not observed", that the Loop Phase carries the run as a backlog task against INT-0001, and that its result is a dated entry in INT-0001's Transition history. INT-0001's observation rule blocks `realized` until then.

## Confidence
proceed-with-caveats

Reviewer's closing notes: clause-to-test coverage was complete in both
directions at the time of review, INT-0001 criteria 1 to 8 each had planned
work and verification, and research risks were carried or deferred. The
reviewer did not itself re-read Legendary's `cli.py`, so the nested-`dlcs`
document shape rests on the planning pass that read it.

Caveats that remain after the responses above:
- Criteria 3 and 8 are tested against a hand-built document and a stand-in
  executable, not a real Legendary install.
- The CI workflow and the non-Windows test branch are unobserved until the
  branch is pushed.
- The default per-user database location is verified as a path only.
- The fixes for this pass added clauses E-008.5, E-010.7, and E-011.7 and
  reworded E-002.1, E-003.1, E-003.2, E-008.4, E-010.5, and E-011.1. They
  were checked mechanically (all 46 clause identifiers in the build plan appear
  in the test plan) but were not sent through a third critic pass.

## Earlier pass

The first pass returned `proceed-with-caveats` with thirteen concerns against
an eight-task draft. Responses, all applied before the second pass:
- Shared files across tasks declared independent → tasks made one strict
  chain; `error.rs` ownership and `Cargo.lock` touches stated.
- Integration tests without an owning task → assigned to T-008.
- Default-path test unsafe on Windows → `PANASEEYA_DATA_DIR` override and a
  pure path resolver.
- Fixture-fidelity risk dropped for criterion 3 → caveat extended to criterion
  3; observation rule added to INT-0001.
- Batch failure with no way to fail → blank-field rejection rule (E-005.5).
- Unspecified CLI output formats → listing text and JSON forms and ownership
  ordering stated; the two-outcome `add` clause split.
- Missing CLI clauses for failed Legendary and bad `--file` → E-011.5, E-011.6.
- Migration ordering untestable with one migration → runner takes a step list;
  ordering and mid-way failure tested (E-004.3, E-004.5).
- Decisions living only in sprint prose → record origin, manual-entry
  identity, file location, and listing contract added to INT-0001 Consequences;
  an `origin` column distinguishes manual from store records.
- Default database location traced to the wrong criterion → moved to criterion 2.
- Oversized tasks → the catalog task split in three and the CLI task in two
  (eleven tasks); the CI and Dependabot files deliberately kept as one task.
- End-to-end status and real-Legendary unlock → observation rule in INT-0001.
- Unpinned dependency versions → deferred with rationale: the committed
  `Cargo.lock` is the pin; the crate set is listed in the build plan.
