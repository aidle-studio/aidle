use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn tc005_rules_include_tdd_cycle_and_definition_of_done() {
    let temp = tempdir().expect("failed to create temp directory");
    let root = temp.path();

    Command::cargo_bin("aidle")
        .expect("failed to locate aidle binary")
        .current_dir(root)
        .arg("init")
        .assert()
        .success();

    let rules = fs::read_to_string(root.join("docs/RULES.md")).expect("failed to read RULES.md");

    assert!(
        rules.contains("Red -> Green -> Refactor"),
        "RULES.md must contain TDD cycle: {rules}"
    );
    assert!(
        rules.contains("Definition of Done"),
        "RULES.md must contain Definition of Done: {rules}"
    );
}
