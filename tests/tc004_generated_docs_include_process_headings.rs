use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn tc004_generated_docs_include_process_and_agreement_headings() {
    let temp = tempdir().expect("failed to create temp directory");
    let root = temp.path();

    Command::cargo_bin("aidle")
        .expect("failed to locate aidle binary")
        .current_dir(root)
        .arg("init")
        .assert()
        .success();

    let spec = fs::read_to_string(root.join("docs/SPEC.md")).expect("failed to read SPEC.md");
    let todo = fs::read_to_string(root.join("docs/TODO.md")).expect("failed to read TODO.md");

    assert!(
        spec.contains("## 1. 目的"),
        "SPEC.md must include process-related structure: {spec}"
    );
    assert!(
        todo.contains("合意ゲート"),
        "TODO.md must include agreement gate heading: {todo}"
    );
}
