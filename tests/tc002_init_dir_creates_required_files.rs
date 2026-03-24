use assert_cmd::cargo::cargo_bin_cmd;
use std::path::Path;
use tempfile::tempdir;
use aidle::core::DEFAULT_TEMPLATE_FILES;

#[test]
fn tc002_init_with_dir_creates_gc1_required_files_under_target_directory() {
    let temp = tempdir().expect("failed to create temp directory");
    let workspace = temp.path();
    let project_dir = workspace.join("my-project");

    cargo_bin_cmd!("aidle")
        .current_dir(workspace)
        .arg("init")
        .arg("my-project")
        .assert()
        .success();

    for rel in DEFAULT_TEMPLATE_FILES {
        assert!(
            Path::new(&project_dir).join(rel).exists(),
            "required file not found: {rel}"
        );
    }
}
