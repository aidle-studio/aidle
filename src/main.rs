pub mod cli;
mod commands;
pub mod core;
pub mod utils;

use serde_json::json;
use std::env;
use std::io::IsTerminal;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

use crate::cli::{AidleConfig, RunOptions, load_config, parse_cli_options, resolve_root};
use crate::core::{create_required_files, load_template_files, resolve_template_source};
use crate::utils::{JsonSummary, RunStats, StatsLog, arg_error, io_error};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err((code, message)) => {
            if code != 0 {
                eprintln!("{message}");
            }
            ExitCode::from(code)
        }
    }
}

fn help_text() -> &'static str {
    r#"aidle - AI-Driven Development (AIDD) Project Initializer

Usage:
    aidle <subcommand> [options]

Subcommands:
    init [dir]    Initialize a new project in [dir] or current directory.
    check         Verify structural consistency between local documents and the template.

Options:
    --output <path>         Set output root directory (cannot be used with [dir]).
    --lang <ja|en>          Language for templates (default: "ja").
    --template <name>       Template name to use (default: "default").
    --agent-format <name>   Agent convention format (default: "agents-md").
    --with-adapters         Generate AI adapters (Copilot, Gemini, Claude).
    --stats-out <path>      Save execution statistics to <path> in JSON format.
    --dry-run               Display what would be done without making any changes.
    --force                 Overwrite existing files.
    --non-interactive       Disable interactive prompts.
    --verbose               Show detailed logs.
    --json                  Output result summary in JSON format.
    -h, --help              Print help information.
"#
}

fn run() -> Result<(), (u8, String)> {
    let started_at = Instant::now();
    let cwd = env::current_dir().map_err(|e| io_error("getting current directory", &e))?;
    let config = load_config(&cwd)?;

    let args_raw: Vec<String> = env::args().skip(1).collect();

    // 全体の引数にヘルプフラグが含まれているかチェック
    if args_raw.iter().any(|arg| arg == "--help" || arg == "-h") {
        println!("{}", help_text());
        return Ok(());
    }

    let mut args = args_raw.into_iter().peekable();
    let command = match args.next() {
        Some(cmd) => cmd,
        None => {
            let msg = format!(
                "Argument Error: subcommand is required\nAction: Specify a subcommand like `init`.\n\n{}",
                help_text()
            );
            return Err((2, msg));
        }
    };

    if command == "check" {
        return handle_check_command(&cwd, &config, args);
    }

    if command != "init" {
        return Err(arg_error(
            format!("unsupported subcommand `{command}`"),
            "Use `aidle init [dir]` to initialize a project.",
        ));
    }

    let cli = parse_cli_options(args)?;

    let force = cli.force
        || config
            .execution
            .as_ref()
            .and_then(|e| e.force)
            .unwrap_or(false);
    let dry_run = cli.dry_run
        || config
            .execution
            .as_ref()
            .and_then(|e| e.dry_run)
            .unwrap_or(false);
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
    let mut with_adapters = cli.with_adapters
        || config
            .adapters
            .as_ref()
            .and_then(|a| a.enabled)
            .unwrap_or(false);
    let stats_out = cli.stats_out.clone().or_else(|| {
        config
            .stats
            .as_ref()
            .and_then(|s| s.output.clone().map(PathBuf::from))
    });
    let mut interactive_project_name: Option<String> = None;

    let default_project_name = cwd
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();

    if !non_interactive {
        use dialoguer::{Confirm, Input, theme::ColorfulTheme};

        if let Ok(input) = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Project Name")
            .default(default_project_name)
            .interact()
        {
            let trimmed = input.trim();
            if !trimmed.is_empty() {
                interactive_project_name = Some(trimmed.to_string());
            }
        }

        if !cli.with_adapters
            && config.adapters.as_ref().and_then(|a| a.enabled).is_none()
            && let Ok(enable) = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Generate AI Adapters?")
                .default(false)
                .interact()
        {
            with_adapters = enable;
        }
    }

    let root = if cli.dir.is_none()
        && cli.output.is_none()
        && config
            .project
            .as_ref()
            .and_then(|p| p.output.as_ref().or(p.name.as_ref()))
            .is_none()
    {
        let cwd_name = cwd
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        match interactive_project_name {
            Some(name) if name != cwd_name => cwd.join(name),
            _ => resolve_root(&cwd, &config, &cli)?,
        }
    } else {
        resolve_root(&cwd, &config, &cli)?
    };

    let template_name = cli
        .template
        .or_else(|| config.template.as_ref().and_then(|t| t.name.clone()))
        .unwrap_or_else(|| "default".to_string());

    let template_source = resolve_template_source(&template_name).ok_or_else(|| {
        arg_error(
            format!("unsupported template name or invalid path `{template_name}`"),
            "Please specify a built-in template name (e.g., 'default') or a valid directory path.",
        )
    })?;

    let agent_format = cli
        .agent_format
        .or_else(|| config.agent.as_ref().and_then(|a| a.format.clone()))
        .unwrap_or_else(|| "agents-md".to_string());
    if agent_format != "agents-md" {
        return Err(arg_error(
            format!("unsupported agent-format `{agent_format}`"),
            "Please specify a supported agent-format (currently 'agents-md').",
        ));
    }

    let lang = cli
        .lang
        .or_else(|| config.language.as_ref().and_then(|l| l.default.clone()))
        .unwrap_or_else(|| "ja".to_string());

    let options = RunOptions {
        template: template_source,
        lang,
        agent_format,
        non_interactive,
        verbose,
        json: json_output,
        with_adapters,
        stats_out,
    };

    let template_files = load_template_files(
        &options.template,
        &options.lang,
        options.with_adapters,
        options.verbose,
    )?;
    let stats = create_required_files(&root, &template_files, dry_run, force)?;
    let duration_ms = started_at.elapsed().as_millis();
    write_stats_log(&options, &stats, &root, duration_ms)?;
    print_summary(&stats, &options, &root);
    Ok(())
}

fn handle_check_command(
    cwd: &Path,
    config: &AidleConfig,
    args: impl Iterator<Item = String>,
) -> Result<(), (u8, String)> {
    let cli = parse_cli_options(args)?;
    let template_name = cli
        .template
        .or_else(|| config.template.as_ref().and_then(|t| t.name.clone()))
        .unwrap_or_else(|| "default".to_string());

    let lang = cli
        .lang
        .or_else(|| config.language.as_ref().and_then(|l| l.default.clone()))
        .unwrap_or_else(|| "ja".to_string());

    // 比較対象となるテンプレートソースを決定する
    let template_source = resolve_template_source(&template_name).ok_or_else(|| {
        arg_error(
            format!("unsupported template name or invalid path `{template_name}`"),
            "Please specify a built-in template name (e.g., 'default') or a valid directory path.",
        )
    })?;

    // テンプレートファイルを読み込む
    let template_files =
        load_template_files(&template_source, &lang, cli.with_adapters, cli.verbose)?;
    let mut missing_found = false;

    println!("--- aidle Structural Consistency Check ---");

    for tf in template_files {
        // docs/ 配下の Markdown ファイルのみを構造チェックの対象とする
        if !tf.rel_path.starts_with("docs/") || !tf.rel_path.ends_with(".md") {
            continue;
        }

        let local_path = cwd.join(&tf.rel_path);
        if !local_path.exists() {
            println!("\n[{}]: File does not exist.", tf.rel_path);
            missing_found = true;
            continue;
        }

        let local_content = std::fs::read_to_string(&local_path)
            .map_err(|e| io_error(&format!("reading {}", local_path.display()), &e))?;

        // テンプレートとローカルファイルの見出しを比較して、不足分（新しい観点）を抽出する
        let missing_headings = commands::check::compare_headings(&tf.content, &local_content);

        if !missing_headings.is_empty() {
            println!("\n[{}]: Missing sections (concepts) detected.", tf.rel_path);
            for h in missing_headings {
                // 不足している見出しのテンプレート本文（スニペット）を抽出して表示
                let snippet = commands::check::extract_section_content(&tf.content, &h);
                println!("  - \"{}\"", h);
                if !snippet.is_empty() {
                    let indent = "    ";
                    for line in snippet.lines().take(5) {
                        println!("{}{}", indent, line);
                    }
                    if snippet.lines().count() > 5 {
                        println!("    ...");
                    }
                }
            }
            missing_found = true;
        }
    }

    if !missing_found {
        println!("\nAll documents are up-to-date with the latest template concepts.");
    } else {
        println!(
            "\nPlease incorporate the missing concepts into your project to align with the latest standards."
        );
    }

    Ok(())
}

fn write_stats_log(
    options: &RunOptions,
    stats: &RunStats,
    root: &Path,
    duration_ms: u128,
) -> Result<(), (u8, String)> {
    let Some(path) = options.stats_out.as_ref() else {
        return Ok(());
    };

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| io_error("creating stats log directory", &e))?;
    }

    let payload = StatsLog {
        duration_ms,
        created: stats.created,
        updated: stats.updated,
        skipped: stats.skipped,
        errors: stats.errors,
        root: root.display().to_string(),
        template: options.template.display_name(),
        agent_format: options.agent_format.clone(),
        with_adapters: options.with_adapters,
    };
    let data = serde_json::to_string_pretty(&payload).map_err(|e| {
        (
            4,
            format!("Internal Error: failed to serialize stats log JSON: {e}"),
        )
    })?;
    std::fs::write(path, data).map_err(|e| io_error("saving stats log", &e))
}

fn print_summary(stats: &RunStats, options: &RunOptions, root: &Path) {
    if options.verbose {
        eprintln!(
            "[verbose] root={} template={} agent_format={} non_interactive={}",
            root.display(),
            options.template.display_name(),
            options.agent_format,
            options.non_interactive
        );
        eprintln!("[verbose] with_adapters={}", options.with_adapters);
        if let Some(stats_out) = options.stats_out.as_ref() {
            eprintln!("[verbose] stats_out={}", stats_out.display());
        }
    }

    if options.json {
        let payload = JsonSummary {
            created: stats.created,
            updated: stats.updated,
            skipped: stats.skipped,
            errors: stats.errors,
            non_interactive: options.non_interactive,
            template: options.template.display_name(),
            agent_format: options.agent_format.clone(),
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
