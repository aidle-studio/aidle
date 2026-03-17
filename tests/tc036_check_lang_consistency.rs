use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_check_command_respects_lang_en() {
    let temp = tempdir().unwrap();
    let project_root = temp.path();

    // 1. Initialize with English
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("init")
        .arg("--lang")
        .arg("en")
        .arg("--non-interactive")
        .current_dir(project_root)
        .assert()
        .success();

    // 2. Modify RULES.md to remove an English section
    let rules_path = project_root.join("docs/RULES.md");
    let content = fs::read_to_string(&rules_path).unwrap();
    let new_content = content.replace("## 7. Process Evolution", "## Removed Section");
    fs::write(&rules_path, new_content).unwrap();

    // 3. Run check with --lang en - should detect English missing section
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("check")
        .arg("--lang")
        .arg("en")
        .current_dir(project_root)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Missing sections (concepts) detected.",
        ))
        .stdout(predicate::str::contains("7. process evolution"));
}

#[test]
fn test_check_command_fails_if_lang_mismatch() {
    let temp = tempdir().unwrap();
    let project_root = temp.path();

    // 1. Initialize with Japanese (default)
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("init")
        .arg("--non-interactive")
        .current_dir(project_root)
        .assert()
        .success();

    // 2. Run check with --lang en
    // English template has "RULES.md" headings like "1. Development Principles",
    // but Japanese local has "1. 開発原則". So it should report many missing concepts.
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("check")
        .arg("--lang")
        .arg("en")
        .current_dir(project_root)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Missing sections (concepts) detected.",
        ))
        .stdout(predicate::str::contains("1. development principles"));
}
