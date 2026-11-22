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

    /// A record was offered with a required text field left blank.
    #[error("cannot record a game with a blank {field}")]
    BlankField { field: &'static str },

    /// A medium was given that is not one of the three the catalog knows.
    #[error("unknown medium {value:?}; use one of: physical, digital, rom")]
    InvalidMedium { value: String },

    /// The catalog file holds something this program never writes.
    #[error("the catalog file is damaged: {detail}")]
    CorruptCatalog { detail: String },

    /// A document offered as `legendary list --json` output could not be used.
    #[error("not a usable Legendary game list: {detail}")]
    LegendaryDocument { detail: String },
}

/// Shorthand for results carrying the core [`Error`].
pub type Result<T> = std::result::Result<T, Error>;
