use crate::helpers::*;

#[test]
fn test_lock_and_unlock() {
    let (_tmp, repo) = setup_repo();
    bonsai_cmd(&repo).args(["init"]).output().unwrap();
    bonsai_cmd(&repo)
        .args(["add", "-c", "feature/lock-test"])
        .output()
        .unwrap();

    // Lock
    let output = bonsai_cmd(&repo)
        .args(["lock", "feature/lock-test", "--reason", "testing"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "lock failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Unlock
    let output = bonsai_cmd(&repo)
        .args(["unlock", "feature/lock-test"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "unlock failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
