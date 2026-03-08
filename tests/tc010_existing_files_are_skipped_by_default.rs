use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn tc010_existing_files_are_not_overwritten_without_force() {
    let temp = tempdir().expect("failed to create temp directory");
    let root = temp.path();

    let original = "# keep me\n";
    fs::write(root.join("AGENTS.md"), original).expect("failed to seed AGENTS.md");

    let output = Command::cargo_bin("aidle")
        .expect("failed to locate aidle binary")
        .current_dir(root)
        .arg("init")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(0), "exit code must be 0");

    let agents = fs::read_to_string(root.join("AGENTS.md")).expect("failed to read AGENTS.md");
    assert_eq!(
        agents, original,
        "AGENTS.md must remain unchanged without --force"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("skipped"),
        "stdout must report skipped files: {stdout}"
    );
}
