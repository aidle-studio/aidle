use assert_cmd::cargo::cargo_bin_cmd;
use std::fs;
use tempfile::tempdir;

#[test]
fn tc012_config_execution_dry_run_is_applied_when_cli_flag_is_absent() {
    let temp = tempdir().expect("failed to create temp directory");
    let root = temp.path();

    fs::write(
        root.join("aidle.toml"),
        r#"[execution]
dry_run = true
"#,
    )
    .expect("failed to write aidle.toml");

    let output = cargo_bin_cmd!("aidle")
        .current_dir(root)
        .arg("init")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(0), "exit code must be 0");
    assert!(
        !root.join("AGENTS.md").exists(),
        "dry_run=true in config must prevent file creation"
    );
}
