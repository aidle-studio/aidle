use serde::Deserialize;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const REQUIRED_FILES: [(&str, &str); 8] = [
    (
        "AGENTS.md",
        "# AGENTS.md\n\n仕様検討は合意後に実装へ進みます。\n",
    ),
    (
        "README.md",
        "# README.md\n\n`aidle init` でAIDD向け初期構成を生成します。\n",
    ),
    (
        "docs/AGENT_CONTEXT.md",
        "# AGENT_CONTEXT.md\n\n## 現在フェーズ\n- 実装フェーズ\n",
    ),
    (
        "docs/RULES.md",
        "# RULES.md\n\n## TDDサイクル\n- Red -> Green -> Refactor\n\n## Definition of Done\n- 対応テストがGreenであること\n",
    ),
    (
        "docs/SPEC.md",
        "# SPEC.md\n\n## 1. 目的\n`aidle` はAIDDの初期セットアップを支援するCLIです。\n",
    ),
    (
        "docs/TODO.md",
        "# TODO.md\n\n## 合意ゲート\n- 提案 -> 合意 -> 反映\n",
    ),
    (
        "docs/TEST_PLAN.md",
        "# TEST_PLAN.md\n\n## トレーサビリティ\n- AC-* -> TC-*\n",
    ),
    (
        "docs/KNOWLEDGE.md",
        "# KNOWLEDGE.md\n\n## ADR\n- 設計判断を記録\n",
    ),
];

#[derive(Default)]
struct RunStats {
    created: usize,
    updated: usize,
    skipped: usize,
    errors: usize,
}

#[derive(Debug, Default, Deserialize)]
struct AidleConfig {
    project: Option<ProjectConfig>,
    execution: Option<ExecutionConfig>,
}

#[derive(Debug, Default, Deserialize)]
struct ProjectConfig {
    name: Option<String>,
    output: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct ExecutionConfig {
    force: Option<bool>,
    dry_run: Option<bool>,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err((code, message)) => {
            eprintln!("{message}");
            ExitCode::from(code)
        }
    }
}

fn run() -> Result<(), (u8, String)> {
    let cwd = env::current_dir().map_err(|e| io_error("カレントディレクトリ取得", &e))?;
    let config = load_config(&cwd)?;

    let mut args = env::args().skip(1);
    let command = args.next().ok_or_else(|| {
        (
            2,
            "引数エラー: サブコマンドが必要です。対処: `aidle init [dir]` を指定してください。".to_string(),
        )
    })?;

    if command != "init" {
        return Err((
            2,
            format!(
                "引数エラー: 未対応サブコマンド `{command}` です。対処: `aidle init [dir]` を使用してください。"
            ),
        ));
    }

    let mut cli_dry_run = false;
    let mut cli_force = false;
    let mut dir: Option<PathBuf> = None;

    for arg in args {
        if arg == "--dry-run" {
            cli_dry_run = true;
            continue;
        }
        if arg == "--force" {
            cli_force = true;
            continue;
        }

        if arg.starts_with('-') {
            return Err((
                2,
                format!(
                    "引数エラー: 未対応オプション `{arg}` です。対処: `aidle init [dir] [--dry-run] [--force]` を使用してください。"
                ),
            ));
        }

        if dir.is_some() {
            return Err((
                2,
                "引数エラー: `init` に指定できるディレクトリは1つまでです。対処: `aidle init [dir] [--dry-run] [--force]` で実行してください。"
                    .to_string(),
            ));
        }

        dir = Some(PathBuf::from(arg));
    }

    let root = match dir {
        Some(path) => path,
        None => resolve_root(&cwd, &config),
    };
    let force = cli_force || config.execution.as_ref().and_then(|e| e.force).unwrap_or(false);
    let dry_run =
        cli_dry_run || config.execution.as_ref().and_then(|e| e.dry_run).unwrap_or(false);

    let stats = create_required_files(&root, dry_run, force)?;
    print_summary(&stats);
    Ok(())
}

fn load_config(cwd: &Path) -> Result<AidleConfig, (u8, String)> {
    let path = cwd.join("aidle.toml");
    if !path.exists() {
        return Ok(AidleConfig::default());
    }

    let text = fs::read_to_string(&path).map_err(|e| {
        (
            2,
            format!(
                "設定エラー: `aidle.toml` の読み込みに失敗しました: {e}\n対処: ファイル権限と内容を確認してください。"
            ),
        )
    })?;

    toml::from_str::<AidleConfig>(&text).map_err(|e| {
        (
            2,
            format!(
                "設定エラー: `aidle.toml` のパースに失敗しました: {e}\n対処: TOML構文とキー名を確認してください。"
            ),
        )
    })
}

fn resolve_root(cwd: &Path, config: &AidleConfig) -> PathBuf {
    let from_config = config
        .project
        .as_ref()
        .and_then(|p| p.output.as_ref().or(p.name.as_ref()));

    match from_config {
        Some(path) => {
            let p = PathBuf::from(path);
            if p.is_absolute() { p } else { cwd.join(p) }
        }
        None => cwd.to_path_buf(),
    }
}

fn io_error(context: &str, e: &std::io::Error) -> (u8, String) {
    (
        3,
        format!(
            "I/Oエラー: {context} に失敗しました: {e}\n対処: パスの妥当性とアクセス権限を確認してください。"
        ),
    )
}

fn rollback_state(created_files: &[PathBuf], overwritten_files: &[(PathBuf, Vec<u8>)]) {
    for path in created_files.iter().rev() {
        let _ = fs::remove_file(path);
    }
    for (path, original) in overwritten_files.iter().rev() {
        let _ = fs::write(path, original);
    }
}

fn print_summary(stats: &RunStats) {
    println!(
        "created={} updated={} skipped={} errors={}",
        stats.created, stats.updated, stats.skipped, stats.errors
    );
}

fn create_required_files(root: &Path, dry_run: bool, force: bool) -> Result<RunStats, (u8, String)> {
    let mut stats = RunStats::default();

    if dry_run {
        return Ok(stats);
    }

    fs::create_dir_all(root).map_err(|e| io_error("出力ディレクトリ作成", &e))?;

    let mut created_files: Vec<PathBuf> = Vec::new();
    let mut overwritten_files: Vec<(PathBuf, Vec<u8>)> = Vec::new();

    for (rel, contents) in REQUIRED_FILES {
        let path = root.join(rel);
        if let Some(parent) = path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                let err = io_error(
                    &format!("親ディレクトリ作成 ({})", parent.display()),
                    &e,
                );
                rollback_state(&created_files, &overwritten_files);
                return Err(err);
            }
        }

        let mut file = if path.exists() {
            if !force {
                stats.skipped += 1;
                continue;
            }

            let original = match fs::read(&path) {
                Ok(bytes) => bytes,
                Err(e) => {
                    let err = io_error(&format!("既存ファイル読み込み ({})", path.display()), &e);
                    rollback_state(&created_files, &overwritten_files);
                    return Err(err);
                }
            };
            overwritten_files.push((path.clone(), original));

            match OpenOptions::new().write(true).truncate(true).open(&path) {
                Ok(f) => {
                    stats.updated += 1;
                    f
                }
                Err(e) => {
                    let err = io_error(&format!("ファイル上書き ({})", path.display()), &e);
                    rollback_state(&created_files, &overwritten_files);
                    return Err(err);
                }
            }
        } else {
            match OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(f) => {
                    created_files.push(path.clone());
                    stats.created += 1;
                    f
                }
                Err(e) => {
                    let err = io_error(&format!("ファイル作成 ({})", path.display()), &e);
                    rollback_state(&created_files, &overwritten_files);
                    return Err(err);
                }
            }
        };

        if let Err(e) = file.write_all(contents.as_bytes()) {
            let err = io_error(&format!("ファイル書き込み ({})", path.display()), &e);
            rollback_state(&created_files, &overwritten_files);
            return Err(err);
        }
    }

    Ok(stats)
}
