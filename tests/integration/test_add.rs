use crate::helpers::*;

#[test]
fn test_add_creates_worktree_with_new_branch() {
    let (_tmp, repo) = setup_repo();
    bonsai_cmd(&repo).args(["init"]).output().unwrap();

    let output = bonsai_cmd(&repo)
        .args(["add", "-c", "feature/test"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "add failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(repo.join(".bonsai/feature-test").is_dir());

    // Verify branch exists
    let branches = run_git(&repo, &["branch"]);
    assert!(branches.contains("feature/test"));
}

#[test]
fn test_add_existing_branch() {
    let (_tmp, repo) = setup_repo();
    bonsai_cmd(&repo).args(["init"]).output().unwrap();

    run_git(&repo, &["branch", "existing-branch"]);

    let output = bonsai_cmd(&repo)
        .args(["add", "existing-branch"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "add failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(repo.join(".bonsai/existing-branch").is_dir());
}

#[test]
fn test_add_fails_for_nonexistent_branch() {
    let (_tmp, repo) = setup_repo();
    bonsai_cmd(&repo).args(["init"]).output().unwrap();

    let output = bonsai_cmd(&repo)
        .args(["add", "nonexistent"])
        .output()
        .unwrap();
    assert!(!output.status.success());
}

#[test]
fn test_add_with_base_branch() {
    let (_tmp, repo) = setup_repo();
    bonsai_cmd(&repo).args(["init"]).output().unwrap();

    // Create a develop branch with a commit
    run_git(&repo, &["checkout", "-b", "develop"]);
    run_git(&repo, &["commit", "--allow-empty", "-m", "develop commit"]);
    run_git(&repo, &["checkout", "main"]);

    let output = bonsai_cmd(&repo)
        .args(["add", "-c", "feature/from-develop", "--base", "develop"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "add failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_add_duplicate_fails() {
    let (_tmp, repo) = setup_repo();
    bonsai_cmd(&repo).args(["init"]).output().unwrap();

    bonsai_cmd(&repo)
        .args(["add", "-c", "feature/dup"])
        .output()
        .unwrap();

    let output = bonsai_cmd(&repo)
        .args(["add", "-c", "feature/dup"])
        .output()
        .unwrap();
    assert!(!output.status.success());
}
