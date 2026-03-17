use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_init_lang_en_creates_english_files() {
    let temp = tempdir().unwrap();
    let project_root = temp.path();

    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("init")
        .arg("--lang")
        .arg("en")
        .arg("--non-interactive")
        .current_dir(project_root)
        .assert()
        .success();

    // Verify RULES.md content is English
    let rules_path = project_root.join("docs/RULES.md");
    let content = fs::read_to_string(rules_path).expect("RULES.md should exist");
    if !content.contains("This document defines the development process") {
        eprintln!("Actual Content:\n{}", content);
    }
    assert!(content.contains("This document defines the development process"));
    assert!(!content.contains("本ドキュメントは、プロジェクトの開発プロセス"));
}

#[test]
fn test_init_lang_ja_is_default() {
    let temp = tempdir().unwrap();
    let project_root = temp.path();

    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("init")
        .arg("--non-interactive")
        .current_dir(project_root)
        .assert()
        .success();

    // Verify RULES.md content is Japanese
    let rules_path = project_root.join("docs/RULES.md");
    let content = fs::read_to_string(rules_path).expect("RULES.md should exist");
    assert!(content.contains("本プロジェクトの開発プロセスと完了条件"));
}
