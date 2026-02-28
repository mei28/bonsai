use std::path::PathBuf;

use colored::Colorize;

use crate::config::Config;
use crate::error::{BonsaiError, Result};
use crate::git::runner::GitRunner;
use crate::git::worktree::AddOptions;
use crate::hooks;

pub struct AddArgs {
    pub branch: String,
    pub create: bool,
    pub base: Option<String>,
    pub path: Option<String>,
    pub name: Option<String>,
    pub detach: bool,
    pub no_hooks: bool,
}

pub fn exec(git: &GitRunner, args: AddArgs, no_color: bool) -> Result<()> {
    let config_path = git.repo_root.join(".bonsai.toml");
    let config = Config::load(&config_path)?;

    let worktree_dir = &config.defaults.worktree_dir;

    // Determine worktree path
    let wt_path: PathBuf = if let Some(ref custom_path) = args.path {
        PathBuf::from(custom_path)
    } else {
        let dir_name = args
            .name
            .as_deref()
            .unwrap_or(&args.branch)
            .replace('/', "-");
        git.repo_root.join(worktree_dir).join(&dir_name)
    };

    if wt_path.exists() {
        return Err(BonsaiError::WorktreeExists {
            name: args.branch.clone(),
        });
    }

    // Check branch existence
    if args.create {
        if git.branch_exists(&args.branch)? {
            return Err(BonsaiError::BranchExists {
                name: args.branch.clone(),
            });
        }
    } else if !args.detach && !git.branch_exists(&args.branch)? {
        return Err(BonsaiError::BranchNotFound {
            name: args.branch.clone(),
        });
    }

    let opts = AddOptions {
        create_branch: args.create,
        base: args.base,
        detach: args.detach,
    };

    git.worktree_add(&wt_path, &args.branch, &opts)?;

    if no_color {
        eprintln!("Created worktree at {}", wt_path.display());
    } else {
        eprintln!(
            "{} worktree at {}",
            "Created".green(),
            wt_path.display().to_string().yellow()
        );
    }

    // Run post_create hooks
    if !args.no_hooks && !config.hooks.post_create.is_empty() {
        hooks::run_hooks(
            &config.hooks.post_create,
            &git.repo_root,
            &wt_path,
            no_color,
        )?;
    }

    Ok(())
}
