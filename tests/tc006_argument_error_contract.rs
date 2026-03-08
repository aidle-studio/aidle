use assert_cmd::Command;

#[test]
fn tc006_argument_error_returns_code_2_and_shows_cause_and_action() {
    let output = Command::cargo_bin("aidle")
        .expect("failed to locate aidle binary")
        .arg("init")
        .arg("--unknown-option")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(2), "exit code must be 2");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("引数エラー"),
        "stderr must contain cause label: {stderr}"
    );
    assert!(
        stderr.contains("対処"),
        "stderr must contain action hint: {stderr}"
    );
}
