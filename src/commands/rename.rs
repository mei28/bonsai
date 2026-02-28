use colored::Colorize;

use crate::config::Config;
use crate::error::{BonsaiError, Result};
use crate::git::runner::GitRunner;

pub fn exec(git: &GitRunner, old: &str, new: &str, no_color: bool) -> Result<()> {
    let worktrees = git.worktree_list()?;

    let wt = worktrees
        .iter()
        .find(|w| w.branch.as_deref() == Some(old))
        .ok_or_else(|| BonsaiError::WorktreeNotFound {
            name: old.to_string(),
        })?;

    if wt.is_main {
        return Err(BonsaiError::WorktreeNotFound {
            name: format!("{old} (cannot rename main worktree)"),
        });
    }

    // Rename branch
    git.branch_rename(old, new)?;

    // Move worktree directory if under managed dir
    let config_path = git.repo_root.join(".bonsai.toml");
    if let Ok(config) = Config::load(&config_path) {
        let managed_dir = git.repo_root.join(&config.defaults.worktree_dir);
        if wt.path.starts_with(&managed_dir) {
            let new_dir_name = new.replace('/', "-");
            let new_path = managed_dir.join(&new_dir_name);
            git.worktree_move(&wt.path, &new_path)?;

            if no_color {
                eprintln!("Renamed {old} -> {new}");
                eprintln!("Moved worktree to {}", new_path.display());
            } else {
                eprintln!("{} {} -> {}", "Renamed".green(), old.cyan(), new.cyan());
                eprintln!(
                    "{} worktree to {}",
                    "Moved".green(),
                    new_path.display().to_string().yellow()
                );
            }
            return Ok(());
        }
    }

    if no_color {
        eprintln!("Renamed {old} -> {new}");
    } else {
        eprintln!("{} {} -> {}", "Renamed".green(), old.cyan(), new.cyan());
    }

    Ok(())
}
