# KNOWLEDGE.md

本プロジェクトの知見、過去の決定事項（ADR）、および技術的なメモを集約します。

## 🏛️ アーキテクチャ決定記録 (ADR)

プロジェクトの重要な決定事項（ADR）は、現在、以下のディレクトリで一元管理されています。

- [ADR-0001: CLI実装言語としてRustを採用](adr/2026-03-08_rust_adoption.md)
- [ADR-0002: 初期リリースのテンプレート対応範囲](adr/2026-03-08_template_scope.md)
- [ADR-0003: 設定ファイルフォーマットにTOMLを採用](adr/2026-03-08_toml_adoption.md)
- [ADR-0004: 対話プロンプト用ライブラリとして dialoguer を採用](adr/2026-03-09_dialoguer_adoption.md)

## 🧪 技術的なメモ

AIエージェントが開発中に発見した、将来役立つ可能性のある技術的な知見や、詰まりどころ（Gotchas）をここに記録してください。

- **カバレッジ計測**: `cargo llvm-cov nextest` を使用。詳細は `docs/COVERAGE_RUNBOOK.md` を参照。
- **テンプレート構成**: Harness Engineering に基づく 3 Pillars 構成を採用（2026-03-15）。
