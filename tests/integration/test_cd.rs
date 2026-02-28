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
fn test_cd_nonexistent_fails() {
    let (_tmp, repo) = setup_repo();
    bonsai_cmd(&repo).args(["init"]).output().unwrap();

    let output = bonsai_cmd(&repo)
        .args(["cd", "nonexistent"])
        .output()
        .unwrap();
    assert!(!output.status.success());
}
