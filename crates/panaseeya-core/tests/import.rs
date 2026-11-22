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
    let mut catalog = Catalog::open_in_memory().unwrap();

    let imported = import_legendary_document(&mut catalog, FIXTURE).unwrap();

    let listing = catalog.list().unwrap();
    let base_games = parse_document(FIXTURE).unwrap();
    assert_eq!(imported, base_games.len());
    assert_eq!(listing.len(), base_games.len());
    for entry in &listing {
        assert_eq!(entry.ownerships, [epic_copy()], "{}", entry.title);
    }

    // Importing again must be recognised by `app_name`: recording each base
    // game by hand under its `app_name` adds nothing, which it would if the
    // importer had stored any other identifier.
    for game in base_games {
        catalog
            .record(&panaseeya_core::NewOwnership {
                origin: Origin::Store,
                source: "epic".to_owned(),
                source_id: game.app_name,
                medium: Medium::Digital,
                title: game.app_title,
            })
            .unwrap();
    }
    assert_eq!(catalog.list().unwrap(), listing);
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
