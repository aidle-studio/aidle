use serde::Serialize;
use std::fs;
use std::path::PathBuf;

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
    (3, format!("Template Load Error: {cause}\nAction: {action}"))
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_rollback_state() {
        let temp = tempdir().unwrap();
        let file1 = temp.path().join("file1.txt");
        let file2 = temp.path().join("file2.txt");

        // 準備: file2 は既存
        fs::write(&file2, "original content").unwrap();

        let created = vec![file1.clone()];
        let overwritten = vec![(file2.clone(), "original content".as_bytes().to_vec())];

        // file1 を作成し、file2 を上書きしたと仮定
        fs::write(&file1, "new file").unwrap();
        fs::write(&file2, "overwritten content").unwrap();

        // ロールバック実行
        rollback_state(&created, &overwritten);

        // 検証: file1 は消えているはず
        assert!(!file1.exists());
        // 検証: file2 は元の内容に戻っているはず
        assert_eq!(fs::read_to_string(&file2).unwrap(), "original content");
    }
}
