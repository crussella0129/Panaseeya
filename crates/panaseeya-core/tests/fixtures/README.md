# Test fixtures

## `legendary-list.json`

A stand-in for the output of `legendary list --json`.

**This file is hand-built. It was not captured from a real Epic account.** Its
shape follows the `list_games` handler in Legendary's `cli.py`: a top-level
array of game objects (`app_name`, `app_title`, `asset_infos`, `base_urls`,
`metadata`), each with a `dlcs` array added. All identifiers are invented.

It contains:

- three base games: `Control` (with two nested DLC entries), `Hades`, and
  `The Witcher 3: Wild Hunt`;
- one top-level entry marked as DLC by a `mainGameItem` key in its `metadata`
  (`Hades Original Soundtrack`), which an importer must skip.

Tests that pass against this file show the importer handles the documented
shape. They do not show it handles a real library; that needs a document saved
from a signed-in Legendary install.
