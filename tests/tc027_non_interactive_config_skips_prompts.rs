use rexpect::session::spawn_command;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn tc027_non_interactive_config_skips_prompts() {
    let temp = tempdir().expect("failed to create temp directory");
    let root = temp.path();

    fs::write(
        root.join("aidle.toml"),
        r#"[execution]
non_interactive = true
"#,
    )
    .expect("failed to write aidle.toml");

    let aidle_bin = assert_cmd::cargo::cargo_bin!("aidle");
    let mut cmd = Command::new(aidle_bin);
    cmd.arg("init");
    cmd.current_dir(root);

    let mut p = spawn_command(cmd, Some(5000)).expect("failed to spawn command with pty");

    // It should NOT ask for Project Name. We expect it to finish without prompting.
    p.exp_eof().expect("expected EOF without any prompts");

    // The command should succeed and create the basic files
    assert!(
        root.join("AGENTS.md").exists(),
        "AGENTS.md must exist (default execution)"
    );
    
    // AI Adapters should NOT be created because the prompt was skipped and the default is false.
    assert!(
        !root.join("CLAUDE.md").exists(),
        "CLAUDE.md must NOT exist because prompt was skipped (default is false)"
    );
}
