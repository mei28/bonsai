use crate::helpers::*;

#[test]
fn test_status_all() {
    let (_tmp, repo) = setup_repo();
    bonsai_cmd(&repo).args(["init"]).output().unwrap();

    let output = bonsai_cmd(&repo).args(["status"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("main"));
}

#[test]
fn test_status_specific_worktree() {
    let (_tmp, repo) = setup_repo();
    bonsai_cmd(&repo).args(["init"]).output().unwrap();
    bonsai_cmd(&repo)
        .args(["add", "-c", "feature/status-test"])
        .output()
        .unwrap();

    let output = bonsai_cmd(&repo)
        .args(["status", "feature/status-test"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("feature/status-test"));
}
