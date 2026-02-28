use colored::Colorize;

use crate::error::{BonsaiError, Result};
use crate::git::runner::GitRunner;

pub fn exec_lock(
    git: &GitRunner,
    worktree: &str,
    reason: Option<&str>,
    no_color: bool,
) -> Result<()> {
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

    git.worktree_lock(&wt.path, reason)?;

    let reason_str = reason.map(|r| format!(" ({r})")).unwrap_or_default();

    if no_color {
        eprintln!(
            "Locked worktree {}{}",
            wt.branch.as_deref().unwrap_or("(detached)"),
            reason_str
        );
    } else {
        eprintln!(
            "{} worktree {}{}",
            "Locked".green(),
            wt.branch.as_deref().unwrap_or("(detached)").cyan(),
            reason_str
        );
    }

    Ok(())
}

pub fn exec_unlock(git: &GitRunner, worktree: &str, no_color: bool) -> Result<()> {
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

    git.worktree_unlock(&wt.path)?;

    if no_color {
        eprintln!(
            "Unlocked worktree {}",
            wt.branch.as_deref().unwrap_or("(detached)")
        );
    } else {
        eprintln!(
            "{} worktree {}",
            "Unlocked".green(),
            wt.branch.as_deref().unwrap_or("(detached)").cyan()
        );
    }

    Ok(())
}
