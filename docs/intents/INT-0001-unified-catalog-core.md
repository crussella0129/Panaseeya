# INT-0001 — Unified catalog core

<!-- sprint-loop-intent-v2 -->
- **Intent ID:** INT-0001
- **State:** proposed
- **Work evidence:** none
- **Completion evidence:** none
- **Code evidence:** none
- **Test evidence:** none
- **Documentation evidence:** none

## Intent
Panaseeya keeps one local catalog of every game a person owns, regardless of
where it came from. A game the person owns in several places appears once, as
a single title with one ownership record per source. The catalog is filled by
importing the machine-readable output of store command-line tools and by
manual entry for games that no store tool knows about (physical copies, ROMs,
package-manager or direct-download games).

This intent covers the headless core only: the data model, the local database,
the import path, the grouping of identical titles, and a command-line surface
that proves them. Its first store source is Epic, through Legendary.

Non-goals of this intent: any graphical interface; cover art download and
display; Steam, GOG, and Amazon sources; DLC and mod management; playtime
aggregation; installing or launching games; signing in to a store from inside
Panaseeya. Each of these is expected to become its own intent.

## Acceptance criteria
1. A Rust workspace builds on stable Rust with `cargo build`, and
   `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test` pass.
2. The catalog is stored in a single local SQLite file whose schema is created
   and versioned by the program itself; opening an older schema version
   migrates it, and re-opening a current one changes nothing.
3. Importing a saved `legendary list --json` document creates one ownership
   record per base game with source `epic`, and does not create a title for an
   entry Legendary marks as DLC.
4. Importing the same document twice leaves the catalog unchanged the second
   time (same titles, same ownership records, no duplicates).
5. A manually added game can be recorded with a title, a source label chosen by
   the person, and a medium of physical, digital, or ROM.
6. Two ownership records whose titles differ only by case, punctuation,
   trademark symbols, or surrounding whitespace are grouped under one title;
   records whose titles differ in words are not.
7. A command-line `list` shows each title once with all of its sources, and can
   emit the same data as JSON.
8. When Legendary is installed and signed in, a live import runs Legendary
   itself and imports its output; when Legendary is absent, the command fails
   with a message that names the missing tool and exits non-zero, without
   touching the catalog.

## Rationale
Every later feature in the README — the cover-art library, cross-store
playtime, DLC and mods in one place — reads from this catalog, so its shape is
the decision that is most expensive to change later. Building it headless
first makes it testable from saved tool output without accounts, network, or a
user interface.

Rust is the project default language. Epic through Legendary is the first
source because Legendary is the only one of the four store tools whose
documentation states a machine-readable owned-library listing
(`legendary list --json`).

## Alternatives
- **Start with the Electron interface.** Rejected for the first sprint: with
  no catalog there is nothing real to show, and the interface would be built
  against invented data.
- **Start with Steam.** Deferred: the Steamworks documentation for
  `GetOwnedGames` requires a Web API key and a visible profile and does not
  document the response fields, and no documented SteamCMD command for listing
  an owned library was confirmed during research.
- **Start with GOG or Amazon.** Deferred: the gogdl and Nile READMEs document
  no library-listing command; the listing path has to be established from
  their source or from the store APIs first.
- **TypeScript core inside Electron, as Heroic does.** Rejected: the project
  default is Rust, and a separate core keeps the later Tauri port cheap.
- **Fuzzy title matching.** Rejected for now: false merges corrupt the catalog
  silently. Normalised exact matching is predictable; a person-controlled
  merge and split is a later intent.

## Consequences
- The interface arrives in a later sprint and will talk to the core across a
  process or binding boundary that this intent does not yet define.
- Title grouping will miss matches that need judgement (subtitle differences,
  "Game of the Year" editions). Those stay separate until manual merge exists.
- The store tools are GPL-3.0 and Panaseeya is Apache-2.0. The tools are run
  as separate programs and their output is read; none of their code is copied
  or linked. Bundling them with a release is a later decision.
- People must install and sign in to Legendary themselves until an in-app
  sign-in intent exists.

## Transition history
- 2026-11-22: created as `proposed` during Sprint 0 research.
