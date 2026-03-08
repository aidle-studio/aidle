use assert_cmd::cargo::cargo_bin_cmd;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn tc002_init_with_dir_creates_gc1_required_files_under_target_directory() {
    let temp = tempdir().expect("failed to create temp directory");
    let workspace = temp.path();
    let project_dir = workspace.join("my-project");

    cargo_bin_cmd!("aidle")
        .current_dir(workspace)
        .arg("init")
        .arg("my-project")
        .assert()
        .success();

    let required = [
        "AGENTS.md",
        "README.md",
        "docs/AGENT_CONTEXT.md",
        "docs/RULES.md",
        "docs/SPEC.md",
        "docs/TODO.md",
        "docs/TEST_PLAN.md",
        "docs/KNOWLEDGE.md",
    ];

    for rel in required {
        assert!(
            Path::new(&project_dir).join(rel).exists(),
            "required file not found: {rel}"
        );
    }
}
