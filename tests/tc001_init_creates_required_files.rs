use assert_cmd::cargo::cargo_bin_cmd;
use std::path::Path;
use tempfile::tempdir;
use aidle::core::DEFAULT_TEMPLATE_FILES;

#[test]
fn tc001_init_creates_gc1_required_files_in_current_directory() {
    let temp = tempdir().expect("failed to create temp directory");
    let root = temp.path();

    cargo_bin_cmd!("aidle")
        .current_dir(root)
        .arg("init")
        .assert()
        .success();

    for rel in DEFAULT_TEMPLATE_FILES {
        assert!(
            Path::new(root).join(rel).exists(),
            "required file not found: {rel}"
        );
    }
}
