//! The single error type of the catalog core.

/// Everything that can go wrong inside the catalog core.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// SQLite reported a failure.
    #[error("database error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    /// The catalog file was written by a newer version of the program.
    #[error(
        "the catalog is at schema version {found}, but this program only \
         understands up to version {supported}; use a newer Panaseeya to open it"
    )]
    SchemaTooNew { found: u32, supported: u32 },

    /// A schema migration step could not be applied.
    #[error("schema migration step {step} failed: {source}")]
    Migration {
        step: u32,
        #[source]
        source: rusqlite::Error,
    },
}

/// Shorthand for results carrying the core [`Error`].
pub type Result<T> = std::result::Result<T, Error>;
