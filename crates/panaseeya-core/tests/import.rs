//! The Legendary importer composed with the catalog.

use panaseeya_core::sources::legendary::{import_legendary_document, parse_document};
use panaseeya_core::{Catalog, Medium, Origin, OwnershipEntry};

const FIXTURE: &str = include_str!("fixtures/legendary-list.json");

fn epic_copy() -> OwnershipEntry {
    OwnershipEntry {
        source: "epic".to_owned(),
        medium: Medium::Digital,
        origin: Origin::Store,
    }
}

#[test]
fn import_document_records_epic_digital_store_ownerships() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("catalog.db");
    let mut catalog = Catalog::open(&path).unwrap();

    let imported = import_legendary_document(&mut catalog, FIXTURE).unwrap();

    let listing = catalog.list().unwrap();
    let base_games = parse_document(FIXTURE).unwrap();
    assert_eq!(imported, base_games.len());
    assert_eq!(listing.len(), base_games.len());
    for entry in &listing {
        assert_eq!(entry.ownerships, [epic_copy()], "{}", entry.title);
    }
    drop(catalog);

    // Read the stored rows directly: each `source_id` must be the game's
    // `app_name`, and each raw title its `app_title`.
    let conn = rusqlite::Connection::open(&path).unwrap();
    let mut stored: Vec<(String, String, String, String, String)> = conn
        .prepare(
            "SELECT source, source_id, raw_title, medium, origin FROM ownerships ORDER BY source_id",
        )
        .unwrap()
        .query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?))
        })
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();
    stored.sort();
    let mut expected: Vec<(String, String, String, String, String)> = base_games
        .into_iter()
        .map(|game| {
            (
                "epic".to_owned(),
                game.app_name,
                game.app_title,
                "digital".to_owned(),
                "store".to_owned(),
            )
        })
        .collect();
    expected.sort();
    assert_eq!(stored, expected);
}

#[test]
fn import_fixture_twice_leaves_catalog_unchanged() {
    let dir = tempfile::tempdir().unwrap();
    let mut catalog = Catalog::open(dir.path().join("catalog.db")).unwrap();

    import_legendary_document(&mut catalog, FIXTURE).unwrap();
    let after_first = catalog.list().unwrap();
    import_legendary_document(&mut catalog, FIXTURE).unwrap();

    assert!(!after_first.is_empty());
    assert_eq!(catalog.list().unwrap(), after_first);
}

#[test]
fn epic_and_manual_copies_group_under_one_title() {
    let mut catalog = Catalog::open_in_memory().unwrap();
    import_legendary_document(&mut catalog, FIXTURE).unwrap();
    let titles_before = catalog.list().unwrap().len();

    catalog
        .add_manual("the witcher 3 - wild hunt", "shelf", Medium::Physical)
        .unwrap();

    let listing = catalog.list().unwrap();
    assert_eq!(listing.len(), titles_before);
    let witcher = listing
        .iter()
        .find(|entry| entry.title == "The Witcher 3: Wild Hunt")
        .unwrap();
    assert_eq!(
        witcher.ownerships,
        [
            epic_copy(),
            OwnershipEntry {
                source: "shelf".to_owned(),
                medium: Medium::Physical,
                origin: Origin::Manual,
            },
        ]
    );
}
