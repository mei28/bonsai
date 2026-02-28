use std::path::{Path, PathBuf};
use std::process::Command;

use colored::Colorize;

use crate::error::{BonsaiError, Result};

pub struct GitRunner {
    pub repo_root: PathBuf,
    pub dry_run: bool,
    pub verbose: bool,
    pub no_color: bool,
}

impl GitRunner {
    pub fn new(repo_root: PathBuf, dry_run: bool, verbose: bool, no_color: bool) -> Self {
        Self {
            repo_root,
            dry_run,
            verbose,
            no_color,
        }
    }

    /// Find the git repository root from the given path.
    pub fn find_repo_root(from: &Path) -> Result<PathBuf> {
        let output = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .current_dir(from)
            .output()
            .map_err(|e| BonsaiError::GitCommandFailed {
                command: "git rev-parse --show-toplevel".to_string(),
                stderr: e.to_string(),
            })?;

        if !output.status.success() {
            return Err(BonsaiError::NotInRepository);
        }

        let root = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(PathBuf::from(root))
    }

    /// Execute a git command and return stdout.
    pub fn run(&self, args: &[&str]) -> Result<String> {
        let cmd_str = format!("git {}", args.join(" "));

        if self.dry_run {
            if self.no_color {
                eprintln!("[dry-run] {cmd_str}");
            } else {
                eprintln!("{} {}", "[dry-run]".yellow(), cmd_str.yellow());
            }
            return Ok(String::new());
        }

        if self.verbose {
            if self.no_color {
                eprintln!("$ {cmd_str}");
            } else {
                eprintln!("{}", format!("$ {cmd_str}").bright_black());
            }
        }

        let output = Command::new("git")
            .args(args)
            .current_dir(&self.repo_root)
            .output()
            .map_err(|e| BonsaiError::GitCommandFailed {
                command: cmd_str.clone(),
                stderr: e.to_string(),
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            return Err(BonsaiError::GitCommandFailed {
                command: cmd_str,
                stderr,
            });
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}
