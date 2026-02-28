use std::path::Path;
use std::process::Command;

use colored::Colorize;

use crate::config::Hook;
use crate::error::{BonsaiError, Result};

pub fn run_hooks(
    hooks: &[Hook],
    repo_root: &Path,
    worktree_path: &Path,
    no_color: bool,
) -> Result<()> {
    for hook in hooks {
        match hook {
            Hook::Copy { from, to } => {
                let src = repo_root.join(from);
                let dst = worktree_path.join(to);
                if src.exists() {
                    if let Some(parent) = dst.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    std::fs::copy(&src, &dst)?;
                    if no_color {
                        eprintln!("Copied {from} -> {to}");
                    } else {
                        eprintln!("{} {} -> {}", "Copied".green(), from.yellow(), to.yellow());
                    }
                }
            }
            Hook::Symlink { from, to } => {
                let src = repo_root.join(from);
                let dst = worktree_path.join(to);
                if src.exists() {
                    if let Some(parent) = dst.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    #[cfg(unix)]
                    std::os::unix::fs::symlink(&src, &dst)?;
                    #[cfg(windows)]
                    std::os::windows::fs::symlink_file(&src, &dst)?;
                    if no_color {
                        eprintln!("Symlinked {from} -> {to}");
                    } else {
                        eprintln!(
                            "{} {} -> {}",
                            "Symlinked".green(),
                            from.yellow(),
                            to.yellow()
                        );
                    }
                }
            }
            Hook::Command { command, env } => {
                if no_color {
                    eprintln!("Running hook: {command}");
                } else {
                    eprintln!("{} {}", "Running hook:".bright_black(), command);
                }
                let mut cmd = Command::new("sh");
                cmd.arg("-c").arg(command).current_dir(worktree_path);
                for (key, value) in env {
                    cmd.env(key, value);
                }
                let status = cmd.status().map_err(|e| {
                    BonsaiError::HookFailed(format!("failed to run '{command}': {e}"))
                })?;
                if !status.success() {
                    return Err(BonsaiError::HookFailed(format!(
                        "command '{command}' exited with {status}"
                    )));
                }
            }
        }
    }
    Ok(())
}
