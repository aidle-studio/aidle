use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_e2e_evolution_flow_with_custom_template() {
    let workspace = tempdir().unwrap();
    let project_root = workspace.path().join("my-project");
    let template_root = workspace.path().join("templates");
    let custom_template = template_root.join("evolution-v2");

    // 1. Setup a "v1" template (simulated by built-in default)
    // 2. Initialize project
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("init")
        .arg("my-project")
        .arg("--non-interactive")
        .current_dir(workspace.path())
        .assert()
        .success();

    // 3. Setup a "v2" template in filesystem
    fs::create_dir_all(&custom_template).unwrap();
    
    // 必須ファイルを全て作成（ダミー）
    let required_files = [
        "AGENTS.md", "ARCHITECTURE.md", "README.md",
        "docs/AGENT_CONTEXT.md", "docs/RULES.md", "docs/SPEC.md",
        "docs/TODO.md", "docs/TEST_PLAN.md", "docs/KNOWLEDGE.md",
        "docs/HARNESS.md", "docs/QUALITY_SCORE.md", "docs/RELIABILITY.md",
        "docs/SECURITY.md", "docs/PRODUCT_SENSE.md", "docs/DESIGN.md",
        "docs/PLANS.md", "docs/adr/index.md", "docs/adr/.gitkeep",
        "docs/design-docs/index.md", "docs/design-docs/core-beliefs.md",
        "docs/exec-plans/active/.gitkeep", "docs/exec-plans/completed/.gitkeep",
        "docs/exec-plans/tech-debt.md", "docs/product-specs/index.md",
        "docs/product-specs/.gitkeep", "docs/references/index.md",
        "docs/references/.gitkeep", "scripts/check_harness.sh",
    ];

    for rel_path in &required_files {
        let path = custom_template.join(rel_path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, "dummy content").unwrap();
    }

    // RULES.md だけ進化させる
    let rules_template_path = custom_template.join("docs/RULES.md");
    let current_rules_in_project = fs::read_to_string(project_root.join("docs/RULES.md")).unwrap();
    // 元の内容に新しいセクションを追記したものを「最新テンプレート」とする
    let evolved_rules = format!("{}\n## 9. 新しい進化の観点 (Evolution)\n詳細内容...", current_rules_in_project);
    fs::write(&rules_template_path, evolved_rules).unwrap();

    // 4. Run aidle check pointing to the "v2" template


    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("check")
        .arg("--template")
        .arg(custom_template.to_str().unwrap())
        .current_dir(&project_root)
        .assert()
        .success()
        .stdout(predicate::str::contains("Missing sections (concepts) detected."))
        .stdout(predicate::str::contains("9. 新しい進化の観点 (evolution)"));

    // 5. Simulate AI merge (manual append in test)
    let mut current_rules = fs::read_to_string(project_root.join("docs/RULES.md")).unwrap();
    current_rules.push_str("\n## 9. 新しい進化の観点 (Evolution)\n詳細内容...");
    fs::write(project_root.join("docs/RULES.md"), current_rules).unwrap();

    // 6. Verify check is now up-to-date
    let mut cmd = Command::cargo_bin("aidle").unwrap();
    cmd.arg("check")
        .arg("--template")
        .arg(custom_template.to_str().unwrap())
        .current_dir(&project_root)
        .assert()
        .success()
        .stdout(predicate::str::contains("All documents are up-to-date"));
}
