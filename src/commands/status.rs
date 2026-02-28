use colored::Colorize;

use crate::error::{BonsaiError, Result};
use crate::git::runner::GitRunner;

pub fn exec(git: &GitRunner, worktree: Option<&str>, no_color: bool) -> Result<()> {
    let worktrees = git.worktree_list()?;

    let targets: Vec<_> = if let Some(name) = worktree {
        let wt = worktrees
            .iter()
            .find(|w| {
                w.branch.as_deref() == Some(name)
                    || w.path.file_name().map(|n| n.to_string_lossy()) == Some(name.into())
            })
            .ok_or_else(|| BonsaiError::WorktreeNotFound {
                name: name.to_string(),
            })?;
        vec![wt]
    } else {
        worktrees.iter().collect()
    };

    for wt in targets {
        let branch = wt.branch.as_deref().unwrap_or("(detached)");
        let status = git.status_summary(&wt.path);
        let last_commit = git.last_commit_date(&wt.path);

        let status_str = match &status {
            Ok(s) => s.short_display(),
            Err(_) => "?".to_string(),
        };

        let commit_str = match &last_commit {
            Ok(d) => d.clone(),
            Err(_) => "unknown".to_string(),
        };

        if no_color {
            println!("{branch}");
            println!("  Path: {}", wt.path.display());
            println!("  Status: {status_str}");
            println!("  Last commit: {commit_str}");
        } else {
            println!("{}", branch.cyan().bold());
            println!("  Path: {}", wt.path.display().to_string().yellow());
            println!("  Status: {status_str}");
            println!("  Last commit: {commit_str}");
        }

        if wt.is_locked {
            let reason = wt
                .lock_reason
                .as_deref()
                .map(|r| format!(" ({r})"))
                .unwrap_or_default();
            if no_color {
                println!("  Locked{reason}");
            } else {
                println!("  {}{reason}", "Locked".red());
            }
        }

        println!();
    }

    Ok(())
}
