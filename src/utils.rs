use serde::Serialize;
use std::path::PathBuf;
use std::fs;

#[derive(Default)]
pub struct RunStats {
    pub created: usize,
    pub updated: usize,
    pub skipped: usize,
    pub errors: usize,
}

#[derive(Debug)]
pub struct TemplateFile {
    pub rel_path: String,
    pub content: String,
}

pub fn rollback_state(created_files: &[PathBuf], overwritten_files: &[(PathBuf, Vec<u8>)]) {
    for path in created_files.iter().rev() {
        let _ = fs::remove_file(path);
    }
    for (path, original) in overwritten_files.iter().rev() {
        let _ = fs::write(path, original);
    }
}

pub fn io_error(context: &str, e: &std::io::Error) -> (u8, String) {
    (
        3,
        format!(
            "I/O Error: failed to {context}: {e}\nAction: Check path validity and access permissions."
        ),
    )
}

pub fn template_error(cause: String, action: &str) -> (u8, String) {
    (
        3,
        format!("Template Load Error: {cause}\nAction: {action}"),
    )
}

pub fn arg_error(cause: String, action: &str) -> (u8, String) {
    (2, format!("Argument Error: {cause}\nAction: {action}"))
}

#[derive(Serialize)]
pub struct JsonSummary {
    pub created: usize,
    pub updated: usize,
    pub skipped: usize,
    pub errors: usize,
    pub non_interactive: bool,
    pub template: String,
    pub agent_format: String,
    pub with_adapters: bool,
    pub root: String,
}

#[derive(Serialize)]
pub struct StatsLog {
    pub duration_ms: u128,
    pub created: usize,
    pub updated: usize,
    pub skipped: usize,
    pub errors: usize,
    pub root: String,
    pub template: String,
    pub agent_format: String,
    pub with_adapters: bool,
}
