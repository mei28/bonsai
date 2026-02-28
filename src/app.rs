use std::path::Path;

use crate::cli::Commands;
use crate::commands;
use crate::error::Result;
use crate::git::runner::GitRunner;

pub fn dispatch(command: Commands, dry_run: bool, verbose: bool, no_color: bool) -> Result<()> {
    let cwd = std::env::current_dir()?;

    match command {
        Commands::Init { force } => {
            let repo_root = GitRunner::find_repo_root(&cwd)?;
            commands::init::exec(&repo_root, force, no_color)
        }
        Commands::Add {
            branch,
            create,
            base,
            path,
            name,
            detach,
            no_hooks,
        } => {
            let git = make_git(&cwd, dry_run, verbose, no_color)?;
            commands::add::exec(
                &git,
                commands::add::AddArgs {
                    branch,
                    create,
                    base,
                    path,
                    name,
                    detach,
                    no_hooks,
                },
                no_color,
            )
        }
        Commands::Remove {
            worktree,
            with_branch,
            force,
        } => {
            let git = make_git(&cwd, dry_run, verbose, no_color)?;
            commands::remove::exec(&git, &worktree, with_branch, force, no_color)
        }
        Commands::List {
            porcelain,
            status,
            names_only,
        } => {
            let git = make_git(&cwd, dry_run, verbose, no_color)?;
            commands::list::exec(&git, porcelain, status, names_only, no_color)
        }
        Commands::Cd { worktree } => {
            let git = make_git(&cwd, dry_run, verbose, no_color)?;
            commands::cd::exec(&git, &worktree)
        }
        Commands::Status { worktree } => {
            let git = make_git(&cwd, dry_run, verbose, no_color)?;
            commands::status::exec(&git, worktree.as_deref(), no_color)
        }
        Commands::Prune {
            merged,
            stale,
            with_branch,
            interactive,
            yes,
        } => {
            let git = make_git(&cwd, dry_run, verbose, no_color)?;
            commands::prune::exec(&git, merged, stale, with_branch, interactive, yes, no_color)
        }
        Commands::Rename { old, new } => {
            let git = make_git(&cwd, dry_run, verbose, no_color)?;
            commands::rename::exec(&git, &old, &new, no_color)
        }
        Commands::Move { worktree, new_path } => {
            let git = make_git(&cwd, dry_run, verbose, no_color)?;
            commands::move_wt::exec(&git, &worktree, &new_path, no_color)
        }
        Commands::Lock { worktree, reason } => {
            let git = make_git(&cwd, dry_run, verbose, no_color)?;
            commands::lock::exec_lock(&git, &worktree, reason.as_deref(), no_color)
        }
        Commands::Unlock { worktree } => {
            let git = make_git(&cwd, dry_run, verbose, no_color)?;
            commands::lock::exec_unlock(&git, &worktree, no_color)
        }
        Commands::Completion { shell } => commands::completion::exec(shell),
        Commands::ShellInit { shell } => commands::shell_init::exec(shell),
    }
}

fn make_git(cwd: &Path, dry_run: bool, verbose: bool, no_color: bool) -> Result<GitRunner> {
    let repo_root = GitRunner::find_repo_root(cwd)?;
    Ok(GitRunner::new(repo_root, dry_run, verbose, no_color))
}
