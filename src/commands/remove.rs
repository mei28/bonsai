use colored::Colorize;

use crate::error::{BonsaiError, Result};
use crate::git::runner::GitRunner;

pub fn exec(
    git: &GitRunner,
    worktree: &str,
    with_branch: bool,
    force: bool,
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

    if wt.is_main {
        return Err(BonsaiError::WorktreeNotFound {
            name: format!("{worktree} (cannot remove main worktree)"),
        });
    }

    let branch_name = wt.branch.clone();
    let wt_path = wt.path.clone();

    git.worktree_remove(&wt_path, force)?;

    if no_color {
        eprintln!("Removed worktree at {}", wt_path.display());
    } else {
        eprintln!(
            "{} worktree at {}",
            "Removed".green(),
            wt_path.display().to_string().yellow()
        );
    }

    if with_branch {
        if let Some(ref branch) = branch_name {
            git.branch_delete(branch, force)?;
            if no_color {
                eprintln!("Deleted branch {branch}");
            } else {
                eprintln!("{} branch {}", "Deleted".green(), branch.cyan());
            }
        }
    }

    Ok(())
}
