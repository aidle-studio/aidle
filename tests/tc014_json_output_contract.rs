use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn tc014_json_output_includes_required_counters() {
    let output = cargo_bin_cmd!("aidle")
        .arg("init")
        .arg("--dry-run")
        .arg("--json")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(0), "exit code must be 0");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"created\""), "must include created: {stdout}");
    assert!(stdout.contains("\"updated\""), "must include updated: {stdout}");
    assert!(stdout.contains("\"skipped\""), "must include skipped: {stdout}");
    assert!(stdout.contains("\"errors\""), "must include errors: {stdout}");
}
