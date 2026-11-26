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

// --- import legendary -------------------------------------------------------

const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../panaseeya-core/tests/fixtures/legendary-list.json"
);

fn import_file(db: &Path, file: &Path) -> Command {
    let mut command = with_db(db);
    command.args(["import", "legendary", "--file"]).arg(file);
    command
}

fn import_live(db: &Path, legendary: &Path) -> Command {
    let mut command = with_db(db);
    command
        .env("PANASEEYA_LEGENDARY_BIN", legendary)
        .args(["import", "legendary"]);
    command
}

/// Writes a stand-in for Legendary into `dir` and returns its path. It prints
/// `stdout`, writes `stderr` to standard error, and exits with `code`.
fn stand_in(dir: &Path, stdout: &str, stderr: &str, code: i32) -> std::path::PathBuf {
    std::fs::write(dir.join("stdout.txt"), stdout).unwrap();
    std::fs::write(dir.join("stderr.txt"), stderr).unwrap();
    #[cfg(windows)]
    {
        let path = dir.join("legendary-stand-in.cmd");
        let script = format!(
            "@echo off\r\n\
             type \"%~dp0stdout.txt\"\r\n\
             type \"%~dp0stderr.txt\" 1>&2\r\n\
             exit /b {code}\r\n"
        );
        std::fs::write(&path, script).unwrap();
        path
    }
    #[cfg(not(windows))]
    {
        use std::os::unix::fs::PermissionsExt;

        let path = dir.join("legendary-stand-in.sh");
        let script = format!(
            "#!/bin/sh\n\
             here=$(dirname \"$0\")\n\
             cat \"$here/stdout.txt\"\n\
             cat \"$here/stderr.txt\" 1>&2\n\
             exit {code}\n"
        );
        std::fs::write(&path, script).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
        path
    }
}

/// Runs a failing command twice — against a database path with no file, and
/// against an existing database — and checks that it exits non-zero, mentions
/// `expected` on standard error, and leaves the database as it found it.
fn assert_fails_and_leaves_db_untouched(command_for: impl Fn(&Path) -> Command, expected: &str) {
    let (_dir, missing) = temp_db();
    command_for(&missing)
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    assert!(!missing.exists(), "a failed import created the database");

    let (_dir, existing) = temp_db();
    add(&existing, "Doom", "box", "physical");
    let before = std::fs::read(&existing).unwrap();
    command_for(&existing)
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    assert_eq!(
        std::fs::read(&existing).unwrap(),
        before,
        "a failed import changed the database"
    );
}

#[test]
fn cli_import_file_then_list_shows_epic_titles() {
    let (_dir, db) = temp_db();

    import_file(&db, Path::new(FIXTURE)).assert().success();

    let listing: serde_json::Value = serde_json::from_str(&list_json(&db)).unwrap();
    let epic_copy = serde_json::json!(
        [{"source": "epic", "medium": "digital", "origin": "store"}]
    );
    let titles: Vec<&str> = listing
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| {
            assert_eq!(entry["ownerships"], epic_copy);
            entry["title"].as_str().unwrap()
        })
        .collect();
    assert_eq!(titles, ["Control", "Hades", "The Witcher 3: Wild Hunt"]);
    let text = list_text(&db);
    for dlc in ["The Foundation", "AWE", "Soundtrack"] {
        assert!(!text.contains(dlc), "{dlc} was listed:\n{text}");
    }
}

#[test]
fn cli_import_file_twice_is_idempotent() {
    let (_dir, db) = temp_db();

    import_file(&db, Path::new(FIXTURE)).assert().success();
    let after_first = list_json(&db);
    import_file(&db, Path::new(FIXTURE)).assert().success();

    assert!(after_first.contains("Hades"));
    assert_eq!(list_json(&db), after_first);
}

#[test]
fn cli_list_after_import_shows_store_and_manual_on_one_line() {
    let (_dir, db) = temp_db();
    import_file(&db, Path::new(FIXTURE)).assert().success();

    add(&db, "hades", "shelf", "physical");

    let text = list_text(&db);
    let hades_lines: Vec<&str> = text.lines().filter(|l| l.starts_with("Hades")).collect();
    assert_eq!(
        hades_lines,
        ["Hades\tepic (digital); shelf (physical, manual)"]
    );
}

#[test]
fn cli_live_import_matches_file_import() {
    let scripts = tempfile::tempdir().unwrap();
    let fixture = std::fs::read_to_string(FIXTURE).unwrap();
    let legendary = stand_in(scripts.path(), &fixture, "", 0);
    let (_dir_live, live_db) = temp_db();
    let (_dir_file, file_db) = temp_db();

    import_live(&live_db, &legendary).assert().success();
    import_file(&file_db, Path::new(FIXTURE)).assert().success();

    let from_live = list_json(&live_db);
    assert!(from_live.contains("Hades"));
    assert_eq!(from_live, list_json(&file_db));
    // The listing hides identifiers, so compare the stored rows as well.
    let live_rows = stored_ownerships(&live_db);
    assert_eq!(live_rows.len(), 3);
    assert_eq!(live_rows, stored_ownerships(&file_db));
}

/// Every stored ownership as (origin, source, source_id, medium, raw_title).
fn stored_ownerships(db: &Path) -> Vec<[String; 5]> {
    let conn = rusqlite::Connection::open(db).unwrap();
    let mut statement = conn
        .prepare(
            "SELECT origin, source, source_id, medium, raw_title FROM ownerships
             ORDER BY origin, source, source_id",
        )
        .unwrap();
    statement
        .query_map([], |row| {
            Ok([
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ])
        })
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap()
}

#[test]
fn cli_live_import_with_default_program_absent_names_legendary() {
    // No override and an empty search path: the default `legendary` program
    // cannot be found, which is the real "Legendary is not installed" case.
    let empty_path = tempfile::tempdir().unwrap();

    assert_fails_and_leaves_db_untouched(
        |db| {
            let mut command = with_db(db);
            command
                .env("PATH", empty_path.path())
                .args(["import", "legendary"]);
            command
        },
        "`legendary`",
    );
}

#[test]
fn cli_live_import_without_legendary_fails_and_leaves_db_untouched() {
    let missing_tool = Path::new("panaseeya-no-such-legendary");

    assert_fails_and_leaves_db_untouched(
        |db| import_live(db, missing_tool),
        "panaseeya-no-such-legendary",
    );
}

#[test]
fn cli_live_import_failed_legendary_fails_and_leaves_db_untouched() {
    let scripts = tempfile::tempdir().unwrap();
    let legendary = stand_in(scripts.path(), "", "Login failed, cannot continue!", 1);

    assert_fails_and_leaves_db_untouched(
        |db| import_live(db, &legendary),
        "Login failed, cannot continue!",
    );
}

#[test]
fn cli_import_bad_file_fails_and_leaves_db_untouched() {
    let dir = tempfile::tempdir().unwrap();
    let absent = dir.path().join("absent-list.json");
    let malformed = dir.path().join("malformed-list.json");
    std::fs::write(&malformed, "{}").unwrap();

    assert_fails_and_leaves_db_untouched(|db| import_file(db, &absent), "absent-list.json");
    assert_fails_and_leaves_db_untouched(|db| import_file(db, &malformed), "malformed-list.json");
}
