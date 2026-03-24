use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_update_command_dry_run() {
    let temp = tempdir().unwrap();
    let project_root = temp.path();

    // 1. Initialize normally
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("init")
        .arg("--non-interactive")
        .current_dir(project_root)
        .assert()
        .success();

    // 2. Modify a static file (should be replaced)
    let rules_path = project_root.join("docs/RULES.md");
    fs::write(&rules_path, "# RULES.md\nModified content").unwrap();

    // 3. Modify a dynamic file (remove a heading to force injection)
    let todo_path = project_root.join("docs/TODO.md");
    let todo_content = fs::read_to_string(&todo_path).unwrap();
    let new_todo = todo_content.replace("## 運用ルール", "## Removed Rules");
    fs::write(&todo_path, new_todo).unwrap();

    // 4. Run update --dry-run
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("update")
        .arg("--dry-run")
        .current_dir(project_root)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "*** DRY RUN: No files will be modified ***",
        ))
        .stdout(predicate::str::contains("[Replace] docs/RULES.md"))
        .stdout(predicate::str::contains("[Inject]  docs/TODO.md"));

    // 5. Verify files were NOT actually changed
    let actual_rules = fs::read_to_string(&rules_path).unwrap();
    assert_eq!(actual_rules, "# RULES.md\nModified content");
}

#[test]
fn test_update_command_applies_changes() {
    let temp = tempdir().unwrap();
    let project_root = temp.path();

    // 1. Initialize normally
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("init")
        .arg("--non-interactive")
        .current_dir(project_root)
        .assert()
        .success();

    // 2. Modify static and dynamic files
    let rules_path = project_root.join("docs/RULES.md");
    fs::write(&rules_path, "# RULES.md\nModified content").unwrap();

    let todo_path = project_root.join("docs/TODO.md");
    let todo_content = fs::read_to_string(&todo_path).unwrap();
    let new_todo = todo_content.replace("## 運用ルール", "## Removed Rules");
    fs::write(&todo_path, new_todo).unwrap();

    // 3. Run actual update
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("update")
        .current_dir(project_root)
        .assert()
        .success()
        .stdout(predicate::str::contains("[Replace] docs/RULES.md"))
        .stdout(predicate::str::contains("[Inject]  docs/TODO.md"));

    // 4. Verify static file was replaced (should be restored to original template)
    let actual_rules = fs::read_to_string(&rules_path).unwrap();
    assert!(actual_rules.contains("本プロジェクトの開発プロセスと完了条件"));

    // 5. Verify dynamic file received the injected section but kept local modifications
    let actual_todo = fs::read_to_string(&todo_path).unwrap();
    assert!(actual_todo.contains("## Removed Rules")); // Local modification should remain
    assert!(actual_todo.contains("## 運用ルール")); // Injected missing section should be added back
}
