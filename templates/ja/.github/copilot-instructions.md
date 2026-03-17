# Copilot Instructions

このリポジトリでは `AGENTS.md` を正本（Source of Truth）として扱います。
実装・仕様・テストの判断は、常に以下の順序で参照してください。

1. `AGENTS.md`
2. `docs/RULES.md`
3. `docs/SPEC.md`
4. `docs/TEST_PLAN.md`
5. `docs/TODO.md`
6. `docs/KNOWLEDGE.md`

## 実行ルール（要約）
- 仕様駆動（SDD）とテスト駆動（TDD）を必須とする。
- 実装前に対応 `TC-*` を先に追加し、`Red -> Green -> Refactor` を守る。
- 変更はトレーサビリティ（`AC-*` と `TC-*` の対応）を維持する。

## 禁止事項（要約）
- 合意未了の仕様を前提に実装を進めない。
- 既存ルールや既存ファイルを無断で破壊的に変更しない。
- `AGENTS.md` と矛盾する独自ルールを正本として扱わない。
