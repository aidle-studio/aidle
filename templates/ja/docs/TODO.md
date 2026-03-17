# TODO.md

## 運用ルール

- 全てのタスクは **「計画 ➔ ハーネス ➔ 実装 ➔ 検証」** のサイクルで進める。
- 合意（Agreement Gate）が取れていない項目は完了扱いにしない。
- 進捗はセッションごとに `docs/AGENT_CONTEXT.md` に反映し、本ファイルの完了マーク（[x]）を更新すること。

## Milestones

### MS1: 基盤構築とハーネス設営 (Bootstrapping)
**達成条件**
- プロジェクトの言語・環境が決定され、`check_harness.sh` が動作している。
- `ARCHITECTURE.md` と `SPEC.md` の初期版が作成され、合意されている。

**TODO (AIエージェントへの指示)**
- [ ] [ ] **Step 1**: ユーザーに開発言語と主要なフレームワークをヒアリング（または推測）し、合意する。
- [ ] [ ] **Step 2**: 決定した言語に合わせて `scripts/check_harness.sh` を実装し、自分のための検証レール（Harness）を設営する。
- [ ] [ ] **Step 3**: プロジェクトの目的をヒアリングし、`ARCHITECTURE.md` (技術スタック) と `SPEC.md` (初期要件 AC-001) を作成する。

### MS2: 主要機能の実装 (Phase 1)
**達成条件**
- `TC-001` が Green であり、`scripts/check_harness.sh` がパスしている。

**TODO**
- [ ] [ ] TC-001 を Red で実装する
- [ ] [ ] TC-001 を Green にする
- [ ] [ ] リファクタリングを実施し、`QUALITY_SCORE.md` を更新する

## 📊 進捗サマリー

- **全タスク数**: 5
- **完了タスク数**: 0
- **現在のマイルストーン**: MS1
