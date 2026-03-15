---
name: aidle-skill
description: "aidleプロジェクトのためのプロフェッショナルなエージェント・ワークフロー。Harnessの設営、仕様(SDD)とテスト(TDD)の連携、およびコンテキストの同期が必要な際に使用します。"
---

# aidle Pro Agent Workflow

このスキルは、`aidle` によって初期化されたプロジェクトにおいて、AIエージェントが最大のパフォーマンスを発揮するための「プロフェッショナルな仕事術」を提供します。

## 1. Harness Pilot (環境設営と自己修復)
プロジェクト参画時や新しい環境に入った際は、真っ先に検証環境（Harness）を設営してください。

*   **言語別のベストプラクティス**: [references/harness-patterns.md](references/harness-patterns.md) を参照し、最も高速なリンターや自動修正フックをセットアップしてください。
*   **完了条件**: `scripts/check_harness.sh` がエラーなく実行できる状態を維持すること。

## 2. SDD Weaver (仕様とテストの仲介)
コーディングを始める前に、必ず仕様とテストのトレーサビリティを確保してください。

*   **詳細手順**: [references/sdd-workflows.md](references/sdd-workflows.md) を参照。
*   **原則**: 仕様 (`SPEC.md` / `AC-*`) -> テスト (`TEST_PLAN.md` / `TC-*`) -> 実装 (Code) の順序を厳守すること。

## 3. Context Manager (文脈の同期)
セッションの開始時、またはマイルストーンの変更時には、プロジェクトの文脈（コンテキスト）を常に最新に保ってください。

*   **同期の実行**:
    必要に応じて `python3 scripts/sync_context.py` を実行（またはスクリプトを拡張）し、`docs/AGENT_CONTEXT.md` の「現在フェーズ」と「次アクション」を正しく更新してください。

## 4. Quality Guardian (品質の保証)
いかなる変更も、`docs/RULES.md` および `docs/QUALITY_SCORE.md` の基準を満たす必要があります。変更後は、人間による承認（Agreement Gate）を求める前に、必ず自己検証（Harnessの実行）を完了させてください。
