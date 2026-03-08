# SPEC.md

このファイルは、プロジェクトの機能要件と仕様を管理する正本（Source of Truth）です。
AIエージェントは、実装を開始する前に必ずこのドキュメントで「要求（REQ）」と「受け入れ基準（AC）」を定義し、人間の開発者と合意してください。

## 1. 目的 (Purpose)
[TODO: プロジェクトが解決する課題や、何を作るためのものかを記載する]

## 2. 対象ユーザー (Target Users)
- [TODO: 想定されるユーザー層を箇条書きで記載する]

## 3. スコープ (Scope)
### 3.1 In Scope (実装範囲内)
- [TODO: このプロジェクトで実装する機能を箇条書きで記載する]

### 3.2 Out of Scope (実装範囲外)
- [TODO: あえて実装しないこと、やらないことを箇条書きで記載する]

## 4. 機能要件 (Functional Requirements)
[TODO: プロジェクトの主要な機能を REQ-* として定義する]

### REQ-001: [機能名]
- **概要**: [機能の概要を記載]
- **詳細**: [機能の詳細な振る舞いを記載]

## 5. 受け入れ基準 (Acceptance Criteria: AC)
[TODO: 各機能要件（REQ）が「満たされた」と判断できる具体的な基準を AC-* として定義する。ここがテスト計画（TEST_PLAN.md）の入力となる。]

- `AC-001`: [具体的な完了条件を記載] (対応: `REQ-001`)
- `AC-002`: [具体的な完了条件を記載] (対応: `REQ-001`)

## 6. 非機能要件 (Non-Functional Requirements)
- NFR-1: 性能 (Performance) - [TODO: レスポンス要件などを記載]
- NFR-2: 互換性 (Compatibility) - [TODO: サポート対象OSやブラウザなどを記載]
- NFR-3: 拡張性 (Maintainability) - [TODO: コードの保守性に関する要件を記載]

## 7. 技術選定 (Technology Stack)
- 言語: [TODO]
- フレームワーク: [TODO]
- データベース: [TODO]
- その他主要ライブラリ: [TODO]
