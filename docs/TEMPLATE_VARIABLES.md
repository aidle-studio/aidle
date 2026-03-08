# テンプレート可変項目一覧 (Template Variables)

本ドキュメントは、`aidle` が生成する `default` テンプレート内に存在する可変項目（プレースホルダー）を整理したものです。
現在、これらの項目は `[TODO: ...]` という静的な文字列で埋め込まれていますが、将来的なテンプレートエンジン（TeraやMiniJinjaなど）への移行、および MS9 / MS10 での対話型入力・言語別テンプレート展開を見据え、どのような変数が存在するかをここで定義します。

## 1. プロジェクト基本情報 (Project Meta)
主に `README.md` や `docs/SPEC.md` の冒頭で使用される、プロジェクトの根幹に関わる情報です。

| 変数名（仮） | 対象ファイル | 説明 |
|---|---|---|
| `project.name` | `README.md` | プロジェクトの名前 |
| `project.description_short` | `README.md` | プロジェクトのキャッチコピー・短い説明 |
| `project.description_long` | `README.md` | プロジェクトの詳細な説明・目的 |
| `project.features` | `README.md` | 主な機能のリスト |
| `project.license` | `README.md` | ライセンス情報 |

## 2. 環境・技術スタック (Environment & Stack)
プロジェクトを動かすために必要な環境や、採用している技術に関する情報です。将来の言語別テンプレート（MS10）で動的に注入されることを想定しています。

| 変数名（仮） | 対象ファイル | 説明 |
|---|---|---|
| `env.prerequisites` | `README.md` | 実行に必要な環境（Node.js, Rustのバージョンなど） |
| `env.setup_commands` | `README.md` | インストールやセットアップに必要なコマンド |
| `env.run_commands` | `README.md` | アプリケーションの起動コマンド |
| `stack.language` | `docs/SPEC.md` | 使用言語 |
| `stack.framework` | `docs/SPEC.md` | 使用フレームワーク |
| `stack.database` | `docs/SPEC.md` | 使用データベース |

## 3. 開発・品質ゲートツール (Development Tools & Quality Gates)
TDDやCI/CDを回すために必要なコマンド群です。プロジェクトごとに異なるツールが使われるため、利用者が明示的に設定する必要があります。

| 変数名（仮） | 対象ファイル | 説明 |
|---|---|---|
| `tools.test_runner` | `docs/RULES.md`, `docs/TEST_PLAN.md` | テスト実行コマンド（例: `cargo nextest run`, `npm test`） |
| `tools.test_runner_specific` | `docs/TEST_PLAN.md` | 特定のテストのみを実行するコマンド例 |
| `tools.linter` | `docs/RULES.md` | Linter/Formatterの実行コマンド（例: `clippy`, `eslint`） |
| `quality.coverage_target` | `docs/RULES.md` | 目標とするテストカバレッジ（例: 全体80%以上） |
| `quality.ci_conditions` | `docs/RULES.md` | CIでの品質ゲートの実行条件 |

## 4. プロジェクト固有のルールと状態 (Context & Rules)
プロジェクトの進行に伴って変化する状態や、そのプロジェクト特有のルールです。これらは初期生成時ではなく、AIエージェントや開発者が運用中に更新していく性質のものです。

| 変数名（仮） | 対象ファイル | 説明 |
|---|---|---|
| `context.current_phase` | `docs/AGENT_CONTEXT.md` | 現在のフェーズ |
| `context.immediate_goal` | `docs/AGENT_CONTEXT.md` | 直近のゴール |
| `context.next_actions` | `docs/AGENT_CONTEXT.md` | AIに期待する次のアクションリスト |
| `rules.custom_rules` | `docs/AGENT_CONTEXT.md` | プロジェクト固有の強調したいルール |

## 5. 仕様と計画 (Specs & Plans)
機能要件やテスト計画など、プロジェクトごとに完全に中身が異なる項目です。これらはテンプレートとしては枠のみを提供します。

| 変数名（仮） | 対象ファイル | 説明 |
|---|---|---|
| `spec.target_users` | `docs/SPEC.md` | 対象ユーザー層 |
| `spec.in_scope` | `docs/SPEC.md` | 実装スコープ内 |
| `spec.out_scope` | `docs/SPEC.md` | 実装スコープ外 |
| `spec.requirements` | `docs/SPEC.md` | REQ-* と AC-* のリスト |
| `spec.nfrs` | `docs/SPEC.md` | 非機能要件（性能、互換性など） |
| `plan.ac_tc_mapping` | `docs/TEST_PLAN.md` | ACとTCのトレーサビリティ紐付け |
| `plan.test_cases` | `docs/TEST_PLAN.md` | テストケース一覧表の中身 |
| `plan.implementation_order`| `docs/TEST_PLAN.md` | 実装順序の計画 |

## 6. マイルストーン管理 (Milestones & ADRs)
TODO管理や技術的判断の記録枠です。

| 変数名（仮） | 対象ファイル | 説明 |
|---|---|---|
| `todo.phase_goal` | `docs/TODO.md` | フェーズの目的 |
| `todo.ms_name` | `docs/TODO.md` | マイルストーン名 |
| `todo.ms_conditions` | `docs/TODO.md` | マイルストーンの達成条件 |
| `todo.ms_tasks` | `docs/TODO.md` | マイルストーンの具体タスク |
| `knowledge.adr_title` | `docs/KNOWLEDGE.md` | ADRのタイトル |
| `knowledge.adr_context` | `docs/KNOWLEDGE.md` | ADRの背景、決定事項、結果など |
