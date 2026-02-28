use colored::Colorize;

use crate::error::Result;
use crate::git::runner::GitRunner;

pub fn exec(
    git: &GitRunner,
    porcelain: bool,
    show_status: bool,
    names_only: bool,
    no_color: bool,
) -> Result<()> {
    let worktrees = git.worktree_list()?;

    if names_only {
        for wt in &worktrees {
            let name = wt.branch.as_deref().unwrap_or("(detached)");
            println!("{name}");
        }
        return Ok(());
    }

    if porcelain {
        for wt in &worktrees {
            let branch = wt.branch.as_deref().unwrap_or("(detached)");
            let path = wt.path.display();
            let head = &wt.head[..8.min(wt.head.len())];
            println!("{branch}\t{path}\t{head}");
        }
        return Ok(());
    }

    // Table display
    let current_dir = std::env::current_dir().ok();

    // Collect data
    struct Row {
        is_current: bool,
        branch: String,
        path: String,
        status: String,
    }

    let mut rows = Vec::new();
    for wt in &worktrees {
        let is_current = current_dir
            .as_ref()
            .map(|cd| cd.starts_with(&wt.path))
            .unwrap_or(false);

        let branch = wt.branch.as_deref().unwrap_or("(detached)").to_string();

        let path = if wt.is_main {
            wt.path.display().to_string()
        } else {
            // Show relative to repo root
            wt.path
                .strip_prefix(&git.repo_root)
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| wt.path.display().to_string())
        };

        let status = if show_status {
            match git.status_summary(&wt.path) {
                Ok(s) => s.short_display(),
                Err(_) => "?".to_string(),
            }
        } else {
            String::new()
        };

        rows.push(Row {
            is_current,
            branch,
            path,
            status,
        });
    }

    // Calculate column widths
    let branch_width = rows
        .iter()
        .map(|r| r.branch.len())
        .max()
        .unwrap_or(6)
        .max(6);
    let path_width = rows.iter().map(|r| r.path.len()).max().unwrap_or(4).max(4);

    // Header
    if show_status {
        let header = format!(
            "  {:<branch_width$}  {:<path_width$}  STATUS",
            "BRANCH", "PATH"
        );
        let separator_len = header.len();
        if no_color {
            println!("{header}");
            println!("{}", "─".repeat(separator_len));
        } else {
            println!("{}", header.blue().bold());
            println!("{}", "─".repeat(separator_len).bright_black());
        }
    } else {
        let header = format!("  {:<branch_width$}  PATH", "BRANCH");
        let separator_len = header.len();
        if no_color {
            println!("{header}");
            println!("{}", "─".repeat(separator_len));
        } else {
            println!("{}", header.blue().bold());
            println!("{}", "─".repeat(separator_len).bright_black());
        }
    }

    // Rows
    for row in &rows {
        let marker = if row.is_current { "*" } else { " " };

        if no_color {
            if show_status {
                println!(
                    "{marker} {:<branch_width$}  {:<path_width$}  {}",
                    row.branch, row.path, row.status
                );
            } else {
                println!("{marker} {:<branch_width$}  {}", row.branch, row.path);
            }
        } else {
            let marker_display = if row.is_current {
                "*".green().bold().to_string()
            } else {
                " ".to_string()
            };
            let branch_display = row.branch.cyan().to_string();
            let path_display = row.path.yellow().to_string();

            if show_status {
                // Pad branch with spaces to align (accounting for ANSI codes)
                let branch_padding = branch_width.saturating_sub(row.branch.len());
                let path_padding = path_width.saturating_sub(row.path.len());
                println!(
                    "{marker_display} {branch_display}{:branch_padding$}  {path_display}{:path_padding$}  {}",
                    "", "", row.status
                );
            } else {
                let branch_padding = branch_width.saturating_sub(row.branch.len());
                println!(
                    "{marker_display} {branch_display}{:branch_padding$}  {path_display}",
                    ""
                );
            }
        }
    }

    Ok(())
}
