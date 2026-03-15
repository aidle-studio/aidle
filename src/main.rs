use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::IsTerminal;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

const DEFAULT_TEMPLATE_FILES: [&str; 28] = [
    "AGENTS.md",
    "ARCHITECTURE.md",
    "README.md",
    "docs/AGENT_CONTEXT.md",
    "docs/RULES.md",
    "docs/SPEC.md",
    "docs/TODO.md",
    "docs/TEST_PLAN.md",
    "docs/KNOWLEDGE.md",
    "docs/HARNESS.md",
    "docs/QUALITY_SCORE.md",
    "docs/RELIABILITY.md",
    "docs/SECURITY.md",
    "docs/PRODUCT_SENSE.md",
    "docs/DESIGN.md",
    "docs/PLANS.md",
    "docs/adr/index.md",
    "docs/adr/.gitkeep",
    "docs/design-docs/index.md",
    "docs/design-docs/core-beliefs.md",
    "docs/exec-plans/active/.gitkeep",
    "docs/exec-plans/completed/.gitkeep",
    "docs/exec-plans/tech-debt.md",
    "docs/product-specs/index.md",
    "docs/product-specs/.gitkeep",
    "docs/references/index.md",
    "docs/references/.gitkeep",
    "scripts/check_harness.sh",
];

#[derive(RustEmbed)]
#[folder = "templates/"]
struct Templates;

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
    stats_out: Option<PathBuf>,
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
    stats_out: Option<PathBuf>,
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
    stats: Option<StatsConfig>,
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

#[derive(Debug, Default, Deserialize)]
struct StatsConfig {
    output: Option<String>,
}

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

Options:
    --output <path>         Set output root directory (cannot be used with [dir]).
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

fn is_template_available(name: &str) -> bool {
    let prefix = format!("{}/", name);
    if Templates::iter().any(|path| path.starts_with(&prefix)) {
        return true;
    }
    let template_dir = template_base_dir().join(name);
    template_dir.exists() && template_dir.is_dir()
}

fn run() -> Result<(), (u8, String)> {
    let started_at = Instant::now();
    let cwd = env::current_dir().map_err(|e| io_error("getting current directory", &e))?;
    let config = load_config(&cwd)?;

    let mut args = env::args().skip(1).peekable();
    let command = match args.next() {
        Some(cmd) if cmd == "-h" || cmd == "--help" => {
            println!("{}", help_text());
            return Ok(());
        }
        Some(cmd) => cmd,
        None => {
            let msg = format!(
                "Argument Error: subcommand is required\nAction: Specify a subcommand like `init`.\n\n{}",
                help_text()
            );
            return Err((2, msg));
        }
    };

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

    let template = cli
        .template
        .or_else(|| config.template.as_ref().and_then(|t| t.name.clone()))
        .unwrap_or_else(|| "default".to_string());

    if !is_template_available(&template) {
        return Err(arg_error(
            format!("unsupported template `{template}`"),
            "Please specify a supported template name (currently 'default' is built-in).",
        ));
    }

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

    let options = RunOptions {
        template,
        agent_format,
        non_interactive,
        verbose,
        json: json_output,
        with_adapters,
        stats_out,
    };

    let template_files = load_template_files(&options.template, options.with_adapters, options.verbose)?;
    let stats = create_required_files(&root, &template_files, dry_run, force)?;
    let duration_ms = started_at.elapsed().as_millis();
    write_stats_log(&options, &stats, &root, duration_ms)?;
    print_summary(&stats, &options, &root);
    Ok(())
}

fn arg_error(cause: String, action: &str) -> (u8, String) {
    (2, format!("Argument Error: {cause}\nAction: {action}"))
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
            "--stats-out" => {
                let value = it.next().ok_or_else(|| {
                    arg_error(
                        "missing value for `--stats-out`".to_string(),
                        "Specify as `--stats-out <path>`.",
                    )
                })?;
                cli.stats_out = Some(PathBuf::from(value));
            }
            "--output" => {
                let value = it.next().ok_or_else(|| {
                    arg_error(
                        "missing value for `--output`".to_string(),
                        "Specify as `--output <path>`.",
                    )
                })?;
                cli.output = Some(PathBuf::from(value));
            }
            "--template" => {
                let value = it.next().ok_or_else(|| {
                    arg_error(
                        "missing value for `--template`".to_string(),
                        "Specify as `--template <name>`.",
                    )
                })?;
                cli.template = Some(value);
            }
            "--agent-format" => {
                let value = it.next().ok_or_else(|| {
                    arg_error(
                        "missing value for `--agent-format`".to_string(),
                        "Specify as `--agent-format <name>`.",
                    )
                })?;
                cli.agent_format = Some(value);
            }
            "-h" | "--help" => {
                println!("{}", help_text());
                return Err((0, String::new()));
            }
            _ if arg.starts_with('-') => {
                return Err(arg_error(
                    format!("unsupported option `{arg}`"),
                    "Run `aidle --help` to see available options.",
                ));
            }
            _ => {
                if cli.dir.is_some() {
                    return Err(arg_error(
                        "only one directory can be specified for `init`".to_string(),
                        "Run as `aidle init [dir]`.",
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
                "Config Error: failed to read `aidle.toml`: {e}\nAction: Check file permissions and content."
            ),
        )
    })?;

    toml::from_str::<AidleConfig>(&text).map_err(|e| {
        (
            2,
            format!(
                "Config Error: failed to parse `aidle.toml`: {e}\nAction: Check TOML syntax and key names."
            ),
        )
    })
}

fn resolve_root(
    cwd: &Path,
    config: &AidleConfig,
    cli: &CliOptions,
) -> Result<PathBuf, (u8, String)> {
    if cli.dir.is_some() && cli.output.is_some() {
        return Err(arg_error(
            "cannot specify both `dir` and `--output`".to_string(),
            "Please specify only one of them.",
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
            "I/O Error: failed to {context}: {e}\nAction: Check path validity and access permissions."
        ),
    )
}

fn template_error(cause: String, action: &str) -> (u8, String) {
    (
        3,
        format!("Template Load Error: {cause}\nAction: {action}"),
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
    _verbose: bool,
) -> Result<Vec<TemplateFile>, (u8, String)> {
    let mut paths = DEFAULT_TEMPLATE_FILES.to_vec();
    if with_adapters {
        paths.extend(DEFAULT_ADAPTER_TEMPLATE_FILES);
    }
    let mut files = Vec::with_capacity(paths.len());

    let template_dir = template_base_dir().join(template_name);

    for rel in paths {
        let embedded_path = format!("{}/{}", template_name, rel);
        let content = if let Some(embedded_file) = Templates::get(&embedded_path) {
            String::from_utf8(embedded_file.data.to_vec()).map_err(|e| {
                template_error(
                    format!("failed to parse embedded file {}: {e}", embedded_path),
                    "Internal error: embedded file is not valid UTF-8.",
                )
            })?
        } else {
            let path = template_dir.join(rel);
            fs::read_to_string(&path).map_err(|e| {
                template_error(
                    format!("failed to read {}: {e}", path.display()),
                    "Check template placement and file permissions.",
                )
            })?
        };

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

#[derive(Serialize)]
struct StatsLog<'a> {
    duration_ms: u128,
    created: usize,
    updated: usize,
    skipped: usize,
    errors: usize,
    root: String,
    template: &'a str,
    agent_format: &'a str,
    with_adapters: bool,
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
        fs::create_dir_all(parent).map_err(|e| io_error("creating stats log directory", &e))?;
    }

    let payload = StatsLog {
        duration_ms,
        created: stats.created,
        updated: stats.updated,
        skipped: stats.skipped,
        errors: stats.errors,
        root: root.display().to_string(),
        template: &options.template,
        agent_format: &options.agent_format,
        with_adapters: options.with_adapters,
    };
    let data = serde_json::to_string_pretty(&payload).map_err(|e| {
        (
            4,
            format!("Internal Error: failed to serialize stats log JSON: {e}"),
        )
    })?;
    fs::write(path, data).map_err(|e| io_error("saving stats log", &e))
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

    fs::create_dir_all(root).map_err(|e| io_error("creating output directory", &e))?;

    let mut created_files: Vec<PathBuf> = Vec::new();
    let mut overwritten_files: Vec<(PathBuf, Vec<u8>)> = Vec::new();

    for tf in template_files {
        let path = root.join(&tf.rel_path);
        if let Some(parent) = path.parent()
            && let Err(e) = fs::create_dir_all(parent)
        {
            let err = io_error(&format!("creating parent directory ({})", parent.display()), &e);
            rollback_state(&created_files, &overwritten_files);
            return Err(err);
        }

        let mut file = if path.exists() {
            if !force {
                stats.skipped += 1;
                continue;
            }

            let original = match fs::read(&path) {
                Ok(bytes) => bytes,
                Err(e) => {
                    let err = io_error(&format!("reading existing file ({})", path.display()), &e);
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
                    let err = io_error(&format!("overwriting file ({})", path.display()), &e);
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
                    let err = io_error(&format!("creating file ({})", path.display()), &e);
                    rollback_state(&created_files, &overwritten_files);
                    return Err(err);
                }
            }
        };

        if let Err(e) = file.write_all(tf.content.as_bytes()) {
            let err = io_error(&format!("writing to file ({})", path.display()), &e);
            rollback_state(&created_files, &overwritten_files);
            return Err(err);
        }
    }

    Ok(stats)
}
