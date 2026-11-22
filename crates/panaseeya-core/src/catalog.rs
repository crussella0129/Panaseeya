//! Catalog operations: recording copies of games and grouping them by title.

use std::path::Path;

use rusqlite::{OptionalExtension, Transaction, params};

use crate::error::{Error, Result};
use crate::model::{Medium, NewOwnership, Origin};
use crate::store::Store;
use crate::title::normalize_title;

/// The catalog of games a person owns.
#[derive(Debug)]
pub struct Catalog {
    store: Store,
}

impl Catalog {
    /// Wraps an already opened store.
    pub fn new(store: Store) -> Self {
        Self { store }
    }

    /// Opens the catalog file at `path`, creating or migrating it as needed.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Store::open(path).map(Self::new)
    }

    /// Opens a private, empty catalog that lives only in memory.
    pub fn open_in_memory() -> Result<Self> {
        Store::open_in_memory().map(Self::new)
    }

    /// Records one copy of a game. See [`Catalog::record_batch`].
    pub fn record(&mut self, ownership: &NewOwnership) -> Result<()> {
        self.record_batch(std::slice::from_ref(ownership))
    }

    /// Records several copies as one unit: either all of them are stored, or,
    /// if any one is rejected, none are.
    ///
    /// Each copy is attached to the title whose normalised key matches its own
    /// title, creating that title if there is none. A copy whose identity
    /// (`origin`, `source`, `source_id`) is already recorded is skipped.
    pub fn record_batch(&mut self, ownerships: &[NewOwnership]) -> Result<()> {
        let tx = self.store.conn_mut().transaction()?;
        for ownership in ownerships {
            record_in(&tx, ownership)?;
        }
        tx.commit()?;
        Ok(())
    }

    /// Records a copy the person entered by hand: a title, a source label of
    /// their choosing (a shelf, a site, a package manager), and a medium.
    ///
    /// A manual entry is identified by its label, its normalised title, and its
    /// medium. Adding the same one again changes nothing; the same game under
    /// the same label with a different medium is a second copy.
    pub fn add_manual(&mut self, title: &str, label: &str, medium: Medium) -> Result<()> {
        self.record(&NewOwnership {
            origin: Origin::Manual,
            source: label.trim().to_owned(),
            source_id: format!("{}:{}", normalize_title(title), medium.as_str()),
            medium,
            title: title.to_owned(),
        })
    }

    #[cfg(test)]
    pub(crate) fn store(&self) -> &Store {
        &self.store
    }
}

fn require_text(field: &'static str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(Error::BlankField { field })
    } else {
        Ok(())
    }
}

fn record_in(tx: &Transaction<'_>, ownership: &NewOwnership) -> Result<()> {
    require_text("title", &ownership.title)?;
    require_text("source", &ownership.source)?;
    require_text("source_id", &ownership.source_id)?;

    let origin = ownership.origin.as_str();
    let already_recorded: bool = tx.query_row(
        "SELECT EXISTS (SELECT 1 FROM ownerships
                        WHERE origin = ?1 AND source = ?2 AND source_id = ?3)",
        params![origin, ownership.source, ownership.source_id],
        |row| row.get(0),
    )?;
    if already_recorded {
        return Ok(());
    }

    let key = normalize_title(&ownership.title);
    let existing: Option<i64> = tx
        .query_row(
            "SELECT id FROM titles WHERE normalized_key = ?1",
            params![key],
            |row| row.get(0),
        )
        .optional()?;
    let title_id = match existing {
        Some(id) => id,
        None => {
            tx.execute(
                "INSERT INTO titles (display_name, normalized_key) VALUES (?1, ?2)",
                params![ownership.title.trim(), key],
            )?;
            tx.last_insert_rowid()
        }
    };

    tx.execute(
        "INSERT INTO ownerships (title_id, origin, source, source_id, medium, raw_title)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            title_id,
            origin,
            ownership.source,
            ownership.source_id,
            ownership.medium.as_str(),
            ownership.title,
        ],
    )?;
    Ok(())
}

#[cfg(test)]
pub(crate) mod test_support {
    use super::Catalog;

    /// Every title as `(display_name, normalized_key)`, in insertion order.
    pub(crate) fn titles(catalog: &Catalog) -> Vec<(String, String)> {
        let conn = catalog.store().conn();
        let mut stmt = conn
            .prepare("SELECT display_name, normalized_key FROM titles ORDER BY id")
            .unwrap();
        stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .collect::<rusqlite::Result<_>>()
            .unwrap()
    }

    /// Every ownership as `(title, origin, source, source_id, medium)`, in
    /// insertion order.
    pub(crate) fn ownerships(catalog: &Catalog) -> Vec<(String, String, String, String, String)> {
        let conn = catalog.store().conn();
        let mut stmt = conn
            .prepare(
                "SELECT t.display_name, o.origin, o.source, o.source_id, o.medium
                 FROM ownerships o JOIN titles t ON t.id = o.title_id
                 ORDER BY o.id",
            )
            .unwrap();
        stmt.query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })
        .unwrap()
        .collect::<rusqlite::Result<_>>()
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::{ownerships, titles};
    use super::*;

    fn row(title: &str, origin: &str, source: &str, source_id: &str, medium: &str) -> OwnershipRow {
        (
            title.to_owned(),
            origin.to_owned(),
            source.to_owned(),
            source_id.to_owned(),
            medium.to_owned(),
        )
    }

    type OwnershipRow = (String, String, String, String, String);

    #[test]
    fn manual_entry_records_label_medium_and_origin() {
        let mut catalog = Catalog::open_in_memory().unwrap();

        catalog
            .add_manual("Chrono Trigger", "shelf", Medium::Rom)
            .unwrap();

        assert_eq!(
            ownerships(&catalog),
            [row(
                "Chrono Trigger",
                "manual",
                "shelf",
                "chrono trigger:rom",
                "rom"
            )]
        );
    }

    #[test]
    fn manual_entry_repeat_is_noop_and_new_medium_adds() {
        let mut catalog = Catalog::open_in_memory().unwrap();
        catalog
            .add_manual("Chrono Trigger", "shelf", Medium::Rom)
            .unwrap();
        let after_first = ownerships(&catalog);

        catalog
            .add_manual("chrono trigger", "shelf", Medium::Rom)
            .unwrap();
        assert_eq!(ownerships(&catalog), after_first);

        catalog
            .add_manual("Chrono Trigger", "shelf", Medium::Physical)
            .unwrap();
        let recorded = ownerships(&catalog);
        assert_eq!(recorded.len(), 2);
        assert_eq!(recorded[1].4, "physical");
        assert_eq!(titles(&catalog).len(), 1);
    }

    #[test]
    fn manual_label_matching_store_name_stays_distinct() {
        let mut catalog = Catalog::open_in_memory().unwrap();

        catalog.record(&store_copy("app1", "Hades")).unwrap();
        catalog
            .add_manual("Hades", "epic", Medium::Digital)
            .unwrap();

        assert_eq!(titles(&catalog).len(), 1);
        assert_eq!(
            ownerships(&catalog),
            [
                row("Hades", "store", "epic", "app1", "digital"),
                row("Hades", "manual", "epic", "hades:digital", "digital"),
            ]
        );
    }

    fn store_copy(source_id: &str, title: &str) -> NewOwnership {
        NewOwnership {
            origin: Origin::Store,
            source: "epic".to_owned(),
            source_id: source_id.to_owned(),
            medium: Medium::Digital,
            title: title.to_owned(),
        }
    }

    #[test]
    fn record_same_ownership_twice_is_noop() {
        let mut catalog = Catalog::open_in_memory().unwrap();
        let hades = store_copy("app1", "Hades");

        catalog.record(&hades).unwrap();
        let titles_after_first = titles(&catalog);
        let ownerships_after_first = ownerships(&catalog);
        catalog.record(&hades).unwrap();

        assert_eq!(titles_after_first.len(), 1);
        assert_eq!(ownerships_after_first.len(), 1);
        assert_eq!(titles(&catalog), titles_after_first);
        assert_eq!(ownerships(&catalog), ownerships_after_first);
    }

    #[test]
    fn ownerships_with_equivalent_titles_share_one_title() {
        let mut catalog = Catalog::open_in_memory().unwrap();

        catalog.record(&store_copy("app1", "Control™")).unwrap();
        catalog.record(&store_copy("app2", "control")).unwrap();

        assert_eq!(
            titles(&catalog),
            [("Control™".to_owned(), "control".to_owned())]
        );
        let recorded = ownerships(&catalog);
        assert_eq!(recorded.len(), 2);
        assert!(recorded.iter().all(|row| row.0 == "Control™"));
    }

    #[test]
    fn ownerships_with_different_titles_stay_separate() {
        let mut catalog = Catalog::open_in_memory().unwrap();

        catalog.record(&store_copy("app1", "  Portal ")).unwrap();
        catalog.record(&store_copy("app2", "Portal 2")).unwrap();

        let names: Vec<String> = titles(&catalog).into_iter().map(|t| t.0).collect();
        assert_eq!(names, ["Portal", "Portal 2"]);
    }

    #[test]
    fn batch_with_rejected_item_records_nothing() {
        let mut catalog = Catalog::open_in_memory().unwrap();
        catalog.record(&store_copy("app0", "Doom")).unwrap();
        let titles_before = titles(&catalog);
        let ownerships_before = ownerships(&catalog);

        let result = catalog.record_batch(&[
            store_copy("app1", "Hades"),
            store_copy("app2", "Portal"),
            store_copy("app3", "   "),
        ]);

        assert!(matches!(result, Err(Error::BlankField { field: "title" })));
        assert_eq!(titles(&catalog), titles_before);
        assert_eq!(ownerships(&catalog), ownerships_before);
    }

    #[test]
    fn record_rejects_blank_fields() {
        let mut catalog = Catalog::open_in_memory().unwrap();
        let blank_title = store_copy("app1", " ");
        let blank_source = NewOwnership {
            source: "\t".to_owned(),
            ..store_copy("app1", "Hades")
        };
        let blank_source_id = store_copy("", "Hades");

        for (ownership, field) in [
            (blank_title, "title"),
            (blank_source, "source"),
            (blank_source_id, "source_id"),
        ] {
            let err = catalog.record(&ownership).unwrap_err();
            assert!(
                matches!(err, Error::BlankField { field: named } if named == field),
                "{err}"
            );
            assert!(err.to_string().contains(field), "{err}");
        }
        assert!(titles(&catalog).is_empty());
        assert!(ownerships(&catalog).is_empty());
    }
}
