use crate::helpers::*;

#[test]
fn test_move_worktree() {
    let (_tmp, repo) = setup_repo();
    bonsai_cmd(&repo).args(["init"]).output().unwrap();
    bonsai_cmd(&repo)
        .args(["add", "-c", "feature/move-test"])
        .output()
        .unwrap();

    let new_path = repo.join(".bonsai/moved-here");
    let output = bonsai_cmd(&repo)
        .args(["move", "feature/move-test", &new_path.to_string_lossy()])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "move failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(!repo.join(".bonsai/feature-move-test").exists());
    assert!(new_path.is_dir());
}
