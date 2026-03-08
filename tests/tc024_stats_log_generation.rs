use assert_cmd::cargo::cargo_bin_cmd;
use std::fs;
use tempfile::tempdir;

#[test]
fn tc024_stats_log_is_saved_when_stats_out_is_specified() {
    let temp = tempdir().expect("failed to create temp directory");
    let root = temp.path();
    let stats_path = root.join("out/stats.json");

    cargo_bin_cmd!("aidle")
        .current_dir(root)
        .arg("init")
        .arg("--stats-out")
        .arg(&stats_path)
        .assert()
        .success();

    assert!(stats_path.exists(), "stats file must be created");
    let content = fs::read_to_string(&stats_path).expect("failed to read stats file");
    assert!(
        content.contains("\"duration_ms\""),
        "stats must include duration_ms: {content}"
    );
    assert!(content.contains("\"created\""), "stats must include created: {content}");
    assert!(content.contains("\"root\""), "stats must include root: {content}");
}

#[test]
fn tc024_stats_log_is_saved_when_config_output_is_specified() {
    let temp = tempdir().expect("failed to create temp directory");
    let root = temp.path();
    let stats_path = root.join("from-config/stats.json");

    fs::write(
        root.join("aidle.toml"),
        "[stats]\noutput = \"from-config/stats.json\"\n",
    )
    .expect("failed to write config");

    cargo_bin_cmd!("aidle")
        .current_dir(root)
        .arg("init")
        .assert()
        .success();

    assert!(stats_path.exists(), "stats file must be created from config");
}
