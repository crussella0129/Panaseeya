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

    /// A program Panaseeya relies on is not installed or not on the search path.
    #[error("could not find the program `{program}`; install it or check that it is on your PATH")]
    ToolNotFound { program: String },

    /// A program exists but could not be started.
    #[error("could not start `{program}`: {source}")]
    ToolNotStarted {
        program: String,
        #[source]
        source: std::io::Error,
    },

    /// A program ran and reported failure.
    #[error("`{program}` failed ({}): {stderr}", exit_text(*.status))]
    ToolFailed {
        program: String,
        /// The exit code, or `None` if the program was stopped by a signal.
        status: Option<i32>,
        /// What the program wrote to standard error.
        stderr: String,
    },
}

fn exit_text(status: Option<i32>) -> String {
    match status {
        Some(code) => format!("exit status {code}"),
        None => "stopped by a signal".to_owned(),
    }
}

/// Shorthand for results carrying the core [`Error`].
pub type Result<T> = std::result::Result<T, Error>;
