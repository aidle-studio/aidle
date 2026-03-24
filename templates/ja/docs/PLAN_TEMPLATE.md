<!--
[AI Agent Instructions for Planning]
このプランを作成する前に、以下の手順で事前調査 (Research) と思考 (Chain of Thought) を実施せよ：
1. **要件と設計の確認**: `docs/TODO.md` で今のタスクを確認し、すでに対象の機能仕様書 (`docs/features/*.md`) と詳細設計書 (`docs/design-docs/*.md`) が作成・合意されていることを確認する。※もし存在しない場合は、プラン作成を中断し、先にそれらを作成すること。
2. **品質の定義**: `docs/TEST_PLAN.md` に基づき対象の Tier を決定し、正常系よりも先に「どう壊れるか（異常系・境界値）」の観点からテストマトリクスを埋める。
これらを完了した後、以下の各項目を埋めて人間に提示し、合意（Agreement Gate）を得ること。
-->

# Plan: [Milestone/Task Name]

## Plan Meta
- Milestone ID:
- Date:
- Status: Draft / Agreed / In Progress / Done

## Context & Goal
- **Why (目的)**: [TODO: なぜこのマイルストーンが必要か、どんな価値・課題解決をもたらすか]
- **What (目標)**: [TODO: このマイルストーンで達成する具体的な状態を記載する]
- 💡 仕様の詳細は **`PRD.md`** または各機能仕様書を参照すること。

## Scope
- **In Scope (やること)**:
  - [TODO]
- **Out of Scope (やらないこと)**:
  - [TODO: 今回のスコープ外として意図的に見送るものを明記し、肥大化を防ぐ]

## Quality Goals (品質目標)
- [ ] **対象Tier**: [Tier 1 / Tier 2 / Tier 3] (※`docs/TEST_PLAN.md` 参照)
- [ ] **品質特性**: [TODO: 機能適合性, 信頼性, 保守性 など、最も重視するISO/IEC 25010の特性]
- [ ] **具体的状態**: [TODO: 「ただ動く」ではなく、客観的に「よい状態」を定義する]

## Test Perspectives (テスト観点)
💡 **`docs/TEST_PLAN.md`** の密度目標と異常系必須ルールに従うこと。

### UT: 境界値・異常値マトリクス
| 対象 (関数/状態) | Valid (正常) | Boundary (境界/極端) | Empty/Zero (空/ゼロ) | Invalid (不正/型違い) |
| :--- | :--- | :--- | :--- | :--- |
| [TODO] | [TODO] | [TODO] | [TODO] | [TODO] |

### IT: FMEA (故障モード) マトリクス
| 依存コンポーネント | Timeout (遅延) | Disconnect (切断) | 5xx Error (異常応答) | Malformed (不正データ) |
| :--- | :--- | :--- | :--- | :--- |
| [TODO] | [TODO] | [TODO] | [TODO] | [TODO] |

## Tasks & Reliability (実装タスクと信頼性設計)
💡 **`docs/RELIABILITY.md`** に従い、本タスクにおけるエラーハンドリングと保護戦略を先に定義せよ。

- **Fail-fast (早期失敗)**: [TODO: どんな不正入力や異常状態をバリデーションで弾くか？]
- **Timeout & Retry**: [TODO: 外部通信や重い処理がある場合、タイムアウト時間と再試行の戦略をどうするか？]
- **Fallback (代替手段)**: [TODO: 依存先が完全にダウンした場合、システムはどう振る舞うか（UXへの影響をどう最小化するか）？]

💡 上記の戦略を踏まえ、タスクを「TDDサイクル（Red->Green->Refactor）」を回せる最小単位（テスト作成と実装のセット）で分割すること。
💡 実装時は **`ARCHITECTURE.md`** (構造) を遵守すること。
- [ ] T1: [TODO: テストと実装を含む最小の作業単位]
- [ ] T2: [TODO: ...]

## Definition of Done (DoD)
💡 以下の完了条件をすべて満たし、チェック `[x]` を入れること。
- [ ] **(Tasks)**: 上記の Tasks がすべて完了している。
- [ ] **(Audit)**: `docs/RULES.md` の監査基準（品質目標・テスト密度）をクリアしたと確信している。
- [ ] **(Harness)**: `scripts/check_harness.sh` が 100% Green である。
- [ ] **(Sync)**: 要件・設計ドキュメント（PRD, features, ARCHITECTURE, design-docs）および状態管理ドキュメント（TODO, AGENT_CONTEXT）を最新化した。
- [ ] **(GC)**: 本プランを `docs/exec-plans/completed/` に移動し、得られた教訓を `docs/KNOWLEDGE.md` に追記した。（※このチェックはプラン移動直前に入れる）
- [ ] **(Custom)**: [TODO: このプラン特有の完了条件があれば記載。特になければ削除]

## Execution Log
- [TODO: YYYY-MM-DD HH:MM: 実行した内容のサマリーや、発生したエラーとその解決策を追記していく]
