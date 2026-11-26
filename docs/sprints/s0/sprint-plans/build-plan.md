Finalized - DO NOT EDIT

# Sprint 0 Build Plan

## Intents
- [INT-0001](../../../intents/INT-0001-unified-catalog-core.md) — state: planned; acceptance criteria covered: 1, 2, 3, 4, 5, 6, 7, 8. Criteria 3 and 8 are verified here only against a hand-built Legendary document and a stand-in executable; neither is observed against a real Legendary install in this sprint, and the intent's observation rule keeps it from being realised until that happens.

## Schema Tree
- Sprint Goal: headless Rust catalog core with Epic import, manual entries, title grouping, and a CLI
  - Workspace and CI
    - T-001: Cargo workspace skeleton
    - T-002: CI workflow and Cargo dependency updates
  - Catalog core (`crates/panaseeya-core`)
    - T-003: Title normalisation
    - T-004: SQLite store and migration runner
    - T-005: Recording and grouping ownerships
    - T-006: Manual entries and medium parsing
    - T-007: Listing
  - Epic source (`crates/panaseeya-core/src/sources`)
    - T-008: Legendary document parser
    - T-009: Legendary runner
  - Command line (`crates/panaseeya-cli`)
    - T-010: `add`, `list`, and database location
    - T-011: `import legendary`

Clause identifiers (`E-NNN.k`) are referenced by the test plan. The tasks form
one chain (each depends on the one before it) because they share
`crates/panaseeya-core/src/lib.rs`, `error.rs`, the crate manifests, and
`Cargo.lock`; they are executed strictly in order, never in parallel.

Crate set: `rusqlite` (feature `bundled`), `serde` (feature `derive`),
`serde_json`, `thiserror`, `clap` (feature `derive`), `directories`; test-only
`tempfile`, `assert_cmd`, `predicates`. Versions are those `cargo add` resolves
when the task runs; the committed `Cargo.lock` is the pin. CI builds on the
`stable` toolchain.

Shared contracts used by several tasks:
- **Crate error type.** `crates/panaseeya-core/src/error.rs` is created by
  T-004 and holds the single core error enum. T-005, T-006, T-008, and T-009
  each add their variants to it.
- **Ownership identity.** An ownership is unique on (`origin`, `source`,
  `source_id`), where `origin` is `store` for imported records and `manual` for
  hand-entered ones.
- **Listing JSON.** An array of
  `{"title": <display name>, "ownerships": [{"source": …, "medium": …, "origin": "store" | "manual"}]}`.
- **Listing text.** One line per title: the display name, a tab, then the
  ownerships separated by `; `, each written `<source> (<medium>)` for a store
  record and `<source> (<medium>, manual)` for a manual one.

## Execution Sequence

### T-001: Create the Cargo workspace skeleton with a core library crate and a CLI binary crate
- **Intent:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)
- **Touches:** `Cargo.toml`, `Cargo.lock`, `crates/panaseeya-core/Cargo.toml`, `crates/panaseeya-core/src/lib.rs`, `crates/panaseeya-cli/Cargo.toml`, `crates/panaseeya-cli/src/main.rs`, `.gitignore`
- **Depends on:** (none)
- **Acceptance criterion:** 1 — the workspace builds on stable Rust and format, lint, and test checks pass.
- **Success criterion (EARS):**
  - **E-001.1** **WHEN** `cargo build --workspace` is run on stable Rust, **THEN** the workspace **SHALL** compile and exit 0.
  - **E-001.2** **WHEN** `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace` are run, **THEN** each command **SHALL** exit 0.
  - **E-001.3** **WHEN** the workspace has been built, **THEN** `git status` **SHALL** show no file under `target/` as untracked.
- **Notes:** binary name `panaseeya`. Add `target/` to `.gitignore` outside the sprint-loops block.

### T-002: Add a CI workflow that runs the project checks and a Cargo entry for dependency updates
- **Intent:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)
- **Touches:** `.github/workflows/sprint-loops-ci.yml`, `.github/dependabot.yml`
- **Depends on:** T-001
- **Acceptance criterion:** 1 — format, lint, and test checks pass, and the host runs them.
- **Success criterion (EARS):**
  - **E-002.1** **WHEN** `.github/workflows/sprint-loops-ci.yml` is read, **THEN** it **SHALL** declare push and pull-request triggers for `dev` and `main`, a job matrix of `windows-latest` and `ubuntu-latest`, and steps that run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace` on the `stable` toolchain.
  - **E-002.2** **WHEN** `.github/dependabot.yml` is read, **THEN** it **SHALL** contain a `cargo` ecosystem entry targeting `dev` alongside the existing `github-actions` entry.
- **Notes:** generate with the bundle's `scripts/scaffold-ci.sh --provider github --base main --work dev`; if it writes nothing or omits any of the three commands, write the workflow by hand. Both files are host configuration serving the same criterion, so they stay one task. The workflow is not observed running until the branch is pushed, which needs an explicit request.

### T-003: Implement title normalisation that decides when two titles are the same game
- **Intent:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)
- **Touches:** `crates/panaseeya-core/src/title.rs`, `crates/panaseeya-core/src/lib.rs`
- **Depends on:** T-002
- **Acceptance criterion:** 6 — titles differing only by case, punctuation, trademark symbols, or surrounding whitespace group together; titles differing in words do not.
- **Success criterion (EARS):**
  - **E-003.1** **WHEN** `normalize_title` is given two titles that contain the same sequence of words — a word being a maximal run of letters and digits, compared without regard to case, with every other character (punctuation, the symbols ™ ® ©, whitespace) acting only as a separator — **THEN** it **SHALL** return the same key for both.
  - **E-003.2** **WHEN** `normalize_title` is given two titles whose word sequences differ in at least one word, including when punctuation inside a word splits it in one title and not the other ("Meier's" against "Meiers"), **THEN** it **SHALL** return different keys.
  - **E-003.3** **WHEN** `normalize_title` is given a title that has at least one non-whitespace character but no letters or digits, **THEN** it **SHALL** return the trimmed, lower-cased original rather than an empty key.
- **Notes:** rule: lower-case; replace every character that is not a letter or digit with a space; collapse runs of whitespace; trim. Pure function, no I/O. A blank title yields an empty key here and is rejected by the catalog (E-005.5).

### T-004: Implement the SQLite store and its ordered migration runner
- **Intent:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)
- **Touches:** `crates/panaseeya-core/src/store.rs`, `crates/panaseeya-core/src/migrations/0001_initial.sql`, `crates/panaseeya-core/src/error.rs`, `crates/panaseeya-core/src/lib.rs`, `crates/panaseeya-core/Cargo.toml`, `Cargo.lock`
- **Depends on:** T-003
- **Acceptance criterion:** 2 — a single local SQLite file whose schema the program creates, versions, and migrates.
- **Success criterion (EARS):**
  - **E-004.1** **WHEN** `Store::open` is given a path where no file exists, **THEN** it **SHALL** create one SQLite file containing the `titles` and `ownerships` tables and set `PRAGMA user_version` to the current schema version.
  - **E-004.2** **WHEN** `Store::open` is given a database already at the current schema version, **THEN** it **SHALL** leave the schema and every stored row unchanged.
  - **E-004.3** **WHEN** the migration runner is given a database at version *v* and a list of *n* > *v* steps, **THEN** it **SHALL** apply steps *v*+1 through *n* in ascending order, skip steps 1 through *v*, preserve existing rows, and set `user_version` to *n*.
  - **E-004.4** **WHEN** `Store::open` is given a database whose `user_version` is higher than the program's current version, **THEN** it **SHALL** return an error naming both versions and **SHALL NOT** modify the file.
  - **E-004.5** **WHEN** a migration step fails, **THEN** the runner **SHALL** return an error naming that step's number, leave `user_version` at the last step that succeeded, and leave none of the failed step's changes in the database.
- **Notes:** schema v1 — `titles(id, display_name, normalized_key UNIQUE)`; `ownerships(id, title_id → titles, origin CHECK IN ('store','manual'), source, source_id, medium CHECK IN ('digital','physical','rom'), raw_title, UNIQUE(origin, source, source_id))`. The runner is a function taking a connection and an ordered slice of SQL steps, each applied in its own transaction together with its `user_version` bump; `Store::open` calls it with the embedded (`include_str!`) list. Tests pass their own multi-step lists so ordering and mid-way failure are observed even though the product has one migration.

### T-005: Implement recording ownerships with title grouping, validation, and an atomic batch
- **Intent:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)
- **Touches:** `crates/panaseeya-core/src/catalog.rs`, `crates/panaseeya-core/src/model.rs`, `crates/panaseeya-core/src/error.rs`, `crates/panaseeya-core/src/lib.rs`
- **Depends on:** T-004
- **Acceptance criterion:** 4, 6 — repeat imports change nothing; equivalent titles group and different titles do not.
- **Success criterion (EARS):**
  - **E-005.1** **WHEN** an ownership whose `origin`, `source`, and `source_id` are already in the catalog is recorded again, **THEN** the catalog **SHALL** contain the same titles and ownership records as before, with no additions.
  - **E-005.2** **WHEN** an ownership is recorded whose title normalises to the key of an existing title, **THEN** the catalog **SHALL** attach it to that title and **SHALL NOT** create a new title.
  - **E-005.3** **WHEN** an ownership is recorded whose title normalises to a key no existing title has, **THEN** the catalog **SHALL** create a new title whose display name is that ownership's title, trimmed.
  - **E-005.4** **WHEN** a batch of ownerships is recorded and any one of them is rejected, **THEN** the catalog **SHALL** contain none of that batch.
  - **E-005.5** **WHEN** an ownership is recorded whose title, `source`, or `source_id` is empty or only whitespace, **THEN** the catalog **SHALL** reject it with an error naming the blank field and **SHALL** store nothing for it.
- **Notes:** uses `normalize_title` (T-003) and `Store` (T-004). This task defines the `Medium` and `Origin` enums in `model.rs` (with their database text forms), because every recorded ownership carries both. The batch runs in one transaction. A title keeps the display name of the record that created it.

### T-006: Implement manual entries and parsing a medium from text
- **Intent:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)
- **Touches:** `crates/panaseeya-core/src/catalog.rs`, `crates/panaseeya-core/src/model.rs`, `crates/panaseeya-core/src/error.rs`
- **Depends on:** T-005
- **Acceptance criterion:** 5 — a manual game is recorded with a title, a person-chosen source label, and a medium.
- **Success criterion (EARS):**
  - **E-006.1** **WHEN** a manual entry is added with a title, a source label, and a medium of `physical`, `digital`, or `rom`, **THEN** the catalog **SHALL** store one ownership with that label, that medium, and origin `manual`.
  - **E-006.2** **WHEN** the same manual entry (same label, same medium, a title with the same normalised key) is added again, **THEN** the catalog **SHALL** be unchanged; **WHEN** it is added with a different medium, **THEN** the catalog **SHALL** hold a second ownership under the same title.
  - **E-006.3** **WHEN** a medium string other than `physical`, `digital`, or `rom` is parsed, **THEN** parsing **SHALL** return an error naming the three accepted values.
  - **E-006.4** **WHEN** a manual entry uses a label equal to a store source name such as `epic` for a game that also has a store record from that source, **THEN** the catalog **SHALL** keep both ownerships, distinguished by origin.
- **Notes:** builds on the `Medium` and `Origin` types from T-005; adds only text parsing for `Medium`. A manual entry's `source_id` is `<normalised key>:<medium>`. The resulting limits are recorded in INT-0001 Consequences.

### T-007: Implement listing titles with their ownerships in a defined order
- **Intent:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)
- **Touches:** `crates/panaseeya-core/src/catalog.rs`, `crates/panaseeya-core/src/model.rs`, `crates/panaseeya-core/Cargo.toml`, `Cargo.lock`
- **Depends on:** T-006
- **Acceptance criterion:** 7 — each title is shown once with all of its sources.
- **Success criterion (EARS):**
  - **E-007.1** **WHEN** titles are listed, **THEN** the catalog **SHALL** return each title exactly once, ordered by display name without regard to case and then by display name exactly.
  - **E-007.2** **WHEN** a title has several ownerships, **THEN** the listing **SHALL** return all of them under that title, each with its source, medium, and origin, ordered by source, then medium, then origin.
- **Notes:** the listing types derive `serde::Serialize` in the Listing JSON shape above, so the CLI and any later interface share one contract.

### T-008: Implement the parser for a `legendary list --json` document
- **Intent:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)
- **Touches:** `crates/panaseeya-core/src/sources/mod.rs`, `crates/panaseeya-core/src/sources/legendary.rs`, `crates/panaseeya-core/src/error.rs`, `crates/panaseeya-core/src/lib.rs`, `crates/panaseeya-core/tests/fixtures/legendary-list.json`, `crates/panaseeya-core/tests/fixtures/README.md`, `crates/panaseeya-core/tests/import.rs`, `crates/panaseeya-core/Cargo.toml`, `Cargo.lock`
- **Depends on:** T-007
- **Acceptance criterion:** 3 — one `epic` ownership per base game; no title for DLC.
- **Success criterion (EARS):**
  - **E-008.1** **WHEN** the parser is given a JSON array of game objects, **THEN** it **SHALL** return one game per top-level element that is not DLC, carrying that element's `app_name` as the source identifier and `app_title` as the title.
  - **E-008.2** **WHEN** a top-level element has a non-empty `dlcs` array, **THEN** the parser **SHALL** return no game for any element of `dlcs`.
  - **E-008.3** **WHEN** a top-level element's `metadata` object contains the key `mainGameItem`, **THEN** the parser **SHALL** return no game for that element.
  - **E-008.4** **WHEN** the input is not valid JSON, is not an array, or has a non-DLC element whose `app_name` or `app_title` is missing, not a string, or blank, **THEN** the parser **SHALL** return an error that says what was wrong and **SHALL NOT** return a partial result.
  - **E-008.5** **WHEN** `import_legendary_document` is given a catalog and a document, **THEN** it **SHALL** parse the document and record, through the T-005 batch, exactly one ownership per returned game with source `epic`, medium `digital`, origin `store`, the game's `app_name` as `source_id`, and its `app_title` as the title.
- **Notes:** shape taken from the `list_games` handler in Legendary's [`cli.py`](https://raw.githubusercontent.com/legendary-gl/legendary/master/legendary/cli.py), read during planning: a top-level list of game objects with a `dlcs` list added to each. The fixture is hand-built from that source, not captured from a real account; `tests/fixtures/README.md` says so. Unknown fields are ignored so real output with more keys still parses. `import_legendary_document` is the one place the Epic mapping lives; the CLI calls it. This task also writes the core integration tests in `tests/import.rs`, which compose the parser with T-005 to T-007.

### T-009: Implement the runner that executes Legendary and returns its output
- **Intent:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)
- **Touches:** `crates/panaseeya-core/src/sources/legendary_runner.rs`, `crates/panaseeya-core/src/sources/mod.rs`, `crates/panaseeya-core/src/error.rs`, `crates/panaseeya-core/Cargo.toml`, `Cargo.lock`
- **Depends on:** T-008
- **Acceptance criterion:** 8 — a live import runs Legendary itself; an absent tool produces a message naming it.
- **Success criterion (EARS):**
  - **E-009.1** **WHEN** the runner is invoked and the given program exits 0, **THEN** it **SHALL** have run that program with exactly the arguments `list --json` and **SHALL** return the program's standard output.
  - **E-009.2** **WHEN** the given program cannot be found, **THEN** the runner **SHALL** return a tool-not-found error whose message contains the program name.
  - **E-009.3** **WHEN** the given program exits non-zero, **THEN** the runner **SHALL** return an error carrying the exit status and the program's standard error text.
- **Notes:** the runner takes the program as a parameter. Tests use a stand-in script written to a temporary directory (`.cmd` on Windows, `sh` elsewhere).

### T-010: Implement the `panaseeya` CLI with `add`, `list`, and database location
- **Intent:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)
- **Touches:** `crates/panaseeya-cli/src/main.rs`, `crates/panaseeya-cli/src/db_path.rs`, `crates/panaseeya-cli/Cargo.toml`, `crates/panaseeya-cli/tests/cli.rs`, `Cargo.lock`
- **Depends on:** T-009
- **Acceptance criterion:** 2, 5, 7 — a single local file in a known place; manual entry; text and JSON listing.
- **Success criterion (EARS):**
  - **E-010.1** **WHEN** `panaseeya --db <path> add <title> --source <label> --medium <medium>` is run with a valid medium, **THEN** the program **SHALL** record that manual entry and exit 0.
  - **E-010.2** **WHEN** `add` is run with a medium that is not `physical`, `digital`, or `rom`, **THEN** the program **SHALL** exit non-zero and leave the catalog unchanged.
  - **E-010.3** **WHEN** `panaseeya --db <path> list` is run, **THEN** the program **SHALL** print one line per title in the Listing text form, in the order T-007 defines.
  - **E-010.4** **WHEN** `panaseeya --db <path> list --json` is run, **THEN** the program **SHALL** print the Listing JSON array holding the same titles and ownerships, in the same order, as the text listing.
  - **E-010.5** **WHEN** `--db` is omitted and the `PANASEEYA_DATA_DIR` environment variable is set, **THEN** the program **SHALL** use `catalog.db` inside that directory.
  - **E-010.6** **WHEN** neither `--db` nor `PANASEEYA_DATA_DIR` is given, **THEN** the path resolver **SHALL** return `catalog.db` inside the per-user data directory that the `directories` crate reports for the application name `Panaseeya`.
  - **E-010.7** **WHEN** the directory that should hold the resolved database path does not exist, **THEN** the program **SHALL** create it, including missing parents, before opening the database.
- **Notes:** `clap` derive. Directory creation is one code path for all three ways of choosing the path, so it is tested through `--db` and `PANASEEYA_DATA_DIR` only. The resolver is a pure function of (`--db` value, `PANASEEYA_DATA_DIR` value) so E-010.6 is tested without creating anything in the real user profile; no automated test runs the binary without `--db` or `PANASEEYA_DATA_DIR`.

### T-011: Implement `panaseeya import legendary` for a saved document and for a live run
- **Intent:** [INT-0001](../../../intents/INT-0001-unified-catalog-core.md)
- **Touches:** `crates/panaseeya-cli/src/main.rs`, `crates/panaseeya-cli/tests/cli.rs`, `README.md`
- **Depends on:** T-010
- **Acceptance criterion:** 3, 4, 8 — file and live import, unchanged on re-import, clean failure when Legendary is absent or fails.
- **Success criterion (EARS):**
  - **E-011.1** **WHEN** `panaseeya --db <path> import legendary --file <document>` is run, **THEN** the program **SHALL** call `import_legendary_document` (E-008.5) on that file's content, so that one `epic`/`digital`/`store` ownership per base game is recorded, and exit 0.
  - **E-011.2** **WHEN** the same `import legendary --file` command is run a second time, **THEN** the output of `list --json` **SHALL** be byte-identical to its output after the first run.
  - **E-011.3** **WHEN** `import legendary` is run without `--file` and the Legendary program exits 0, **THEN** the program **SHALL** leave the catalog in the same state as importing that program's output from a file.
  - **E-011.4** **WHEN** `import legendary` is run without `--file` and the Legendary program cannot be found, **THEN** the program **SHALL** print a message naming the missing tool to standard error, exit non-zero, and neither create nor modify the database file.
  - **E-011.5** **WHEN** `import legendary` is run without `--file` and the Legendary program exits non-zero, **THEN** the program **SHALL** print that program's standard error text to standard error, exit non-zero, and neither create nor modify the database file.
  - **E-011.6** **WHEN** `--file` names a file that does not exist or whose content the parser rejects, **THEN** the program **SHALL** exit non-zero with a message naming the file and neither create nor modify the database file.
  - **E-011.7** **WHEN** a title has a store ownership from an import and a manual ownership, **THEN** `list` **SHALL** print them on one line as `<title><TAB>epic (digital); <label> (<medium>, manual)`.
- **Notes:** the Legendary program defaults to `legendary` and is overridden by the `PANASEEYA_LEGENDARY_BIN` environment variable, read here and passed to the T-009 runner. Import obtains and parses the document before opening the database, then writes through `import_legendary_document`; because the parser rejects blank identifiers and titles (E-008.4), a document that parses cannot be rejected by the catalog's blank-field rule. Add a short usage section to `README.md`.
