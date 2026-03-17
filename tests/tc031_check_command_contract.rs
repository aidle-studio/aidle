use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_check_command_success_when_all_match() {
    let temp = tempdir().unwrap();
    let project_root = temp.path();

    // 1. Initialize a project first
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("init")
        .arg("--non-interactive")
        .current_dir(project_root)
        .assert()
        .success();

    // 2. Run check - should be up-to-date
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("check")
        .current_dir(project_root)
        .assert()
        .success()
        .stdout(predicate::str::contains("All documents are up-to-date"));
}

#[test]
fn test_check_command_detects_missing_section() {
    let temp = tempdir().unwrap();
    let project_root = temp.path();

    // 1. Initialize
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("init")
        .arg("--non-interactive")
        .current_dir(project_root)
        .assert()
        .success();

    // 2. Modify RULES.md to remove a specific section (e.g., "7. Process Evolution")
    let rules_path = project_root.join("docs/RULES.md");
    let content = fs::read_to_string(&rules_path).unwrap();
    // Remove the section "7. プロセスの進化" (Heading normalized to "7. プロセスの進化 (process evolution)")
    let new_content = content.replace("## 7. プロセスの進化 (Process Evolution)", "## Removed Section");
    fs::write(&rules_path, new_content).unwrap();

    // 3. Run check - should detect the missing section
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("check")
        .current_dir(project_root)
        .assert()
        .success()
        .stdout(predicate::str::contains("Missing sections (concepts) detected."))
        .stdout(predicate::str::contains("7. プロセスの進化 (process evolution)"));
}

#[test]
fn test_check_command_detects_missing_file() {
    let temp = tempdir().unwrap();
    let project_root = temp.path();

    // 1. Initialize
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("init")
        .arg("--non-interactive")
        .current_dir(project_root)
        .assert()
        .success();

    // 2. Delete RULES.md
    let rules_path = project_root.join("docs/RULES.md");
    fs::remove_file(rules_path).unwrap();

    // 3. Run check - should detect missing file
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("check")
        .current_dir(project_root)
        .assert()
        .success()
        .stdout(predicate::str::contains("File does not exist."))
        .stdout(predicate::str::contains("docs/RULES.md"));
}
