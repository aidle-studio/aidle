# CONFIG.md

## 1. 目的
`aidle.toml` の初期スキーマを定義し、CLI実行時の設定解釈を明確化する。

## 2. ファイル名と配置
- ファイル名: `aidle.toml`
- 配置: プロジェクトルート

## 3. スキーマ（v0.1）

### `[project]`
- `name` (string, optional): 生成対象プロジェクト名。CLI引数が優先される。
- `output` (string, optional): 出力先パス。CLI引数/オプションが優先される。

### `[template]`
- `name` (string, optional, default=`"default"`): テンプレート名（`default`）。

### `[agent]`
- `format` (string, optional, default=`"agents-md"`): エージェント規約形式。

### `[execution]`
- `non_interactive` (bool, optional, default=`false`): 非対話モード。
- `force` (bool, optional, default=`false`): 既存ファイル上書き許可。
- `dry_run` (bool, optional, default=`false`): ファイル未作成で予定のみ表示。
- `verbose` (bool, optional, default=`false`): 詳細ログ表示。
- `json` (bool, optional, default=`false`): 構造化結果出力。

### `[adapters]`
- `enabled` (bool, optional, default=`false`): GC-2適応レイヤの生成有効化。

### `[stats]`
- `output` (string, optional): 実行統計JSONの保存先ファイルパス。

## 4. 優先順位
1. CLI引数/オプション
2. `aidle.toml`
3. 既定値

## 5. サンプル
```toml
[project]
name = "my-app"
output = "./my-app"

[template]
name = "default"

[agent]
format = "agents-md"

[execution]
non_interactive = true
force = false
dry_run = false
verbose = false
json = true

[adapters]
enabled = true

[stats]
output = "logs/aidle-stats.json"
```
