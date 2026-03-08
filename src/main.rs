use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::IsTerminal;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const DEFAULT_TEMPLATE_FILES: [&str; 8] = [
    "AGENTS.md",
    "README.md",
    "docs/AGENT_CONTEXT.md",
    "docs/RULES.md",
    "docs/SPEC.md",
    "docs/TODO.md",
    "docs/TEST_PLAN.md",
    "docs/KNOWLEDGE.md",
];
const DEFAULT_ADAPTER_TEMPLATE_FILES: [&str; 4] = [
    ".github/copilot-instructions.md",
    ".github/instructions/general.instructions.md",
    "GEMINI.md",
    "CLAUDE.md",
];

#[derive(Default)]
struct RunStats {
    created: usize,
    updated: usize,
    skipped: usize,
    errors: usize,
}

#[derive(Debug)]
struct TemplateFile {
    rel_path: String,
    content: String,
}

#[derive(Debug)]
struct RunOptions {
    template: String,
    agent_format: String,
    non_interactive: bool,
    verbose: bool,
    json: bool,
    with_adapters: bool,
}

#[derive(Debug, Default)]
struct CliOptions {
    dir: Option<PathBuf>,
    output: Option<PathBuf>,
    dry_run: bool,
    force: bool,
    non_interactive: bool,
    verbose: bool,
    json: bool,
    with_adapters: bool,
    template: Option<String>,
    agent_format: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct AidleConfig {
    project: Option<ProjectConfig>,
    template: Option<TemplateConfig>,
    agent: Option<AgentConfig>,
    execution: Option<ExecutionConfig>,
    adapters: Option<AdaptersConfig>,
}

#[derive(Debug, Default, Deserialize)]
struct ProjectConfig {
    name: Option<String>,
    output: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct TemplateConfig {
    name: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct AgentConfig {
    format: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct ExecutionConfig {
    force: Option<bool>,
    dry_run: Option<bool>,
    non_interactive: Option<bool>,
    verbose: Option<bool>,
    json: Option<bool>,
}

#[derive(Debug, Default, Deserialize)]
struct AdaptersConfig {
    enabled: Option<bool>,
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

    let cli = parse_cli_options(args)?;

    let root = resolve_root(&cwd, &config, &cli)?;
    let force = cli.force || config.execution.as_ref().and_then(|e| e.force).unwrap_or(false);
    let dry_run = cli.dry_run || config.execution.as_ref().and_then(|e| e.dry_run).unwrap_or(false);
    let non_interactive = cli.non_interactive
        || config
            .execution
            .as_ref()
            .and_then(|e| e.non_interactive)
            .unwrap_or(false)
        || !std::io::stdin().is_terminal();
    let verbose = cli.verbose
        || config
            .execution
            .as_ref()
            .and_then(|e| e.verbose)
            .unwrap_or(false);
    let json_output = cli.json
        || config
            .execution
            .as_ref()
            .and_then(|e| e.json)
            .unwrap_or(false);
    let with_adapters = cli.with_adapters
        || config
            .adapters
            .as_ref()
            .and_then(|a| a.enabled)
            .unwrap_or(false);

    let template = cli
        .template
        .or_else(|| config.template.as_ref().and_then(|t| t.name.clone()))
        .unwrap_or_else(|| "default".to_string());
    if template != "default" {
        return Err(arg_error(
            format!("未対応テンプレート `{template}` です。"),
            "サポート対象のテンプレート名を指定してください（現在は `default` のみ）。",
        ));
    }

    let agent_format = cli
        .agent_format
        .or_else(|| config.agent.as_ref().and_then(|a| a.format.clone()))
        .unwrap_or_else(|| "agents-md".to_string());
    if agent_format != "agents-md" {
        return Err(arg_error(
            format!("未対応 agent-format `{agent_format}` です。"),
            "サポート対象の agent-format を指定してください（現在は `agents-md` のみ）。",
        ));
    }

    let options = RunOptions {
        template,
        agent_format,
        non_interactive,
        verbose,
        json: json_output,
        with_adapters,
    };

    let template_files = load_template_files(&options.template, options.with_adapters)?;
    let stats = create_required_files(&root, &template_files, dry_run, force)?;
    print_summary(&stats, &options, &root);
    Ok(())
}

fn arg_error(cause: String, action: &str) -> (u8, String) {
    (2, format!("引数エラー: {cause}\n対処: {action}"))
}

fn parse_cli_options(args: impl Iterator<Item = String>) -> Result<CliOptions, (u8, String)> {
    let mut cli = CliOptions::default();
    let mut it = args.peekable();

    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--dry-run" => cli.dry_run = true,
            "--force" => cli.force = true,
            "--non-interactive" => cli.non_interactive = true,
            "--verbose" => cli.verbose = true,
            "--json" => cli.json = true,
            "--with-adapters" => cli.with_adapters = true,
            "--output" => {
                let value = it.next().ok_or_else(|| {
                    arg_error(
                        "`--output` の値が不足しています。".to_string(),
                        "`--output <path>` の形式で指定してください。",
                    )
                })?;
                cli.output = Some(PathBuf::from(value));
            }
            "--template" => {
                let value = it.next().ok_or_else(|| {
                    arg_error(
                        "`--template` の値が不足しています。".to_string(),
                        "`--template <name>` の形式で指定してください。",
                    )
                })?;
                cli.template = Some(value);
            }
            "--agent-format" => {
                let value = it.next().ok_or_else(|| {
                    arg_error(
                        "`--agent-format` の値が不足しています。".to_string(),
                        "`--agent-format <name>` の形式で指定してください。",
                    )
                })?;
                cli.agent_format = Some(value);
            }
            _ if arg.starts_with('-') => {
                return Err(arg_error(
                    format!("未対応オプション `{arg}` です。"),
                    "`aidle init [dir] [--output <path>] [--dry-run] [--force] [--non-interactive] [--verbose] [--json] [--with-adapters] [--template <name>] [--agent-format <name>]` を使用してください。",
                ));
            }
            _ => {
                if cli.dir.is_some() {
                    return Err(arg_error(
                        "`init` に指定できるディレクトリは1つまでです。".to_string(),
                        "`aidle init [dir]` の形式で実行してください。",
                    ));
                }
                cli.dir = Some(PathBuf::from(arg));
            }
        }
    }

    Ok(cli)
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

fn resolve_root(cwd: &Path, config: &AidleConfig, cli: &CliOptions) -> Result<PathBuf, (u8, String)> {
    if cli.dir.is_some() && cli.output.is_some() {
        return Err(arg_error(
            "`dir` と `--output` の同時指定はできません。".to_string(),
            "どちらか一方のみ指定してください。",
        ));
    }

    if let Some(path) = cli.output.as_ref().or(cli.dir.as_ref()) {
        return Ok(if path.is_absolute() {
            path.clone()
        } else {
            cwd.join(path)
        });
    }

    let from_config = config
        .project
        .as_ref()
        .and_then(|p| p.output.as_ref().or(p.name.as_ref()));

    match from_config {
        Some(path) => {
            let p = PathBuf::from(path);
            Ok(if p.is_absolute() { p } else { cwd.join(p) })
        }
        None => Ok(cwd.to_path_buf()),
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

fn template_error(cause: String, action: &str) -> (u8, String) {
    (
        3,
        format!("テンプレート読み込みエラー: {cause}\n対処: {action}"),
    )
}

fn template_base_dir() -> PathBuf {
    if let Ok(custom) = env::var("AIDLE_TEMPLATE_ROOT") {
        return PathBuf::from(custom);
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates")
}

fn load_template_files(
    template_name: &str,
    with_adapters: bool,
) -> Result<Vec<TemplateFile>, (u8, String)> {
    let template_dir = template_base_dir().join(template_name);
    let mut paths = DEFAULT_TEMPLATE_FILES.to_vec();
    if with_adapters {
        paths.extend(DEFAULT_ADAPTER_TEMPLATE_FILES);
    }
    let mut files = Vec::with_capacity(paths.len());

    for rel in paths {
        let path = template_dir.join(rel);
        let content = fs::read_to_string(&path).map_err(|e| {
            template_error(
                format!("{} の読み込みに失敗しました: {e}", path.display()),
                "テンプレート配置とファイル権限を確認してください。",
            )
        })?;
        files.push(TemplateFile {
            rel_path: rel.to_string(),
            content,
        });
    }

    Ok(files)
}

fn rollback_state(created_files: &[PathBuf], overwritten_files: &[(PathBuf, Vec<u8>)]) {
    for path in created_files.iter().rev() {
        let _ = fs::remove_file(path);
    }
    for (path, original) in overwritten_files.iter().rev() {
        let _ = fs::write(path, original);
    }
}

#[derive(Serialize)]
struct JsonSummary<'a> {
    created: usize,
    updated: usize,
    skipped: usize,
    errors: usize,
    non_interactive: bool,
    template: &'a str,
    agent_format: &'a str,
    with_adapters: bool,
    root: String,
}

fn print_summary(stats: &RunStats, options: &RunOptions, root: &Path) {
    if options.verbose {
        eprintln!(
            "[verbose] root={} template={} agent_format={} non_interactive={}",
            root.display(),
            options.template,
            options.agent_format,
            options.non_interactive
        );
        eprintln!("[verbose] with_adapters={}", options.with_adapters);
    }

    if options.json {
        let payload = JsonSummary {
            created: stats.created,
            updated: stats.updated,
            skipped: stats.skipped,
            errors: stats.errors,
            non_interactive: options.non_interactive,
            template: &options.template,
            agent_format: &options.agent_format,
            with_adapters: options.with_adapters,
            root: root.display().to_string(),
        };
        println!(
            "{}",
            serde_json::to_string(&json!(payload)).expect("failed to serialize json summary")
        );
        return;
    }

    println!(
        "created={} updated={} skipped={} errors={}",
        stats.created, stats.updated, stats.skipped, stats.errors
    );
}

fn create_required_files(
    root: &Path,
    template_files: &[TemplateFile],
    dry_run: bool,
    force: bool,
) -> Result<RunStats, (u8, String)> {
    let mut stats = RunStats::default();

    if dry_run {
        return Ok(stats);
    }

    fs::create_dir_all(root).map_err(|e| io_error("出力ディレクトリ作成", &e))?;

    let mut created_files: Vec<PathBuf> = Vec::new();
    let mut overwritten_files: Vec<(PathBuf, Vec<u8>)> = Vec::new();

    for tf in template_files {
        let path = root.join(&tf.rel_path);
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

        if let Err(e) = file.write_all(tf.content.as_bytes()) {
            let err = io_error(&format!("ファイル書き込み ({})", path.display()), &e);
            rollback_state(&created_files, &overwritten_files);
            return Err(err);
        }
    }

    Ok(stats)
}
