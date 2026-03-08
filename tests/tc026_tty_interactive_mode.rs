use rexpect::session::spawn_command;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn tc026_tty_interactive_mode_prompts_and_applies_input() {
    let temp = tempdir().expect("failed to create temp directory");
    let root = temp.path();

    let aidle_bin = assert_cmd::cargo::cargo_bin!("aidle");
    let mut cmd = Command::new(aidle_bin);
    cmd.arg("init");
    cmd.current_dir(root);

    let mut p = spawn_command(cmd, Some(5000)).expect("failed to spawn command with pty");

    // Expect the first prompt for Project Name
    p.exp_string("Project Name").expect("expected Project Name prompt");
    // Send input "MyAwesomeProject" followed by Enter (\r)
    p.send_line("MyAwesomeProject").expect("failed to send project name");

    // Expect the second prompt for AI Adapters
    p.exp_string("Generate AI Adapters?").expect("expected AI Adapters prompt");
    // Send 'y' for yes
    p.send_line("y").expect("failed to send 'y'");

    // Wait for the command to finish
    p.exp_eof().expect("expected EOF");

    let claude = root.join("CLAUDE.md");

    // We expect that `--with-adapters` was effectively set to true 
    // because we answered 'y' to the second prompt!
    assert!(
        claude.exists(),
        "CLAUDE.md must exist because we answered 'y' to the adapter prompt"
    );
}
