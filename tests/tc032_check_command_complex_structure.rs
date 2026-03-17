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
    let security_path = project_root.join("docs/SECURITY.md");

    let rules_content = fs::read_to_string(&rules_path).unwrap();
    let new_rules = rules_content.replace(
        "## 7. プロセスの進化 (Process Evolution)",
        "## Removed Rules",
    );
    fs::write(&rules_path, new_rules).unwrap();

    let security_content = fs::read_to_string(&security_path).unwrap();
    let new_security = security_content.replace(
        "## 🛡️ セキュアコーディング原則 (Secure by Design)",
        "## Removed Security",
    );
    fs::write(&security_path, new_security).unwrap();

    // 3. Run check - should detect all missing sections
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("check")
        .current_dir(project_root)
        .assert()
        .success()
        .stdout(predicate::str::contains("docs/RULES.md"))
        .stdout(predicate::str::contains(
            "7. プロセスの進化 (process evolution)",
        ))
        .stdout(predicate::str::contains("docs/SECURITY.md"))
        .stdout(predicate::str::contains(
            "セキュアコーディング原則 (secure by design)",
        ));
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
