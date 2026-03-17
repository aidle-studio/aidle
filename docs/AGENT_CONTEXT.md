# AGENT_CONTEXT.md

AIエージェント向けの最重要コンテキストです。
本ファイルは、プロジェクトの **「現在進行形」の状態を管理する動的な地図** です。

## 📍 現在フェーズ

- フェーズ名: MS3_PURIFICATION (aidle 浄化作戦)
- 現在のステータス: 実行中 (Refactoring Preparation)

## 🎯 直近のゴール

- `src/main.rs` を解体し、`core`, `cli`, `utils` モジュールに分割してテスト可能性を高める。
- 現状の「赤点」品質スコアを正しく記録し、改善の道筋をつける。

## 📅 次のアクション (Next Actions)

- [ ] `QUALITY_SCORE.md` を現在の実態（カバレッジ、同期不足等）で更新する。
- [ ] `src/lib.rs` を作成し、`main.rs` からテンプレートロード等の非I/Oロジックを移動する。
- [ ] 分割したロジックに対して最初の UT (Branch 100%) を実装する。

## 📖 優先ドキュメント読了順

1. **[AGENTS.md](../AGENTS.md)**: カイゼン魂の確認。
2. **[docs/TODO.md](TODO.md)**: マイルストーン3の詳細確認。
3. **[AGENT_CONTEXT.md](AGENT_CONTEXT.md)**: 本ファイル（現在の文脈）。
4. **[RULES.md](RULES.md)**: Tier 1 基準の再確認。

## 📊 進捗サマリー

- MS3: 20% 完了 (Documentation normalized)
- 残件タスク数: 4件 (MS3)
- 未解消の技術負債: 1件 (Monolithic main.rs)
- **Self-Review: [x]** (Documentation sync plan reviewed)
