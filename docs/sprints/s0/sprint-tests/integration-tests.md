# Sprint 0 Integration Test Results

- **Tested head:** `50dc3511dfe497afe6994c64d69e8e201d2a0e60` (branch `dev`, not pushed)
- **Machine:** Windows 11, `rustc 1.99.1`
- **Runner:** `cargo test --workspace` (suite `crates/panaseeya-core/tests/import.rs`): 3 passed, 0 failed
- **Intent:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)

| Test | Clauses | Components composed | Result |
|------|---------|---------------------|--------|
| `import_document_records_epic_digital_store_ownerships` | E-008.5 | T-004 store, T-005 record batch, T-007 list, T-008 parser and importer | pass |
| `import_fixture_twice_leaves_catalog_unchanged` | E-005.1, E-008.5 | T-004 temp-file store, T-005, T-007, T-008 | pass |
| `epic_and_manual_copies_group_under_one_title` | E-005.2, E-006.1, E-007.2, E-008.5 | T-003 normalisation, T-005, T-006 manual entry, T-007, T-008 | pass |

## Notes

- `import_document_records_epic_digital_store_ownerships` was tightened in the
  Test phase (commit `49af771`). In the build it inferred each stored
  `source_id` by re-recording games under their `app_name`; it now opens the
  temp-file database with `rusqlite` and asserts the stored
  (`source`, `source_id`, `raw_title`, `medium`, `origin`) rows equal
  (`epic`, `app_name`, `app_title`, `digital`, `store`) for every base game, as
  the locked test plan describes. After the test critique (commit `3fede0c`)
  the expected rows are written out by hand (Control, Hades, The Witcher 3)
  instead of coming from the parser, so a DLC that leaked through the parser
  would fail this test too.
- All three use the hand-built fixture; no real Legendary document was used.
