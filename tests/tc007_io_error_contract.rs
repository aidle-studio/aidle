use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn tc007_io_error_returns_code_3_and_shows_cause_and_action() {
    let temp = tempdir().expect("failed to create temp directory");
    let root = temp.path();

    let blocked = root.join("blocked");
    fs::write(&blocked, "not a directory").expect("failed to create blocker file");

    let output = Command::cargo_bin("aidle")
        .expect("failed to locate aidle binary")
        .current_dir(root)
        .arg("init")
        .arg("blocked/child")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(3), "exit code must be 3");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("I/Oエラー"),
        "stderr must contain cause label: {stderr}"
    );
    assert!(
        stderr.contains("対処"),
        "stderr must contain action hint: {stderr}"
    );
}
