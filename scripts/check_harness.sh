#!/bin/bash

# check_harness.sh (Rust Optimized)
# aidle プロジェクト自体の品質を保証するための統合検証スクリプトです。

set -e # エラーが発生したら即座に終了

# --- 🎨 カラー設定 ---
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}--- 🛠️ aidle Harness Verification Starting ---${NC}"

# --- ⚙️ 設定 & 引数解析 ---
TIER=${1:-1} # aidle 自体は常に Tier 1
SCOPE=${2:-"all"} # all, review, sync, lint, test

echo -e "Target Tier: ${TIER}"
echo -e "Verification Scope: ${SCOPE}"

# --- 🛡️ 1. 環境チェック (Self-Check) ---
check_tool() {
    if ! command -v $1 &> /dev/null; then
        echo -e "${RED}[ERROR] Tool not found: $1${NC}"
        exit 1
    fi
}

check_tool "cargo"

# --- 📝 2. 自己レビュー検証 (Review Check) ---
run_review_check() {
    echo -e "\n${YELLOW}[PHASE 0] Self-Review Verification${NC}"
    REVIEW_FILE="docs/AGENT_CONTEXT.md"
    if grep -q "Self-Review: \[x\]" "$REVIEW_FILE"; then
        echo -e "${GREEN}SUCCESS: Self-Review record found in $REVIEW_FILE.${NC}"
    else
        echo -e "${RED}[ERROR] Self-Review record not found!${NC}"
        echo -e "Please conduct self-review and add 'Self-Review: [x]' to $REVIEW_FILE. 🍛"
        exit 1
    fi
}

# --- 📝 3. コンテキスト同期検証 (Context Sync Check) ---
run_sync_check() {
    echo -e "\n${YELLOW}[PHASE 0.5] Context Sync Verification${NC}"
    REQUIRED_DOCS=("docs/AGENT_CONTEXT.md" "docs/TODO.md" "docs/KNOWLEDGE.md" "docs/QUALITY_SCORE.md")
    for doc in "${REQUIRED_DOCS[@]}"; do
        if [ ! -f "$doc" ]; then
            echo -e "${RED}[ERROR] Required document $doc not found!${NC}"
            exit 1
        fi
    done
    echo -e "${GREEN}SUCCESS: Required context files are present.${NC}"
}

# --- 🔍 4. 静的解析 (Lint & Format) ---
run_lint() {
    echo -e "\n${YELLOW}[PHASE 1] Lint & Format Check${NC}"
    echo "Running cargo fmt..."
    cargo fmt -- --check
    echo "Running cargo clippy..."
    cargo clippy -- -D warnings
    echo -e "${GREEN}SUCCESS: Lint & Format are clean.${NC}"
}

# --- 🧪 5. テスト実行 (UT/IT) ---
run_tests() {
    echo -e "\n${YELLOW}[PHASE 2] Multi-Layered Testing${NC}"
    
    echo "Running All Tests (UT + IT)..."
    cargo test
    
    echo -e "${GREEN}SUCCESS: All tests passed.${NC}"
}

# --- 🚀 実行制御 ---
case $SCOPE in
    "review")   run_review_check ;;
    "sync")     run_sync_check ;;
    "lint")     run_lint ;;
    "test")     run_tests ;;
    "all")      run_review_check; run_sync_check; run_lint; run_tests ;;
    *)          echo "Unknown scope: $SCOPE"; exit 1 ;;
esac

echo -e "\n${GREEN}--- ✅ aidle Harness Verification SUCCESS ---${NC}"
exit 0
