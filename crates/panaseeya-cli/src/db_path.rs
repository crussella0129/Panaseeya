//! Deciding which file holds the catalog.

use std::path::{Path, PathBuf};

use directories::ProjectDirs;

/// Environment variable naming a directory to keep the catalog in.
pub const DATA_DIR_ENV: &str = "PANASEEYA_DATA_DIR";

/// The catalog's file name inside its data directory.
pub const DB_FILE_NAME: &str = "catalog.db";

/// The per-user data directory for Panaseeya on this system, if the system
/// can name one.
pub fn default_data_dir() -> Option<PathBuf> {
    ProjectDirs::from("", "", "Panaseeya").map(|dirs| dirs.data_dir().to_path_buf())
}

/// Chooses the catalog file.
///
/// An explicit `--db` path wins. Otherwise the catalog is `catalog.db` inside
/// the directory named by `PANASEEYA_DATA_DIR`, or failing that inside the
/// per-user data directory. Nothing is created here.
pub fn resolve_db_path(
    db_flag: Option<&Path>,
    data_dir_override: Option<&Path>,
) -> Result<PathBuf, String> {
    if let Some(path) = db_flag {
        return Ok(path.to_path_buf());
    }
    if let Some(dir) = data_dir_override {
        return Ok(dir.join(DB_FILE_NAME));
    }
    default_data_dir()
        .map(|dir| dir.join(DB_FILE_NAME))
        .ok_or_else(|| {
            format!(
                "could not work out where to keep the catalog; \
                 pass --db <path> or set {DATA_DIR_ENV}"
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_db_path_defaults_to_user_data_dir() {
        let flag = Path::new("flag.db");
        let dir = Path::new("override-dir");

        let default = resolve_db_path(None, None).unwrap();
        assert_eq!(default.file_name().unwrap(), DB_FILE_NAME);
        let expected_parent = ProjectDirs::from("", "", "Panaseeya")
            .unwrap()
            .data_dir()
            .to_path_buf();
        assert_eq!(default.parent().unwrap(), expected_parent);
        assert!(
            expected_parent
                .components()
                .any(|part| part.as_os_str() == "Panaseeya"),
            "{}",
            expected_parent.display()
        );

        assert_eq!(resolve_db_path(Some(flag), Some(dir)).unwrap(), flag);
        assert_eq!(
            resolve_db_path(None, Some(dir)).unwrap(),
            dir.join(DB_FILE_NAME)
        );
    }
}
