use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_check_command_i18n_heading_consistency() {
    let temp = tempdir().unwrap();
    let project_root = temp.path();

    // 1. Initialize
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("init")
        .arg("--non-interactive")
        .current_dir(project_root)
        .assert()
        .success();

    // 2. Modify KNOWLEDGE.md heading with slight variation (if any) or just verify it works
    let knowledge_path = project_root.join("docs/KNOWLEDGE.md");
    let content = fs::read_to_string(&knowledge_path).unwrap();

    // Japanese headings are included in the template. Let's make sure they are matched.
    assert!(content.contains("## 📚 蓄積された知見 (Knowledge Base)"));

    // 3. Run check - should be up-to-date even with Japanese characters and emojis
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("check")
        .current_dir(project_root)
        .assert()
        .success()
        .stdout(predicate::str::contains("All documents are up-to-date"));
}

#[test]
fn test_check_command_normalization_handles_spacing() {
    let temp = tempdir().unwrap();
    let project_root = temp.path();

    // 1. Initialize
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("init")
        .arg("--non-interactive")
        .current_dir(project_root)
        .assert()
        .success();

    // 2. Add extra spaces around a heading in RULES.md
    let rules_path = project_root.join("docs/RULES.md");
    let content = fs::read_to_string(&rules_path).unwrap();
    let new_content = content.replace(
        "## 1. 開発原則 (Harness-First)",
        "##    1. 開発原則 (Harness-First)    ",
    );
    fs::write(&rules_path, new_content).unwrap();

    // 3. Run check - should still be up-to-date due to normalization (trim)
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("check")
        .current_dir(project_root)
        .assert()
        .success()
        .stdout(predicate::str::contains("All documents are up-to-date"));
}
