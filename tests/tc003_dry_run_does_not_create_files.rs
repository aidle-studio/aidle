use assert_cmd::Command;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn tc003_dry_run_does_not_create_gc1_required_files() {
    let temp = tempdir().expect("failed to create temp directory");
    let root = temp.path();

    Command::cargo_bin("aidle")
        .expect("failed to locate aidle binary")
        .current_dir(root)
        .arg("init")
        .arg("--dry-run")
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
            !Path::new(root).join(rel).exists(),
            "file should not be created in dry-run: {rel}"
        );
    }

    assert!(
        !Path::new(root).join("--dry-run").exists(),
        "option token must not be treated as a directory name"
    );
}
