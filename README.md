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

## 🔌 AI Adapters (AIアダプタ)

主要なAIエディタやツール（GitHub Copilot, Gemini CLI, Claude Code等）が、`aidle` の規約をスムーズに理解できるようにするための「薄いアダプタ（設定ファイル）」を生成します。

* **対応ツール**: GitHub Copilot, Gemini CLI, Claude Code
* **生成方法**: `aidle init --with-adapters` を実行、または対話モードで `Yes` を選択。
* **役割**: `AGENTS.md` へのポインタとして機能し、どのツールを使っても一貫した開発体験を提供します。

---

## 📁 生成されるプロジェクト構造 (aidle v2 Architecture)

AIが「迷わず、嘘をつかず、コンテキストを溢れさせない」ための究極のディレクトリツリー（The Ultimate AIDD Protocol）を生成します。

```text
/ (Project Root)
├── README.md                  # 人間向け概要＆AIへの入り口
├── AGENTS.md                  # ★起点: エージェントの目次・ハブ
├── PRD.md                     # ★最上位仕様 (What): 機能一覧
├── ARCHITECTURE.md            # ★基本仕様書 (How): システム全体像
│
├── docs/                      # システム・オブ・レコード（正本）
│   ├── RULES.md               # 憲法: 開発の掟と各法律へのポータル
│   ├── HARNESS.md             # 環境: 検証ツールの自己修復ルール
│   ├── AGENT_CONTEXT.md       # 動的文脈: 今何してるか（現在地）
│   │
│   ├── PLAN_TEMPLATE.md       # 雛形: 強制思考プロンプト付きプラン
│   ├── TEST_PLAN.md           # 法律: Tier別テスト密度・FMEA
│   ├── RELIABILITY.md         # 法律: 冪等性・TraceID・エラー処理
│   ├── AUDIT.md               # 法律: コードレビューと自己監査基準
│   ├── QUALITY_SCORE.md       # 状態: 現在の品質ダッシュボード
│   │
│   ├── TODO.md                # 全体のマクロなマイルストーン
│   ├── PLANS.md               # 全体のロードマップ
│   ├── KNOWLEDGE.md           # 教訓と暗黙知の蓄積
│   │
│   ├── features/              # 機能仕様書 (Whatの詳細/AC)
│   ├── design-docs/           # 詳細設計書 (Howの詳細/DB/API)
│   ├── adr/                   # 技術的決定の記録
│   └── exec-plans/            # 現場のタスク実行プラン
│
└── scripts/
    └── check_harness.sh       # 完了条件（DoD）の一括チェッカー
```

---

## 🔄 The Ultimate Loop (エージェントの完璧な導線)

aidle v2 のアーキテクチャは、AIがタスクを実行する上で「絶対にサボれない・迷子にならない」ための思考のレール（導線）を提供します。

1. **参画と状況把握**: `AGENTS.md` を起点に `PRD.md` や `ARCHITECTURE.md` を読み込み、`TODO.md` から次のマイルストーンを把握する。
2. **要件と設計 (Spec & Design)**: いきなりコードを書かず、まずは `FEATURE_TEMPLATE.md` と `DESIGN_TEMPLATE.md` を使って「何を作るか(What)」と「どう作るか(How)」を定義し、人間の合意を得る。
3. **実行計画 (Plan)**: 仕様と設計をもとに `PLAN_TEMPLATE.md` を作成。隠しプロンプトに怒られながら、FMEAマトリクス（異常系のテスト観点）を強制的に埋めさせられる。
4. **実装と自己修復 (Act & Harness)**: テスト駆動(TDD)で実装し、エラーが出れば `HARNESS.md` の自己修復ルールに従ってLinterやコンパイラと対話する。
5. **自己監査と検証 (Audit & Verify)**: `AUDIT.md` に基づき「変数名は適切か？ ズルしてないか？」を自己批判し、`check_harness.sh` の機械的合格を証明する。
6. **同期と完了 (Sync)**: `QUALITY_SCORE.md` を更新し、学んだ教訓を `KNOWLEDGE.md` に残して次のタスクへ進む。

## 💡 なぜ "aidle" なのか？

AI駆動開発において、コードの質は「与えられた文脈」の質で決まります。
aidle は、人間が手動で行っていた「文脈の整理」と「環境のガードレール設置」を自動化。
プロジェクト開始1分後から、AIがあなたの専属プロデューサーとしてフルパフォーマンスを発揮できる状態を作ります。

> Made for the next generation of developer stars. 🌟🍛
