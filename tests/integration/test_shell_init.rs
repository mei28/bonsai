use crate::helpers::*;

#[test]
fn test_shell_init_bash() {
    let (_tmp, repo) = setup_repo();

    let output = bonsai_cmd(&repo)
        .args(["shell-init", "bash"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("bonsai()"));
    assert!(stdout.contains("bn()"));
}

#[test]
fn test_shell_init_zsh() {
    let (_tmp, repo) = setup_repo();

    let output = bonsai_cmd(&repo)
        .args(["shell-init", "zsh"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("bonsai()"));
}

#[test]
fn test_shell_init_fish() {
    let (_tmp, repo) = setup_repo();

    let output = bonsai_cmd(&repo)
        .args(["shell-init", "fish"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("function bonsai"));
}

#[test]
fn test_completion_bash() {
    let (_tmp, repo) = setup_repo();

    let output = bonsai_cmd(&repo)
        .args(["completion", "bash"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.is_empty());
}
