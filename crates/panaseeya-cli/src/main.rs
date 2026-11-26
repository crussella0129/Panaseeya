//! The `panaseeya` command-line program.

mod db_path;

use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use panaseeya_core::sources::{legendary, legendary_runner};
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
    /// Bring in the games you own on a store.
    Import {
        #[command(subcommand)]
        source: ImportSource,
    },
    /// Show every title once, with all the places you own it.
    List {
        /// Print the listing as JSON.
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum ImportSource {
    /// Import your Epic library using Legendary.
    ///
    /// Without --file this runs `legendary list --json`, so Legendary must be
    /// installed and signed in. Set PANASEEYA_LEGENDARY_BIN to use a Legendary
    /// that is not on your PATH.
    Legendary {
        /// Read a saved `legendary list --json` document instead of running
        /// Legendary.
        #[arg(long, value_name = "PATH")]
        file: Option<PathBuf>,
    },
}

/// Environment variable naming the Legendary program to run.
const LEGENDARY_BIN_ENV: &str = "PANASEEYA_LEGENDARY_BIN";

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
        Command::Import {
            source: ImportSource::Legendary { file },
        } => {
            // Fetch and check the document before the catalog file is opened,
            // so a failed import never creates or touches it.
            let (document, origin) = match file {
                Some(path) => {
                    let origin = path.display().to_string();
                    let document = std::fs::read_to_string(&path)
                        .map_err(|err| format!("could not read {origin}: {err}"))?;
                    (document, origin)
                }
                None => {
                    let program = std::env::var_os(LEGENDARY_BIN_ENV)
                        .filter(|value| !value.is_empty())
                        .unwrap_or_else(|| legendary_runner::DEFAULT_PROGRAM.into());
                    let origin = format!("the output of `{}`", program.to_string_lossy());
                    (legendary_runner::run_legendary_list(&program)?, origin)
                }
            };
            legendary::parse_document(&document).map_err(|err| format!("{origin}: {err}"))?;

            let mut catalog = open_catalog(&db)?;
            let games = legendary::import_legendary_document(&mut catalog, &document)?;
            println!("Imported {games} Epic games.");
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
