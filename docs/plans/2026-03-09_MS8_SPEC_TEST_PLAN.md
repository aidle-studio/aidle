# PLAN_TEMPLATE.md

## Plan Meta
- Milestone ID: MS8 (Partial)
- Milestone Name: `SPEC.md` と `TEST_PLAN.md` テンプレートの汎用化
- Date: 2026-03-09
- Owner: Gemini (AI)
- Status: Agreed

## Goal
- `templates/default/docs/SPEC.md` と `templates/default/docs/TEST_PLAN.md` を、仕様駆動開発（SDD）およびテスト駆動開発（TDD）のトレーサビリティ（AC-* -> TC-*）を担保するための汎用フォーマットとして完成させる。

## Scope
- In Scope:
  - `templates/default/docs/SPEC.md` の雛形作成（要求、スコープ、受け入れ基準のフォーマット）
  - `templates/default/docs/TEST_PLAN.md` の雛形作成（ACからTCへのマッピング、テストケース管理表のフォーマット）
- Out of Scope:
  - 既存プロジェクト（`aidle` 自身）の仕様書の変更
  - 他のテンプレートの修正

## Tasks
- [ ] T1: `templates/default/docs/SPEC.md` の作成
  - プロジェクトの目的、対象ユーザー、スコープ（In/Out）、機能要件（要求: `REQ-*` と 受け入れ基準: `AC-*`）を記述するための構造を定義する。
- [ ] T2: `templates/default/docs/TEST_PLAN.md` の作成
  - 仕様（`AC-*`）とテストケース（`TC-*`）の紐付け（トレーサビリティ）を管理する表、およびRed/Greenの状態を管理するフォーマットを定義する。
- [ ] T3: せんぱいのレビューと合意形成

## Iteration Plan
1. Iteration 1: `SPEC.md` と `TEST_PLAN.md` の汎用フォーマット作成（T1, T2）
2. Iteration 2: レビューと修正（T3）

## Agreement
- 合意日: 2026-03-09
- 合意内容: T1〜T3のタスクおよび進め方について合意。
- 合意者: せんぱい (ユーザー), Gemini (AI)

## Execution Log
- (empty)

## Re-plan Notes
- (empty)
