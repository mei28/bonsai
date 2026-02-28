use crate::helpers::*;

#[test]
fn test_remove_worktree() {
    let (_tmp, repo) = setup_repo();
    bonsai_cmd(&repo).args(["init"]).output().unwrap();
    bonsai_cmd(&repo)
        .args(["add", "-c", "feature/remove-test"])
        .output()
        .unwrap();

    let output = bonsai_cmd(&repo)
        .args(["remove", "feature/remove-test"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "remove failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(!repo.join(".bonsai/feature-remove-test").exists());
}

#[test]
fn test_remove_with_branch() {
    let (_tmp, repo) = setup_repo();
    bonsai_cmd(&repo).args(["init"]).output().unwrap();
    bonsai_cmd(&repo)
        .args(["add", "-c", "feature/remove-branch"])
        .output()
        .unwrap();

    let output = bonsai_cmd(&repo)
        .args(["remove", "feature/remove-branch", "--with-branch"])
        .output()
        .unwrap();
    assert!(output.status.success());

    let branches = run_git(&repo, &["branch"]);
    assert!(!branches.contains("feature/remove-branch"));
}

#[test]
fn test_remove_nonexistent_fails() {
    let (_tmp, repo) = setup_repo();
    bonsai_cmd(&repo).args(["init"]).output().unwrap();

    let output = bonsai_cmd(&repo)
        .args(["remove", "nonexistent"])
        .output()
        .unwrap();
    assert!(!output.status.success());
}
