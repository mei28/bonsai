use crate::error::Result;
use crate::git::runner::GitRunner;

impl GitRunner {
    pub fn branch_delete(&self, name: &str, force: bool) -> Result<()> {
        let flag = if force { "-D" } else { "-d" };
        self.run(&["branch", flag, name])?;
        Ok(())
    }

    pub fn branch_rename(&self, old: &str, new: &str) -> Result<()> {
        self.run(&["branch", "-m", old, new])?;
        Ok(())
    }

    pub fn branch_exists(&self, name: &str) -> Result<bool> {
        match self.run(&["rev-parse", "--verify", &format!("refs/heads/{name}")]) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn branch_is_merged(&self, branch: &str, base: &str) -> Result<bool> {
        let output = self.run(&["branch", "--merged", base])?;
        Ok(output
            .lines()
            .any(|line| line.trim().trim_start_matches("* ") == branch))
    }

    pub fn current_branch(&self) -> Result<String> {
        self.run(&["rev-parse", "--abbrev-ref", "HEAD"])
    }
}
