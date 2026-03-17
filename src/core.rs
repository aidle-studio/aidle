use rust_embed::RustEmbed;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::utils::{TemplateFile, RunStats, rollback_state, io_error, template_error};

#[derive(RustEmbed)]
#[folder = "templates/"]
pub struct Templates;

pub const DEFAULT_TEMPLATE_FILES: [&str; 28] = [
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

pub const DEFAULT_ADAPTER_TEMPLATE_FILES: [&str; 4] = [
    ".github/copilot-instructions.md",
    ".github/instructions/general.instructions.md",
    "GEMINI.md",
    "CLAUDE.md",
];

#[derive(Debug, Clone)]
pub enum TemplateSource {
    Embedded(String),
    Filesystem(PathBuf),
}

impl TemplateSource {
    pub fn display_name(&self) -> String {
        match self {
            TemplateSource::Embedded(name) => name.clone(),
            TemplateSource::Filesystem(path) => path.display().to_string(),
        }
    }
}

pub fn resolve_template_source(name: &str) -> Option<TemplateSource> {
    // 1. 埋め込みアセットを優先チェック
    let prefix = format!("{}/", name);
    if Templates::iter().any(|path| path.starts_with(&prefix)) {
        return Some(TemplateSource::Embedded(name.to_string()));
    }

    // 2. ファイルシステム（直接パス）
    let path = PathBuf::from(name);
    if path.is_dir() {
        return Some(TemplateSource::Filesystem(path));
    }

    // 3. ファイルシステム（テンプレートベースディレクトリからの相対）
    let path = template_base_dir().join(name);
    if path.is_dir() {
        return Some(TemplateSource::Filesystem(path));
    }

    None
}

pub fn template_base_dir() -> PathBuf {
    if let Ok(custom) = std::env::var("AIDLE_TEMPLATE_ROOT") {
        return PathBuf::from(custom);
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates")
}

pub fn load_template_files(
    source: &TemplateSource,
    with_adapters: bool,
    _verbose: bool,
) -> Result<Vec<TemplateFile>, (u8, String)> {
    let mut paths = DEFAULT_TEMPLATE_FILES.to_vec();
    if with_adapters {
        paths.extend(DEFAULT_ADAPTER_TEMPLATE_FILES);
    }
    let mut files = Vec::with_capacity(paths.len());

    for rel in paths {
        let content = match source {
            TemplateSource::Embedded(name) => {
                let embedded_path = format!("{}/{}", name, rel);
                let embedded_file = Templates::get(&embedded_path).ok_or_else(|| {
                    template_error(
                        format!("embedded file {} not found", embedded_path),
                        "Internal error: built-in template is incomplete.",
                    )
                })?;
                String::from_utf8(embedded_file.data.to_vec()).map_err(|e| {
                    template_error(
                        format!("failed to parse embedded file {}: {e}", embedded_path),
                        "Internal error: embedded file is not valid UTF-8.",
                    )
                })?
            }
            TemplateSource::Filesystem(path) => {
                let full_path = path.join(rel);
                fs::read_to_string(&full_path).map_err(|e| {
                    template_error(
                        format!("failed to read {}: {e}", full_path.display()),
                        "Check template placement and file permissions.",
                    )
                })?
            }
        };

        files.push(TemplateFile {
            rel_path: rel.to_string(),
            content,
        });
    }

    Ok(files)
}

pub fn create_required_files(
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
        if let Some(parent) = path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                let err = io_error(&format!("creating parent directory ({})", parent.display()), &e);
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
