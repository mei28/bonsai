use crate::helpers::*;

#[test]
fn test_cd_main_worktree() {
    let (_tmp, repo) = setup_repo();
    bonsai_cmd(&repo).args(["init"]).output().unwrap();

    let output = bonsai_cmd(&repo).args(["cd", "@"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let path = stdout.trim();
    assert!(!path.is_empty());
}

#[test]
fn test_cd_named_worktree() {
    let (_tmp, repo) = setup_repo();
    bonsai_cmd(&repo).args(["init"]).output().unwrap();
    bonsai_cmd(&repo)
        .args(["add", "-c", "feature/cd-test"])
        .output()
        .unwrap();

    let output = bonsai_cmd(&repo)
        .args(["cd", "feature/cd-test"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.trim().contains("feature-cd-test"));
}

#[test]
fn test_cd_at_from_worktree_returns_main() {
    let (_tmp, repo) = setup_repo();
    bonsai_cmd(&repo).args(["init"]).output().unwrap();
    bonsai_cmd(&repo)
        .args(["add", "-c", "feature/at-test"])
        .output()
        .unwrap();

    // Get the worktree path
    let wt_output = bonsai_cmd(&repo)
        .args(["cd", "feature/at-test"])
        .output()
        .unwrap();
    let wt_path = String::from_utf8_lossy(&wt_output.stdout).trim().to_string();

    // Run `cd @` from inside the worktree — should return main repo, not worktree
    let output = bonsai_cmd(std::path::Path::new(&wt_path))
        .args(["cd", "@"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let main_repo = repo.canonicalize().unwrap();
    let result_path = std::path::PathBuf::from(&result).canonicalize().unwrap();
    assert_eq!(result_path, main_repo, "cd @ from worktree should return main repo path");
}

#[test]
fn test_cd_nonexistent_fails() {
    let (_tmp, repo) = setup_repo();
    bonsai_cmd(&repo).args(["init"]).output().unwrap();

    let output = bonsai_cmd(&repo)
        .args(["cd", "nonexistent"])
        .output()
        .unwrap();
    assert!(!output.status.success());
}
