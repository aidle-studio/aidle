use assert_cmd::cargo::cargo_bin_cmd;
use std::path::Path;
use tempfile::tempdir;
use aidle::core::DEFAULT_TEMPLATE_FILES;

#[test]
fn tc003_dry_run_does_not_create_gc1_required_files() {
    let temp = tempdir().expect("failed to create temp directory");
    let root = temp.path();

    cargo_bin_cmd!("aidle")
        .current_dir(root)
        .arg("init")
        .arg("--dry-run")
        .assert()
        .success();

    for rel in DEFAULT_TEMPLATE_FILES {
        assert!(
            !Path::new(root).join(rel).exists(),
            "file should not be created in dry-run: {rel}"
        );
    }

    assert!(
        !Path::new(root).join("--dry-run").exists(),
        "option token must not be treated as a directory name"
    );
}
