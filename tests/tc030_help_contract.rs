use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn tc030_no_args_shows_help_and_exits_with_code_2() {
    let output = cargo_bin_cmd!("aidle")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(2), "exit code must be 2 for missing subcommand");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Usage:"), "stderr must contain usage info");
    assert!(stderr.contains("init [dir]"), "stderr must contain init command info");
}

#[test]
fn tc030_help_flag_shows_help_and_exits_with_code_0() {
    for flag in ["-h", "--help"] {
        let output = cargo_bin_cmd!("aidle")
            .arg(flag)
            .output()
            .expect("failed to execute aidle");

        assert_eq!(output.status.code(), Some(0), "exit code must be 0 for help with {flag}");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Usage:"), "stdout must contain usage info with {flag}");
    }
}

#[test]
fn tc030_init_help_flag_shows_help_and_exits_with_code_0() {
    let output = cargo_bin_cmd!("aidle")
        .arg("init")
        .arg("--help")
        .output()
        .expect("failed to execute aidle");

    assert_eq!(output.status.code(), Some(0), "exit code must be 0 for init --help");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage:"), "stdout must contain usage info");
}
