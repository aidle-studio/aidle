use assert_cmd::cargo::cargo_bin_cmd;
use std::fs;
use tempfile::tempdir;

#[test]
fn tc013_config_execution_force_overwrites_existing_file_without_cli_force() {
    let temp = tempdir().expect("failed to create temp directory");
    let root = temp.path();

    fs::write(root.join("AGENTS.md"), "# old agents\n").expect("failed to seed AGENTS.md");
    fs::write(
        root.join("aidle.toml"),
        r#"[execution]
force = true
"#,
    )
    .expect("failed to write aidle.toml");

    let output = cargo_bin_cmd!("aidle")
        .current_dir(root)
        .arg("init")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(0), "exit code must be 0");

    let agents = fs::read_to_string(root.join("AGENTS.md")).expect("failed to read AGENTS.md");
    assert!(
        agents.starts_with("# AGENTS.md\n"),
        "force=true in config must allow overwrite: {agents}"
    );
}
