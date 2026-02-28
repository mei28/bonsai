use std::io::{self, Write};

use colored::Colorize;

use crate::error::Result;
use crate::git::runner::GitRunner;
use crate::git::worktree::WorktreeInfo;

pub fn exec(
    git: &GitRunner,
    merged: Option<Option<String>>,
    _stale: Option<u64>,
    with_branch: bool,
    _interactive: bool,
    yes: bool,
    no_color: bool,
) -> Result<()> {
    // First, run git worktree prune to clean up stale entries
    git.worktree_prune()?;

    let worktrees = git.worktree_list()?;

    let mut targets: Vec<&WorktreeInfo> = Vec::new();

    if let Some(base_opt) = merged {
        let base = base_opt.unwrap_or_else(|| detect_default_branch(git));

        for wt in &worktrees {
            if wt.is_main {
                continue;
            }
            if let Some(ref branch) = wt.branch {
                if branch == &base {
                    continue;
                }
                if git.branch_is_merged(branch, &base).unwrap_or(false) {
                    targets.push(wt);
                }
            }
        }
    }

    if targets.is_empty() {
        if no_color {
            eprintln!("Nothing to prune.");
        } else {
            eprintln!("{}", "Nothing to prune.".bright_black());
        }
        return Ok(());
    }

    // Show targets
    eprintln!("Worktrees to remove:");
    for wt in &targets {
        let branch = wt.branch.as_deref().unwrap_or("(detached)");
        if no_color {
            eprintln!("  - {branch} ({})", wt.path.display());
        } else {
            eprintln!(
                "  - {} ({})",
                branch.cyan(),
                wt.path.display().to_string().yellow()
            );
        }
    }

    if !yes {
        eprint!("Proceed? [y/N] ");
        io::stderr().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            eprintln!("Aborted.");
            return Ok(());
        }
    }

    for wt in &targets {
        let branch_name = wt.branch.clone();
        git.worktree_remove(&wt.path, false)?;

        if no_color {
            eprintln!("Removed worktree at {}", wt.path.display());
        } else {
            eprintln!(
                "{} worktree at {}",
                "Removed".green(),
                wt.path.display().to_string().yellow()
            );
        }

        if with_branch {
            if let Some(ref branch) = branch_name {
                git.branch_delete(branch, false)?;
                if no_color {
                    eprintln!("Deleted branch {branch}");
                } else {
                    eprintln!("{} branch {}", "Deleted".green(), branch.cyan());
                }
            }
        }
    }

    Ok(())
}

fn detect_default_branch(git: &GitRunner) -> String {
    // Try main, then master
    if git.branch_exists("main").unwrap_or(false) {
        "main".to_string()
    } else {
        "master".to_string()
    }
}
