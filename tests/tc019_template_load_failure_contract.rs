use assert_cmd::cargo::cargo_bin_cmd;
use tempfile::tempdir;

#[test]
fn tc019_template_load_failure_returns_code_3_with_cause_and_action() {
    let temp = tempdir().expect("failed to create temp directory");
    let output = cargo_bin_cmd!("aidle")
        .env("AIDLE_TEMPLATE_ROOT", temp.path())
        .arg("init")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(3), "exit code must be 3");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Template Load Error"),
        "stderr must contain cause label: {stderr}"
    );
    assert!(
        stderr.contains("Action"),
        "stderr must contain action hint: {stderr}"
    );
}
