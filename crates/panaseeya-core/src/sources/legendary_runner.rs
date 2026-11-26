//! Running Legendary itself to obtain the Epic library.
//!
//! Legendary is a separate program that the person installs and signs in to.
//! This module only starts it and reads what it prints; none of its code is
//! part of Panaseeya.

use std::ffi::OsStr;
use std::io::ErrorKind;
use std::process::Command;

use crate::error::{Error, Result};

/// The program name looked up on the search path when none is configured.
pub const DEFAULT_PROGRAM: &str = "legendary";

/// The arguments that make Legendary print the owned library as JSON.
pub const LIST_ARGS: [&str; 2] = ["list", "--json"];

/// Runs `<program> list --json` and returns what it printed.
///
/// Fails with [`Error::ToolNotFound`] when the program does not exist, and with
/// [`Error::ToolFailed`] — carrying its exit status and error output — when it
/// runs but reports failure, as Legendary does when nobody is signed in.
pub fn run_legendary_list(program: &OsStr) -> Result<String> {
    let name = program.to_string_lossy().into_owned();
    let output = Command::new(program)
        .args(LIST_ARGS)
        .output()
        .map_err(|err| match err.kind() {
            ErrorKind::NotFound => Error::ToolNotFound {
                program: name.clone(),
            },
            _ => Error::ToolNotStarted {
                program: name.clone(),
                source: err,
            },
        })?;

    if !output.status.success() {
        return Err(Error::ToolFailed {
            program: name,
            status: output.status.code(),
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        });
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::*;

    /// Writes a stand-in for Legendary into `dir` and returns its path.
    ///
    /// The stand-in records its arguments in `args.txt` beside itself, prints
    /// `stdout`, writes `stderr` to standard error, and exits with `code`.
    fn stand_in(dir: &Path, stdout: &str, stderr: &str, code: i32) -> PathBuf {
        std::fs::write(dir.join("stdout.txt"), stdout).unwrap();
        std::fs::write(dir.join("stderr.txt"), stderr).unwrap();
        #[cfg(windows)]
        {
            let path = dir.join("legendary-stand-in.cmd");
            let script = format!(
                "@echo off\r\n\
                 echo %*> \"%~dp0args.txt\"\r\n\
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
                 echo \"$*\" > \"$here/args.txt\"\n\
                 cat \"$here/stdout.txt\"\n\
                 cat \"$here/stderr.txt\" 1>&2\n\
                 exit {code}\n"
            );
            std::fs::write(&path, script).unwrap();
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
            path
        }
    }

    #[test]
    fn runner_returns_stdout_on_success() {
        let dir = tempfile::tempdir().unwrap();
        let document = r#"[{"app_name": "a", "app_title": "Hades"}]"#;
        let program = stand_in(dir.path(), document, "", 0);

        let printed = run_legendary_list(program.as_os_str()).unwrap();

        assert_eq!(printed.trim(), document);
        let args = std::fs::read_to_string(dir.path().join("args.txt")).unwrap();
        assert_eq!(args.trim(), "list --json");
    }

    #[test]
    fn runner_reports_missing_tool_by_name() {
        let err = run_legendary_list(OsStr::new("panaseeya-no-such-legendary")).unwrap_err();

        assert!(
            matches!(&err, Error::ToolNotFound { program } if program == "panaseeya-no-such-legendary"),
            "{err}"
        );
        assert!(
            err.to_string().contains("panaseeya-no-such-legendary"),
            "{err}"
        );
    }

    #[test]
    fn runner_reports_nonzero_exit_with_stderr() {
        let dir = tempfile::tempdir().unwrap();
        let program = stand_in(dir.path(), "", "Login failed, cannot continue!", 1);

        let err = run_legendary_list(program.as_os_str()).unwrap_err();

        match &err {
            Error::ToolFailed { status, stderr, .. } => {
                assert_eq!(*status, Some(1));
                assert!(
                    stderr.contains("Login failed, cannot continue!"),
                    "{stderr}"
                );
            }
            other => panic!("unexpected error: {other}"),
        }
        assert!(
            err.to_string().contains("Login failed, cannot continue!"),
            "{err}"
        );
    }
}
