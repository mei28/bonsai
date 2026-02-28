use std::path::{Path, PathBuf};
use std::process::Command;

#[allow(deprecated)]
use assert_cmd::cargo::cargo_bin;
use tempfile::TempDir;

/// Create a temporary git repository with an initial commit.
pub fn setup_repo() -> (TempDir, PathBuf) {
    let tmp = TempDir::new().expect("failed to create temp dir");
    let repo = tmp.path().to_path_buf();

    run_git(&repo, &["init", "-b", "main"]);
    run_git(&repo, &["config", "user.email", "test@test.com"]);
    run_git(&repo, &["config", "user.name", "Test"]);
    run_git(&repo, &["commit", "--allow-empty", "-m", "initial"]);

    (tmp, repo)
}

/// Run a git command in the given directory.
pub fn run_git(dir: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .expect("failed to run git");

    if !output.status.success() {
        panic!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

/// Create a bonsai Command targeting the test repo.
pub fn bonsai_cmd(repo: &Path) -> Command {
    let mut cmd = Command::new(cargo_bin!("bonsai"));
    cmd.current_dir(repo);
    cmd.env("NO_COLOR", "1");
    cmd
}
