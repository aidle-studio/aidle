use assert_cmd::cargo::cargo_bin_cmd;
use std::fs;
use tempfile::tempdir;

#[test]
fn tc004_generated_docs_include_process_and_agreement_headings() {
    let temp = tempdir().expect("failed to create temp directory");
    let root = temp.path();

    cargo_bin_cmd!("aidle")
        .current_dir(root)
        .arg("init")
        .assert()
        .success();

    let spec = fs::read_to_string(root.join("docs/SPEC.md")).expect("failed to read SPEC.md");
    let todo = fs::read_to_string(root.join("docs/TODO.md")).expect("failed to read TODO.md");
    let rules = fs::read_to_string(root.join("docs/RULES.md")).expect("failed to read RULES.md");
    let test_plan =
        fs::read_to_string(root.join("docs/TEST_PLAN.md")).expect("failed to read TEST_PLAN.md");

    assert!(
        spec.contains("## 1. 目的"),
        "SPEC.md must include process-related structure: {spec}"
    );
    assert!(
        spec.contains("## 5. 受け入れ基準"),
        "SPEC.md must include acceptance criteria section: {spec}"
    );
    assert!(
        todo.contains("Agreement Gate"),
        "TODO.md must include agreement gate keyword: {todo}"
    );
    assert!(
        rules.contains("Agreement Gate"),
        "RULES.md must mention agreement gate: {rules}"
    );
    assert!(
        rules.contains("TDD"),
        "RULES.md must contain TDD: {rules}"
    );
    assert!(
        test_plan.contains("トレーサビリティ"),
        "TEST_PLAN.md must contain traceability matrix: {test_plan}"
    );
}
