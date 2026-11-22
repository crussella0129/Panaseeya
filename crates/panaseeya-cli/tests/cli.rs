//! End-to-end tests that run the built `panaseeya` binary.

use std::path::Path;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

/// The binary, with the environment variables it reads cleared so the tests
/// never depend on, or write into, the real user profile.
fn panaseeya() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_panaseeya"));
    command
        .env_remove("PANASEEYA_DATA_DIR")
        .env_remove("PANASEEYA_LEGENDARY_BIN");
    command
}

fn with_db(db: &Path) -> Command {
    let mut command = panaseeya();
    command.arg("--db").arg(db);
    command
}

fn stdout_of(command: &mut Command) -> String {
    let output = command.assert().success().get_output().stdout.clone();
    String::from_utf8(output).unwrap()
}

fn add(db: &Path, title: &str, source: &str, medium: &str) {
    with_db(db)
        .args(["add", title, "--source", source, "--medium", medium])
        .assert()
        .success();
}

fn list_text(db: &Path) -> String {
    stdout_of(with_db(db).arg("list"))
}

fn list_json(db: &Path) -> String {
    stdout_of(with_db(db).args(["list", "--json"]))
}

fn temp_db() -> (TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("catalog.db");
    (dir, db)
}

#[test]
fn cli_add_then_list_shows_manual_entry() {
    let (_dir, db) = temp_db();

    add(&db, "Chrono Trigger", "shelf", "rom");

    assert_eq!(list_text(&db), "Chrono Trigger\tshelf (rom, manual)\n");
}

#[test]
fn cli_add_with_invalid_medium_fails_and_changes_nothing() {
    let (_dir, db) = temp_db();
    add(&db, "Chrono Trigger", "shelf", "rom");
    let before = list_json(&db);

    with_db(&db)
        .args(["add", "Doom", "--source", "shelf", "--medium", "cartridge"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cartridge"));

    assert_eq!(list_json(&db), before);
}

#[test]
fn cli_list_prints_one_line_per_title() {
    let (_dir, db) = temp_db();

    add(&db, "Hades", "shelf", "rom");
    add(&db, "hades", "shelf", "physical");
    add(&db, "Doom", "box", "physical");

    assert_eq!(
        list_text(&db),
        "Doom\tbox (physical, manual)\n\
         Hades\tshelf (physical, manual); shelf (rom, manual)\n"
    );
}

#[test]
fn cli_list_json_matches_text_listing() {
    let (_dir, db) = temp_db();
    add(&db, "Hades", "shelf", "rom");
    add(&db, "hades", "shelf", "physical");
    add(&db, "Doom", "box", "physical");

    let listing: serde_json::Value = serde_json::from_str(&list_json(&db)).unwrap();

    let mut rendered = String::new();
    for entry in listing.as_array().unwrap() {
        let copies: Vec<String> = entry["ownerships"]
            .as_array()
            .unwrap()
            .iter()
            .map(|copy| {
                let source = copy["source"].as_str().unwrap();
                let medium = copy["medium"].as_str().unwrap();
                match copy["origin"].as_str().unwrap() {
                    "manual" => format!("{source} ({medium}, manual)"),
                    "store" => format!("{source} ({medium})"),
                    other => panic!("unexpected origin {other}"),
                }
            })
            .collect();
        rendered.push_str(&format!(
            "{}\t{}\n",
            entry["title"].as_str().unwrap(),
            copies.join("; ")
        ));
    }
    assert_eq!(rendered, list_text(&db));
    assert_eq!(listing.as_array().unwrap().len(), 2);
}

#[test]
fn cli_data_dir_override_holds_catalog_db() {
    let dir = tempfile::tempdir().unwrap();
    let data_dir = dir.path().join("not-yet-there");

    panaseeya()
        .env("PANASEEYA_DATA_DIR", &data_dir)
        .args(["add", "Hades", "--source", "shelf", "--medium", "rom"])
        .assert()
        .success();

    assert!(data_dir.join("catalog.db").is_file());
}

#[test]
fn cli_creates_missing_parent_directories() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("new").join("sub").join("catalog.db");

    add(&db, "Hades", "shelf", "rom");

    assert!(db.is_file());
}
