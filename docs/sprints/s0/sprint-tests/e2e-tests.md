# Sprint 0 End-to-End Test Results

- **Tested head:** `50dc3511dfe497afe6994c64d69e8e201d2a0e60` (branch `dev`, not pushed)
- **Machine:** Windows 11, `rustc 1.99.1`
- **Runner:** `cargo test --workspace` (suite `crates/panaseeya-cli/tests/cli.rs`, which runs the built `panaseeya` binary through `assert_cmd` with `PANASEEYA_DATA_DIR` and `PANASEEYA_LEGENDARY_BIN` cleared): 14 passed, 0 failed
- **Status:** possible, with one test not executable this sprint (below)
- **Intent:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)

| Test | Clause | Result |
|------|--------|--------|
| `cli_add_then_list_shows_manual_entry` | E-010.1 | pass |
| `cli_add_with_invalid_medium_fails_and_changes_nothing` | E-010.2 | pass |
| `cli_list_prints_one_line_per_title` | E-010.3 | pass |
| `cli_list_json_matches_text_listing` | E-010.4 | pass |
| `cli_data_dir_override_holds_catalog_db` | E-010.5 | pass |
| `cli_creates_missing_parent_directories` | E-010.7 | pass |
| `cli_import_file_then_list_shows_epic_titles` | E-011.1 | pass |
| `cli_import_file_twice_is_idempotent` | E-011.2 | pass |
| `cli_live_import_matches_file_import` | E-011.3 | pass (Windows `.cmd` stand-in only) |
| `cli_live_import_without_legendary_fails_and_leaves_db_untouched` | E-011.4 | pass |
| `cli_live_import_with_default_program_absent_names_legendary` | E-011.4, criterion 8 | pass (added after the test critique: no override and an empty `PATH`, so the default `legendary` program is looked up and reported by name). The test skips itself, saying so on standard error, when `legendary` is on this process's `PATH`: on Windows the lookup can fall back to that `PATH`, and the test must never run the owner's real Legendary. Once Legendary is installed this test will skip on that machine; CI machines still run it. |
| `cli_live_import_failed_legendary_fails_and_leaves_db_untouched` | E-011.5 | pass (Windows `.cmd` stand-in only) |
| `cli_import_bad_file_fails_and_leaves_db_untouched` | E-011.6 | pass |
| `cli_list_after_import_shows_store_and_manual_on_one_line` | E-011.7 | pass |
| `manual_real_legendary_import` | criteria 3 and 8, real tool | **not verified** — not executable this sprint |

`cli_live_import_matches_file_import` also compares the stored ownership rows
(origin, source, `source_id`, medium, raw title) of both databases, read with
`rusqlite`, since `list --json` does not show identifiers (added after the
test critique).

## Not verified on Linux

The stand-in executable has a separate `sh` branch on non-Windows systems, and
it has never run: `dev` is not pushed, so CI has not run. Writing a script and
executing it at once can fail intermittently on Linux with "Text file busy"
(ETXTBSY) while other test threads spawn processes. The runner and
`PANASEEYA_LEGENDARY_BIN` take a single program and always append
`list --json`, so the stand-in cannot be launched as `sh <script>`. If the
first `ubuntu-latest` run flakes, retry the spawn on ETXTBSY in the test
helper, or run the stand-in tests one at a time. Unverified until that first
run.

## Not verified: `manual_real_legendary_import`

Legendary is not installed on the build machine, and signing in needs the
owner's Epic account. Criteria 3 and 8 are therefore **tested, not observed**:
they pass against the hand-built fixture and a stand-in executable only.

Unlocked by the project owner installing and signing in to Legendary, then:

1. `legendary list --json > epic.json`
2. `panaseeya --db <temp>/a.db import legendary --file epic.json`
3. `panaseeya --db <temp>/b.db import legendary`
4. Compare both `list` outputs with each other and with the Epic library.

Per INT-0001's observation rule the intent cannot become `realized` until that
run is recorded as a dated entry in its Transition history. The Loop phase
carries it forward as a backlog task.

## Manual run (supplementary, build phase)

The release binary was also run by hand: importing the fixture, adding a
physical copy of "The Witcher 3: Wild Hunt" grouped it with the Epic copy on
one line, and a live import with no Legendary on `PATH` exited 1 without
creating the database file. This is supporting evidence only; the automated
tests above are the record.
