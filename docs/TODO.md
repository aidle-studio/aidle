# TODO.md

## 運用ルール
- 各タスクは「提案作成 -> せんぱい合意 -> ドキュメント反映」の順で進める。
- 合意が取れていない項目は完了扱いにしない。
- すべてのタスクは、必ずどれかのマイルストーンに所属させる。

## Milestones

### MS1: 仕様基盤の確立（完了）
**達成条件**
- 仕様・受け入れ基準・テスト計画・ADRが揃い、実装開始可能である。

**TODO**
- [x] Product Scope確定（P1-1）
- [x] CLI UX設計（P1-2）
- [x] 生成物契約定義（P1-3）
- [x] 品質要件数値化（P1-4）
- [x] 受け入れ基準のテスト計画化（P1-5）
- [x] ADR整理（P1-6）
- [x] `aidle.toml` スキーマ初版定義（ADR-0003 Follow-up）

### MS2: Core Init動作の仕様準拠（完了）
**達成条件**
- AC-001〜AC-006 を満たす主要フローが実装され、対応テストが Green である。

**TODO（完了済み）**
- [x] `TC-001`〜`TC-011` を Green 化
- [x] `execution.dry_run` / `execution.force` の config 読み込みを実装

**TODO（未完了）**
- [x] `TC-012` / `TC-013` を `docs/TEST_PLAN.md` に正式追記する（ACトレーサビリティ維持）
- [x] `nextest.toml` を作成し、タイムアウト規約（15s/45s/90s）を実設定する

### MS3: CLIオプションと設定適用の完成（完了）
**達成条件**
- `SPEC.md` で定義した主要オプションと config キーが実装済みで、契約テストが存在する。

**TODO**
- [x] `--json` を実装し、`created/updated/skipped/errors` を機械可読で返す
- [x] `--non-interactive` を実装し、TTYなし時の自動非対話モードを反映する
- [x] `--output <path>` を実装し、優先順位（CLI > config > default）を担保する
- [x] `--verbose` を実装し、通常出力と詳細ログを分離する
- [x] `--template <name>` を実装し、未対応テンプレート時は終了コード `2` で失敗させる
- [x] `--agent-format <name>` を実装する（初期は `agents-md` のみ正式サポート）
- [x] `aidle.toml` の未実装キー（`template.name`, `agent.format`, `execution.non_interactive`, `execution.verbose`, `execution.json`）を反映する
- [x] `--json` 契約テスト（`TC-014`）を追加する
- [x] `--non-interactive` 契約テスト（`TC-015`）を追加する
- [x] `--output` 契約テスト（`TC-016`）を追加する
- [x] `--template` / `--agent-format` 契約テスト（`TC-017` / `TC-018`）を追加する

### MS4: テンプレート分離と保守性向上（完了）
**達成条件**
- テンプレートが `templates/` に分離され、実装ロジックとテンプレデータが独立している。

**TODO**
- [x] テンプレート実体を `templates/default/` に分離する
- [x] コード直書きテンプレートを廃止する
- [x] テンプレート読み込み失敗時のエラー契約を追加する

### MS5: 品質ゲート運用の定着（完了）
**達成条件**
- カバレッジ基準（全体80%/差分90%）を継続計測し、運用フローに組み込む。

**TODO**
- [x] MS5プランを作成し、合意を記録する（`docs/plans/2026-03-08_MS5.md`）
- [x] カバレッジ運用Runbookを整備する（`docs/COVERAGE_RUNBOOK.md`）
- [x] カバレッジ未達時の例外運用テンプレートを整備する（`docs/COVERAGE_EXCEPTION_TEMPLATE.md`）
- [x] `cargo llvm-cov nextest` をCI品質ゲートへ組み込む
- [x] 差分カバレッジ90%の自動判定を導入する
- [x] 通し検証（ローカル/CI）を実施し、MS5達成を記録する

### MS6: マルチエージェント適応レイヤ生成（計画中）
**達成条件**
- `.github` 系と `GEMINI.md` / `CLAUDE.md` の互換アダプタを任意生成できる。
- `AGENTS.md` 正本 + アダプタ補助の運用が仕様・テストで担保される。

**TODO**
- [x] MS6プランに合意する（`docs/plans/2026-03-09_MS6.md`）
- [x] GC-2拡張仕様（最小内容・生成スイッチ）とAC-011/AC-012を定義する
- [x] `.github/copilot-instructions.md` と `.github/instructions/*.instructions.md` の生成テンプレートを追加する
- [x] `GEMINI.md` / `CLAUDE.md` の互換アダプタ生成を追加する
- [x] 生成ON/OFFのCLIオプションと設定キーを追加する
- [x] 契約テストを追加してGreen化する
- [ ] 通し検証（`cargo nextest run`）とドキュメント更新を完了する

## 改善アイデア（マイルストーン外）
- [ ] コマンド実行の統計ログ（所要時間、出力件数）を保存できるようにする
