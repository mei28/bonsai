use std::path::Path;

use colored::Colorize;

use crate::config::Config;
use crate::error::{BonsaiError, Result};

pub fn exec(repo_root: &Path, force: bool, no_color: bool) -> Result<()> {
    let config_path = repo_root.join(".bonsai.toml");
    let worktree_dir = repo_root.join(".bonsai");

    if config_path.exists() && !force {
        return Err(BonsaiError::AlreadyInitialized);
    }

    // Create .bonsai directory
    std::fs::create_dir_all(&worktree_dir)?;

    // Write default config
    let config = Config::default_config();
    config.save(&config_path)?;

    // Update .gitignore
    update_gitignore(repo_root)?;

    if no_color {
        eprintln!("Initialized bonsai in {}", repo_root.display());
    } else {
        eprintln!(
            "{} bonsai in {}",
            "Initialized".green(),
            repo_root.display().to_string().yellow()
        );
    }

    Ok(())
}

fn update_gitignore(repo_root: &Path) -> Result<()> {
    let gitignore_path = repo_root.join(".gitignore");
    let entry = ".bonsai/";

    if gitignore_path.exists() {
        let content = std::fs::read_to_string(&gitignore_path)?;
        if content.lines().any(|line| line.trim() == entry) {
            return Ok(());
        }
        let separator = if content.ends_with('\n') { "" } else { "\n" };
        std::fs::write(&gitignore_path, format!("{content}{separator}{entry}\n"))?;
    } else {
        std::fs::write(&gitignore_path, format!("{entry}\n"))?;
    }

    Ok(())
}
