use assert_cmd::cargo::cargo_bin_cmd;
use tempfile::tempdir;

#[test]
fn tc025_stats_log_is_not_created_when_unspecified() {
    let temp = tempdir().expect("failed to create temp directory");
    let root = temp.path();
    let stats_path = root.join("stats.json");

    cargo_bin_cmd!("aidle")
        .current_dir(root)
        .arg("init")
        .assert()
        .success();

    assert!(
        !stats_path.exists(),
        "stats file must not be created by default"
    );
}
