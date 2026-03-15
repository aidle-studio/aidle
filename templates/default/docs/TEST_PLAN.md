# TEST_PLAN.md

本ドキュメントは、製品仕様書 (`SPEC.md`) の受け入れ基準 (`AC-*`) を検証するためのテスト計画を定義します。
AI駆動開発（AIDD）において、本ファイルは **「仕様と実装を繋ぐトレーサビリティの要」** となります。

## 🧪 テスト設計の原則

1. **AC-TC トレーサビリティ**: 1つの `AC-*` に対して、少なくとも1つの `TC-*` を作成せよ。
2. **Harness Verification**: すべてのテストは `scripts/check_harness.sh` で一括実行可能であれ。
3. **SDD & TDD**: テストコードは、AIに対する最も明確な「プロンプト」である。

## 🗺️ トレーサビリティ・マトリクス

| TC ID | 対応 AC | 観点 (Normal/Edge/Error) | 実装状況 (Red/Provisional/Green) |
| :--- | :--- | :--- | :--- |
| **TC-001** | AC-001 | Normal | [ ] Red |
| **TC-002** | AC-002 | Edge | [ ] Red |

## 🛠️ テストケース詳細 (TC-*)

### TC-001: [テスト名]
- **事前条件**: [例: DBにデータがある]
- **操作手順**: [例: APIを呼び出す]
- **期待結果**: [例: 200 OK が返る]

## 🚦 テストランナーと環境

- **Runner**: [TODO: `cargo nextest`, `jest`, `pytest` 等]
- **Commands**:
  - Unit: [TODO: コマンド]
  - Integration: [TODO: コマンド]
- **Config**: [TODO: `nextest.toml` 等の設定ファイル]

## 🚀 実装順序 (AIへの指示)

1. `AC-*` に基づく `TC-*` をリストアップせよ。
2. `TC-*` に対応する失敗するテスト（Red）を実装せよ。
3. テストを通す最小実装（Green）を行え。
4. `scripts/check_harness.sh` を通して検証を完了せよ。
