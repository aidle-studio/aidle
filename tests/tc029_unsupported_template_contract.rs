use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn tc029_unsupported_template_fails_with_code_2_and_action_hint() {
    let output = cargo_bin_cmd!("aidle")
        .arg("init")
        .arg("--template")
        .arg("python-cli")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(2), "exit code must be 2");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Argument Error"),
        "stderr must contain cause label: {stderr}"
    );
    assert!(
        stderr.contains("Action"),
        "stderr must contain action hint: {stderr}"
    );
}

#[test]
fn tc029_rust_cli_is_no_longer_supported() {
    let output = cargo_bin_cmd!("aidle")
        .arg("init")
        .arg("--template")
        .arg("rust-cli")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(2), "exit code must be 2");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unsupported template `rust-cli`"),
        "stderr must contain cause: {stderr}"
    );
}
