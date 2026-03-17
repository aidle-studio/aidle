use crate::core::TemplateSource;
use crate::utils::arg_error;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct RunOptions {
    pub template: TemplateSource,
    pub lang: String,
    pub agent_format: String,
    pub non_interactive: bool,
    pub verbose: bool,
    pub json: bool,
    pub with_adapters: bool,
    pub stats_out: Option<PathBuf>,
}

#[derive(Debug, Default)]
pub struct CliOptions {
    pub dir: Option<PathBuf>,
    pub output: Option<PathBuf>,
    pub lang: Option<String>,
    pub dry_run: bool,
    pub force: bool,
    pub non_interactive: bool,
    pub verbose: bool,
    pub json: bool,
    pub with_adapters: bool,
    pub stats_out: Option<PathBuf>,
    pub template: Option<String>,
    pub agent_format: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct AidleConfig {
    pub project: Option<ProjectConfig>,
    pub template: Option<TemplateConfig>,
    pub language: Option<LanguageConfig>,
    pub agent: Option<AgentConfig>,
    pub execution: Option<ExecutionConfig>,
    pub adapters: Option<AdaptersConfig>,
    pub stats: Option<StatsConfig>,
}

#[derive(Debug, Default, Deserialize)]
pub struct ProjectConfig {
    pub name: Option<String>,
    pub output: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct TemplateConfig {
    pub name: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct LanguageConfig {
    pub default: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct AgentConfig {
    pub format: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct ExecutionConfig {
    pub force: Option<bool>,
    pub dry_run: Option<bool>,
    pub non_interactive: Option<bool>,
    pub verbose: Option<bool>,
    pub json: Option<bool>,
}

#[derive(Debug, Default, Deserialize)]
pub struct AdaptersConfig {
    pub enabled: Option<bool>,
}

#[derive(Debug, Default, Deserialize)]
pub struct StatsConfig {
    pub output: Option<String>,
}

pub fn parse_cli_options(args: impl Iterator<Item = String>) -> Result<CliOptions, (u8, String)> {
    let mut cli = CliOptions::default();
    let mut it = args.peekable();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--lang" => {
                let value = it.next().ok_or_else(|| {
                    arg_error(
                        "missing value for `--lang`".to_string(),
                        "Specify as `--lang <ja|en>`.",
                    )
                })?;
                cli.lang = Some(value);
            }
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
                // main.rs で処理されるが、解析上は許可する
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

pub fn load_config(cwd: &Path) -> Result<AidleConfig, (u8, String)> {
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

pub fn resolve_root(
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_parse_cli_options_basic() {
        let args = vec!["init", "myproj", "--force", "--verbose"]
            .into_iter()
            .map(String::from);
        let cli = parse_cli_options(args.skip(1)).unwrap();
        assert_eq!(cli.dir, Some(PathBuf::from("myproj")));
        assert!(cli.force);
        assert!(cli.verbose);
    }
    #[test]
    fn test_parse_cli_options_error_duplicate_dir() {
        let args = vec!["init", "dir1", "dir2"].into_iter().map(String::from);
        let res = parse_cli_options(args.skip(1));
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_cli_options_error_missing_stats_out_value() {
        let args = vec!["init", "--stats-out"].into_iter().map(String::from);
        let res = parse_cli_options(args.skip(1));
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_cli_options_error_missing_output_value() {
        let args = vec!["init", "--output"].into_iter().map(String::from);
        let res = parse_cli_options(args.skip(1));
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_cli_options_error_missing_template_value() {
        let args = vec!["init", "--template"].into_iter().map(String::from);
        let res = parse_cli_options(args.skip(1));
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_cli_options_error_missing_agent_format_value() {
        let args = vec!["init", "--agent-format"].into_iter().map(String::from);
        let res = parse_cli_options(args.skip(1));
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_cli_options_error_unsupported_option() {
        let args = vec!["init", "--unknown-flag"].into_iter().map(String::from);
        let res = parse_cli_options(args.skip(1));
        assert!(res.is_err());
    }

    #[test]
    fn test_load_config_nonexistent() {
        let temp = tempdir().unwrap();
        let config = load_config(temp.path()).unwrap();
        assert!(config.project.is_none());
    }
    #[test]
    fn test_load_config_valid() {
        let temp = tempdir().unwrap();
        let toml_path = temp.path().join("aidle.toml");
        fs::write(
            toml_path,
            r#"[project]
name = "test-project"
"#,
        )
        .unwrap();
        let config = load_config(temp.path()).unwrap();
        assert_eq!(
            config.project.unwrap().name,
            Some("test-project".to_string())
        );
    }

    #[test]
    fn test_resolve_root_absolute() {
        let cwd = PathBuf::from("/cwd");
        let mut cli = CliOptions::default();
        cli.dir = Some(PathBuf::from("/abs/path"));
        let root = resolve_root(&cwd, &AidleConfig::default(), &cli).unwrap();
        assert_eq!(root, PathBuf::from("/abs/path"));
    }

    #[test]
    fn test_resolve_root_relative() {
        let cwd = PathBuf::from("/cwd");
        let mut cli = CliOptions::default();
        cli.dir = Some(PathBuf::from("rel/path"));
        let root = resolve_root(&cwd, &AidleConfig::default(), &cli).unwrap();
        assert_eq!(root, cwd.join("rel/path"));
    }
}
