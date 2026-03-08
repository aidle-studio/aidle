# TEST_PLAN.md

## 1. 目的
本書は、仕様駆動開発（SDD）で定義した受け入れ基準（`AC-*`）を、テスト駆動開発（TDD）で実装可能なテストケース（`TC-*`）へ変換するための計画書である。

## 2. トレーサビリティ（AC -> TC）
- `AC-001` -> `TC-001`, `TC-002`, `TC-003`
- `AC-002` -> `TC-004`
- `AC-003` -> `TC-005`
- `AC-004` -> `TC-006`, `TC-007`
- `AC-005` -> `TC-008`, `TC-009`
- `AC-006` -> `TC-010`, `TC-011`
- `AC-001` -> `TC-012`
- `AC-006` -> `TC-013`
- `AC-007` -> `TC-014`
- `AC-008` -> `TC-015`, `TC-026`, `TC-027`
- `AC-009` -> `TC-016`
- `AC-010` -> `TC-017`, `TC-018`
- `AC-004` -> `TC-019`
- `AC-004` -> `TC-020`, `TC-021`
- `AC-009` -> `TC-021`
- `AC-011` -> `TC-022`
- `AC-012` -> `TC-023`
- `AC-013` -> `TC-024`
- `AC-014` -> `TC-025`

## 3. テストケース一覧（初版 / Red）

| TC ID | 対応AC | レベル | 観点 | 初期状態 |
|---|---|---|---|---|
| TC-001 | AC-001 | integration | `aidle init` でカレント配下にGC-1必須ファイルが生成される | Green |
| TC-002 | AC-001 | integration | `aidle init <dir>` で指定ディレクトリ配下にGC-1必須ファイルが生成される | Green |
| TC-003 | AC-001 | integration | `--dry-run` 時にファイルは作成されず、作成予定のみ出力される | Green |
| TC-004 | AC-002 | integration | 生成ドキュメントに開発手順・合意ゲートを示す必須見出しが含まれる | Green |
| TC-005 | AC-003 | integration | TDDサイクルとDefinition of Doneが規約ファイルに存在する | Green |
| TC-006 | AC-004 | integration | 引数エラー時に終了コード2と原因/対処メッセージが出力される | Green |
| TC-007 | AC-004 | integration | I/Oエラー時に終了コード3と原因/対処メッセージが出力される | Green |
| TC-008 | AC-005 | integration | 途中失敗時、新規作成ファイルがロールバックで削除される | Green |
| TC-009 | AC-005 | integration | `--force` 上書き失敗時、既存ファイルが実行前状態へ復元される | Green |
| TC-010 | AC-006 | integration | 既存ディレクトリ導入時、既存ファイルは上書きされず `skipped` になる | Green |
| TC-011 | AC-006 | integration | 既存ディレクトリ導入時、`--force` で既存ファイルが上書きされる | Green |
| TC-012 | AC-001 | integration | `aidle.toml` の `execution.dry_run=true` がCLI未指定時に適用される | Green |
| TC-013 | AC-006 | integration | `aidle.toml` の `execution.force=true` がCLI未指定時に適用される | Green |
| TC-014 | AC-007 | integration | `--json` 指定時に `created/updated/skipped/errors` を含むJSONを返す | Green |
| TC-015 | AC-008 | integration | `--non-interactive` 指定時に非対話モードが有効化される | Green |
| TC-016 | AC-009 | integration | `--output` 指定時に指定パス配下へ生成される | Green |
| TC-017 | AC-010 | integration | 未対応 `--template` 値で終了コード2と原因/対処を返す | Green |
| TC-018 | AC-010 | integration | 未対応 `--agent-format` 値で終了コード2と原因/対処を返す | Green |
| TC-019 | AC-004 | integration | テンプレート読み込み失敗時に終了コード3と原因/対処を返す | Green |
| TC-020 | AC-004 | integration | 引数の境界ケース（サブコマンド不足・値不足・競合指定）で終了コード2と原因/対処を返す | Green |
| TC-021 | AC-004, AC-009 | integration | 設定ファイルの境界ケース（不正TOML・project root解決）とverbose出力契約を満たす | Green |
| TC-022 | AC-011 | integration | 適応レイヤ生成を有効化した場合、GC-2対象ファイルが生成される | Green |
| TC-023 | AC-012 | integration | 適応レイヤ生成が無効（既定）な場合、GC-2対象ファイルは生成されない | Green |
| TC-024 | AC-013 | integration | `--stats-out` または設定指定時に統計JSONが保存される | Green |
| TC-025 | AC-014 | integration | 統計ログ未指定（既定）時は統計ファイルが生成されない | Green |
| TC-026 | AC-008 | integration | TTY接続時かつ対話モード有効時にプロンプトが表示され入力値が反映される | Green |
| TC-027 | AC-008 | integration | `execution.non_interactive=true` 設定時にプロンプトがスキップされる | Green |

## 4. 実装順序（推奨）
1. `TC-001`, `TC-002`（基本生成）
2. `TC-003`（dry-run）
3. `TC-006`, `TC-007`（エラー契約）
4. `TC-008`, `TC-009`（ロールバック）
5. `TC-010`, `TC-011`（部分導入）
6. `TC-004`, `TC-005`（ドキュメント品質）

## 5. テスト実装ガイド（Rust）
- 統合テストは `assert_cmd` + `tempfile` を基本とする。
- 生成ファイル検証は存在確認と必須見出し確認を分離する。
- 将来、出力の構造化検証が必要な場合は `insta` の導入を検討する。
- テストランナーは `cargo nextest` を標準とする。

## 6. カバレッジルール
- 計測対象は `src/` 配下のRustコードとする。
- 最低基準は行カバレッジ 80% とする。
- 変更対象モジュールの差分カバレッジは 90% を原則必須（CIゲート）とする。
- 必須フロー（`TC-001`〜`TC-027`）に対応する統合テストは、常に実行対象とする。
- 差分カバレッジ未達を例外的に許容する場合、PRに未達理由、期限付き追補予定、合意記録を記載する。

## 7. テスト実行品質（タイムアウト）
- 既定タイムアウトは 15 秒とする。
- `slow` に分類したテストのみ 45 秒まで許可する。
- `very_slow` は例外分類とし、90 秒まで許可する（PRで理由必須）。
- 設定は `nextest.toml` で管理する。
- タイムアウトが3回連続で発生したテストは flaky 扱いとして優先修正対象にする。

## 8. 計測手順（Rust）
1. `cargo nextest run`
2. `cargo llvm-cov nextest --workspace --lcov --output-path coverage.lcov`
3. レポートを確認し、全体80%以上かつ変更モジュール90%以上（原則必須）を判定する
4. 詳細運用は `docs/COVERAGE_RUNBOOK.md` に従う
5. CIでは `.github/workflows/coverage-gate.yml` により自動実行する

## 9. 合意・更新ルール
- 新しい `AC-*` が追加された場合、同一PR内で `TC-*` を追加する。
- `TC-*` の状態変更（Red -> Green）は、対応実装と同時に更新する。
