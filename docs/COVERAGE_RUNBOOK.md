# COVERAGE_RUNBOOK.md

## 1. 目的
- カバレッジ品質ゲートの計測・判定・例外運用を、再現可能な手順として定義する。

## 2. 基準
- 全体行カバレッジ: 80%以上
- 変更対象の差分カバレッジ: 90%以上（原則必須）

## 3. ローカル計測手順
1. テスト実行  
   `cargo nextest run`
2. カバレッジ計測  
   `cargo llvm-cov nextest --workspace --lcov --output-path coverage.lcov`
3. レポート確認  
   `coverage.lcov` を確認し、全体80%以上を判定する。
4. 閾値判定  
   `BASE_REF=HEAD~1 MIN_TOTAL=80 MIN_DIFF=90 ./scripts/check_coverage_gate.sh coverage.lcov`

## 4. 差分カバレッジ判定（運用方針）
- PRで変更したモジュール（`src/**/*.rs`）を対象に、90%以上を確認する。
- 判定は `scripts/check_coverage_gate.sh` を正本とし、LCOVから機械計算する。
- `BASE_REF` が解決できない場合や対象差分がない場合は `diff coverage: skipped` として扱う。

## 5. 未達時の扱い
- 原則は修正して基準達成後にマージする。
- 例外運用を行う場合は `docs/COVERAGE_EXCEPTION_TEMPLATE.md` を使用する。
- 例外には解消期限を必ず設定し、期限超過時は最優先で解消する。

## 6. 記録ルール
- 例外運用を実施した場合、PRとコミットメッセージの両方で参照可能にする。
- 合意者と合意日を明記する。

## 7. CI運用
- CI品質ゲートは `.github/workflows/coverage-gate.yml` で実行する。
- `pull_request` では `origin/<base_branch>` を `BASE_REF` として差分判定する。
- `push` では `HEAD~1` を `BASE_REF` とする。
