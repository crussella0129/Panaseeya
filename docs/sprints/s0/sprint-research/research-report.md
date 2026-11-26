# Sprint 0 Research Report

## Intents Reviewed
- [INT-0001](../../../intents/INT-0001-unified-catalog-core.md) — created; relevance: defines the headless catalog that this sprint builds and that every README feature depends on; current state: `proposed`.

## 1. Sprint Goal
Stand up the first working slice of Panaseeya as a headless Rust core: a
versioned local SQLite catalog, an importer for saved and live
`legendary list --json` output (Epic), manual entries for physical, digital,
and ROM copies, grouping of identical titles across sources, and a small
command-line program that imports and lists. The repository contains only a
README today, so the sprint also establishes the workspace, formatting,
linting, tests, and CI. No graphical interface, cover art, or other stores are
in this sprint.

## 2. Existing Code Survey
| File | Relevance | Notes |
|------|-----------|-------|
| README.md | high | The only statement of product intent: Heroic-like catalog over Steam, GOG, Epic, Amazon plus private collection; grouping of identical titles; cover art; DLC and mods; cross-source playtime |
| LICENSE | medium | Apache-2.0; the store tools are GPL-3.0, so they are run as separate programs, not linked |
| .gitignore | low | Python-template ignore file; has no Rust `target/` entry yet |
| docs/work/remote-profile.md | medium | GitHub provider, `dev` → `main`, human-approve merge |
| .github/dependabot.yml | low | Only `github-actions` ecosystem; `cargo` should be added once a manifest exists |

No source code exists. No CI workflow was generated at convergence because the
repository contained no detectable language; one is needed once the Rust
workspace exists.

Local toolchain observed on this machine: cargo 1.99.1, rustc 1.99.1,
node 24.12.0, pnpm 11.9.0, python 3.13.11, gh 2.97.0, SteamCMD at
`C:\steamcmd`. Legendary, Nile, and gogdl are not installed.

## 3. External Sources
- [Legendary README](https://github.com/derrod/legendary) — states `legendary list` and `list-installed` accept `--json`, `--csv`, `--tsv`; `legendary auth` flags; GPL-3.0. Does not show the JSON field layout.
- [Legendary `models/game.py`](https://raw.githubusercontent.com/legendary-gl/legendary/master/legendary/models/game.py) — `Game` has `app_name`, `app_title`, `metadata`, `asset_infos`, `base_urls`; `is_dlc` is true when `metadata` contains `mainGameItem`. No image key appears in this file.
- [Nile README](https://github.com/imLinguin/nile) — GPL-3.0, Python; documents no library sync or list command and no output format.
- [heroic-gogdl README](https://github.com/Heroic-Games-Launcher/heroic-gogdl) — GPL-3.0, Python with a C module; described as a download, cloud-save, and launch helper for other applications; no library-listing command documented; manages its own auth token file.
- [Steamworks `IPlayerService`](https://partner.steamgames.com/doc/webapi/IPlayerService) — `GetOwnedGames` requires `key` and `steamid`, offers `include_appinfo` and `include_played_free_games`, and returns games only when the profile's game details are visible. Response fields are not documented on the page.

The Valve Developer Community wiki pages for the Steam Web API and SteamCMD
refused automated fetches (HTTP 403) and were not read.

## 4. Risks, Unknowns, Dependencies
- **Unknown:** the exact JSON document `legendary list --json` prints. The README confirms the flag and the model file confirms the `Game` fields, but neither shows the serialised output (top-level array or object, whether DLC is nested or flat, where cover-art `keyImages` live). Legendary is not installed here, so no real sample was captured.
- **Risk:** a hand-written test fixture built from the model file may not match real output, so the importer could pass tests and fail on a real library. Mitigation: read Legendary's `cli.py` list handler during planning, and treat criterion 8 of INT-0001 (live import) as unverified until a real run is observed on a signed-in machine.
- **Risk:** title normalisation that is too aggressive merges different games; too timid and the headline feature does nothing. Mitigation: a table of should-merge and should-not-merge pairs as tests.
- **Unknown:** whether SteamCMD can list an owned library at all. Not confirmed by any source read. The README's assumption that SteamCMD supplies the Steam library may not hold; the Web API or local Steam client files may be needed instead. Out of scope this sprint, flagged for the Steam intent.
- **Unknown:** how GOG and Amazon libraries are listed. Neither tool's README documents it. Out of scope this sprint.
- **Dependency:** live import needs the person to install Legendary and run `legendary auth` themselves.
- **Dependency:** Rust crates for SQLite, JSON, and argument parsing (candidates: `rusqlite` with bundled SQLite, `serde`/`serde_json`, `clap`). Versions to be pinned in planning.

## 5. Recommended Approach
Primary: a Cargo workspace with a library crate (domain types, SQLite store
with numbered migrations, title normalisation, a `Source` importer trait, the
Legendary importer that parses a JSON document and separately a thin runner
that executes `legendary list --json`) and a binary crate exposing
`import legendary [--file <path>]`, `add`, and `list [--json]`. Tests run
entirely from fixture files and an in-memory or temp-file database. Add a
Rust CI workflow and a `cargo` Dependabot entry.

Alternative considered: begin with an Electron shell over mock data, or a
TypeScript core as Heroic uses.

Rationale: the catalog schema is the most expensive thing to change later and
can be proven without accounts, network, or UI. Separating "parse a document"
from "run the tool" keeps nearly all behaviour testable on a machine without
Legendary, and isolates the one part that cannot be verified here.

## Artifacts
None saved.
