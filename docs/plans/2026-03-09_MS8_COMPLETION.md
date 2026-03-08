# PLAN_TEMPLATE.md

## Plan Meta
- Milestone ID: MS8
- Milestone Name: テンプレート基盤の完成
- Date: 2026-03-09
- Owner: Gemini (AI)
- Status: Agreed

## Goal
- MS8の残タスクである「可変項目/固定項目の整理」と「テンプレート品質の契約テストの追加」を完了し、MS8を正式に達成する。

## Scope
- In Scope:
  - テンプレートの可変項目（`[TODO: ...]`）の抽出とドキュメント化
  - テンプレート品質を保証する契約テスト（`tc004` の拡張または新規追加）の実装
  - `docs/TODO.md` のMS8完了ステータスへの更新
- Out of Scope:
  - 対話モード（MS9）の実装
  - 言語/フレームワークテンプレート（MS10）の追加

## Tasks
- [ ] T1: 可変項目/固定項目の整理
  - `templates/default/` 配下の各ファイルから `[TODO: ...]` となっているプレースホルダーを抽出し、将来的なテンプレートエンジン（Tera/MiniJinjaなど）への移行を見据えた「可変項目一覧」として整理し、ADRまたは専用のドキュメント（`docs/TEMPLATE_VARIABLES.md` など）にまとめる。
- [ ] T2: テンプレート品質の契約テストの追加
  - `tc004_generated_docs_include_process_headings.rs` などを拡張し、`AGENTS.md`, `RULES.md`, `SPEC.md`, `TEST_PLAN.md` などの必須見出しや参照整合（例: `RULES.md` が `AGENTS.md` に言及しているかなど）を保証するテストを追加する。
- [ ] T3: せんぱいのレビューと合意形成（合意後、実装・テストを実行）
- [ ] T4: MS8 の完了処理 (`docs/TODO.md` 更新)

## Iteration Plan
1. Iteration 1: 整理ドキュメントの作成とテスト実装案の提示（T1, T2）
2. Iteration 2: せんぱいレビュー（T3）
3. Iteration 3: テスト実行と完了処理（T4）

## Agreement
- 合意日: 2026-03-09
- 合意内容: T1〜T4のタスクおよび進め方について合意。
- 合意者: せんぱい (ユーザー), Gemini (AI)

## Execution Log
- (empty)

## Re-plan Notes
- (empty)
