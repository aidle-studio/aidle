# PLAN_TEMPLATE.md

## Plan Meta
- Milestone ID: MS8 (Partial)
- Milestone Name: `AGENTS.md` と `RULES.md` テンプレートの汎用化
- Date: 2026-03-09
- Owner: Gemini (AI)
- Status: Agreed

## Goal
- `templates/default/AGENTS.md` と `templates/default/docs/RULES.md` を、特定のプロジェクト（Rustやaidle）に依存しない汎用的なAI駆動開発（AIDD）のベストプラクティスを反映したテンプレートとして完成させる。

## Scope
- In Scope:
  - `templates/default/AGENTS.md` の汎用化・肉付け
  - `templates/default/docs/RULES.md` の汎用化・肉付け
- Out of Scope:
  - `SPEC.md`, `TEST_PLAN.md` など、他テンプレートの修正（別タスクとして扱う）
  - Rustコード（CLI本体）の実装変更

## Tasks
- [ ] T1: `templates/default/AGENTS.md` の作成
  - 現在のプロジェクトの `AGENTS.md` をベースに、特定の言語やツールに依存しない形（例えば「検証」の例からclippyなどを外す）で、TDDサイクルと合意ゲートのルールを汎用化して記述する。
- [ ] T2: `templates/default/docs/RULES.md` の作成
  - 現在の `docs/RULES.md` から、汎用的な「仕様駆動（SDD）」「TDDサイクル」「AI駆動開発の標準手順」「コミット規約」を抽出し、プロジェクト固有の品質ゲート（カバレッジパーセンテージやRust専用ツール）をプレースホルダー化して記述する。
- [ ] T3: せんぱいのレビューと合意形成

## Iteration Plan
1. Iteration 1: 汎用化案の作成と適用（T1, T2）
2. Iteration 2: せんぱいレビューと修正（T3）

## Agreement
- 合意日: 2026-03-09
- 合意内容: T1〜T3のタスクおよび進め方について合意。
- 合意者: せんぱい (ユーザー), Gemini (AI)

## Execution Log
- (empty)

## Re-plan Notes
- (empty)
