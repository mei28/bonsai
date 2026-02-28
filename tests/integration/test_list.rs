use crate::helpers::*;

#[test]
fn test_list_shows_worktrees() {
    let (_tmp, repo) = setup_repo();
    bonsai_cmd(&repo).args(["init"]).output().unwrap();
    bonsai_cmd(&repo)
        .args(["add", "-c", "feature/list-test"])
        .output()
        .unwrap();

    let output = bonsai_cmd(&repo).args(["list"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("main"));
    assert!(stdout.contains("feature/list-test"));
}

#[test]
fn test_list_porcelain() {
    let (_tmp, repo) = setup_repo();
    bonsai_cmd(&repo).args(["init"]).output().unwrap();
    bonsai_cmd(&repo)
        .args(["add", "-c", "feature/porcelain"])
        .output()
        .unwrap();

    let output = bonsai_cmd(&repo)
        .args(["list", "--porcelain"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("main\t"));
    assert!(stdout.contains("feature/porcelain\t"));
}

#[test]
fn test_list_names_only() {
    let (_tmp, repo) = setup_repo();
    bonsai_cmd(&repo).args(["init"]).output().unwrap();
    bonsai_cmd(&repo)
        .args(["add", "-c", "feature/names"])
        .output()
        .unwrap();

    let output = bonsai_cmd(&repo)
        .args(["list", "--names-only"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let names: Vec<&str> = stdout.trim().lines().collect();
    assert!(names.contains(&"main"));
    assert!(names.contains(&"feature/names"));
}
