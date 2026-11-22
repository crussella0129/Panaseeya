//! Epic library import from the output of `legendary list --json`.
//!
//! Legendary prints a JSON array of game objects. Each carries `app_name`
//! (Epic's identifier), `app_title`, a `metadata` object, and a `dlcs` array
//! holding that game's DLC. DLC is recognised by a `mainGameItem` key in its
//! `metadata`. Only base games become catalog entries.

use serde_json::Value;

use crate::catalog::Catalog;
use crate::error::{Error, Result};
use crate::model::{Medium, NewOwnership, Origin};

/// The source name under which Epic copies are recorded.
pub const SOURCE: &str = "epic";

/// One base game found in a Legendary document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegendaryGame {
    /// Epic's identifier for the game.
    pub app_name: String,
    /// The game's title as Epic spells it.
    pub app_title: String,
}

fn malformed(detail: impl Into<String>) -> Error {
    Error::LegendaryDocument {
        detail: detail.into(),
    }
}

/// Reads the base games out of a `legendary list --json` document.
///
/// Nested `dlcs` entries and top-level entries marked as DLC are left out.
/// Fields this program does not use are ignored. If any part of the document
/// is malformed the whole document is rejected; no partial list is returned.
pub fn parse_document(document: &str) -> Result<Vec<LegendaryGame>> {
    let value: Value = serde_json::from_str(document)
        .map_err(|err| malformed(format!("not valid JSON: {err}")))?;
    let entries = value
        .as_array()
        .ok_or_else(|| malformed("expected a JSON array of games at the top level"))?;

    let mut games = Vec::new();
    for (index, entry) in entries.iter().enumerate() {
        let position = index + 1;
        let object = entry
            .as_object()
            .ok_or_else(|| malformed(format!("entry {position} is not a JSON object")))?;

        let is_dlc = object
            .get("metadata")
            .and_then(Value::as_object)
            .is_some_and(|metadata| metadata.contains_key("mainGameItem"));
        if is_dlc {
            continue;
        }

        let text_field = |name: &str| -> Result<String> {
            match object.get(name) {
                None => Err(malformed(format!("entry {position} has no `{name}`"))),
                Some(Value::String(text)) if text.trim().is_empty() => {
                    Err(malformed(format!("entry {position} has a blank `{name}`")))
                }
                Some(Value::String(text)) => Ok(text.clone()),
                Some(_) => Err(malformed(format!(
                    "entry {position} has a `{name}` that is not text"
                ))),
            }
        };
        games.push(LegendaryGame {
            app_name: text_field("app_name")?,
            app_title: text_field("app_title")?,
        });
    }
    Ok(games)
}

/// Imports a `legendary list --json` document into the catalog.
///
/// Every base game becomes one copy with source `epic`, medium `digital`, and
/// origin `store`, identified by its `app_name`. The import is all-or-nothing,
/// and importing the same document again changes nothing. Returns the number
/// of base games the document held.
pub fn import_legendary_document(catalog: &mut Catalog, document: &str) -> Result<usize> {
    let ownerships: Vec<NewOwnership> = parse_document(document)?
        .into_iter()
        .map(|game| NewOwnership {
            origin: Origin::Store,
            source: SOURCE.to_owned(),
            source_id: game.app_name,
            medium: Medium::Digital,
            title: game.app_title,
        })
        .collect();
    catalog.record_batch(&ownerships)?;
    Ok(ownerships.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = include_str!("../../tests/fixtures/legendary-list.json");

    fn titles(games: &[LegendaryGame]) -> Vec<&str> {
        games.iter().map(|game| game.app_title.as_str()).collect()
    }

    #[test]
    fn parse_returns_one_game_per_base_entry() {
        let games = parse_document(FIXTURE).unwrap();

        let found: Vec<(&str, &str)> = games
            .iter()
            .map(|game| (game.app_name.as_str(), game.app_title.as_str()))
            .collect();
        assert_eq!(
            found,
            [
                ("fixture-app-control", "Control"),
                ("fixture-app-hades", "Hades"),
                ("fixture-app-witcher3", "The Witcher 3: Wild Hunt"),
            ]
        );
    }

    #[test]
    fn parse_ignores_nested_dlcs() {
        let document = r#"[{
            "app_name": "base", "app_title": "Control", "metadata": {},
            "dlcs": [
                {"app_name": "dlc1", "app_title": "Control: The Foundation",
                 "metadata": {"mainGameItem": {}}},
                {"app_name": "dlc2", "app_title": "Control: AWE", "metadata": {}}
            ]
        }]"#;

        let games = parse_document(document).unwrap();

        assert_eq!(titles(&games), ["Control"]);
    }

    #[test]
    fn parse_skips_top_level_dlc_entry() {
        let document = r#"[
            {"app_name": "base", "app_title": "Hades", "metadata": {}, "dlcs": []},
            {"app_name": "ost", "app_title": "Hades Original Soundtrack",
             "metadata": {"mainGameItem": {"id": "x"}}, "dlcs": []}
        ]"#;

        let games = parse_document(document).unwrap();

        assert_eq!(titles(&games), ["Hades"]);
        assert!(!titles(&parse_document(FIXTURE).unwrap()).contains(&"Hades Original Soundtrack"));
    }

    #[test]
    fn parse_rejects_malformed_document() {
        let good = r#"{"app_name": "a", "app_title": "Hades"}"#;
        let cases = [
            ("not json at all", "not valid JSON"),
            (r#"{"app_name": "a", "app_title": "Hades"}"#, "JSON array"),
            (r#"[{"app_name": "a"}]"#, "no `app_title`"),
            (
                r#"[{"app_name": "a", "app_title": "   "}]"#,
                "blank `app_title`",
            ),
            (
                r#"[{"app_name": "", "app_title": "Hades"}]"#,
                "blank `app_name`",
            ),
            (r#"[{"app_name": 7, "app_title": "Hades"}]"#, "not text"),
            ("[42]", "not a JSON object"),
        ];
        for (document, expected) in cases {
            let err = parse_document(document).unwrap_err();
            assert!(matches!(err, Error::LegendaryDocument { .. }), "{err}");
            assert!(err.to_string().contains(expected), "{document}: {err}");
        }

        // A bad entry after a good one still yields no games at all.
        let mixed = format!(r#"[{good}, {{"app_name": "b"}}]"#);
        assert!(parse_document(&mixed).is_err());
    }
}
