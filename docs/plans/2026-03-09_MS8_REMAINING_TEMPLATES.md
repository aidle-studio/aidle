# PLAN_TEMPLATE.md

## Plan Meta
- Milestone ID: MS8 (Partial)
- Milestone Name: 残りのテンプレート（TODO, KNOWLEDGE, AGENT_CONTEXT, README）の汎用化
- Date: 2026-03-09
- Owner: Gemini (AI)
- Status: Done

## Goal
- `templates/default/docs/TODO.md`、`templates/default/docs/KNOWLEDGE.md`、`templates/default/docs/AGENT_CONTEXT.md`、および `templates/default/README.md` を、特定のプロジェクトに依存しない汎用的な初期状態として完成させる。

## Scope
- In Scope:
  - `templates/default/docs/TODO.md` の雛形作成（マイルストーン管理、合意ゲートの運用ルール）
  - `templates/default/docs/KNOWLEDGE.md` の雛形作成（ADRのフォーマット）
  - `templates/default/docs/AGENT_CONTEXT.md` の雛形作成（現在のフェーズ、次アクションのプレースホルダー）
  - `templates/default/README.md` の雛形作成（プロジェクト概要、セットアップ手順のプレースホルダー）
- Out of Scope:
  - アダプタレイヤ（`GEMINI.md`, `CLAUDE.md`, `.github/`）の変更（これらは現状ですでに汎用的なため今回は除外）
  - 実装コードの変更

## Tasks
- [x] T1: `templates/default/docs/TODO.md` の作成
  - タスクのステータス管理（未着手、進行中、完了）と、マイルストーン単位での目標設定のフォーマットを定義する。
- [x] T2: `templates/default/docs/KNOWLEDGE.md` の作成
  - Architecture Decision Record (ADR) のフォーマット（Context, Decision, Consequences）を定義する。
- [x] T3: `templates/default/docs/AGENT_CONTEXT.md` の作成
  - AIが常に最新の状態を把握するための「現在フェーズ」と「直近のゴール・次アクション」をプレースホルダーとして定義する。
- [x] T4: `templates/default/README.md` の作成
  - プロジェクトの顔として、タイトル、概要、セットアップ手順、主要なコマンドを記述するプレースホルダーを定義する。
- [x] T5: せんぱいのレビューと合意形成

## Iteration Plan
1. Iteration 1: 各ファイルの汎用フォーマット作成（T1, T2, T3, T4）
2. Iteration 2: レビューと修正（T5）

## Agreement
- 合意日: 2026-03-09
- 合意内容: T1〜T5のタスクおよび進め方について合意。
- 合意者: せんぱい (ユーザー), Gemini (AI)

## Execution Log
- (empty)

## Re-plan Notes
- (empty)
