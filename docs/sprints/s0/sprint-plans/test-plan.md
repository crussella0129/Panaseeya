Finalized - DO NOT EDIT

# Sprint 0 Test Plan

Clause identifiers (`E-NNN.k`) are defined in the build plan.

## Intent Traceability
| Intent | Acceptance criterion | Build task / EARS clause | Verification |
|--------|----------------------|--------------------------|--------------|
| [INT-0001](../../../intents/INT-0001-unified-catalog-core.md) | 1 — builds; fmt, clippy, test pass | T-001 / E-001.1, E-001.2, E-001.3 | `check_workspace_builds`, `check_fmt_clippy_test_pass`, `check_target_dir_ignored` |
| [INT-0001](../../../intents/INT-0001-unified-catalog-core.md) | 1 — checks are run by the host | T-002 / E-002.1, E-002.2 | `check_ci_workflow_triggers_and_commands`, `check_dependabot_has_cargo_entry` |
| [INT-0001](../../../intents/INT-0001-unified-catalog-core.md) | 2 — single SQLite file, program-owned versioned schema | T-004 / E-004.1, E-004.2, E-004.3, E-004.4, E-004.5; T-010 / E-010.5, E-010.6, E-010.7 | `open_creates_schema_at_current_version`, `reopen_current_schema_changes_nothing`, `migrate_applies_pending_steps_in_order`, `open_rejects_newer_schema`, `migrate_stops_at_failed_step`, `cli_data_dir_override_holds_catalog_db`, `resolve_db_path_defaults_to_user_data_dir`, `cli_creates_missing_parent_directories` |
| [INT-0001](../../../intents/INT-0001-unified-catalog-core.md) | 3 — one `epic` ownership per base game, none for DLC (hand-built document only; real document not observed) | T-008 / E-008.1, E-008.2, E-008.3, E-008.4, E-008.5; T-011 / E-011.1, E-011.6 | `parse_returns_one_game_per_base_entry`, `parse_ignores_nested_dlcs`, `parse_skips_top_level_dlc_entry`, `parse_rejects_malformed_document`, `import_document_records_epic_digital_store_ownerships`, `cli_import_file_then_list_shows_epic_titles`, `cli_import_bad_file_fails_and_leaves_db_untouched`; real document: `manual_real_legendary_import` (not executable in this sprint) |
| [INT-0001](../../../intents/INT-0001-unified-catalog-core.md) | 4 — importing twice changes nothing | T-005 / E-005.1, E-005.4, E-005.5; T-011 / E-011.2 | `record_same_ownership_twice_is_noop`, `batch_with_rejected_item_records_nothing`, `record_rejects_blank_fields`, `import_fixture_twice_leaves_catalog_unchanged`, `cli_import_file_twice_is_idempotent` |
| [INT-0001](../../../intents/INT-0001-unified-catalog-core.md) | 5 — manual entry with label and medium | T-006 / E-006.1, E-006.2, E-006.3, E-006.4; T-010 / E-010.1, E-010.2 | `manual_entry_records_label_medium_and_origin`, `manual_entry_repeat_is_noop_and_new_medium_adds`, `medium_parse_rejects_unknown_value`, `manual_label_matching_store_name_stays_distinct`, `cli_add_then_list_shows_manual_entry`, `cli_add_with_invalid_medium_fails_and_changes_nothing` |
| [INT-0001](../../../intents/INT-0001-unified-catalog-core.md) | 6 — equivalent titles group, different titles do not | T-003 / E-003.1, E-003.2, E-003.3; T-005 / E-005.2, E-005.3 | `normalize_merges_case_punctuation_trademark_whitespace`, `normalize_keeps_different_words_apart`, `normalize_falls_back_when_only_punctuation`, `ownerships_with_equivalent_titles_share_one_title`, `ownerships_with_different_titles_stay_separate`, `epic_and_manual_copies_group_under_one_title` |
| [INT-0001](../../../intents/INT-0001-unified-catalog-core.md) | 7 — `list` shows each title once with all sources, also as JSON | T-007 / E-007.1, E-007.2; T-010 / E-010.3, E-010.4; T-011 / E-011.7 | `list_returns_each_title_once_in_name_order`, `list_orders_ownerships_within_title`, `cli_list_prints_one_line_per_title`, `cli_list_json_matches_text_listing`, `cli_list_after_import_shows_store_and_manual_on_one_line` |
| [INT-0001](../../../intents/INT-0001-unified-catalog-core.md) | 8 — live import runs Legendary; absent tool fails cleanly (stand-in executable only; real Legendary not observed) | T-009 / E-009.1, E-009.2, E-009.3; T-011 / E-011.3, E-011.4, E-011.5 | `runner_returns_stdout_on_success`, `runner_reports_missing_tool_by_name`, `runner_reports_nonzero_exit_with_stderr`, `cli_live_import_matches_file_import`, `cli_live_import_without_legendary_fails_and_leaves_db_untouched`, `cli_live_import_failed_legendary_fails_and_leaves_db_untouched`; real tool: `manual_real_legendary_import` (not executable in this sprint) |

## Unit Tests

### T-001 project checks
- **Intent:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)
- `check_workspace_builds` (E-001.1): run `cargo build --workspace` → exit 0.
- `check_fmt_clippy_test_pass` (E-001.2): run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` → each exits 0.
- `check_target_dir_ignored` (E-001.3): after a build, `git status --porcelain` lists nothing under `target/`.
- These are command checks recorded with their output in the test report, not Rust test functions.

### T-002 configuration checks
- **Intent:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)
- `check_ci_workflow_triggers_and_commands` (E-002.1): read `.github/workflows/sprint-loops-ci.yml` → push and pull_request triggers for `dev` and `main`; a `windows-latest` and `ubuntu-latest` matrix; `stable` toolchain; the three commands verbatim. This checks file content only; the workflow is not observed running until the branch is pushed, and the test report says so. The build machine is Windows, so the non-Windows branch of the stand-in-script tests (T-009, T-011) is unverified until that first CI run; the test report records that too.
- `check_dependabot_has_cargo_entry` (E-002.2): read `.github/dependabot.yml` → a `cargo` entry with `target-branch: "dev"` exists and the `github-actions` entry remains.

### T-003 unit tests
- **Intent:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)
- `normalize_merges_case_punctuation_trademark_whitespace` (E-003.1): pairs with the same word sequence, which must yield equal keys — `"DOOM"`/`"Doom"`; `"Half-Life"`/`"Half Life"`; `"Sid Meier's Civilization® VI"`/`"Sid Meier’s Civilization VI"`; `"  Hades  "`/`"Hades"`; `"The Witcher 3: Wild Hunt"`/`"The Witcher 3 - Wild Hunt"`; `"Control™"`/`"Control"`.
- `normalize_keeps_different_words_apart` (E-003.2): pairs whose word sequences differ, which must yield different keys — `"Portal"`/`"Portal 2"`; `"HalfLife"`/`"Half Life"`; `"Hades"`/`"Hades II"`; `"Doom"`/`"Doom Eternal"`; `"Sid Meier's Pirates"`/`"Sid Meiers Pirates"`.
- `normalize_falls_back_when_only_punctuation` (E-003.3): `"  !!!  "` → `"!!!"`, not an empty key.
- Stubs: none.

### T-004 unit tests
- **Intent:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)
- `open_creates_schema_at_current_version` (E-004.1): open a path in a temp directory → one file exists; `titles` and `ownerships` are present; `user_version` equals the current version.
- `reopen_current_schema_changes_nothing` (E-004.2): open, insert rows, close, reopen → `user_version`, the `sqlite_master` schema text, and the rows are identical.
- `migrate_applies_pending_steps_in_order` (E-004.3): a test-supplied three-step list in which step 3 alters the table step 2 creates; run it against a database already at version 1 holding a row → steps 2 and 3 applied, step 1 not re-run (it would fail if it were), the row intact, `user_version` 3.
- `open_rejects_newer_schema` (E-004.4): set `user_version` to current + 1, open → error naming both versions; file bytes unchanged.
- `migrate_stops_at_failed_step` (E-004.5): a three-step list whose step 2 creates a table and then fails → error naming step 2; `user_version` 1; step 2's table absent; step 3 not applied.
- Stubs: none; real SQLite on temp files.

### T-005 unit tests
- **Intent:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)
- `record_same_ownership_twice_is_noop` (E-005.1): record store ownership `(epic, app1, "Hades")` twice → title count, ownership count, and row contents equal after each.
- `ownerships_with_equivalent_titles_share_one_title` (E-005.2): record `"Control™"` then `"control"` under different source identifiers → one title, two ownerships.
- `ownerships_with_different_titles_stay_separate` (E-005.3): record `"  Portal "` and `"Portal 2"` → two titles with display names `"Portal"` and `"Portal 2"`.
- `batch_with_rejected_item_records_nothing` (E-005.4): batch of three whose last item has a blank title → error; catalog rows equal those before the batch.
- `record_rejects_blank_fields` (E-005.5): blank title, blank source, blank `source_id` in turn → each returns an error naming that field; nothing stored.
- Stubs: none; in-memory SQLite, rows read back with direct queries (listing arrives in T-007).

### T-006 unit tests
- **Intent:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)
- `manual_entry_records_label_medium_and_origin` (E-006.1): add `"Chrono Trigger"`, label `shelf`, medium `rom` → one ownership with source `shelf`, medium `rom`, origin `manual`.
- `manual_entry_repeat_is_noop_and_new_medium_adds` (E-006.2): repeat that add with the title written `"chrono trigger"` → unchanged; add it again as `physical` → two ownerships under one title.
- `medium_parse_rejects_unknown_value` (E-006.3): parse `"cartridge"` → error text contains `physical`, `digital`, and `rom`.
- `manual_label_matching_store_name_stays_distinct` (E-006.4): record a store ownership of `"Hades"` from `epic`, then a manual `"Hades"` with label `epic`, medium `digital` → one title, two ownerships with origins `store` and `manual`.
- Stubs: none; in-memory SQLite.

### T-007 unit tests
- **Intent:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)
- `list_returns_each_title_once_in_name_order` (E-007.1): titles `"portal"`, `"Hades"`, `"Doom"` inserted in that order, one of them with two ownerships → three entries in the order Doom, Hades, portal.
- `list_orders_ownerships_within_title` (E-007.2): one title with ownerships inserted as (`shelf`, `rom`, manual), (`epic`, `digital`, store), (`shelf`, `physical`, manual) → returned as epic/digital/store, shelf/physical/manual, shelf/rom/manual, each carrying all three fields; serialising the listing yields the Listing JSON keys `title`, `ownerships`, `source`, `medium`, `origin`.
- Stubs: none; in-memory SQLite.

### T-008 unit tests
- **Intent:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)
- `parse_returns_one_game_per_base_entry` (E-008.1): the fixture's base games → same count, matching `app_name` and `app_title`.
- `parse_ignores_nested_dlcs` (E-008.2): a fixture game with two nested `dlcs` → one game returned, neither DLC name present.
- `parse_skips_top_level_dlc_entry` (E-008.3): a top-level element with `metadata.mainGameItem` → absent from the result.
- `parse_rejects_malformed_document` (E-008.4): invalid JSON; a JSON object instead of an array; an element without `app_title`; an element whose `app_title` is `"   "`; an element whose `app_name` is `""` → an error each time, no games returned.
- Stubs: fixture `crates/panaseeya-core/tests/fixtures/legendary-list.json`, hand-built from Legendary's source.

### T-009 unit tests
- **Intent:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)
- `runner_returns_stdout_on_success` (E-009.1): stand-in script that writes its arguments to a file and prints a document → runner returns that document; the recorded arguments are exactly `list --json`.
- `runner_reports_missing_tool_by_name` (E-009.2): a program name that does not exist → tool-not-found error containing that name.
- `runner_reports_nonzero_exit_with_stderr` (E-009.3): stand-in that writes `Login failed, cannot continue!` to standard error and exits 1 → error carries status 1 and that text.
- Stubs: stand-in script in a temp directory (`.cmd` on Windows, `sh` elsewhere).

### T-010 unit tests
- **Intent:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)
- `resolve_db_path_defaults_to_user_data_dir` (E-010.6): call the resolver with no `--db` and no data-directory override → the path's file name is `catalog.db` and its parent equals the data directory the `directories` crate reports for `Panaseeya`. Also asserts the precedence: a `--db` value wins over an override, and an override wins over the default. Creates no files.
- Stubs: none.

## Integration Tests

### Catalog core integration (written in T-008, `crates/panaseeya-core/tests/import.rs`)
- **Intents:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)
- `import_document_records_epic_digital_store_ownerships` (E-008.5; T-005 + T-007 + T-008 composed): call `import_legendary_document` with the fixture → the listing has exactly one ownership per base game, each `epic`/`digital`/`store`, and direct queries show each `source_id` equals the game's `app_name`.
- `import_fixture_twice_leaves_catalog_unchanged` (E-005.1, E-008.5; T-004 + T-005 + T-007 + T-008 composed): call `import_legendary_document` with the fixture on a temp-file store twice → the listing after the second pass equals the first.
- `epic_and_manual_copies_group_under_one_title` (E-005.2, E-006.1, E-007.2, E-008.5; T-003 + T-005 + T-006 + T-007 + T-008 composed): import the fixture, then manually add a physical copy of one fixture game written with different case and punctuation → that title lists the `epic` store ownership and the manual one together; title count unchanged.

## End-to-End Tests
- **Status:** possible
- "Possible" covers every test below except `manual_real_legendary_import`, which is listed separately with its unlock.
- All run the built `panaseeya` binary through `assert_cmd` with `--db` pointing at a temp file unless stated. "Leaves the database untouched" is checked twice in each such test: against a path with no file (the file must still not exist) and against an existing database (its bytes must be unchanged).
- `cli_add_then_list_shows_manual_entry` (E-010.1): `add "Chrono Trigger" --source shelf --medium rom` → exit 0; `list` prints `Chrono Trigger<TAB>shelf (rom, manual)`.
- `cli_add_with_invalid_medium_fails_and_changes_nothing` (E-010.2): `--medium cartridge` → non-zero exit; `list --json` unchanged.
- `cli_list_prints_one_line_per_title` (E-010.3): after `add "Hades" --source shelf --medium rom`, `add "hades" --source shelf --medium physical`, and `add "Doom" --source box --medium physical` → exactly two lines, `Doom<TAB>box (physical, manual)` then `Hades<TAB>shelf (physical, manual); shelf (rom, manual)`.
- `cli_creates_missing_parent_directories` (E-010.7): `--db <temp>/new/sub/catalog.db` where `new/sub` does not exist, run `add` → exit 0 and the file exists.
- `cli_list_json_matches_text_listing` (E-010.4): `list --json` parses as an array of `{title, ownerships:[{source, medium, origin}]}`; rendering it in the text form reproduces the `list` output exactly.
- `cli_data_dir_override_holds_catalog_db` (E-010.5): with `PANASEEYA_DATA_DIR` set to a temp directory that does not yet exist and no `--db`, run `add` → `catalog.db` exists inside it.
- `cli_import_file_then_list_shows_epic_titles` (E-011.1): import the fixture → exit 0; `list --json` has every base game with one `epic`/`digital`/`store` ownership; no DLC name appears.
- `cli_import_file_twice_is_idempotent` (E-011.2): `list --json` output after the second import is byte-identical to the first.
- `cli_list_after_import_shows_store_and_manual_on_one_line` (E-011.7): import the fixture, then `add` a physical copy of one fixture game with label `shelf` → `list` shows that title on exactly one line, `<title><TAB>epic (digital); shelf (physical, manual)`.
- `cli_live_import_matches_file_import` (E-011.3): `PANASEEYA_LEGENDARY_BIN` points at a stand-in that prints the fixture → `list --json` is byte-identical to that of a second database filled by importing the fixture with `--file`.
- `cli_live_import_without_legendary_fails_and_leaves_db_untouched` (E-011.4): `PANASEEYA_LEGENDARY_BIN` names a program that does not exist → non-zero exit; standard error contains that name; database untouched.
- `cli_live_import_failed_legendary_fails_and_leaves_db_untouched` (E-011.5): stand-in writes `Login failed, cannot continue!` to standard error and exits 1 → non-zero exit; standard error contains that text; database untouched.
- `cli_import_bad_file_fails_and_leaves_db_untouched` (E-011.6): `--file` naming a missing file, then a file holding `{}` → non-zero exit each time; standard error names the file; database untouched.
- `manual_real_legendary_import` (criteria 3 and 8, real tool): on a machine with Legendary installed and signed in, save `legendary list --json` to a file and run `panaseeya import legendary --file` on it, then run `panaseeya import legendary` live, and compare `list` with the Epic library. **Not executable in this sprint** — Legendary is not installed on the build machine and signing in needs the owner's Epic account. The test report must record it as not verified, never as passed. Unlocked by: the project owner running it after installing and signing in to Legendary; per INT-0001's observation rule the intent cannot become `realized` until that run is recorded. Where it lands: Sprint 0's completion evidence lists criteria 3 and 8 as "tested, not observed"; the Loop Phase carries the run forward as a backlog task against INT-0001; and its result is recorded as a dated entry in INT-0001's Transition history.
