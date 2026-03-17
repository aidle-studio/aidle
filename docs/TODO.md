# TODO.md

## 運用ルール

- 全てのタスクは **「計画 ➔ ハーネス ➔ 実装 ➔ レビュー ➔ 検証」** のサイクルで進める。
- 合意（Agreement Gate）が取れていない項目は完了扱いにしない。
- 進捗はセッションごとに `docs/AGENT_CONTEXT.md` に反映し、本ファイルの完了マーク（[x]）を更新すること。
- **断捨離 (GC)**: 完了したプランは `completed/` へ移動し、教訓を `KNOWLEDGE.md` に抽出すること。

## Milestones

### MS1: 基盤構築と初期テンプレート開発 [DONE]
**達成条件**
- Rust による CLI のプロトタイプが動作し、初期テンプレートが生成できる。
- GitHub Actions による CI/CD（Release Build）が構築されている。

**TODO**
- [x] プロジェクトの初期化 (`aidle init`) 機能の実装
- [x] 埋め込みテンプレートエンジンの実装
- [x] 基本的なドキュメントセット（RULES, SPEC等）の作成
- [x] `tc001`〜`tc030` までの統合テストの実装
- [x] リリース用 CI/CD パイプラインの構築

### MS2: AIDD ワークフローの究極進化 [DONE]
**達成条件**
- テンプレートが Tier 別テスト密度、自己レビュー、Secure by Design に対応。
- `aidle check` サブコマンドにより、構造的進化が可能になっている。

**TODO**
- [x] テンプレートの網羅的アップデート（RULES, TEST_PLAN, SECURITY 等）
- [x] `aidle check` (Structural Concept Diff) 機能の実装
- [x] `aidle-skill` (システムスキル) の最新化とインストール
- [x] 本プロジェクト自体への `aidle check` の適用とマージ

### MS3: aidle 浄化作戦（Tier 1 基準への完全準拠）[IN PROGRESS]
**達成条件**
- `src/main.rs` のモジュール分割が完了し、lib 化されている。
- 分岐網羅 100% を目指す UT が実装され、FMEA が網羅されている。
- `QUALITY_SCORE.md` が GREEN (Tier 1 合格) になっている。

**TODO**
- [x] ドキュメント（TODO, AGENT_CONTEXT, QUALITY_SCORE）の正常化
- [ ] `src/main.rs` のリファクタリング（`lib.rs` とサブモジュールへの分割）
- [ ] 分割した各モジュールへの UT 実装（分岐網羅 100% 目標）
- [ ] `docs/TEST_PLAN.md` への FMEA（失敗モード）の追記とテスト補完
- [ ] 全体のセキュリティ再監査と Hardening

## 📊 進捗サマリー

- **全タスク数**: 15
- **完了タスク数**: 11
- **現在のマイルストーン**: MS3 (Purification)
