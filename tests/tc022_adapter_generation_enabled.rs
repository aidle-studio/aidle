use assert_cmd::cargo::cargo_bin_cmd;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn tc022_with_adapters_enabled_creates_gc2_files() {
    let temp = tempdir().expect("failed to create temp directory");
    let root = temp.path();

    cargo_bin_cmd!("aidle")
        .current_dir(root)
        .arg("init")
        .arg("--with-adapters")
        .assert()
        .success();

    let required = [
        ".github/copilot-instructions.md",
        ".github/instructions/general.instructions.md",
        "GEMINI.md",
        "CLAUDE.md",
    ];

    for rel in required {
        assert!(
            Path::new(root).join(rel).exists(),
            "adapter file not found: {rel}"
        );
    }
}

