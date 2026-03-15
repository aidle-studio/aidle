# 🎤 aidle (アイドル)

> **Ready. Set. AI.** 🚀  
> AI駆動開発の最高傑作を、あなたの手で。

`aidle` は、CursorやWindsurf、Claude CodeなどのAIエディタがプロジェクトの文脈（Context）を即座に理解し、高精度なコード生成を行えるように設計された、**AI駆動開発（AIDD）専用のプロジェクト基盤生成ツール**です。

OpenAIや逆瀬川氏（@gyakuse）が提唱する「Harness Engineering」のベストプラクティスを凝縮し、AIが自律的に最高の仕事場を設営できる「ステージ」を提供します。

---

## ✨ 3つのコアコンセプト (3 Pillars)

`aidle` は、以下の3つの柱に基づいてプロジェクトをセットアップします。

1. **🛠️ Harness Engineering (機械的制約)**: 
   AIへの「お願い」を卒業し、リンターやテストスクリプトによる「機械的な合格」を完了条件（DoD）とします。AIが自律的に検証レールを設営するための `HARNESS.md` を提供します。
2. **🧵 Specification-Driven Development (SDD)**: 
   仕様（REQ/AC）とテスト（TC）のトレーサビリティを徹底。AIに対して「曖昧さのないゴール」を提示します。
3. **🗺️ Context Engineering (段階的開示)**: 
   巨大なプロンプトを避け、AIの短期記憶を節約するために最適化された階層ドキュメント構造（OpenAI Style）を採用。

---

## 🚀 クイックスタート

```bash
# 新しいプロジェクトディレクトリを作成して初期化
aidle init my-awesome-app

# 既存のプロジェクトにAIDD規約を導入
cd my-project
aidle init
```

## 🧭 開発を加速させる `aidle-skill`

Gemini CLI ユーザー向けに、`aidle` プロジェクトでの開発を120%加速させる **`aidle-skill`** も提供しています。

* **Harness Pilot**: 言語に合わせた高速リンターと自動修正フックをAIが自律的にセットアップ。
* **SDD Weaver**: 仕様書からテストケースへの書き出しとトレーサビリティ管理を自動化。
* **Context Manager**: `AGENT_CONTEXT.md` の状態をAIが常に最新に保ちます。

---

## 📁 生成されるプロジェクト構造 (AI-Optimized)

AIが「迷わず、嘘をつかず、爆速で自己修正できる」ための OpenAI / 逆瀬川スタイルのディレクトリツリーを生成します。

```text
/ (Project Root)
├── AGENTS.md                  # 50行以内の極小マップ（AIの入り口）
├── ARCHITECTURE.md            # プロジェクトの不変の地図
├── docs/                      # システム・オブ・レコード（正本）
│   ├── HARNESS.md             # 【最重要】検証レールの設営ガイド
│   ├── RULES.md               # 開発プロセス（計画→ハーネス→実装→検証）
│   ├── product-specs/         # 仕様駆動開発の正本 (REQ/AC)
│   ├── exec-plans/            # 実行プラン（active/completed）
│   ├── adr/                   # 決定履歴 (ADR)
│   └── ...                    # 各専門領域の規約 (SECURITY, RELIABILITY等)
└── scripts/
    └── check_harness.sh       # 完了条件（DoD）の一括チェッカー
```

## 💡 なぜ "aidle" なのか？

AI駆動開発において、コードの質は「与えられた文脈」の質で決まります。
aidle は、人間が手動で行っていた「文脈の整理」と「環境のガードレール設置」を自動化。
プロジェクト開始1分後から、AIがあなたの専属プロデューサーとしてフルパフォーマンスを発揮できる状態を作ります。

> Made for the next generation of developer stars. 🌟🍛
