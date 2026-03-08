use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn tc015_non_interactive_flag_is_reflected_in_json_output() {
    let output = cargo_bin_cmd!("aidle")
        .arg("init")
        .arg("--dry-run")
        .arg("--json")
        .arg("--non-interactive")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(0), "exit code must be 0");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("\"non_interactive\":true"),
        "json output must reflect non_interactive=true: {stdout}"
    );
}
