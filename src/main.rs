use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const REQUIRED_FILES: [(&str, &str); 8] = [
    ("AGENTS.md", "# AGENTS.md\n"),
    ("README.md", "# README.md\n"),
    ("docs/AGENT_CONTEXT.md", "# AGENT_CONTEXT.md\n"),
    ("docs/RULES.md", "# RULES.md\n"),
    ("docs/SPEC.md", "# SPEC.md\n"),
    ("docs/TODO.md", "# TODO.md\n"),
    ("docs/TEST_PLAN.md", "# TEST_PLAN.md\n"),
    ("docs/KNOWLEDGE.md", "# KNOWLEDGE.md\n"),
];

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

    let root = match args.next() {
        Some(dir) => PathBuf::from(dir),
        None => env::current_dir()
            .map_err(|e| (3, format!("I/Oエラー: カレントディレクトリ取得に失敗しました: {e}")))?,
    };

    if args.next().is_some() {
        return Err((
            2,
            "引数エラー: `init` に指定できる引数は1つまでです。対処: `aidle init [dir]` で実行してください。"
                .to_string(),
        ));
    }

    create_required_files(&root)
}

fn create_required_files(root: &Path) -> Result<(), (u8, String)> {
    fs::create_dir_all(root)
        .map_err(|e| (3, format!("I/Oエラー: 出力ディレクトリを作成できませんでした: {e}")))?;

    for (rel, contents) in REQUIRED_FILES {
        let path = root.join(rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                (
                    3,
                    format!(
                        "I/Oエラー: 親ディレクトリを作成できませんでした ({}): {e}",
                        parent.display()
                    ),
                )
            })?;
        }

        let mut file = match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(f) => f,
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => {
                return Err((
                    3,
                    format!(
                        "I/Oエラー: ファイル作成に失敗しました ({}): {e}",
                        path.display()
                    ),
                ))
            }
        };

        file.write_all(contents.as_bytes()).map_err(|e| {
            (
                3,
                format!(
                    "I/Oエラー: ファイル書き込みに失敗しました ({}): {e}",
                    path.display()
                ),
            )
        })?;
    }

    Ok(())
}
