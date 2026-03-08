# MS10 Plan (2026-03-09)

## Plan Meta
- Milestone ID: MS10
- Milestone Name: 言語/フレームワークテンプレート選択
- Date: 2026-03-09
- Owner: Codex
- Status: Agreed

## Goal
- `aidle init` 実行時に、`default` 以外のテンプレートを選択して生成できるようにする。
- 新規テンプレート追加時に既存テンプレートへの影響を最小化する。

## Scope
- In Scope:
  - テンプレート識別子の仕様化（`--template` / `aidle.toml`）
  - 追加テンプレートを1種類以上実装（最初の対象を `rust-cli` とする）
  - テンプレート選択の契約テスト追加
  - ドキュメント更新（SPEC / CONFIG / TEST_PLAN / TODO）
- Out of Scope:
  - 複数言語を同時選択する合成テンプレート
  - リモートテンプレート配布

## Tasks
- [x] T1: テンプレート選択仕様（キー、互換性、デフォルト）を定義する
- [ ] T2: `templates/rust-cli/` を追加する
- [ ] T3: `--template rust-cli` と `template.name = \"rust-cli\"` を受理する
- [ ] T4: 契約テスト（選択成功/未対応値失敗）を追加してGreen化する
- [ ] T5: `docs/SPEC.md` / `docs/CONFIG.md` / `docs/TEST_PLAN.md` / `docs/TODO.md` を更新する
- [ ] T6: 通し検証（`cargo nextest run`）を実施し、結果を記録する

## Iteration Plan
1. Iteration 1: 仕様化（T1, T5の一部）
2. Iteration 2: テンプレート追加（T2）
3. Iteration 3: 実装（T3）
4. Iteration 4: テスト追加・Green化（T4）
5. Iteration 5: 最終反映と検証（T5残, T6）

## Agreement
- 合意日: 2026-03-09
- 合意内容: 本プランでMS10を反復実行する
- 合意者: せんぱい

## Execution Log
- 2026-03-09: MS10開始。Iteration 1 に着手。
- 2026-03-09: Iteration 1 完了。仕様（AC/TC）を定義。

## Re-plan Notes
- (empty)
