//! The catalog's SQLite file and the migrations that shape it.
//!
//! The program owns the schema. Its version lives in SQLite's `user_version`
//! header field, and equals the number of migration steps applied so far.

use std::path::Path;

use rusqlite::Connection;

use crate::error::{Error, Result};

/// The product's migration steps, in order. Step *n* takes a database from
/// schema version *n − 1* to *n*. Steps are append-only: never edit or reorder
/// one that has shipped.
pub const MIGRATIONS: &[&str] = &[include_str!("migrations/0001_initial.sql")];

/// An open catalog database at the current schema version.
#[derive(Debug)]
pub struct Store {
    conn: Connection,
}

impl Store {
    /// Opens the catalog at `path`, creating the file and its schema if it does
    /// not exist and migrating it if it is older than this program.
    ///
    /// The directory holding `path` must already exist.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Self::prepare(Connection::open(path)?)
    }

    /// Opens a private, empty catalog that lives only in memory.
    pub fn open_in_memory() -> Result<Self> {
        Self::prepare(Connection::open_in_memory()?)
    }

    fn prepare(mut conn: Connection) -> Result<Self> {
        migrate(&mut conn, MIGRATIONS)?;
        conn.pragma_update(None, "foreign_keys", true)?;
        Ok(Self { conn })
    }

    /// The schema version of this catalog.
    pub fn schema_version(&self) -> Result<u32> {
        user_version(&self.conn)
    }

    #[cfg(test)]
    pub(crate) fn conn(&self) -> &Connection {
        &self.conn
    }

    pub(crate) fn conn_mut(&mut self) -> &mut Connection {
        &mut self.conn
    }
}

fn user_version(conn: &Connection) -> Result<u32> {
    Ok(conn.pragma_query_value(None, "user_version", |row| row.get(0))?)
}

/// Brings `conn` up to schema version `steps.len()`.
///
/// Steps the database already has are skipped; the rest are applied in
/// ascending order. Each step runs in its own transaction together with its
/// version bump, so a failing step leaves the database exactly at the last
/// version that succeeded. A database newer than `steps` is refused untouched.
pub fn migrate(conn: &mut Connection, steps: &[&str]) -> Result<()> {
    let found = user_version(conn)?;
    let supported = u32::try_from(steps.len()).expect("migration count fits in u32");
    if found > supported {
        return Err(Error::SchemaTooNew { found, supported });
    }
    for step in (found + 1)..=supported {
        let sql = steps[(step - 1) as usize];
        let tx = conn.transaction()?;
        tx.execute_batch(sql)
            .and_then(|()| tx.pragma_update(None, "user_version", step))
            .map_err(|source| Error::Migration { step, source })?;
        tx.commit()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table_names(conn: &Connection) -> Vec<String> {
        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name")
            .unwrap();
        stmt.query_map([], |row| row.get(0))
            .unwrap()
            .collect::<rusqlite::Result<_>>()
            .unwrap()
    }

    fn schema_text(conn: &Connection) -> Vec<String> {
        let mut stmt = conn
            .prepare("SELECT sql FROM sqlite_master WHERE sql IS NOT NULL ORDER BY name")
            .unwrap();
        stmt.query_map([], |row| row.get(0))
            .unwrap()
            .collect::<rusqlite::Result<_>>()
            .unwrap()
    }

    fn title_rows(conn: &Connection) -> Vec<(i64, String, String)> {
        let mut stmt = conn
            .prepare("SELECT id, display_name, normalized_key FROM titles ORDER BY id")
            .unwrap();
        stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
            .unwrap()
            .collect::<rusqlite::Result<_>>()
            .unwrap()
    }

    #[test]
    fn open_creates_schema_at_current_version() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("catalog.db");

        let store = Store::open(&path).unwrap();

        assert!(path.is_file());
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
        let tables = table_names(store.conn());
        assert!(tables.contains(&"titles".to_owned()));
        assert!(tables.contains(&"ownerships".to_owned()));
        assert_eq!(store.schema_version().unwrap(), MIGRATIONS.len() as u32);
    }

    #[test]
    fn reopen_current_schema_changes_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("catalog.db");

        let store = Store::open(&path).unwrap();
        store
            .conn()
            .execute(
                "INSERT INTO titles (display_name, normalized_key) VALUES ('Hades', 'hades')",
                [],
            )
            .unwrap();
        let version = store.schema_version().unwrap();
        let schema = schema_text(store.conn());
        let rows = title_rows(store.conn());
        drop(store);

        let reopened = Store::open(&path).unwrap();

        assert_eq!(reopened.schema_version().unwrap(), version);
        assert_eq!(schema_text(reopened.conn()), schema);
        assert_eq!(title_rows(reopened.conn()), rows);
        assert_eq!(rows.len(), 1);
    }

    #[test]
    fn migrate_applies_pending_steps_in_order() {
        // Step 1 would fail if it were run again, and step 3 only works after
        // step 2, so success proves both the skip and the order.
        let steps = [
            "CREATE TABLE a (x INTEGER);",
            "CREATE TABLE b (y INTEGER);",
            "ALTER TABLE b ADD COLUMN z TEXT; INSERT INTO b (y, z) VALUES (2, 'two');",
        ];
        let mut conn = Connection::open_in_memory().unwrap();
        migrate(&mut conn, &steps[..1]).unwrap();
        conn.execute("INSERT INTO a (x) VALUES (41)", []).unwrap();
        assert_eq!(user_version(&conn).unwrap(), 1);

        migrate(&mut conn, &steps).unwrap();

        assert_eq!(user_version(&conn).unwrap(), 3);
        let kept: i64 = conn.query_row("SELECT x FROM a", [], |r| r.get(0)).unwrap();
        assert_eq!(kept, 41);
        let z: String = conn.query_row("SELECT z FROM b", [], |r| r.get(0)).unwrap();
        assert_eq!(z, "two");
    }

    #[test]
    fn open_rejects_newer_schema() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("catalog.db");
        drop(Store::open(&path).unwrap());
        let current = MIGRATIONS.len() as u32;
        {
            let conn = Connection::open(&path).unwrap();
            conn.pragma_update(None, "user_version", current + 1)
                .unwrap();
        }
        let before = std::fs::read(&path).unwrap();

        let err = Store::open(&path).unwrap_err();

        assert!(matches!(
            err,
            Error::SchemaTooNew { found, supported }
                if found == current + 1 && supported == current
        ));
        let message = err.to_string();
        assert!(message.contains(&(current + 1).to_string()), "{message}");
        assert!(message.contains(&current.to_string()), "{message}");
        assert_eq!(std::fs::read(&path).unwrap(), before);
    }

    #[test]
    fn migrate_stops_at_failed_step() {
        let steps = [
            "CREATE TABLE a (x INTEGER);",
            "CREATE TABLE b (y INTEGER); INSERT INTO no_such_table VALUES (1);",
            "CREATE TABLE c (z INTEGER);",
        ];
        let mut conn = Connection::open_in_memory().unwrap();

        let err = migrate(&mut conn, &steps).unwrap_err();

        assert!(matches!(err, Error::Migration { step: 2, .. }));
        assert!(err.to_string().contains("step 2"), "{err}");
        assert_eq!(user_version(&conn).unwrap(), 1);
        assert_eq!(table_names(&conn), ["a"]);
    }
}
