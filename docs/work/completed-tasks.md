# Completed Tasks Log (Append-Only)

## T-001 (sprint 0)
- **Description:** Create the Cargo workspace skeleton with a core library crate and a CLI binary crate
- **Intent:** [INT-0001](../intents/INT-0001-unified-catalog-core.md)
- **Completed:** 2026-11-22T20:53:29Z
- **Files modified:** Cargo.toml, Cargo.lock, crates/panaseeya-core/Cargo.toml, crates/panaseeya-core/src/lib.rs, crates/panaseeya-cli/Cargo.toml, crates/panaseeya-cli/src/main.rs
- **Commit:** `5fa9525fffa4ac02b4e79d83a31887d0e388404e`

## T-002 (sprint 0)
- **Description:** Add a CI workflow that runs the project checks and a Cargo entry for dependency updates
- **Intent:** [INT-0001](../intents/INT-0001-unified-catalog-core.md)
- **Completed:** 2026-11-22T20:53:42Z
- **Files modified:** .github/workflows/sprint-loops-ci.yml, .github/dependabot.yml
- **Commit:** `99581c14fb42b72766a4a6f2d2964ce98da198d1`

## T-003 (sprint 0)
- **Description:** Implement title normalisation that decides when two titles are the same game
- **Intent:** [INT-0001](../intents/INT-0001-unified-catalog-core.md)
- **Completed:** 2026-11-22T20:54:07Z
- **Files modified:** crates/panaseeya-core/src/title.rs, crates/panaseeya-core/src/lib.rs
- **Commit:** `455d93d7685b574892da7b331035c80c943db1be`

## T-004 (sprint 0)
- **Description:** Implement the SQLite store and its ordered migration runner
- **Intent:** [INT-0001](../intents/INT-0001-unified-catalog-core.md)
- **Completed:** 2026-11-22T20:55:51Z
- **Files modified:** crates/panaseeya-core/src/store.rs, crates/panaseeya-core/src/migrations/0001_initial.sql, crates/panaseeya-core/src/error.rs, crates/panaseeya-core/src/lib.rs, crates/panaseeya-core/Cargo.toml, Cargo.lock
- **Commit:** `8e515b2d25f31714145963e69ad0f628a2a126c0`

## T-005 (sprint 0)
- **Description:** Implement recording ownerships with title grouping, validation, and an atomic batch
- **Intent:** [INT-0001](../intents/INT-0001-unified-catalog-core.md)
- **Completed:** 2026-11-22T20:56:47Z
- **Files modified:** crates/panaseeya-core/src/catalog.rs, crates/panaseeya-core/src/model.rs, crates/panaseeya-core/src/error.rs, crates/panaseeya-core/src/lib.rs, crates/panaseeya-core/src/store.rs
- **Commit:** `b54005dc6c721a45c47b818af57ff64988d3520d`

## T-006 (sprint 0)
- **Description:** Implement manual entries and parsing a medium from text
- **Intent:** [INT-0001](../intents/INT-0001-unified-catalog-core.md)
- **Completed:** 2026-11-22T20:57:20Z
- **Files modified:** crates/panaseeya-core/src/catalog.rs, crates/panaseeya-core/src/model.rs, crates/panaseeya-core/src/error.rs
- **Commit:** `ae45fe4a06e042a44a1ae8049c8aef4aed21fafb`

## T-007 (sprint 0)
- **Description:** Implement listing titles with their ownerships in a defined order
- **Intent:** [INT-0001](../intents/INT-0001-unified-catalog-core.md)
- **Completed:** 2026-11-22T20:58:05Z
- **Files modified:** crates/panaseeya-core/src/catalog.rs, crates/panaseeya-core/src/model.rs, crates/panaseeya-core/src/error.rs, crates/panaseeya-core/src/store.rs, crates/panaseeya-core/src/lib.rs, crates/panaseeya-core/Cargo.toml, Cargo.lock
- **Commit:** `c03c0137acf3c8571c9b6e0dd1bd8c1ed553341f`

## T-008 (sprint 0)
- **Description:** Implement the parser for a legendary list --json document
- **Intent:** [INT-0001](../intents/INT-0001-unified-catalog-core.md)
- **Completed:** 2026-11-22T20:59:42Z
- **Files modified:** crates/panaseeya-core/src/sources/mod.rs, crates/panaseeya-core/src/sources/legendary.rs, crates/panaseeya-core/src/error.rs, crates/panaseeya-core/src/lib.rs, crates/panaseeya-core/tests/fixtures/legendary-list.json, crates/panaseeya-core/tests/fixtures/README.md, crates/panaseeya-core/tests/import.rs
- **Commit:** `70c5b313304f5ddca7d9c592ea195d65c2bfafb6`
