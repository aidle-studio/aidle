# TODO.md

## 運用ルール

- 全てのタスクは **「計画 ➔ ハーネス ➔ 実装 ➔ 検証」** のサイクルで進める。
- 計画は `docs/exec-plans/active/` に保存し、人間と合意（Agreement Gate）すること。
- 合意が取れていない項目は完了扱いにしない。
- 進捗はセッションごとに `docs/AGENT_CONTEXT.md` に反映し、本ファイルの完了マーク（[x]）を更新すること。
- 各マイルストーンで、追加/変更した `AC-*` に対応する `TC-*`（integration + 必要なunit）を同時に追加する。
- 完了宣言の前に必ず `scripts/check_harness.sh` を実行し、機械的な合格を確認すること。

## Milestones

### Phase 1: Core Foundation（完了）
- 目的: `aidle init` の基盤機能・品質ゲート・適応レイヤ・統計保存までを実装する。

### MS1: 仕様基盤の確立（完了）
**TODO**
- [x] Product Scope確定（P1-1）
- [x] CLI UX設計（P1-2）
- [x] 生成物契約定義（P1-3）
- [x] 品質要件数値化（P1-4）
- [x] 受け入れ基準のテスト計画化（P1-5）
- [x] ADR整理（P1-6）
- [x] `aidle.toml` スキーマ初版定義（ADR-0003 Follow-up）

### MS2: Core Init動作の仕様準拠（完了）
**TODO（完了済み）**
- [x] `TC-001`〜`TC-011` を Green 化
- [x] `execution.dry_run` / `execution.force` の config 読み込みを実装

### MS3: CLIオプションと設定適用の完成（完了）
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
**TODO**
- [x] テンプレート実体を `templates/default/` に分離する
- [x] コード直書きテンプレートを廃止する
- [x] テンプレート読み込み失敗時のエラー契約を追加する

### MS5: 品質ゲート運用の定着（完了）
**TODO**
- [x] MS5プランを作成し、合意を記録する
- [x] カバレッジ運用Runbookを整備する
- [x] カバレッジ未達時の例外運用テンプレートを整備する
- [x] `cargo llvm-cov nextest` をCI品質ゲートへ組み込む
- [x] 差分カバレッジ90%の自動判定を導入する
- [x] 通し検証（ローカル/CI）を実施し、MS5達成を記録する

### MS6: マルチエージェント適応レイヤ生成（完了）
**TODO**
- [x] MS6プランに合意する
- [x] GC-2拡張仕様（最小内容・生成スイッチ）とAC-011/AC-012を定義する
- [x] `.github/copilot-instructions.md` と `.github/instructions/*.instructions.md` の生成テンプレートを追加する
- [x] `GEMINI.md` / `CLAUDE.md` の互換アダプタ生成を追加する
- [x] 生成ON/OFFのCLIオプションと設定キーを追加する
- [x] 契約テストを追加してGreen化する
- [x] 通し検証（`cargo nextest run`）とドキュメント更新を完了する

### MS7: 実行統計ログ保存（完了）
**TODO**
- [x] MS7プランに合意する
- [x] 統計ログ契約（保存項目・保存条件・失敗時挙動）を仕様化する
- [x] 統計ログのCLIオプションと設定キーを追加する
- [x] 統計JSON保存処理を実装する
- [x] 契約テストを追加してGreen化する
- [x] 通し検証（`cargo nextest run`）とドキュメント更新を完了する

### Phase 2: Product Completeness（未着手）
- 目的: `SPEC.md` の FR-4 / FR-5 とユースケース上の未実装要件を実装する。

### MS8: テンプレート基盤の完成（完了）
**TODO**
- [x] `templates/default/` の各ファイルを完成版として見直す（内容不足・重複・矛盾の解消）
- [x] テンプレート内の可変項目を明確化し、将来テンプレート追加の差分点を整理する
- [x] テンプレート品質の契約テスト（必須見出し/参照整合）を追加する

### MS9: 対話モードの実装（完了）
**TODO**
- [x] 不足情報の判定ルールを仕様化する
- [x] 対話プロンプト実装（入力検証・再入力・キャンセル）を追加する
- [x] `--non-interactive` / TTY自動判定との整合テストを追加する

### MS10: 言語/フレームワークテンプレート選択（完了）
**TODO**
- [x] テンプレート選択仕様（キー、互換性、デフォルト）を定義する
- [x] 少なくとも1つの追加テンプレートを実装する
- [x] テンプレートごとの差分契約テストを追加する
- [x] MS10プランに沿って完了まで反復実行する

### Phase 4: Harness Rule Hardening（完了）
- 目的: Harness Engineeringの知見を取り込み、テンプレート生成物のルールを「短く・検証可能・運用可能」に強化する。

### MS14: テンプレートの抜本적再設計（完了）
**TODO**
- [x] `templates/rust-cli/` を削除し、CLI 実装とテストを `default` 一本に修正する
- [x] `templates/default/AGENTS.md` を 50行以内のポインタ型マップに書き換える
- [x] `templates/default/ARCHITECTURE.md` を新規作成し、不変の地図を定義する
- [x] `templates/default/docs/HARNESS.md` を作成し、自己設営ハーネスのガイドラインを定義する
- [x] `templates/default/docs/` 配下を刷新する
- [x] `templates/default/scripts/check_harness.sh` を配置し、機械的検証レールの器を用意する
- [x] 統合テスト (`tests/tc001`, `tc004`, `tc005` 等) を新構成に合わせて修正・強化する
- [x] `cargo nextest run` で全テストの Green を確認する

### Phase 5: Self-Evolving Ecosystem（実行中）
- 目的: プロジェクト自身のドキュメンテーションとツールを刷新し、自食（Dogfooding）を開始する。

### MS15: プロジェクト自身の再初期化（Dogfooding）
**達成条件**
- `aidle` 自身が最新の「3本柱」テンプレートで構成されている。
- `scripts/check_harness.sh` が Rust 向けに実稼働している。

**TODO**
- [x] 現状のコンテキストをバックアップする
- [x] `aidle init` でプロジェクトを再初期化する
- [x] 既存の記憶（TODO, ADR, SPEC, TEST_PLAN）を新構造へマージする
- [x] `ARCHITECTURE.md` を実体に合わせて更新する
- [x] `scripts/check_harness.sh` を Rust ツールで実装する
- [ ] `aidle-skill` を活用してコンテキストを最終同期する

## 📊 進捗サマリー

- **全タスク数**: 60
- **完了タスク数**: 56
- **現在のマイルストーン**: MS15
