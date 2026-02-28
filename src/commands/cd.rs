use crate::error::{BonsaiError, Result};
use crate::git::runner::GitRunner;

pub fn exec(git: &GitRunner, worktree: &str) -> Result<()> {
    if worktree == "@" {
        println!("{}", git.repo_root.display());
        return Ok(());
    }

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

    println!("{}", wt.path.display());
    Ok(())
}
