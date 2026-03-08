use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn tc009_force_overwrite_failure_restores_preexisting_file() {
    let temp = tempdir().expect("failed to create temp directory");
    let root = temp.path();

    let original_agents = "# original agents\n";
    fs::write(root.join("AGENTS.md"), original_agents).expect("failed to seed AGENTS.md");
    fs::write(root.join("docs"), "not a directory").expect("failed to create blocking docs file");

    let output = Command::cargo_bin("aidle")
        .expect("failed to locate aidle binary")
        .current_dir(root)
        .arg("init")
        .arg("--force")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(3), "exit code must be 3");

    let restored = fs::read_to_string(root.join("AGENTS.md")).expect("failed to read AGENTS.md");
    assert_eq!(
        restored, original_agents,
        "pre-existing AGENTS.md must be restored on failure"
    );
}
