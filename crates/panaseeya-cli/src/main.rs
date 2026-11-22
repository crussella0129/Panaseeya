//! The `panaseeya` command-line program.

mod db_path;

use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use panaseeya_core::{Catalog, Medium, Origin, TitleEntry};

/// One catalog of every game you own, wherever it came from.
#[derive(Debug, Parser)]
#[command(name = "panaseeya", version, about)]
struct Cli {
    /// The catalog file to use. Defaults to catalog.db in the directory named
    /// by PANASEEYA_DATA_DIR, or in your per-user data directory.
    #[arg(long, global = true, value_name = "PATH")]
    db: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Record a game you own that no store knows about.
    Add {
        /// The game's title.
        title: String,
        /// Where this copy lives: a label of your choosing, such as "shelf".
        #[arg(long, value_name = "LABEL")]
        source: String,
        /// The form of this copy: physical, digital, or rom.
        #[arg(long, value_name = "MEDIUM")]
        medium: Medium,
    },
    /// Show every title once, with all the places you own it.
    List {
        /// Print the listing as JSON.
        #[arg(long)]
        json: bool,
    },
}

type CliResult<T> = Result<T, Box<dyn Error>>;

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("panaseeya: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli) -> CliResult<()> {
    let data_dir = std::env::var_os(db_path::DATA_DIR_ENV)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from);
    let db = db_path::resolve_db_path(cli.db.as_deref(), data_dir.as_deref())?;

    match cli.command {
        Command::Add {
            title,
            source,
            medium,
        } => {
            let mut catalog = open_catalog(&db)?;
            catalog.add_manual(&title, &source, medium)?;
        }
        Command::List { json } => {
            let listing = open_catalog(&db)?.list()?;
            if json {
                println!("{}", serde_json::to_string(&listing)?);
            } else {
                for entry in &listing {
                    println!("{}", text_line(entry));
                }
            }
        }
    }
    Ok(())
}

/// Opens the catalog at `path`, first creating the directory that holds it.
fn open_catalog(path: &Path) -> CliResult<Catalog> {
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)
            .map_err(|err| format!("could not create {}: {err}", parent.display()))?;
    }
    Ok(Catalog::open(path)?)
}

/// One listing line: the title, a tab, then each copy as `source (medium)`,
/// with `, manual` added for hand-entered copies, separated by `; `.
fn text_line(entry: &TitleEntry) -> String {
    let copies: Vec<String> = entry
        .ownerships
        .iter()
        .map(|copy| match copy.origin {
            Origin::Store => format!("{} ({})", copy.source, copy.medium),
            Origin::Manual => format!("{} ({}, manual)", copy.source, copy.medium),
        })
        .collect();
    format!("{}\t{}", entry.title, copies.join("; "))
}
