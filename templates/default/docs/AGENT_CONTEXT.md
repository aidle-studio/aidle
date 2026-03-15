# AGENT_CONTEXT.md

AIエージェント向けの最重要コンテキストです。
本ファイルは、プロジェクトの **「現在進行形」の状態を管理する動的な地図** です。
セッションの開始時、または状況の変化があった際に、自律的に最新状態に更新してください。

## 📍 現在フェーズ

- フェーズ名: [例: MS1_SETUP / Phase 2 Implementation]
- 現在のステータス: [例: 実行中 / 計画策定中 / 検証中]

## 🎯 直近のゴール

- [例: テンプレート刷新のための AGENTS.md を作成する]
- [例: AC-001 のユニットテストを Green にする]

## 📅 次のアクション (Next Actions)

- [ ] [例: ARCHITECTURE.md を作成し、構成案を提示する]
- [ ] [例: scripts/check_harness.sh のベースラインを実装する]

## 📖 優先ドキュメント読了順

AIエージェントが状況を正確に把握するために、以下の順序でドキュメントを読み込むことを推奨します。

1. **[AGENTS.md](../AGENTS.md)**: 全体のポインタ。
2. **[AGENT_CONTEXT.md](AGENT_CONTEXT.md)**: 本ファイル（現在の文脈）。
3. **[HARNESS.md](HARNESS.md)**: 検証環境の確認。
4. **[TODO.md](TODO.md)**: 全体の進捗状況。
5. **[RULES.md](RULES.md)**: 開発ルールとDoDの確認。

## 📊 進捗サマリー

- [マイルストーンID]: [XX]% 完了
- 残件タスク数: [N]件
- 未解消の技術負債: [M]件（詳細は `docs/exec-plans/tech-debt.md` を参照）
