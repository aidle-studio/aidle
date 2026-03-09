use assert_cmd::cargo::cargo_bin_cmd;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn tc023_without_adapters_option_does_not_create_gc2_files() {
    let temp = tempdir().expect("failed to create temp directory");
    let root = temp.path();

    cargo_bin_cmd!("aidle")
        .current_dir(root)
        .arg("init")
        .assert()
        .success();

    let forbidden = [
        ".github/copilot-instructions.md",
        ".github/instructions/general.instructions.md",
        "GEMINI.md",
        "CLAUDE.md",
    ];

    for rel in forbidden {
        assert!(
            !Path::new(root).join(rel).exists(),
            "adapter file should not be created by default: {rel}"
        );
    }
}
