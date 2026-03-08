use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn tc011_force_overwrites_existing_file_and_reports_updated() {
    let temp = tempdir().expect("failed to create temp directory");
    let root = temp.path();

    fs::write(root.join("AGENTS.md"), "# old agents\n").expect("failed to seed AGENTS.md");

    let output = Command::cargo_bin("aidle")
        .expect("failed to locate aidle binary")
        .current_dir(root)
        .arg("init")
        .arg("--force")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(0), "exit code must be 0");

    let agents = fs::read_to_string(root.join("AGENTS.md")).expect("failed to read AGENTS.md");
    assert_eq!(agents, "# AGENTS.md\n", "AGENTS.md must be overwritten with template content");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("updated="),
        "stdout must include updated count: {stdout}"
    );
}
