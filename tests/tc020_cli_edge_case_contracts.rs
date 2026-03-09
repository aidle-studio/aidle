use assert_cmd::cargo::cargo_bin_cmd;
use tempfile::tempdir;

#[test]
fn tc020_missing_subcommand_returns_code_2() {
    let output = cargo_bin_cmd!("aidle")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(2), "exit code must be 2");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("引数エラー"),
        "stderr must contain error label: {stderr}"
    );
    assert!(
        stderr.contains("対処"),
        "stderr must contain action hint: {stderr}"
    );
}

#[test]
fn tc020_unknown_subcommand_returns_code_2() {
    let output = cargo_bin_cmd!("aidle")
        .arg("plan")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(2), "exit code must be 2");
}

#[test]
fn tc020_missing_option_values_return_code_2() {
    for args in [
        vec!["init", "--output"],
        vec!["init", "--template"],
        vec!["init", "--agent-format"],
    ] {
        let output = cargo_bin_cmd!("aidle")
            .args(&args)
            .output()
            .expect("failed to execute aidle");
        assert_eq!(
            output.status.code(),
            Some(2),
            "exit code must be 2 for {args:?}"
        );
    }
}

#[test]
fn tc020_multiple_dirs_returns_code_2() {
    let output = cargo_bin_cmd!("aidle")
        .arg("init")
        .arg("a")
        .arg("b")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(2), "exit code must be 2");
}

#[test]
fn tc020_dir_and_output_together_returns_code_2() {
    let temp = tempdir().expect("failed to create temp directory");
    let output = cargo_bin_cmd!("aidle")
        .current_dir(temp.path())
        .arg("init")
        .arg("dir")
        .arg("--output")
        .arg("out")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(2), "exit code must be 2");
}
