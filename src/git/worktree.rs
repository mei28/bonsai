use std::path::PathBuf;

use crate::error::Result;
use crate::git::runner::GitRunner;

#[derive(Debug, Clone)]
pub struct WorktreeInfo {
    pub path: PathBuf,
    pub head: String,
    pub branch: Option<String>,
    pub is_bare: bool,
    pub is_main: bool,
    pub is_locked: bool,
    pub lock_reason: Option<String>,
    pub is_prunable: bool,
}

pub struct AddOptions {
    pub create_branch: bool,
    pub base: Option<String>,
    pub detach: bool,
}

impl GitRunner {
    pub fn worktree_add(
        &self,
        path: &std::path::Path,
        branch: &str,
        opts: &AddOptions,
    ) -> Result<()> {
        let path_str = path.to_string_lossy();
        let mut args: Vec<&str> = vec!["worktree", "add"];

        if opts.create_branch {
            args.push("-b");
            args.push(branch);
            args.push(&path_str);
            if let Some(ref base) = opts.base {
                args.push(base);
            }
        } else if opts.detach {
            args.push("--detach");
            args.push(&path_str);
            args.push(branch);
        } else {
            args.push(&path_str);
            args.push(branch);
        }

        self.run(&args)?;
        Ok(())
    }

    pub fn worktree_remove(&self, path: &std::path::Path, force: bool) -> Result<()> {
        let path_str = path.to_string_lossy();
        let mut args = vec!["worktree", "remove"];
        if force {
            args.push("--force");
        }
        args.push(&path_str);

        self.run(&args)?;
        Ok(())
    }

    pub fn worktree_list(&self) -> Result<Vec<WorktreeInfo>> {
        let output = self.run(&["worktree", "list", "--porcelain"])?;
        Ok(parse_worktree_porcelain(&output))
    }

    pub fn worktree_move(
        &self,
        worktree: &std::path::Path,
        new_path: &std::path::Path,
    ) -> Result<()> {
        let old = worktree.to_string_lossy();
        let new = new_path.to_string_lossy();
        self.run(&["worktree", "move", &old, &new])?;
        Ok(())
    }

    pub fn worktree_prune(&self) -> Result<()> {
        self.run(&["worktree", "prune"])?;
        Ok(())
    }

    pub fn worktree_lock(&self, worktree: &std::path::Path, reason: Option<&str>) -> Result<()> {
        let path_str = worktree.to_string_lossy();
        let mut args = vec!["worktree", "lock"];
        if let Some(r) = reason {
            args.push("--reason");
            args.push(r);
        }
        args.push(&path_str);
        self.run(&args)?;
        Ok(())
    }

    pub fn worktree_unlock(&self, worktree: &std::path::Path) -> Result<()> {
        let path_str = worktree.to_string_lossy();
        self.run(&["worktree", "unlock", &path_str])?;
        Ok(())
    }
}

fn parse_worktree_porcelain(output: &str) -> Vec<WorktreeInfo> {
    let mut worktrees = Vec::new();
    let mut is_first = true;

    for block in output.split("\n\n") {
        let block = block.trim();
        if block.is_empty() {
            continue;
        }

        let mut path = PathBuf::new();
        let mut head = String::new();
        let mut branch = None;
        let mut is_bare = false;
        let mut is_locked = false;
        let mut lock_reason = None;
        let mut is_prunable = false;

        for line in block.lines() {
            if let Some(p) = line.strip_prefix("worktree ") {
                path = PathBuf::from(p);
            } else if let Some(h) = line.strip_prefix("HEAD ") {
                head = h.to_string();
            } else if let Some(b) = line.strip_prefix("branch ") {
                branch = Some(b.strip_prefix("refs/heads/").unwrap_or(b).to_string());
            } else if line == "bare" {
                is_bare = true;
            } else if line == "locked" {
                is_locked = true;
            } else if let Some(reason) = line.strip_prefix("locked ") {
                is_locked = true;
                lock_reason = Some(reason.to_string());
            } else if line == "prunable" {
                is_prunable = true;
            }
        }

        let is_main = is_first;
        is_first = false;

        worktrees.push(WorktreeInfo {
            path,
            head,
            branch,
            is_bare,
            is_main,
            is_locked,
            lock_reason,
            is_prunable,
        });
    }

    worktrees
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_worktree_porcelain() {
        let input = "\
worktree /home/user/project
HEAD abc1234567890123456789012345678901234abcd
branch refs/heads/main

worktree /home/user/project/.bonsai/feature-auth
HEAD def4567890123456789012345678901234567890
branch refs/heads/feature/auth

worktree /home/user/project/.bonsai/detached
HEAD 1234567890123456789012345678901234567890
detached
";
        let result = parse_worktree_porcelain(input);
        assert_eq!(result.len(), 3);

        assert_eq!(result[0].branch.as_deref(), Some("main"));
        assert!(result[0].is_main);

        assert_eq!(result[1].branch.as_deref(), Some("feature/auth"));
        assert!(!result[1].is_main);

        assert_eq!(result[2].branch, None);
        assert!(!result[2].is_main);
    }

    #[test]
    fn test_parse_worktree_locked() {
        let input = "\
worktree /home/user/project
HEAD abc1234567890123456789012345678901234abcd
branch refs/heads/main

worktree /home/user/project/.bonsai/locked-wt
HEAD def4567890123456789012345678901234567890
branch refs/heads/locked-branch
locked reason for locking
";
        let result = parse_worktree_porcelain(input);
        assert_eq!(result.len(), 2);
        assert!(result[1].is_locked);
        assert_eq!(result[1].lock_reason.as_deref(), Some("reason for locking"));
    }
}
