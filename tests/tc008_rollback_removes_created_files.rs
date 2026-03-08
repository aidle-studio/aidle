use assert_cmd::cargo::cargo_bin_cmd;
use std::fs;
use tempfile::tempdir;

#[test]
fn tc008_on_partial_failure_newly_created_files_are_rolled_back() {
    let temp = tempdir().expect("failed to create temp directory");
    let root = temp.path();

    let blocking_file = root.join("docs");
    fs::write(&blocking_file, "not a directory").expect("failed to create blocking file");

    let output = cargo_bin_cmd!("aidle")
        .current_dir(root)
        .arg("init")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(3), "exit code must be 3");

    let should_be_rolled_back = ["AGENTS.md", "README.md"];

    for rel in should_be_rolled_back {
        assert!(
            !root.join(rel).exists(),
            "newly created file should be rolled back: {rel}"
        );
    }

    assert!(blocking_file.is_file(), "pre-existing blocking file must remain");
}
