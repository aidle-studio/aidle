<!--
[AI Agent Instructions for Architecture]
⚠️ 警告: このファイルは Core Context であり、すべてのセッションで読み込まれます。
コンテキスト肥大化を防ぐため、以下の更新ルールを厳守してください：
1. **概要に留める**: 本ファイルには「システムの全体像」と「主要な技術スタック」のみを簡潔に記載すること。
2. **詳細の追い出し**: 特定のモジュールや機能の詳細な内部設計は、本ファイルではなく `docs/design-docs/*.md` に記述し、リンクのみを貼ること。
3. **歴史の分離**: 技術選定の「理由（背景）」は、本ファイルではなく `docs/adr/*.md` に記述すること。
-->

# ARCHITECTURE.md (基本仕様書)

本ドキュメントは、システムの全体構造と技術スタックを示す「不変の地図」です。
要件（What）の全体像については、製品仕様書 ➡️ **[PRD](PRD.md)** を参照してください。

## 1. 🏗️ システム全体図 (System Overview)
- [TODO: システム全体のアーキテクチャ（例: フロントエンドとバックエンドの構成等）を数行で記述]
- 💡 全体的な設計思想（Core Beliefs）は ➡️ `docs/design-docs/core-beliefs.md` を参照。

## 2. 📚 技術スタック (Tech Stack)
※各技術の選定理由は `docs/adr/` 配下の記録を参照すること。
※各技術やライブラリの**「プロジェクト固有のベストプラクティス（AI向けカンペ）」**が存在する場合は、実装前に必ず ➡️ `docs/references/` 配下を参照すること。

- **Frontend**: [TODO: 例: React (Next.js)] ➡️ `docs/references/[TODO].md`
- **Backend**: [TODO: 例: Rust (Axum)]
- **Database**: [TODO: 例: PostgreSQL]

## 3. 📂 主要コンポーネントとディレクトリ構造

<!-- 
[AI Agent Instructions for Updating this Table]
- 追加する粒度は「主要なディレクトリ（モジュール）単位」とし、個別のファイルは記載しないこと。
- 「役割 (Role)」の列は、長くても2文（50文字程度）以内に収めること。
- 新しいコンポーネントを追加した場合は、必ず「詳細設計へのリンク」に `docs/design-docs/` 配下のパスを指定すること（ファイルが未作成でも可）。

✅ Ideal Example (理想的な追加の例):
| コンポーネント (パス) | 役割 (Role) | 詳細設計へのリンク |
| :--- | :--- | :--- |
| `src/api/` | 外部向けREST APIのエンドポイント層。ルーティングと入出力バリデーションを担当。 | ➡️ `docs/design-docs/api-design.md` |
| `src/core/` | ビジネスロジックとドメインモデル。外部依存を持たない純粋なロジック層。 | ➡️ `docs/design-docs/domain-model.md` |
| `src/infra/` | DB接続や外部APIクライアントの実装層。 | ➡️ `docs/design-docs/infra-layer.md` |
-->

| コンポーネント (パス) | 役割 (Role) | 詳細設計へのリンク |
| :--- | :--- | :--- |
| `src/` | [TODO: アプリケーションのメインソースコード] | ➡️ `docs/design-docs/main-structure.md` |
| `docs/` | エージェントおよび人間向けのドキュメント群。ルールや仕様を管理する。 | - |

## 4. 🔗 共通の技術ルールへの導線
実装の際に守るべきプロジェクト共通の法律（ルール）は以下を参照すること。

- ➡️ **エラー処理とセキュリティ制約**: `docs/RELIABILITY.md`
