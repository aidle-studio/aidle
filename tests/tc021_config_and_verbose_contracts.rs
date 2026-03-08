use assert_cmd::cargo::cargo_bin_cmd;
use std::fs;
use tempfile::tempdir;

#[test]
fn tc021_invalid_toml_returns_code_2() {
    let temp = tempdir().expect("failed to create temp dir");
    fs::write(temp.path().join("aidle.toml"), "not = [valid")
        .expect("failed to write invalid config");

    let output = cargo_bin_cmd!("aidle")
        .current_dir(temp.path())
        .arg("init")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(2), "exit code must be 2");
}

#[test]
fn tc021_project_output_in_config_sets_root() {
    let temp = tempdir().expect("failed to create temp dir");
    fs::write(
        temp.path().join("aidle.toml"),
        "[project]\noutput = \"from-config\"\n",
    )
    .expect("failed to write config");

    let output = cargo_bin_cmd!("aidle")
        .current_dir(temp.path())
        .arg("init")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(0), "exit code must be 0");
    assert!(
        temp.path().join("from-config/AGENTS.md").exists(),
        "AGENTS.md must be created under project.output"
    );
}

#[test]
fn tc021_project_name_in_config_is_used_as_fallback_root() {
    let temp = tempdir().expect("failed to create temp dir");
    fs::write(
        temp.path().join("aidle.toml"),
        "[project]\nname = \"named-root\"\n",
    )
    .expect("failed to write config");

    let output = cargo_bin_cmd!("aidle")
        .current_dir(temp.path())
        .arg("init")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(0), "exit code must be 0");
    assert!(
        temp.path().join("named-root/AGENTS.md").exists(),
        "AGENTS.md must be created under project.name fallback"
    );
}

#[test]
fn tc021_verbose_flag_emits_debug_line_to_stderr() {
    let temp = tempdir().expect("failed to create temp dir");
    let output = cargo_bin_cmd!("aidle")
        .current_dir(temp.path())
        .arg("init")
        .arg("--verbose")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(0), "exit code must be 0");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("[verbose]"),
        "stderr must contain verbose line: {stderr}"
    );
}

