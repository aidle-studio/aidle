#!/bin/bash

# check_harness.sh
# AI駆動開発 (AIDD) における「品質の番人」となる統合検証スクリプトです。
# AIエージェントは「完了 (Done)」を宣言する前に必ずこのスクリプトを実行し、
# 全ての検証（Harness）が SUCCESS であることを確認してください。

set -e # エラーが発生したら即座に終了

# --- 🎨 カラー設定 ---
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}--- 🛠️ Harness Verification Starting ---${NC}"

# --- ⚙️ 設定 & 引数解析 ---
TIER=${1:-2} # デフォルトは Tier 2
SCOPE=${2:-"all"} # all, review, lint, test, coverage

echo -e "Target Tier: ${TIER}"
echo -e "Verification Scope: ${SCOPE}"

# --- 🛡️ 1. 環境チェック (Self-Check) ---
check_tool() {
    if ! command -v $1 &> /dev/null; then
        echo -e "${RED}[ERROR] Tool not found: $1${NC}"
        echo -e "AIエージェントは自律的に環境をセットアップしてください。"
        exit 1
    fi
}

# --- 📝 2. 自己レビュー検証 (Review Check) ---
run_review_check() {
    echo -e "\n${YELLOW}[PHASE 0] Self-Review Verification${NC}"
    
    # 自己レビューが記録されているか確認する例（プロジェクトに合わせて調整）
    # ここでは docs/AGENT_CONTEXT.md 内に "Self-Review: [x]" という記述があるかチェックします。
    REVIEW_FILE="docs/AGENT_CONTEXT.md"
    
    if [ ! -f "$REVIEW_FILE" ]; then
        echo -e "${RED}[ERROR] $REVIEW_FILE not found!${NC}"
        exit 1
    fi

    if grep -q "Self-Review: \[x\]" "$REVIEW_FILE"; then
        echo -e "${GREEN}SUCCESS: Self-Review record found in $REVIEW_FILE.${NC}"
    else
        echo -e "${RED}[ERROR] Self-Review record not found!${NC}"
        echo -e "AIエージェントは 'docs/RULES.md' のチェックリストに従い、"
        echo -e "自己レビューを実施して $REVIEW_FILE に 'Self-Review: [x]' と追記してください。🍛"
        exit 1
    fi
}

# --- 📝 0.5. コンテキスト同期検証 (Context Sync Check) ---
run_sync_check() {
    echo -e "\n${YELLOW}[PHASE 0.5] Context Sync Verification${NC}"

    # 必須ドキュメントの存在チェック
    REQUIRED_DOCS=("docs/AGENT_CONTEXT.md" "docs/TODO.md" "docs/KNOWLEDGE.md")
    for doc in "${REQUIRED_DOCS[@]}"; do
        if [ ! -f "$doc" ]; then
            echo -e "${RED}[ERROR] Required document $doc not found!${NC}"
            exit 1
        fi
    done

    # TODO.md に未完了のタスクがないか、または適切に更新されているかの簡易チェック例
    # （実際のプロジェクト運用に合わせて厳密化すること）
    echo -e "${GREEN}SUCCESS: Required context files are present.${NC}"
    echo -e "AIエージェントは、これらのファイルがアトミックに同期されていることを保証してください。"
}

# --- 🔍 1. 静的解析 (Lint & Format) ---
run_lint() {
    echo -e "\n${YELLOW}[PHASE 1] Lint & Format Check${NC}"
    # 例: Rust の場合
    # cargo fmt -- --check || exit 1
    # cargo clippy -- -D warnings || exit 1
    echo -e "${GREEN}SUCCESS: Lint & Format are clean.${NC}"
}

# --- 🧪 4. テスト実行 (UT/IT/E2E) ---
run_tests() {
    echo -e "\n${YELLOW}[PHASE 2] Multi-Layered Testing${NC}"
    
    # Unit Tests (UT) - Tier 1 は全パターン必須
    echo -e "Running Unit Tests..."
    # cargo nextest run --lib || exit 1
    
    # Integration Tests (IT) - Tier 1/2 で必須
    if [ "$TIER" -le 2 ]; then
        echo -e "Running Integration Tests..."
        # cargo nextest run --test '*' || exit 1
    fi
    
    # End-to-End Tests (E2E) - Tier 1 でのみ必須（重いため）
    if [ "$TIER" -eq 1 ]; then
        echo -e "Running E2E Scenarios..."
        # ./scripts/run_e2e.sh || exit 1
    fi
    
    echo -e "${GREEN}SUCCESS: All tests passed for Tier ${TIER}.${NC}"
}

# --- 📊 5. カバレッジ & 密度チェック (Coverage & Density) ---
run_coverage() {
    echo -e "\n${YELLOW}[PHASE 3] Coverage & Density Check${NC}"
    # 例: llvm-cov を使用した差分カバレッジチェック
    # cargo llvm-cov --fail-under-lines 80 || exit 1
    
    # TODO: TEST_PLAN.md との整合性（密度）をチェックするカスタムロジック
    echo -e "${GREEN}SUCCESS: Coverage targets met.${NC}"
}

# --- 🚀 実行制御 ---
case $SCOPE in
    "review")   run_review_check ;;
    "sync")     run_sync_check ;;
    "lint")     run_lint ;;
    "test")     run_tests ;;
    "coverage") run_coverage ;;
    "all")      run_review_check; run_sync_check; run_lint; run_tests; run_coverage ;;
    *)          echo "Unknown scope: $SCOPE"; exit 1 ;;
esac

echo -e "\n${GREEN}--- ✅ Harness Verification SUCCESS ---${NC}"
exit 0
