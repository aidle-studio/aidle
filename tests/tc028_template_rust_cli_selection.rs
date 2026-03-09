use assert_cmd::cargo::cargo_bin_cmd;
use std::fs;
use tempfile::tempdir;

#[test]
fn tc028_template_rust_cli_can_be_selected_by_cli_option() {
    let temp = tempdir().expect("failed to create temp directory");
    let root = temp.path();

    cargo_bin_cmd!("aidle")
        .current_dir(root)
        .arg("init")
        .arg("--template")
        .arg("rust-cli")
        .assert()
        .success();

    let readme = fs::read_to_string(root.join("README.md")).expect("failed to read README.md");
    assert!(
        readme.contains("Rust CLI Template"),
        "README must be generated from rust-cli template"
    );
}

#[test]
fn tc028_template_rust_cli_can_be_selected_by_config() {
    let temp = tempdir().expect("failed to create temp directory");
    let root = temp.path();
    fs::write(root.join("aidle.toml"), "[template]\nname = \"rust-cli\"\n")
        .expect("failed to write aidle.toml");

    cargo_bin_cmd!("aidle")
        .current_dir(root)
        .arg("init")
        .assert()
        .success();

    let readme = fs::read_to_string(root.join("README.md")).expect("failed to read README.md");
    assert!(
        readme.contains("Rust CLI Template"),
        "README must be generated from rust-cli template"
    );
}
