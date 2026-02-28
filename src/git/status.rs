use std::path::Path;

use crate::error::Result;
use crate::git::runner::GitRunner;

#[derive(Debug, Clone)]
pub struct StatusSummary {
    pub modified: usize,
    pub added: usize,
    pub deleted: usize,
    pub untracked: usize,
    pub ahead: usize,
    pub behind: usize,
}

impl StatusSummary {
    pub fn is_clean(&self) -> bool {
        self.modified == 0 && self.added == 0 && self.deleted == 0 && self.untracked == 0
    }

    pub fn short_display(&self) -> String {
        if self.is_clean() && self.ahead == 0 && self.behind == 0 {
            return "clean".to_string();
        }

        let mut parts = Vec::new();
        if self.modified > 0 {
            parts.push(format!("{}M", self.modified));
        }
        if self.added > 0 {
            parts.push(format!("{}A", self.added));
        }
        if self.deleted > 0 {
            parts.push(format!("{}D", self.deleted));
        }
        if self.untracked > 0 {
            parts.push(format!("{}?", self.untracked));
        }
        if self.ahead > 0 {
            parts.push(format!("⇡{}", self.ahead));
        }
        if self.behind > 0 {
            parts.push(format!("⇣{}", self.behind));
        }

        if parts.is_empty() {
            "clean".to_string()
        } else {
            parts.join(" ")
        }
    }
}

impl GitRunner {
    pub fn status_summary(&self, path: &Path) -> Result<StatusSummary> {
        let path_str = path.to_string_lossy();
        let output = self.run(&["-C", &path_str, "status", "--porcelain=v2", "--branch"])?;

        let mut summary = StatusSummary {
            modified: 0,
            added: 0,
            deleted: 0,
            untracked: 0,
            ahead: 0,
            behind: 0,
        };

        for line in output.lines() {
            if let Some(rest) = line.strip_prefix("# branch.ab ") {
                for part in rest.split_whitespace() {
                    if let Some(n) = part.strip_prefix('+') {
                        summary.ahead = n.parse().unwrap_or(0);
                    } else if let Some(n) = part.strip_prefix('-') {
                        summary.behind = n.parse().unwrap_or(0);
                    }
                }
            } else if line.starts_with("1 ") || line.starts_with("2 ") {
                let xy: &str = line.split_whitespace().nth(1).unwrap_or("");
                let x = xy.chars().next().unwrap_or('.');
                let y = xy.chars().nth(1).unwrap_or('.');

                match x {
                    'A' => summary.added += 1,
                    'D' => summary.deleted += 1,
                    'M' | 'R' | 'C' => summary.modified += 1,
                    _ => {}
                }
                match y {
                    'M' => summary.modified += 1,
                    'D' => summary.deleted += 1,
                    _ => {}
                }
            } else if line.starts_with("? ") {
                summary.untracked += 1;
            }
        }

        Ok(summary)
    }

    pub fn last_commit_date(&self, path: &Path) -> Result<String> {
        let path_str = path.to_string_lossy();
        self.run(&["-C", &path_str, "log", "-1", "--format=%cr"])
    }
}
