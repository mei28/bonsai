use crate::helpers::*;

#[test]
fn test_init_creates_config_and_dir() {
    let (_tmp, repo) = setup_repo();

    let output = bonsai_cmd(&repo).args(["init"]).output().unwrap();
    assert!(
        output.status.success(),
        "init failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(repo.join(".bonsai").is_dir());
    assert!(repo.join(".bonsai.toml").exists());

    let gitignore = std::fs::read_to_string(repo.join(".gitignore")).unwrap();
    assert!(gitignore.contains(".bonsai/"));
}

#[test]
fn test_init_fails_if_already_initialized() {
    let (_tmp, repo) = setup_repo();

    bonsai_cmd(&repo).args(["init"]).output().unwrap();

    let output = bonsai_cmd(&repo).args(["init"]).output().unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("already initialized"));
}

#[test]
fn test_init_force_reinitializes() {
    let (_tmp, repo) = setup_repo();

    bonsai_cmd(&repo).args(["init"]).output().unwrap();

    let output = bonsai_cmd(&repo)
        .args(["init", "--force"])
        .output()
        .unwrap();
    assert!(output.status.success());
}

#[test]
fn test_init_preserves_existing_gitignore() {
    let (_tmp, repo) = setup_repo();

    std::fs::write(repo.join(".gitignore"), "node_modules/\n").unwrap();

    bonsai_cmd(&repo).args(["init"]).output().unwrap();

    let gitignore = std::fs::read_to_string(repo.join(".gitignore")).unwrap();
    assert!(gitignore.contains("node_modules/"));
    assert!(gitignore.contains(".bonsai/"));
}

#[test]
fn test_init_skips_duplicate_gitignore_entry() {
    let (_tmp, repo) = setup_repo();

    std::fs::write(repo.join(".gitignore"), ".bonsai/\n").unwrap();

    bonsai_cmd(&repo).args(["init"]).output().unwrap();

    let gitignore = std::fs::read_to_string(repo.join(".gitignore")).unwrap();
    let count = gitignore.matches(".bonsai/").count();
    assert_eq!(count, 1);
}
