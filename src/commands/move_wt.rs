use std::path::PathBuf;

use colored::Colorize;

use crate::error::{BonsaiError, Result};
use crate::git::runner::GitRunner;

pub fn exec(git: &GitRunner, worktree: &str, new_path: &str, no_color: bool) -> Result<()> {
    let worktrees = git.worktree_list()?;

    let wt = worktrees
        .iter()
        .find(|w| {
            w.branch.as_deref() == Some(worktree)
                || w.path.file_name().map(|n| n.to_string_lossy()) == Some(worktree.into())
        })
        .ok_or_else(|| BonsaiError::WorktreeNotFound {
            name: worktree.to_string(),
        })?;

    if wt.is_main {
        return Err(BonsaiError::WorktreeNotFound {
            name: format!("{worktree} (cannot move main worktree)"),
        });
    }

    let target = PathBuf::from(new_path);
    git.worktree_move(&wt.path, &target)?;

    if no_color {
        eprintln!(
            "Moved worktree from {} to {}",
            wt.path.display(),
            target.display()
        );
    } else {
        eprintln!(
            "{} worktree from {} to {}",
            "Moved".green(),
            wt.path.display().to_string().yellow(),
            target.display().to_string().yellow()
        );
    }

    Ok(())
}
