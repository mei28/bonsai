use crate::helpers::*;

#[test]
fn test_rename_branch_and_worktree() {
    let (_tmp, repo) = setup_repo();
    bonsai_cmd(&repo).args(["init"]).output().unwrap();
    bonsai_cmd(&repo)
        .args(["add", "-c", "feature/old-name"])
        .output()
        .unwrap();

    let output = bonsai_cmd(&repo)
        .args(["rename", "feature/old-name", "feature/new-name"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "rename failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let branches = run_git(&repo, &["branch"]);
    assert!(!branches.contains("feature/old-name"));
    assert!(branches.contains("feature/new-name"));

    // Worktree dir should have been moved
    assert!(!repo.join(".bonsai/feature-old-name").exists());
    assert!(repo.join(".bonsai/feature-new-name").is_dir());
}
