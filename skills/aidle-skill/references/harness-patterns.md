# Harness Patterns (言語別ベストプラクティス)

このドキュメントは、AIエージェントが `docs/HARNESS.md` の指示に従って検証環境（Harness）を設営する際の具体的なツールや構成パターンのリファレンスです。

## Rust 🦀
- **リンター/フォーマッター**: `cargo clippy`, `cargo fmt` を標準とする。速度が速いため、デフォルト構成で十分。
- **テスト**: 標準の `cargo test` ではなく、実行が高速な `cargo nextest` をインストール・使用する (`cargo install cargo-nextest`)。
- **フック (Claude Code)**:
  - ファイル: `.claude/settings.json`
  - 内容:
    ```json
    {
      "postToolUse": [
        {
          "toolName": "ReplaceTool",
          "command": "cargo clippy -- -D warnings && cargo fmt --check",
          "addOutputToContext": true
        }
      ]
    }
    ```

## TypeScript / Node.js 🟨
- **リンター**: 速度最優先のため、`ESLint` ではなく **`Oxlint`** または **`Biome`** を推奨する。
- **フォーマッター**: **`Biome`** を推奨。
- **フック (Claude Code)**:
  - 内容 (Biome の例):
    ```json
    {
      "postToolUse": [
        {
          "toolName": "ReplaceTool",
          "command": "npx @biomejs/biome check --write ./src",
          "addOutputToContext": true
        }
      ]
    }
    ```

## Python 🐍
- **リンター/フォーマッター**: 超高速な **`Ruff`** を絶対の標準とする (`pip install ruff` または `uv add ruff`)。
- **テスト**: `pytest` を使用。
- **フック (Claude Code)**:
  - 内容:
    ```json
    {
      "postToolUse": [
        {
          "toolName": "ReplaceTool",
          "command": "ruff check --fix . && ruff format .",
          "addOutputToContext": true
        }
      ]
    }
    ```

## 共通の考え方 (エラーの教育化)
リンターがエラーを出した際、単に「エラーです」で終わらせず、スクリプト側で `echo "解決策: docs/RULES.md の第X項に従い、依存関係を修正してください"` のようなヒントを出力（教育化）し、それを `addOutputToContext: true` でAIに読ませる構成が最も強力です。
