use assert_cmd::cargo::cargo_bin_cmd;
use std::fs;
use tempfile::tempdir;

#[test]
fn tc019_incomplete_template_returns_code_3_with_cause_and_action() {
    let temp = tempdir().expect("failed to create temp directory");
    let template_root = temp.path().join("templates");
    let my_template = template_root.join("my-incomplete-template");
    fs::create_dir_all(&my_template).expect("failed to create template dir");

    // Create only one of the required files, leaving others missing
    fs::write(my_template.join("AGENTS.md"), "test").expect("failed to write AGENTS.md");

    let output = cargo_bin_cmd!("aidle")
        .env("AIDLE_TEMPLATE_ROOT", &template_root)
        .arg("init")
        .arg("--template")
        .arg("my-incomplete-template")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(
        output.status.code(),
        Some(3),
        "exit code must be 3 for incomplete template"
    );

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
