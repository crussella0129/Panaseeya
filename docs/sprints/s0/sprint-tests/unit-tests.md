# Sprint 0 Unit Test Results

- **Tested head:** `50dc3511dfe497afe6994c64d69e8e201d2a0e60` (branch `dev`, not pushed)
- **Machine:** Windows 11, `rustc 1.99.1`, `cargo 1.99.1`
- **Runner:** `cargo test --workspace` (the project's canonical suite runner; the same command CI runs)
- **Intent:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)

## Project checks (T-001, T-002)

These are command and file checks, not Rust test functions.

| Check | Clause | Command or file | Result |
|-------|--------|-----------------|--------|
| `check_workspace_builds` | E-001.1 | `cargo build --workspace` | pass, exit 0 |
| `check_fmt_clippy_test_pass` | E-001.2 | `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace` | pass, each exit 0 |
| `check_target_dir_ignored` | E-001.3 | `git status --porcelain` after the build | pass, 0 lines under `target/` |
| `check_ci_workflow_triggers_and_commands` | E-002.1 | `.github/workflows/sprint-loops-ci.yml` | pass on file content: push and pull_request on `main` and `dev`; matrix `windows-latest`, `ubuntu-latest`; `dtolnay/rust-toolchain@stable`; the three commands verbatim. **Not observed running**: `dev` has not been pushed. |
| `check_dependabot_has_cargo_entry` | E-002.2 | `.github/dependabot.yml` | pass: `cargo` entry with `target-branch: "dev"`; `github-actions` entry kept |

## Rust unit tests

`panaseeya-core` (`src/lib.rs` unit tests): 26 passed, 0 failed.
`panaseeya-cli` (`src/main.rs` unit tests): 1 passed, 0 failed.

| Task | Test | Clause | Result |
|------|------|--------|--------|
| T-003 | `title::tests::normalize_merges_case_punctuation_trademark_whitespace` | E-003.1 | pass |
| T-003 | `title::tests::normalize_keeps_different_words_apart` | E-003.2 | pass |
| T-003 | `title::tests::normalize_falls_back_when_only_punctuation` | E-003.3 | pass |
| T-004 | `store::tests::open_creates_schema_at_current_version` | E-004.1 | pass |
| T-004 | `store::tests::reopen_current_schema_changes_nothing` | E-004.2 | pass |
| T-004 | `store::tests::migrate_applies_pending_steps_in_order` | E-004.3 | pass |
| T-004 | `store::tests::open_rejects_newer_schema` | E-004.4 | pass |
| T-004 | `store::tests::migrate_stops_at_failed_step` | E-004.5 | pass |
| T-005 | `catalog::tests::record_same_ownership_twice_is_noop` | E-005.1 | pass |
| T-005 | `catalog::tests::ownerships_with_equivalent_titles_share_one_title` | E-005.2 | pass |
| T-005 | `catalog::tests::ownerships_with_different_titles_stay_separate` | E-005.3 | pass |
| T-005 | `catalog::tests::batch_with_rejected_item_records_nothing` | E-005.4 | pass |
| T-005 | `catalog::tests::record_rejects_blank_fields` | E-005.5 | pass |
| T-006 | `catalog::tests::manual_entry_records_label_medium_and_origin` | E-006.1 | pass |
| T-006 | `catalog::tests::manual_entry_repeat_is_noop_and_new_medium_adds` | E-006.2 | pass |
| T-006 | `model::tests::medium_parse_rejects_unknown_value` | E-006.3 | pass |
| T-006 | `catalog::tests::manual_label_matching_store_name_stays_distinct` | E-006.4 | pass |
| T-007 | `catalog::tests::list_returns_each_title_once_in_name_order` | E-007.1 | pass |
| T-007 | `catalog::tests::list_orders_ownerships_within_title` | E-007.2 | pass |
| T-008 | `sources::legendary::tests::parse_returns_one_game_per_base_entry` | E-008.1 | pass |
| T-008 | `sources::legendary::tests::parse_ignores_nested_dlcs` | E-008.2 | pass |
| T-008 | `sources::legendary::tests::parse_skips_top_level_dlc_entry` | E-008.3 | pass |
| T-008 | `sources::legendary::tests::parse_rejects_malformed_document` | E-008.4 | pass |
| T-009 | `sources::legendary_runner::tests::runner_returns_stdout_on_success` | E-009.1 | pass (Windows `.cmd` stand-in only) |
| T-009 | `sources::legendary_runner::tests::runner_reports_missing_tool_by_name` | E-009.2 | pass |
| T-009 | `sources::legendary_runner::tests::runner_reports_nonzero_exit_with_stderr` | E-009.3 | pass (Windows `.cmd` stand-in only) |
| T-010 | `db_path::tests::resolve_db_path_defaults_to_user_data_dir` | E-010.6 | pass |

## Tightened after the test critique (commit `3fede0c`)

- `list_returns_each_title_once_in_name_order` also records `"Beta"` then
  `"alpha"` and expects `alpha, Beta, Doom, Hades, portal`. The locked
  scenario's titles (`portal`, `Hades`, `Doom`) sort the same way by bytes, so
  on their own they could not show the "without regard to case" half of E-007.1.
- `manual_label_matching_store_name_stays_distinct` now also checks `list()`:
  the two `epic`/`digital` copies come out manual then store, although the
  store copy was inserted first. This exercises E-007.2's origin tie-break.
- `open_rejects_newer_schema` checks the rendered phrases
  `schema version 2` and `up to version 1`, not bare digits (E-004.4).
- `resolve_db_path_defaults_to_user_data_dir` also checks that the default
  path has a `Panaseeya` component. Its main oracle remains what `directories`
  reports, as the locked plan defines (E-010.6).

## Stubs

- T-008 uses the hand-built fixture `crates/panaseeya-core/tests/fixtures/legendary-list.json`, shaped from Legendary's `cli.py`, not captured from a real run.
- T-009 uses a stand-in script written to a temp directory. Only the Windows `.cmd` branch has executed; the `sh` branch runs first in CI on `ubuntu-latest`.
