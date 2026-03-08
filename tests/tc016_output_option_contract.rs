use assert_cmd::cargo::cargo_bin_cmd;
use tempfile::tempdir;

#[test]
fn tc016_output_option_sets_generation_root() {
    let temp = tempdir().expect("failed to create temp dir");
    let root = temp.path();
    let out = root.join("out-dir");

    let output = cargo_bin_cmd!("aidle")
        .current_dir(root)
        .arg("init")
        .arg("--output")
        .arg(&out)
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(0), "exit code must be 0");
    assert!(out.join("AGENTS.md").exists(), "AGENTS.md must be created under --output path");
}
