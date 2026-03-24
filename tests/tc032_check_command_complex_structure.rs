use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_check_command_detects_multiple_missing_concepts() {
    let temp = tempdir().unwrap();
    let project_root = temp.path();

    // 1. Initialize
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("init")
        .arg("--non-interactive")
        .current_dir(project_root)
        .assert()
        .success();

    // 2. Remove multiple sections across different files
    let rules_path = project_root.join("docs/RULES.md");
    let audit_path = project_root.join("docs/AUDIT.md");

    let rules_content = fs::read_to_string(&rules_path).unwrap();
    let new_rules = rules_content
        .replace(
            "## 2. Standard Development Process and Navigation",
            "## Removed Rules",
        )
        .replace("## 2. 標準開発プロセスと導線", "## Removed Rules");
    fs::write(&rules_path, new_rules).unwrap();

    let audit_content = fs::read_to_string(&audit_path).unwrap();
    let new_audit = audit_content
        .replace("## 5. Security", "## Removed Security")
        .replace("## 5. セキュリティ (Security)", "## Removed Security");
    fs::write(&audit_path, new_audit).unwrap();

    // 3. Run check - should detect all missing sections
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("check")
        .current_dir(project_root)
        .assert()
        .success()
        .stdout(predicate::str::contains("docs/RULES.md"))
        .stdout(
            predicate::str::contains("2. standard development process and navigation")
                .or(predicate::str::contains("2. 標準開発プロセスと導線")),
        )
        .stdout(predicate::str::contains("docs/AUDIT.md"))
        .stdout(
            predicate::str::contains("5. security")
                .or(predicate::str::contains("5. セキュリティ (security)")),
        );
}

#[test]
fn test_check_command_heading_order_independence() {
    let temp = tempdir().unwrap();
    let project_root = temp.path();

    // 1. Initialize
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("init")
        .arg("--non-interactive")
        .current_dir(project_root)
        .assert()
        .success();

    // 2. Swap sections in RULES.md
    let rules_path = project_root.join("docs/RULES.md");
    let rules_content = fs::read_to_string(&rules_path).unwrap();

    // Simple swap of Section 1 and Section 2 (simplified approach for test)
    let parts: Vec<&str> = rules_content.split("##").collect();
    if parts.len() > 3 {
        let mut reordered = parts[0].to_string();
        reordered.push_str("##");
        reordered.push_str(parts[2]);
        reordered.push_str("##");
        reordered.push_str(parts[1]);
        for part in &parts[3..] {
            reordered.push_str("##");
            reordered.push_str(part);
        }
        fs::write(&rules_path, reordered).unwrap();
    }

    // 3. Run check - should still be up-to-date
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("check")
        .current_dir(project_root)
        .assert()
        .success()
        .stdout(predicate::str::contains("All documents are up-to-date"));
}
