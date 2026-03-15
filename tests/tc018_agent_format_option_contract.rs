use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn tc018_unsupported_agent_format_fails_with_code_2() {
    let output = cargo_bin_cmd!("aidle")
        .arg("init")
        .arg("--agent-format")
        .arg("unknown-format")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(2), "exit code must be 2");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Argument Error"),
        "must contain cause label: {stderr}"
    );
    assert!(
        stderr.contains("Action"),
        "must contain action hint: {stderr}"
    );
}
